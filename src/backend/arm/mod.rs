pub(crate) mod codegen;
#[cfg_attr(feature = "gcc_assembler", allow(dead_code))] // Built-in assembler unused when gcc handles assembly
pub(crate) mod assembler;
#[cfg_attr(feature = "gcc_linker", allow(dead_code))] // Built-in linker unused when gcc handles linking
pub(crate) mod linker;

pub(crate) use codegen::emit::ArmCodegen;

#[cfg(test)]
mod pbt_asm {
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);

    fn run_dir() -> std::path::PathBuf {
        let d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/10_arm/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    /// Generated AArch64 instruction forms (label-free, no relocations):
    /// deterministic operand slots filled per opcode family.
    fn render_inst(k: u8, r1: u8, r2: u8, r3: u8, imm: u16, sh: u8) -> String {
        let x = |r: u8| format!("x{}", r % 31);
        let w = |r: u8| format!("w{}", r % 31);
        let i12 = imm % 4096;
        match k % 16 {
            0 => format!("add {}, {}, {}", x(r1), x(r2), x(r3)),
            1 => format!("sub {}, {}, {}", w(r1), w(r2), w(r3)),
            2 => format!("and {}, {}, {}", x(r1), x(r2), x(r3)),
            3 => format!("orr {}, {}, {}", x(r1), x(r2), x(r3)),
            4 => format!("eor {}, {}, {}", x(r1), x(r2), x(r3)),
            5 => format!("bic {}, {}, {}", w(r1), w(r2), w(r3)),
            6 => format!("add {}, {}, {}", x(r1), x(r2), i12),
            7 => format!("sub {}, {}, #{}", w(r1), w(r2), i12),
            8 => format!("add {}, {}, {}, lsl #{}", x(r1), x(r2), x(r3), sh % 64),
            9 => format!("cmp {}, {}", x(r1), x(r2)),
            10 => format!("cmp {}, #{}", w(r1), i12),
            11 => {
                let hw = sh % 4;
                format!("movz {}, #{}, lsl #{}", x(r1), imm, hw * 16)
            }
            12 => {
                let off = (imm % 4096) * 8;
                format!("ldr {}, [{}, #{}]", x(r1), x(r2), off)
            }
            13 => {
                let off = (imm % 256) * 4;
                format!("str {}, [{}, #{}]", w(r1), x(r2), off)
            }
            14 => {
                let off = -1 - (imm as i32 % 255); // ldur unscaled imm: [-256, -1]
                format!("ldur {}, [{}, #{}]", x(r1), x(r2), off)
            }
            _ => format!("sxtw {}, {}", x(r1), w(r2)),
        }
    }

    /// readelf -x .text hex-dump → contiguous hex string.
    fn text_hex(path: &std::path::Path) -> String {
        let out = Command::new("readelf").args(["-x", ".text", path.to_str().unwrap()]).output().unwrap();
        let text = String::from_utf8_lossy(&out.stdout);
        let mut hex = String::new();
        for line in text.lines() {
            let t = line.trim_start();
            if !t.starts_with("0x") { continue; }
            // hex groups are whitespace-separated fields 2..=5 before the ASCII column
            for f in t.split_whitespace().skip(1) {
                if f.len() == 8 && f.chars().all(|c| c.is_ascii_hexdigit()) {
                    hex.push_str(f);
                }
            }
        }
        hex
    }

    fn assemble_ccc(src: &std::path::Path, out: &std::path::Path) -> Result<(), String> {
        let argv: Vec<String> = vec![
            "ccc".into(), "-mlittle-endian".into(), "-c".into(),
            "-o".into(), out.to_str().unwrap().into(), src.to_str().unwrap().into(),
        ];
        let mut d = Driver::new();
        if d.parse_cli_args(&argv)? { return Err("flag consumed".into()); }
        d.run()
    }

    /// P1: the builtin AArch64 assembler's .text bytes must equal llvm-mc's
    /// over generated label-free instruction sequences (general non-NEON
    /// surface). llvm-mc is the repo's established encoder reference.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]
        #[test]
        fn p1_arm_asm_matches_llvm_mc(
            insts in prop::collection::vec(any::<(u8, u8, u8, u8, u16, u8)>(), 20..41),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("a{:04}.s", id));
            let mut prog = String::from(".text\n");
            for (k, r1, r2, r3, imm, sh) in &insts {
                prog.push_str(&render_inst(*k, *r1, *r2, *r3, *imm, *sh));
                prog.push('\n');
            }
            std::fs::write(&src, prog).unwrap();

            let ccc_o = dir.join(format!("a{:04}_ccc.o", id));
            let mc_o = dir.join(format!("a{:04}_mc.o", id));
            assemble_ccc(&src, &ccc_o)
                .map_err(|e| proptest::test_runner::TestCaseError::fail(format!("ccc assemble failed: {}", e)))?;
            let mc = Command::new("llvm-mc")
                .args(["-triple=aarch64", "-filetype=obj", "-o", mc_o.to_str().unwrap(), src.to_str().unwrap()])
                .output().unwrap();
            prop_assert!(mc.status.success(), "llvm-mc failed: {}", String::from_utf8_lossy(&mc.stderr));

            let got = text_hex(&ccc_o);
            let want = text_hex(&mc_o);
            prop_assert_eq!(got, want, "byte mismatch (case a{:04}, {} insts)", id, insts.len());

            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o);
        }
    }
}
