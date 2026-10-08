//! Shared linker infrastructure for all backends.
//!
//! Split into focused submodules:
//!
//! - `types`: ELF64 object file types (Elf64Section, Elf64Symbol, Elf64Rela, Elf64Object, DynSymbol)
//! - `parse_object`: ELF64 relocatable object file parser
//! - `parse_shared`: Shared library (.so) symbol parsing and SONAME extraction
//! - `section_map`: Input-to-output section name mapping
//! - `dynstr`: Dynamic string table builder for .dynstr emission
//! - `hash`: GNU and SysV ELF hash functions
//! - `symbols`: InputSection, OutputSection, GlobalSymbolOps trait, linker-defined symbols
//! - `merge`: Section merging and common symbol allocation
//! - `dynamic`: Dynamic symbol matching, library loading, and symbol registration
//! - `archive`: Archive loading and generic file dispatch
//! - `resolve_lib`: Library name resolution helper
//! - `write`: ELF64 binary emission helpers (section/program headers, alignment)
//! - `args`: Shared linker argument parsing
//! - `check`: Post-link undefined symbol checking
//! - `eh_frame`: .eh_frame_hdr builder for stack unwinding
//! - `gc_sections`: Garbage collection (`--gc-sections`) for ELF64 linkers
//!
//! This module extracts the duplicated linker code that was copied across x86,
//! ARM, RISC-V, and (partially) i686 backends. It provides:
//!
//! - **ELF64 object parser**: `parse_elf64_object()` replaces near-identical
//!   `parse_object()` functions in x86, ARM, and RISC-V linkers.
//! - **Shared library parser**: `parse_shared_library_symbols()` and `parse_soname()`
//!   for extracting dynamic symbols from .so files.
//! - **Dynamic symbol matching**: `match_shared_library_dynsyms()`,
//!   `load_shared_library_elf64()`, and `resolve_dynamic_symbols_elf64()` for
//!   matching undefined globals against shared library exports with WEAK alias
//!   detection and as-needed semantics.
//! - **Linker-defined symbols**: `LINKER_DEFINED_SYMBOLS` constant and
//!   `is_linker_defined_symbol()` for the superset of symbols the linker
//!   provides during layout (used by all 4 backends).
//! - **Archive loading**: `load_archive_members()` and `member_resolves_undefined()`
//!   for iterative archive resolution (the --start-group algorithm).
//! - **Section mapping**: `map_section_name()` for input-to-output section mapping.
//! - **DynStrTab**: Dynamic string table builder for dynamic linking.
//! - **GNU hash**: `build_gnu_hash()` for .gnu.hash section generation.
//! - **ELF64 writing helpers**: `write_elf64_shdr()`, `write_elf64_phdr()`,
//!   `write_elf64_phdr_at()`, `align_up_64()`, `pad_to()` for binary emission.
//! - **Argument parsing**: `parse_linker_args()` and `LinkerArgs` for shared
//!   `-Wl,` flag parsing across backends.
//! - **Undefined symbol checking**: `check_undefined_symbols_elf64()` for
//!   post-link validation via the `GlobalSymbolOps` trait.
//!
//! Each backend linker still handles its own:
//! - Architecture-specific relocation application
//! - PLT/GOT layout (different instruction sequences per arch)
//! - ELF header emission (different e_machine, base addresses)
//! - Dynamic linking specifics (version tables, etc.)

// ── Submodule declarations ──────────────────────────────────────────────

mod types;
mod parse_object;
mod parse_shared;
mod section_map;
mod dynstr;
mod hash;
mod symbols;
mod merge;
mod dynamic;
mod archive;
mod resolve_lib;
mod write;
mod args;
mod check;
mod eh_frame;
mod gc_sections;

// ── Re-exports ──────────────────────────────────────────────────────────
//
// Re-export all public items at the linker_common:: level so that external
// callers see no change from the previous flat-file layout.

// types.rs
pub use types::{Elf64Section, Elf64Symbol, Elf64Rela, Elf64Object, DynSymbol};

// parse_object.rs
pub use parse_object::parse_elf64_object;

// parse_shared.rs
pub use parse_shared::{parse_shared_library_symbols, parse_soname};

// dynstr.rs
pub use dynstr::DynStrTab;

// hash.rs
pub use hash::{gnu_hash, sysv_hash};

// symbols.rs
pub use symbols::{
    OutputSection, GlobalSymbolOps,
    is_linker_defined_symbol,
    is_valid_c_identifier_for_section, resolve_start_stop_symbols,
};

// merge.rs
pub use merge::{merge_sections_elf64, merge_sections_elf64_gc, allocate_common_symbols_elf64};

// dynamic.rs
pub use dynamic::{
    load_shared_library_elf64,
    resolve_dynamic_symbols_elf64, register_symbols_elf64,
};

// archive.rs
pub use archive::{load_archive_elf64, load_thin_archive_elf64};

// resolve_lib.rs
pub use resolve_lib::resolve_lib;

// write.rs
pub use write::{write_elf64_shdr, write_elf64_phdr, write_elf64_phdr_at, align_up_64, pad_to};

// args.rs
pub use args::parse_linker_args;

// check.rs
pub use check::check_undefined_symbols_elf64;

// eh_frame.rs
pub use eh_frame::{count_eh_frame_fdes, build_eh_frame_hdr};

// gc_sections.rs
pub use gc_sections::gc_collect_sections_elf64;

#[cfg(test)]
mod pbt_multitu_link {
    //! Round-33: linker_common deep PBT — multi-TU linking differential vs gcc.
    //! Covers symbol resolution across files, extern data, cross-file calls,
    //! tentative definitions, function pointer tables.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/33_linker_deep/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// 生成 3 个 TU：main + 2 个提供者，随机分布符号
    fn render_tus(seeds: &[(u32, i64)]) -> (String, String, String) {
        let mut main_body = String::new();
        let mut b_body = String::new();
        let mut c_body = String::new();
        for (i, &(kind, v)) in seeds.iter().enumerate() {
            match kind % 6 {
                0 => { // extern int 全局
                    let val = v % 100000;
                    b_body.push_str(&format!("int g{} = {};\n", i, val));
                    main_body.push_str(&format!("printf(\"g{}=%d\\\\n\", g{});\n", i, i));
                }
                1 => { // 数组
                    c_body.push_str(&format!("int arr{}[4] = {{ {}, {}, {}, {} }};\n",
                        i, v % 97, (v>>8) % 89, (v>>16) % 83, (v>>24) % 79));
                    main_body.push_str(&format!(
                        "printf(\"arr{}=%d %d\\\\n\", arr{}[0], arr{}[3]);\n", i, i, i));
                }
                2 => { // 跨文件函数
                    let c2 = (v % 50) + 1;
                    c_body.push_str(&format!(
                        "int f{}(int x) {{ return x * {} + {}; }}\n", i, c2, (v>>12) % 100));
                    main_body.push_str(&format!("printf(\"f{}=%d\\\\n\", f{}({}));\n",
                        i, i, (v % 30) - 15));
                }
                3 => { // tentative definition (common symbol 语义)
                    b_body.push_str(&format!("int tent{};\n", i));
                    c_body.push_str(&format!("int tent{};\n", i)); // 两个 TU 都 tentative
                    b_body.push_str(&format!("int init_tent{}() {{ tent{} = {}; return tent{}; }}\n", i, i, v % 500, i));
                    main_body.push_str(&format!("printf(\"tent{}=%d\\\\n\", init_tent{}());\n", i, i));
                }
                4 => { // 函数指针表跨 TU
                    c_body.push_str(&format!(
                        "static int h{}a(int x) {{ return x + {}; }}\nint h{}b(int x) {{ return x * {}; }}\n", i, v % 71, i, 2 + v % 9));
                    b_body.push_str(&format!(
                        "extern int h{}b(int);\ntypeedef int (*fp{})(int);\nfp{} tbl{}[2] = {{ 0, h{}b }};\n", i, i, i, i, i));
                    main_body.push_str(&format!("printf(\"tbl{}=%d\\\\n\", tbl{}[1]({}));\n", i, i, v % 20));
                }
                _ => { // 双层跨文件调用: main→b→c
                    b_body.push_str(&format!(
                        "extern int deep{}(int);\nint mid{}(int x) {{ return deep{}(x + {}) * 2; }}\n", i, i, i, v % 13));
                    c_body.push_str(&format!(
                        "int deep{}(int x) {{ return x - {}; }}\n", i, v % 61));
                    main_body.push_str(&format!("printf(\"mid{}=%d\\\\n\", mid{}({}));\n", i, i, (v % 40) - 20));
                }
            }
        }
        // main 需要所有跨 TU 符号的 extern 声明（现代 gcc 无隐式声明）
        let mut decls = String::from("#include <stdio.h>\n");
        for (i, &(kind, _)) in seeds.iter().enumerate() {
            match kind % 6 {
                0 => decls.push_str(&format!("extern int g{};\n", i)),
                1 => decls.push_str(&format!("extern int arr{}[4];\n", i)),
                2 => decls.push_str(&format!("extern int f{}(int);\n", i)),
                4 => decls.push_str(&format!(
                    "typedef int (*fp_t{})(int); extern fp_t{} tbl{}[2];\n", i, i, i)),
                _ => decls.push_str(&format!("extern int mid{}(int);\n", i)),
            }
        }
        let main = format!("{}int main(void) {{\n    {}\n    return 0;\n}}\n", decls, main_body);
        (main, b_body, c_body)
    }

    /// P-MULTITU: 3-TU 随机符号分布链接后行为 == gcc
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]
        #[test]
        fn p_multitu_link_vs_gcc(
            k0 in any::<u32>(), v0 in any::<i64>(),
            k1 in any::<u32>(), v1 in any::<i64>(),
            k2 in any::<u32>(), v2 in any::<i64>(),
            k3 in any::<u32>(), v3 in any::<i64>(),
            k4 in any::<u32>(), v4 in any::<i64>(),
            k5 in any::<u32>(), v5 in any::<i64>(),
        ) {
            let seeds = vec![(k0, v0), (k1, v1), (k2, v2), (k3, v3), (k4, v4), (k5, v5)];
            // 保证至少一个 kind 0（extern int），使 main 的 g0 声明有效
            let mut seeds = seeds;
            seeds[0].0 = 0;
            let (main_c, b_c, c_c) = render_tus(&seeds);
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let f_main = dir.join(format!("L{}a.c", case));
            let f_b = dir.join(format!("L{}b.c", case));
            let f_c = dir.join(format!("L{}c.c", case));
            std::fs::write(&f_main, &main_c).unwrap();
            std::fs::write(&f_b, &b_c).unwrap();
            std::fs::write(&f_c, &c_c).unwrap();
            let ccc_out = dir.join(format!("L{}ccc", case));
            let gcc_out = dir.join(format!("L{}gcc", case));

            let gc = Command::new("gcc").args(["-w","-O0","-o"]).arg(&gcc_out)
                .arg(&f_main).arg(&f_b).arg(&f_c).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject: {}", String::from_utf8_lossy(&gc.stderr).lines().next().unwrap_or(""));

            let argv = vec!["ccc".to_string(),
                f_main.to_string_lossy().into_owned(),
                f_b.to_string_lossy().into_owned(),
                f_c.to_string_lossy().into_owned(),
                "-o".to_string(), ccc_out.to_string_lossy().into_owned()];
            let mut d = crate::driver::Driver::new();
            let cc = match d.parse_cli_args(&argv) { Ok(true) => Err("cli consumed".to_string()), Ok(false) => d.run(), Err(e) => Err(e) };
            match cc {
                Ok(()) => {}
                Err(e) => prop_assert!(false, "ccc link FAILED where gcc ok: {}\nmain:\n{}\nb:\n{}\nc:\n{}",
                    &e[..e.len().min(200)], main_c, b_c, c_c),
            }

            let r1 = Command::new(&ccc_out).output().unwrap();
            let r2 = Command::new(&gcc_out).output().unwrap();
            let _ = std::fs::remove_file(&f_main); let _ = std::fs::remove_file(&f_b);
            let _ = std::fs::remove_file(&f_c); let _ = std::fs::remove_file(&ccc_out); let _ = std::fs::remove_file(&gcc_out);
            prop_assert!(r1.status.success(), "ccc binary crashed");
            let o1 = String::from_utf8_lossy(&r1.stdout);
            let o2 = String::from_utf8_lossy(&r2.stdout);
            prop_assert_eq!(o1, o2, "multi-TU link behavior differs\nmain:\n{}\nb:\n{}\nc:\n{}", main_c, b_c, c_c);
        }
    }
}

#[cfg(test)]
mod pbt_linker_fuzz {
    //! Round-59: Randomized linker symbol fuzzing — random combinations of
    //! strong/weak/tentative/common/extern symbols across 2-5 TUs, verify
    //! final linked values match gcc.
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);
    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/59_linker_fuzz/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    #[derive(Debug, Clone, Copy)]
    enum SymKind {
        StrongInit,     // int s = value;
        Tentative,      // int s;
        WeakInit,       // __attribute__((weak)) int s = value;
        Extern,         // extern int s;
        None,           // not present in this TU
    }

    fn arb_sym_kind() -> impl Strategy<Value = SymKind> {
        prop_oneof![
            3 => Just(SymKind::StrongInit),
            2 => Just(SymKind::Tentative),
            2 => Just(SymKind::WeakInit),
            1 => Just(SymKind::Extern),
            1 => Just(SymKind::None),
        ]
    }

    fn render_tu(name: &str, kinds: &[SymKind], value: i32) -> String {
        let mut s = String::new();
        for (i, kind) in kinds.iter().enumerate() {
            let sym = format!("sym{}", i);
            match kind {
                SymKind::StrongInit => s.push_str(&format!("int {} = {};\n", sym, value + i as i32)),
                SymKind::Tentative => s.push_str(&format!("int {};\n", sym)),
                SymKind::WeakInit => s.push_str(&format!("__attribute__((weak)) int {} = {};\n", sym, value + i as i32 + 100)),
                SymKind::Extern => s.push_str(&format!("extern int {};\n", sym)),
                SymKind::None => {},
            }
        }
        s
    }

    fn render_main(kinds: &[SymKind]) -> String {
        let mut s = String::from("#include <stdio.h>\n");
        for i in 0..kinds.len() {
            if !matches!(kinds[i], SymKind::None) {
                s.push_str(&format!("extern int sym{};\n", i));
            }
        }
        s.push_str("int main(void){\n");
        for i in 0..kinds.len() {
            if !matches!(kinds[i], SymKind::None) {
                s.push_str(&format!("    printf(\"%d \", sym{});\n", i));
            }
        }
        s.push_str("    putchar('\\n'); return 0;\n}\n");
        s
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn p_linker_sym_fuzz(
            k0_0 in arb_sym_kind(), k0_1 in arb_sym_kind(), k0_2 in arb_sym_kind(),
            k1_0 in arb_sym_kind(), k1_1 in arb_sym_kind(), k1_2 in arb_sym_kind(),
            val0 in 1i32..1000, val1 in 1001i32..2000,
        ) {
            let kinds = [k0_0, k0_1, k0_2];
            let kinds2 = [k1_0, k1_1, k1_2];
            let case = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();

            let f0 = dir.join(format!("L{}_a.c", case));
            let f1 = dir.join(format!("L{}_b.c", case));
            let fm = dir.join(format!("L{}_m.c", case));
            std::fs::write(&f0, render_tu("a", &kinds, val0)).unwrap();
            std::fs::write(&f1, render_tu("b", &kinds2, val1)).unwrap();
            std::fs::write(&fm, render_main(&kinds)).unwrap();

            // gcc baseline
            let g = dir.join(format!("L{}_gcc", case));
            let gc = Command::new("gcc").args(["-w","-o"]).arg(&g)
                .arg(&fm).arg(&f0).arg(&f1).output().unwrap();
            prop_assume!(gc.status.success(), "gcc reject");

            // ccc
            let c = dir.join(format!("L{}_ccc", case));
            let argv = vec!["ccc".to_string(),
                fm.to_string_lossy().into_owned(),
                f0.to_string_lossy().into_owned(),
                f1.to_string_lossy().into_owned(),
                "-o".to_string(), c.to_string_lossy().into_owned()];
            let mut drv = crate::driver::Driver::new();
            let cc = match drv.parse_cli_args(&argv) { Ok(false) => drv.run(), Ok(true) => Err("cli".into()), Err(e) => Err(e) };

            let g_run = Command::new(&g).output().unwrap();
            let want = String::from_utf8_lossy(&g_run.stdout).into_owned();
            let _ = std::fs::remove_file(&g);

            match cc {
                Ok(()) => {
                    // skip known #1198: tentative-first + strong-second
                    if kinds.iter().zip(kinds2.iter())
                        .any(|(a, b)| matches!(a, SymKind::Tentative) && matches!(b, SymKind::StrongInit)) {
                        return Ok(());
                    }
                    let c_run = Command::new(&c).output().unwrap();
                    let _ = std::fs::remove_file(&c);
                    let got = String::from_utf8_lossy(&c_run.stdout);
                    prop_assert_eq!(got, want.as_str(),
                        "LINKER SYM MISMATCH:\n  TU1 kinds={:?} val={}\n  TU2 kinds={:?} val={}",
                        kinds, val0, kinds2, val1);
                }
                Err(_) => {
                    // ccc rejects what gcc accepts — potential bug unless known
                    // duplicate strong symbols (#1166)
                    let has_double_strong = kinds.iter().zip(kinds2.iter())
                        .any(|(a, b)| matches!(a, SymKind::StrongInit) && matches!(b, SymKind::StrongInit));
                    prop_assert!(has_double_strong,
                        "ccc REJECTED what gcc accepted:\n  kinds={:?}/{:?}\n  val={}/{}",
                        kinds, kinds2, val0, val1);
                }
            }
            let _ = std::fs::remove_file(&f0);
            let _ = std::fs::remove_file(&f1);
            let _ = std::fs::remove_file(&fm);
        }
    }
}
