pub(crate) mod codegen;
#[cfg_attr(feature = "gcc_assembler", allow(dead_code))] // Built-in assembler unused when gcc handles assembly
pub(crate) mod assembler;
#[cfg_attr(feature = "gcc_linker", allow(dead_code, unused_imports))] // Built-in linker unused when gcc handles linking; unused_imports: mod.rs has pub use re-exports
pub(crate) mod linker;

pub(crate) use codegen::emit::RiscvCodegen;

#[cfg(test)]
mod pbt_asm {
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);

    fn run_dir() -> std::path::PathBuf {
        let d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/11_riscv/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    fn reg(r: u8) -> String {
        const R: [&str; 32] = ["zero","ra","sp","gp","tp","t0","t1","t2","s0","s1",
            "a0","a1","a2","a3","a4","a5","a6","a7","s2","s3","s4","s5","s6","s7","s8","s9",
            "s10","s11","t3","t4","t5","t6"];
        R[(r % 32) as usize].to_string()
    }

    fn render_inst(k: u8, r1: u8, r2: u8, r3: u8, imm: u16) -> String {
        let i = (imm % 4096) as i32;
        let si = if i > 2047 { i - 4096 } else { i }; // [-2048, 2047]
        match k % 13 {
            0 => format!("add {}, {}, {}", reg(r1), reg(r2), reg(r3)),
            1 => format!("sub {}, {}, {}", reg(r1), reg(r2), reg(r3)),
            2 => format!("xor {}, {}, {}", reg(r1), reg(r2), reg(r3)),
            3 => format!("sll {}, {}, {}", reg(r1), reg(r2), reg(r3)),
            4 => format!("sra {}, {}, {}", reg(r1), reg(r2), reg(r3)),
            5 => format!("sltu {}, {}, {}", reg(r1), reg(r2), reg(r3)),
            6 => format!("addi {}, {}, {}", reg(r1), reg(r2), si),
            7 => format!("andi {}, {}, {}", reg(r1), reg(r2), si),
            8 => format!("lui {}, {}", reg(r1), imm),
            9 => format!("ld {}, {}({})", reg(r1), (imm % 256) * 8, reg(r2)), // max 2040 <= 2047
            10 => format!("sw {}, {}({})", reg(r1), (imm % 512) * 4, reg(r2)),
            11 => format!("slli {}, {}, {}", reg(r1), reg(r2), imm % 32),
            _ => format!("addw {}, {}, {}", reg(r1), reg(r2), reg(r3)),
        }
    }

    fn text_hex(path: &std::path::Path) -> String {
        let out = Command::new("readelf").args(["-x", ".text", path.to_str().unwrap()]).output().unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        let mut hex = String::new();
        for line in text.lines() {
            let t = line.trim_start();
            if !t.starts_with("0x") { continue; }
            for f in t.split_whitespace().skip(1) {
                if f.len() == 8 && f.chars().all(|c| c.is_ascii_hexdigit()) { hex.push_str(f); }
            }
        }
        hex
    }

    /// P1: builtin RISC-V assembler .text bytes == llvm-mc over generated
    /// label-free RV64 instruction sequences.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]
        #[test]
        fn p1_riscv_asm_matches_llvm_mc(
            insts in prop::collection::vec(any::<(u8, u8, u8, u8, u16)>(), 20..41),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("r{:04}.s", id));
            let mut prog = String::from(".text\n");
            for (k, r1, r2, r3, imm) in &insts {
                prog.push_str(&render_inst(*k, *r1, *r2, *r3, *imm));
                prog.push('\n');
            }
            std::fs::write(&src, prog).unwrap();

            let ccc_o = dir.join(format!("r{:04}_ccc.o", id));
            let mc_o = dir.join(format!("r{:04}_mc.o", id));
            let argv: Vec<String> = vec![
                "ccc-riscv".into(), "-c".into(),
                "-o".into(), ccc_o.to_str().unwrap().into(), src.to_str().unwrap().into(),
            ];
            let mut d = Driver::new();
            let parsed = d.parse_cli_args(&argv)
                .map_err(|e| proptest::test_runner::TestCaseError::fail(format!("cli: {}", e)))?;
            if parsed { return Err(proptest::test_runner::TestCaseError::fail(String::from("flag consumed"))); }
            d.run().map_err(|e| proptest::test_runner::TestCaseError::fail(format!("ccc-riscv assemble failed: {}", e)))?;
            let mc = Command::new("llvm-mc")
                .args(["-triple=riscv64", "-filetype=obj", "-o", mc_o.to_str().unwrap(), src.to_str().unwrap()])
                .output().unwrap();
            prop_assert!(mc.status.success(), "llvm-mc failed: {}", String::from_utf8_lossy(&mc.stderr));

            let got = text_hex(&ccc_o);
            let want = text_hex(&mc_o);
            prop_assert_eq!(got, want, "byte mismatch (case r{:04}, {} insts)", id, insts.len());

            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o);
        }
    }
}

#[cfg(test)]
mod pbt_regression_asm {
    use crate::driver::Driver;
    /// RED WITNESSES — round 17 (issues filed from the deep sweep):
    /// llvm-mc rejects every form below (ISA encoding limits / formats);
    /// ccc currently accepts them silently.
    fn check_reject(form: &str, tag: &str) {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/17_riscv_asm_deep/run");
        let _ = std::fs::create_dir_all(&dir);
        let src = dir.join(format!("r{}.s", tag));
        std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
        let out = dir.join(format!("r{}.o", tag));
        let argv: Vec<String> = vec![
            "ccc-riscv".into(), "-c".into(),
            "-o".into(), out.to_str().unwrap().into(), src.to_str().unwrap().into(),
        ];
        let mut d = Driver::new();
        let parsed = d.parse_cli_args(&argv).unwrap();
        assert!(!parsed);
        let result = d.run();
        let produced = out.exists();
        assert!(result.is_err() || !produced, "silently accepted: {}", form);
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn test_riscvasm_regression_imm12_range() {
        check_reject("addi a0, a1, 2048", "imm12a");
        check_reject("addi a0, a1, -2049", "imm12b");
        check_reject("andi a0, a1, 4096", "imm12c");
        check_reject("ld a0, 2048(a1)", "imm12d");
    }

    #[test]
    fn test_riscvasm_regression_shamt_range() {
        check_reject("slli a0, a1, 64", "shamt1");
        check_reject("slli a0, a1, -1", "shamt2");
        check_reject("slliw a0, a1, 32", "shamt3");
    }

    #[test]
    fn test_riscvasm_regression_malformed() {
        check_reject("add a0, a1, 2048", "mf1");
        check_reject("beq a0, a1, 4096", "mf2");
        check_reject("fence.i a0, a1", "mf3");
    }
}

#[cfg(test)]
mod pbt_witness {
    //! Round 17b: PBT witnesses for the RISC-V validation classes — each
    //! generated invalid form is rejected by llvm-mc (ISA encoding limits)
    //! and must be rejected by ccc too. Properties fail and SHRINK.
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;

    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/17_riscv_asm_deep/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    fn check_rejected_by_ccc(form: &str) {
        let dir = run_dir();
        let src = dir.join("pw.s");
        std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
        let mc = Command::new("llvm-mc")
            .args(["-triple=riscv64", "-filetype=obj", "-o", dir.join("pw_m.o").to_str().unwrap(), src.to_str().unwrap()])
            .output().unwrap();
        if !mc.status.success() {
            let argv: Vec<String> = vec![
                "ccc-riscv".into(), "-c".into(),
                "-o".into(), dir.join("pw_c.o").to_str().unwrap().into(), src.to_str().unwrap().into(),
            ];
            let mut d = Driver::new();
            let parsed = d.parse_cli_args(&argv).unwrap();
            assert!(!parsed);
            let cc_ok = d.run().is_ok() && dir.join("pw_c.o").exists();
            if cc_ok {
                panic!("ccc silently accepted an llvm-mc-rejected (ISA-limit) form: {}", form);
            }
        }
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(dir.join("pw_m.o")); let _ = std::fs::remove_file(dir.join("pw_c.o"));
    }

    /// #796 witness: imm12 range violations across I-type ops and ld/sd.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_imm12_range_witness(op in 0u8..5, r1 in any::<u8>(), r2 in any::<u8>(), over in any::<bool>()) {
            let ops = ["addi", "andi", "ori", "xori", "addiw"];
            let r = |v: u8| v % 32;
            let form = if over {
                format!("{} x{}, x{}, 2048", ops[(op % 5) as usize], r(r1), r(r2))
            } else {
                format!("{} x{}, x{}, -2049", ops[(op % 5) as usize], r(r1), r(r2))
            };
            check_rejected_by_ccc(&form);
        }
    }

    /// #797 witness: shamt range violations.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_shamt_range_witness(op in 0u8..3, r1 in any::<u8>(), r2 in any::<u8>(), w_form in any::<bool>()) {
            let r = |v: u8| v % 32;
            let (mnem, amt) = if w_form {
                (["slliw", "srliw", "sraiw"][(op % 3) as usize], 32)
            } else {
                (["slli", "srli", "srai"][(op % 3) as usize], 64)
            };
            check_rejected_by_ccc(&format!("{} x{}, x{}, {}", mnem, r(r1), r(r2), amt));
        }
    }

    /// #798 witness: malformed operand forms.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p_malformed_witness(kind in 0u8..3, r1 in any::<u8>(), r2 in any::<u8>()) {
            let r = |v: u8| v % 32;
            let form = match kind % 3 {
                0 => format!("add x{}, x{}, 2048", r(r1), r(r2)), // imm in reg slot
                1 => format!("beq x{}, x{}, 4096", r(r1), r(r2)), // raw offset
                _ => "fence.i x0, x1".to_string(), // operands on operand-less form
            };
            check_rejected_by_ccc(&form);
        }
    }
}
