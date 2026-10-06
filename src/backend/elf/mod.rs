//! Shared ELF types, constants, and utilities used by all assembler and linker backends.
//!
//! Split into focused submodules:
//!
//! - `constants`: ELF format constants (section types, flags, symbol bindings, etc.)
//! - `string_table`: `StringTable` for building ELF string tables
//! - `io`: Binary read/write helpers and ELF structure writers
//! - `archive`: Archive (.a) and linker script parsing
//! - `linker_symbols`: Linker-defined symbols and section helpers
//! - `section_flags`: Section flags parsing for `.section` directives
//! - `parse_string`: String literal parser for assembler directives
//! - `object_writer`: Shared relocatable ELF object (.o) writer
//! - `numeric_labels`: Numeric local label resolution (x86/i686)
//! - `symbol_table`: Shared symbol table builder for assembler backends
//! - `writer_base`: `ElfWriterBase` for ARM/RISC-V assembler backends

mod constants;
mod string_table;
mod io;
mod archive;
mod linker_symbols;
mod section_flags;
pub(crate) mod parse_string;
mod object_writer;
mod numeric_labels;
mod symbol_table;
mod writer_base;

// Re-export everything at the elf:: level so existing `use crate::backend::elf::{...}`
// imports continue to work without any changes.

// constants
#[allow(unused_imports)]
pub use constants::*;

// string_table
#[allow(unused_imports)]
pub use string_table::StringTable;

// io
#[allow(unused_imports)]
pub use io::*;

// archive
#[allow(unused_imports)]
pub use archive::{
    is_thin_archive, parse_thin_archive_members, parse_archive_members,
    LinkerScriptEntry, parse_linker_script, parse_linker_script_entries,
};

// linker_symbols
#[allow(unused_imports)]
pub use linker_symbols::{
    LinkerSymbolAddresses, LinkerDefinedSym, get_standard_linker_symbols,
    section_index, default_section_flags,
};

// section_flags
pub use section_flags::parse_section_flags;

// parse_string
pub use parse_string::parse_string_literal;

// object_writer
pub use object_writer::{
    ElfConfig, ObjSection, ObjReloc,
    write_relocatable_object,
};

// numeric_labels
#[allow(unused_imports)]
pub use numeric_labels::{
    is_numeric_label, parse_numeric_ref, resolve_numeric_labels,
    resolve_numeric_name, resolve_numeric_refs_in_expr,
};

// symbol_table
pub use symbol_table::{ObjSymbol, SymbolTableInput, build_elf_symbol_table};

// writer_base
pub use writer_base::ElfWriterBase;

#[cfg(test)]
mod pbt_tests {
    use super::*;
    use proptest::prelude::*;

    /// P1: StringTable interning invariants (ELF strtab semantics).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p1_string_table_interning(
            strings in prop::collection::vec("[a-zA-Z0-9_.]{1,12}", 0..21),
        ) {
            let mut st = string_table::StringTable::new();
            let mut seen: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
            for s in &strings {
                let off = st.add(s);
                if s.is_empty() {
                    prop_assert_eq!(off, 0, "empty string must map to offset 0");
                    continue;
                }
                if let Some(&prev) = seen.get(s) {
                    prop_assert_eq!(off, prev, "add not idempotent for {:?}", s);
                } else {
                    for (other, ooff) in seen.iter() {
                        if *other != *s {
                            prop_assert_ne!(off, *ooff, "distinct strings share an offset");
                        }
                    }
                    seen.insert(s.clone(), off);
                }
                // content addressing: bytes at offset == s + NUL
                let bytes = st.as_bytes();
                let end = off as usize + s.len();
                prop_assert!(end < bytes.len(), "offset {:?} out of range", off);
                prop_assert_eq!(&bytes[off as usize..end], s.as_bytes());
                prop_assert_eq!(bytes[end], 0, "entry not NUL-terminated");
            }
        }
    }

    /// P2: parse_string_literal == independent renderer over the documented
    /// escape forms (printables + \n \t \r \\ \" \a \b \f \v + octal + hex).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p2_parse_string_literal_matches_renderer(
            parts in prop::collection::vec(
                prop_oneof![
                    6 => (32u8..127).prop_map(|b| (b, false)),
                    1 => proptest::sample::select(vec![(b'\n' as u8, true), (b'\t' as u8, true), (b'\r' as u8, true), (b'\\' as u8, true), (b'"' as u8, true), (7u8, true), (8u8, true), (12u8, true), (11u8, true), (0u8, true)]),
                ], 0..17),
        ) {
            // render: printable non-quote/backslash chars verbatim; special
            // bytes via the documented escapes; NUL as \0
            let mut text = String::from("\"");
            let mut want: Vec<u8> = Vec::new();
            for &(b, escape) in &parts {
                if escape {
                    let esc = match b { b'\n' => "\\n", b'\t' => "\\t", b'\r' => "\\r", b'\\' => "\\\\", b'"' => "\\\"", 7 => "\\a", 8 => "\\b", 12 => "\\f", 11 => "\\v", 0 => "\\000", _ => unreachable!() }; // fixed-width octal: unambiguous before digits
                    text.push_str(esc);
                } else {
                    let c = b as char;
                    prop_assume!(c != '"' && c != '\\');
                    text.push(c);
                }
                want.push(b);
            }
            text.push('"');
            let got = parse_string::parse_string_literal(&text)
                .unwrap_or_else(|e| panic!("parse failed for {}: {}", text, e));
            prop_assert_eq!(got, want, "literal: {}", text);
        }
    }

    /// P3: numeric label helper laws (GNU 1f/1b semantics).
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p3_numeric_label_laws(n in "[0-9]{1,4}") {
            prop_assert!(numeric_labels::is_numeric_label(&n), "{:?} should be numeric", n);
            prop_assert!(!numeric_labels::is_numeric_label(&format!("{}f", n)), "{}f should not be a bare numeric label", n);
            let f = format!("{}f", n);
            let b = format!("{}b", n);
            let x = format!("{}x", n);
            match numeric_labels::parse_numeric_ref(&f) {
                Some((num, true)) => prop_assert_eq!(num, n.as_str()),
                other => panic!("{}f → {:?}, expected Some((n,true))", n, other),
            }
            match numeric_labels::parse_numeric_ref(&b) {
                Some((num, false)) => prop_assert_eq!(num, n.as_str()),
                other => panic!("{}b → {:?}, expected Some((n,false))", n, other),
            }
            prop_assert!(numeric_labels::parse_numeric_ref(&x).is_none(), "{}x should not parse", n);
            prop_assert!(numeric_labels::parse_numeric_ref(n.as_str()).is_none(), "bare number should not parse as a ref");
        }
    }
}
