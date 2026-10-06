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

#[cfg(test)]
mod pbt_asm_deep {
    //! Round 16: ARM assembler deep sweep (NON-NEON surface only — the
    //! teammate's campaign owns NEON, #474-#773). Double-reference oracle:
    //! llvm-mc -triple=aarch64 AND aarch64-linux-gnu-as.
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    static CASE: AtomicUsize = AtomicUsize::new(0);

    fn run_dir() -> std::path::PathBuf {
        let d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/16_arm_asm_deep/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    const X: [&str; 32] = ["x0","x1","x2","x3","x4","x5","x6","x7","x8","x9","x10","x11",
        "x12","x13","x14","x15","x16","x17","x18","x19","x20","x21","x22","x23","x24","x25",
        "x26","x27","x28","x29","x30","sp"];
    const W: [&str; 32] = ["w0","w1","w2","w3","w4","w5","w6","w7","w8","w9","w10","w11",
        "w12","w13","w14","w15","w16","w17","w18","w19","w20","w21","w22","w23","w24","w25",
        "w26","w27","w28","w29","w30","wsp"];
    fn x_(i: u8) -> &'static str { X[(i % 31) as usize] } // x0..x30; SP only in add/sub slots (family 20 uses sp explicitly)
    fn w_(i: u8) -> &'static str { W[(i % 31) as usize] }

    /// Valid forms across ~22 NON-NEON families.
    fn valid_form(fam: u8, a: u8, b: u8, c: u8, imm: u16, sh: u8) -> String {
        let i12 = imm % 4096;
        let i16 = imm;
        match fam % 22 {
            0 => format!("add {}, {}, #{}", x_(a), x_(b), i12),
            1 => format!("sub {}, {}, #{}", w_(a), w_(b), i12),
            2 => format!("add {}, {}, #{}, lsl #12", x_(a), x_(b), i12),
            3 => format!("add {}, {}, {}", x_(a), x_(b), x_(c)),
            4 => format!("sub {}, {}, {}", w_(a), w_(b), w_(c)),
            5 => format!("and {}, {}, {}", x_(a), x_(b), x_(c)),
            6 => format!("eor {}, {}, {}", w_(a), w_(b), w_(c)),
            7 => format!("cmp {}, #{}", x_(a), i12),
            8 => format!("cmn {}, {}", x_(a), x_(b)),
            9 => format!("neg {}, {}", x_(a), x_(b)),
            10 => format!("mvn {}, {}", w_(a), w_(b)),
            11 => format!("movz {}, #{}, lsl #{}", x_(a), i16, (sh % 4) * 16),
            12 => format!("movk {}, #{}, lsl #{}", w_(a), i16, (sh % 2) * 16), // W: hw ∈ {0,16} only
            13 => format!("movn {}, #{}", x_(a), i16),
            14 => format!("lsl {}, {}, {}", x_(a), x_(b), x_(c)),
            15 => format!("lsl {}, {}, #{}", x_(a), x_(b), sh % 64),
            16 => format!("ror {}, {}, #{}", w_(a), w_(b), sh % 32),
            17 => format!("ldr {}, [{}, #{}]", x_(a), x_(b), (imm % 512) * 8),
            18 => format!("str {}, [{}, #{}]", w_(a), x_(b), (imm % 256) * 4), // base must be X
            19 => format!("ldur {}, [{}, #{}]", x_(a), x_(b), -(1 + (imm as i32 % 256))),
            20 => format!("ldp {}, {}, [sp, #{}]", x_(a), x_(b), (imm % 64) * 8), // imm7×8 ≤ 504
            _ => format!("sxtw {}, {}", x_(a), w_(b)),
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
                if f.len() == 8 && f.chars().all(|ch| ch.is_ascii_hexdigit()) { hex.push_str(f); }
            }
        }
        hex
    }

    fn ccc_assemble(src: &std::path::Path, out: &std::path::Path) -> Result<String, String> {
        let argv: Vec<String> = vec![
            "ccc".into(), "-mlittle-endian".into(), "-c".into(),
            "-o".into(), out.to_str().unwrap().into(), src.to_str().unwrap().into(),
        ];
        let mut d = Driver::new();
        let parsed = d.parse_cli_args(&argv)?;
        if parsed { return Err("flag consumed".into()); }
        d.run()?;
        if !out.exists() { return Err("no output".into()); }
        Ok(text_hex(out))
    }

    fn mc_assemble(src: &std::path::Path, out: &std::path::Path) -> Result<String, String> {
        let o = Command::new("llvm-mc").args(["-triple=aarch64", "-filetype=obj", "-o", out.to_str().unwrap(), src.to_str().unwrap()]).output().unwrap();
        if !o.status.success() { return Err(String::from_utf8_lossy(&o.stderr).to_string()); }
        Ok(text_hex(out))
    }

    fn gas_assemble(src: &std::path::Path, out: &std::path::Path) -> bool {
        Command::new("aarch64-linux-gnu-as").args(["-o", out.to_str().unwrap(), src.to_str().unwrap()]).output().unwrap().status.success()
    }

    /// P1: valid NON-NEON forms — both references accept ⇒ ccc accepts and
    /// bytes match llvm-mc; both references reject ⇒ ccc rejects.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]
        #[test]
        fn p1_arm_asm_deep_valid_forms(
            fam in 0u8..22, a in any::<u8>(), b in any::<u8>(), c in any::<u8>(),
            imm in any::<u16>(), sh in any::<u8>(),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("v{:04}.s", id));
            let form = valid_form(fam, a, b, c, imm, sh);
            std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
            let ccc_o = dir.join(format!("v{:04}_c.o", id));
            let mc_o = dir.join(format!("v{:04}_m.o", id));
            let gas_o = dir.join(format!("v{:04}_g.o", id));
            let mc = mc_assemble(&src, &mc_o);
            let gas_ok = gas_assemble(&src, &gas_o);
            let cc = ccc_assemble(&src, &ccc_o);
            static GAPS: Mutex<Option<std::collections::BTreeSet<String>>> = Mutex::new(None);
            match (&mc, &cc) {
                (Ok(m), Ok(c)) => prop_assert_eq!(c, m, "byte mismatch for: {}", form),
                (Err(_), Ok(_)) => {
                    if !gas_ok {
                        let mut g = GAPS.lock().unwrap();
                        g.get_or_insert_with(Default::default).insert(format!("SILENT-ACCEPT {}", form));
                    }
                }
                (Ok(_), Err(_)) => {
                    if gas_ok {
                        let mut g = GAPS.lock().unwrap();
                        g.get_or_insert_with(Default::default).insert(format!("REJECT-VALID {}", form));
                    }
                }
                (Err(_), Err(_)) => {}
            }
            {
                let g = GAPS.lock().unwrap();
                if let Some(set) = g.as_ref() {
                    prop_assert!(set.is_empty(), "distinct failure classes ({}): {:?}", set.len(), set);
                }
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o); let _ = std::fs::remove_file(&gas_o);
        }
    }

    /// P2: targeted validation mutations (the hand-probed classes):
    /// width swap / imm range / shift range / ldp alignment / index scale.
    fn mutate(base: &str, m: u8) -> String {
        let words: Vec<&str> = base.split_whitespace().collect();
        match m % 5 {
            0 => {
                // W/X width swap on the first register token found
                let mut s = String::new();
                let mut done = false;
                for w in words {
                    let t = if !done && (w.starts_with('x') || w.starts_with('w')) && w.len() <= 3 && w.chars().skip(1).all(|c| c.is_ascii_digit()) {
                        done = true;
                        if w.starts_with('x') { format!("w{}", &w[1..]) } else { format!("x{}", &w[1..]) }
                    } else { w.to_string() };
                    s.push_str(&t); s.push(' ');
                }
                s.trim_end().to_string()
            }
            1 => base.replace('#', "#99999"), // imm way out of range
            2 => {
                // bump shift amounts by +64
                let mut s = base.to_string();
                if let Some(pos) = s.rfind('#') {
                    let tail = s[pos + 1..].to_string();
                    if let Ok(v) = tail.trim().parse::<u32>() {
                        s = format!("{}#{}", &s[..pos + 1], v + 64);
                    }
                }
                s
            }
            3 => base.replace(", #", ", #3"), // misalign pair offsets
            _ => format!("{} lsl #7", base), // invalid index scale
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]
        #[test]
        fn p2_arm_asm_deep_invalid_rejected(
            fam in 0u8..22, a in any::<u8>(), b in any::<u8>(), c in any::<u8>(),
            imm in any::<u16>(), sh in any::<u8>(), m in 0u8..5,
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let base = valid_form(fam, a, b, c, imm, sh);
            let form = mutate(&base, m);
            let src = dir.join(format!("i{:04}.s", id));
            std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
            let ccc_o = dir.join(format!("i{:04}_c.o", id));
            let mc_o = dir.join(format!("i{:04}_m.o", id));
            let gas_o = dir.join(format!("i{:04}_g.o", id));
            let mc = mc_assemble(&src, &mc_o);
            let gas_ok = gas_assemble(&src, &gas_o);
            let cc = ccc_assemble(&src, &ccc_o);
            static GAPS: Mutex<Option<std::collections::BTreeSet<String>>> = Mutex::new(None);
            if mc.is_err() && !gas_ok && cc.is_ok() {
                let mut g = GAPS.lock().unwrap();
                g.get_or_insert_with(Default::default).insert(format!("SILENT-ACCEPT(m{}) {} (base {})", m % 5, form, base));
            }
            {
                let g = GAPS.lock().unwrap();
                if let Some(set) = g.as_ref() {
                    prop_assert!(set.is_empty(), "distinct invalid-accept classes ({}): {:?}", set.len(), set);
                }
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o); let _ = std::fs::remove_file(&gas_o);
        }
    }

    /// RED WITNESSES — round 16 (issues #791-#795), all double-reference
    /// confirmed (aarch64-linux-gnu-as AND llvm-mc reject; ccc accepts).
    fn check_reject(form: &str, tag: &str) {
        let dir = run_dir();
        let src = dir.join(format!("r{}.s", tag));
        std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
        let out = dir.join(format!("r{}.o", tag));
        assert!(ccc_assemble(&src, &out).is_err(), "#{}: silently accepted: {}", tag, form);
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn test_armasm_regression_791() { check_reject("add w0, x1, x2", "791a"); check_reject("add x0, w1, #5", "791b"); check_reject("str w1, [w28, #8]", "791c"); }

    #[test]
    fn test_armasm_regression_792() { check_reject("movk x0, #65536", "792a"); check_reject("movz w0, #65536", "792b"); check_reject("ldur x0, [x1, #-257]", "792c"); check_reject("ldp x23, x27, [sp, #1936]", "792d"); }

    #[test]
    fn test_armasm_regression_793() { check_reject("lsl x0, x1, #64", "793a"); check_reject("ror w0, w1, #32", "793b"); check_reject("movk w14, #25050, lsl #32", "793c"); check_reject("add x0, x1, #1, lsl #13", "793d"); }

    #[test]
    fn test_armasm_regression_794() { check_reject("ldp x0, x1, [sp, #3]", "794a"); check_reject("str x0, [x1, x2, lsl #4]", "794b"); }

    #[test]
    fn test_armasm_regression_795() { check_reject("and sp, sp, x28", "795a"); check_reject("mul w0, wsp, w2", "795b"); }
}

#[cfg(test)]
mod pbt_witness {
    //! Round 16b: targeted PBT witnesses for the hand-probed classes (each
    //! property fails and SHRINKS to a minimal double-reference-confirmed
    //! counterexample).
    use super::pbt_asm_deep_support as sup;
    use proptest::prelude::*;

    /// W/X width mixing: swap the width of the SECOND register operand of a
    /// valid reg-reg ALU form; both references must reject ⇒ ccc must reject.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]
        #[test]
        fn p2b_arm_width_mix_rejected(
            op in 0u8..4, r1 in any::<u8>(), r2 in any::<u8>(), r3 in any::<u8>(),
        ) {
            let ops = ["add", "sub", "and", "eor"];
            let form = format!("{} x{}, x{}, w{}", ops[(op % 4) as usize], r1 % 31, r2 % 31, r3 % 31);
            sup::assert_rejected_if_references_reject(&form);
        }
    }
}

#[cfg(test)]
pub(super) mod pbt_asm_deep_support {
    use crate::driver::Driver;
    use std::path::PathBuf;

    pub fn run_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/16_arm_asm_deep/run")
    }

    /// Double-reference rejection oracle: if BOTH aarch64-linux-gnu-as and
    /// llvm-mc reject the form, ccc must reject it too.
    pub fn assert_rejected_if_references_reject(form: &str) {
        let dir = run_dir();
        let _ = std::fs::create_dir_all(&dir);
        let src = dir.join(format!("w{}.s", form.len()));
        std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
        let ccc_o = dir.join("w_c.o");
        let mc_o = dir.join("w_m.o");
        let gas_o = dir.join("w_g.o");
        let mc = std::process::Command::new("llvm-mc")
            .args(["-triple=aarch64", "-filetype=obj", "-o", mc_o.to_str().unwrap(), src.to_str().unwrap()])
            .output().unwrap();
        let gas = std::process::Command::new("aarch64-linux-gnu-as")
            .args(["-o", gas_o.to_str().unwrap(), src.to_str().unwrap()])
            .output().unwrap();
        let argv: Vec<String> = vec![
            "ccc".into(), "-mlittle-endian".into(), "-c".into(),
            "-o".into(), ccc_o.to_str().unwrap().into(), src.to_str().unwrap().into(),
        ];
        let mut d = Driver::new();
        let parsed = d.parse_cli_args(&argv).unwrap();
        assert!(!parsed);
        let cc_ok = d.run().is_ok() && ccc_o.exists();
        if !mc.status.success() && !gas.status.success() && cc_ok {
            // panic-based so callers cannot accidentally swallow a returned
            // TestCaseError (proptest catches the panic and shrinks it)
            panic!("ccc silently accepted a double-reference-rejected form: {}", form);
        }
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o);
        let _ = std::fs::remove_file(&mc_o); let _ = std::fs::remove_file(&gas_o);
    }
}
