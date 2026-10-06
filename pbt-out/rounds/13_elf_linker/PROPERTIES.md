# PROPERTIES.md — round 13 (elf + linker_common + stack_layout)

## p1_string_table_interning
- Tier: 2
- Doc contract: src/backend/elf/string_table.rs — ELF strtab semantics (dedup, NUL-terminated, offset 0 empty)
- Formal: ∀ sequences of adds: add(s) idempotent per string; offsets distinct across distinct strings; bytes at offset == s+'\0'; empty → 0.
- Test file: src/backend/elf/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::elf::string_table::StringTable::add
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [strings]
  domain: { strings: list(any_ascii, 0..=20) }
  relation:
    op: holds
    expr: "interning invariants hold (idempotent, distinct, content-addressed, NUL-terminated)"
generators:
  strings: { gen: list, elem: { gen: string, pattern: "[a-zA-Z0-9_.]{1,12}" }, minLen: 0, maxLen: 20 }
evidence: ELF strtab layout convention (offset 0 = empty string, entries NUL-terminated)
```

## p2_parse_string_literal_matches_renderer
- Re-verified: one renderer defect fixed en route (\0 before a digit is ambiguous — C octal escapes consume up to 3 digits greedily; renderer now uses fixed-width \000). After the fix: 512 cases green.
- Tier: 2
- Doc contract: src/backend/elf/parse_string.rs:5-13 (canonical C/GNU escape list)
- Formal: ∀ byte sequences B (printable + documented escapes): parse_string_literal(quote(render(B))) == Ok(B).
- Test file: src/backend/elf/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::elf::parse_string_literal
oracle: differential
predicate:
  quantifier: forall
  vars: [bytes]
  domain: { bytes: list(escape_forms, 0..=16) }
  relation:
    op: eq
    lhs: parse_string_literal(render(bytes))
    rhs: Ok(eval(bytes))
generators:
  bytes: { gen: list, elem: { gen: oneof, variants: [ { gen: int, min: 32, max: 126, type: u8 }, { gen: const, val: escape_form } ] }, minLen: 0, maxLen: 16 }
evidence: parse_string.rs doc "Supports the standard C/GNU assembler escape sequences"
```

## p3_numeric_label_laws
- Tier: 2
- Doc contract: src/backend/elf/numeric_labels.rs:23-24 "Each forward reference Nf refers to the next definition of N, and each backward reference Nb refers to the most recent definition of N"
- Formal: ∀ n ∈ digit strings: is_numeric_label(n); parse_numeric_ref(n+"f") == Some((n,true)); parse_numeric_ref(n+"b") == Some((n,false)); non-digits → None.
- Test file: src/backend/elf/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::elf::numeric_labels::parse_numeric_ref
oracle: reference
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: digit_strings }
  relation:
    op: holds
    expr: "parse_numeric_ref(n+\"f\") == Some((n,true)) && parse_numeric_ref(n+\"b\") == Some((n,false)) && parse_numeric_ref(n+\"x\").is_none() && !is_numeric_label(n+\"f\")"
generators:
  n: { gen: string, pattern: "[0-9]{1,4}" }
evidence: numeric_labels.rs doc (GNU numeric label semantics)
```

## p4_multi_target_elf_structure
- Re-verified: harness fixes en route (readelf column padding — match on names, not "Key: Value" alignment). After the fix: all four targets produce ET_REL objects with the correct e_machine.
- Tier: 3
- Doc contract: ELF e_machine per target (EM_X86_64/EM_AARCH64/EM_RISCV/EM_386), e_type ET_REL for -c
- Formal: ∀ target ∈ {x86-64, arm, riscv, i686}: compiling a fixed C file with -c yields an ELF whose e_machine matches the target and e_type == ET_REL.
- Test file: src/backend/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::{x86,arm,riscv,i686} object writers (via Driver -c)
oracle: reference
predicate:
  quantifier: forall
  vars: [target]
  domain: { target: {x86_64, aarch64, riscv64, i686} }
  relation:
    op: holds
    expr: "readelf_machine(compile_c(target, -c)) == expected_machine(target) && e_type == ET_REL"
generators:
  target: { gen: int, min: 0, max: 3, type: u8 }
evidence: ELF spec e_machine values; four-backend architecture list in x86/README pipeline
```
