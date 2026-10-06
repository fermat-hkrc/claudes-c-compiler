pub(crate) mod codegen;
#[cfg_attr(feature = "gcc_assembler", allow(dead_code))] // Built-in assembler unused when gcc handles assembly
pub(crate) mod assembler;
#[cfg_attr(feature = "gcc_linker", allow(dead_code))] // Built-in linker unused when gcc handles linking
pub(crate) mod linker;

pub(crate) use codegen::emit::X86Codegen;

#[cfg(test)]
mod pbt_e2e {
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::path::PathBuf;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);

    fn run_dir() -> PathBuf {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/09_x86/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    // ── C program generator ────────────────────────────────────────────────
    // u64 wrap arithmetic + bitwise + compares + casts (defined semantics);
    // div/rem by nonzero constants; shifts by constants < 64.
    #[derive(Debug, Clone)]
    struct FnSpec { n_args: u8, tokens: Vec<(u8, i64)> }

    fn render_fn(idx: usize, f: &FnSpec) -> String {
        let mut s = format!("unsigned long long f{}(", idx);
        for i in 0..f.n_args {
            if i > 0 { s.push_str(", "); }
            s.push_str(&format!("u64 a{}", i));
        }
        s.push_str(") {\n");
        // RPN stack seeded with params; each token pushes one temp.
        let mut stack: Vec<String> = (0..f.n_args).map(|i| format!("a{}", i)).collect();
        let mut t = 0;
        for &(k, v) in &f.tokens {
            let name = format!("t{}", t); t += 1;
            let pick = |stack: &Vec<String>, k: u8, off: usize| -> String {
                let i = (k as usize + off) % stack.len().max(1);
                stack[i.min(stack.len() - 1)].clone()
            };
            let stmt = match k % 13 {
                0 => { let a = pick(&stack, k, 0); let c = (v.unsigned_abs() % 1000007) as u64; format!("{name} = {a} + {c}ull;") }
                1 => { let a = pick(&stack, k, 1); let c = (v.unsigned_abs() % 65519) + 1; format!("{name} = {a} * {c}ull;") }
                2 => { let a = pick(&stack, k, 2); let c = (v.unsigned_abs() % 1000007) as u64; format!("{name} = {a} & {c}ull;") }
                3 => { let a = pick(&stack, k, 3); let c = (v.unsigned_abs() % 1000007) as u64; format!("{name} = {a} | {c}ull;") }
                4 => { let a = pick(&stack, k, 4); let c = (v.unsigned_abs() % 1000007) as u64; format!("{name} = {a} ^ {c}ull;") }
                5 => { let a = pick(&stack, k, 5); let c = v.unsigned_abs() % 64; format!("{name} = {a} << {c}ull;") }
                6 => { let a = pick(&stack, k, 6); let c = v.unsigned_abs() % 64; format!("{name} = {a} >> {c}ull;") }
                7 => { let a = pick(&stack, k, 7); let c = (v.unsigned_abs() % 997) + 2; format!("{name} = {a} / {c}ull;") }
                8 => { let a = pick(&stack, k, 8); let c = (v.unsigned_abs() % 997) + 2; format!("{name} = {a} % {c}ull;") }
                9 => {
                    if stack.len() >= 2 {
                        let a = stack[(k as usize) % stack.len()].clone();
                        let b = stack[(k as usize + 1) % stack.len()].clone();
                        format!("{name} = {a} - {b};")
                    } else { let a = pick(&stack, k, 0); format!("{name} = {a} - {a};") }
                }
                10 => {
                    if stack.len() >= 2 {
                        let a = stack[(k as usize) % stack.len()].clone();
                        let b = stack[(k as usize + 2) % stack.len()].clone();
                        format!("{name} = ({a} < {b}) ? 114 : 88;")
                    } else { let a = pick(&stack, k, 0); format!("{name} = ({a} == 0) ? 114 : 88;") }
                }
                11 => { let a = pick(&stack, k, 9); format!("{name} = (unsigned long long)(long long){a};") }
                _ => { let a = pick(&stack, k, 10); let c = (v.unsigned_abs() % 1000007) as u64; format!("{name} = {a} + {c}ull;") }
            };
            s.push_str(&format!("    unsigned long long {};\n    {}\n", name, stmt));
            stack.push(name);
        }
        let ret = stack.last().cloned().unwrap_or_else(|| "0ull".to_string());
        s.push_str(&format!("    return {};\n}}\n", ret));
        s
    }

    fn render_program(fns: &[FnSpec], loop_n: u64, loop_k: u64) -> String {
        let mut s = String::from("#include <stdio.h>\ntypedef unsigned long long u64;\n");
        for (i, f) in fns.iter().enumerate() {
            s.push_str(&render_fn(i, f));
        }
        s.push_str("u64 loopk(u64 n, u64 k) { u64 s = 0; for (u64 i = 0; i < (n & 15u); i++) s = s * k + i; return s; }\n");
        s.push_str("int main(void) {\n");
        for (i, f) in fns.iter().enumerate() {
            for set in 0..4u64 {
                let args: Vec<String> = (0..f.n_args).map(|a| {
                    let v = match set {
                        0 => 0,
                        1 => u64::MAX,
                        2 => (a as u64) + 1,
                        _ => 0x9e3779b97f4a7c15u64.wrapping_mul(a as u64 + 0x1234),
                    };
                    format!("{}ull", v)
                }).collect();
                s.push_str(&format!("    printf(\"%llu\\n\", f{}({}));\n", i, args.join(", ")));
            }
        }
        s.push_str(&format!("    printf(\"%llu\\n\", loopk({}, {}));\n", loop_n, loop_k));
        s.push_str("    return 0;\n}\n");
        s
    }

    fn compile_with(args: &[&str]) -> Result<(), String> {
        let argv: Vec<String> = std::iter::once("ccc".to_string()).chain(args.iter().map(|s| s.to_string())).collect();
        let mut d = Driver::new();
        if d.parse_cli_args(&argv)? { return Err("cli flag consumed the run".into()); }
        d.run()
    }

    fn run_quiet(path: &std::path::Path) -> String {
        match Command::new(path).output() {
            Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
            Err(e) => format!("<spawn failed: {}>", e),
        }
    }

    fn e2e_case(fns: &[FnSpec], loop_n: u64, loop_k: u64) -> Result<(), proptest::test_runner::TestCaseError> {
        let id = CASE.fetch_add(1, Ordering::SeqCst);
        let dir = run_dir();
        let src = dir.join(format!("p{:04}.c", id));
        std::fs::write(&src, render_program(fns, loop_n, loop_k)).unwrap();

        let gcc_out = dir.join(format!("p{:04}_gcc", id));
        let ccc_o0 = dir.join(format!("p{:04}_o0", id));
        let ccc_o2 = dir.join(format!("p{:04}_o2", id));
        let src_str = src.to_str().unwrap();

        let g = Command::new("gcc").args(["-w", "-O2", "-o", gcc_out.to_str().unwrap(), src_str]).output().unwrap();
        prop_assert!(g.status.success(), "gcc failed: {}", String::from_utf8_lossy(&g.stderr));

        compile_with(&["-o", ccc_o0.to_str().unwrap(), src_str])
            .map_err(|e| proptest::test_runner::TestCaseError::fail(format!("ccc -O0 failed: {}", e)))?;
        compile_with(&["-O2", "-o", ccc_o2.to_str().unwrap(), src_str])
            .map_err(|e| proptest::test_runner::TestCaseError::fail(format!("ccc -O2 failed: {}", e)))?;

        let ref_out = run_quiet(&gcc_out);
        let o0 = run_quiet(&ccc_o0);
        let o2 = run_quiet(&ccc_o2);
        prop_assert_eq!(&o0, &ref_out, "ccc -O0 != gcc (case p{:04})", id);
        prop_assert_eq!(&o2, &o0, "ccc -O2 != ccc -O0 (case p{:04})", id);

        let _ = std::fs::remove_file(&src);
        let _ = std::fs::remove_file(&gcc_out);
        let _ = std::fs::remove_file(&ccc_o0);
        let _ = std::fs::remove_file(&ccc_o2);
        Ok(())
    }

    fn fn_spec_arb() -> impl proptest::prelude::Strategy<Value = FnSpec> {
        (1u8..11, proptest::collection::vec(any::<(u8, i64)>(), 2..9))
            .prop_map(|(n_args, tokens)| FnSpec { n_args, tokens })
    }

    /// P1 + P2: the compiled program's observable behavior must equal gcc's
    /// (P1, -O0 vs gcc) and be preserved by the optimizer (P2, -O2 vs -O0),
    /// over generated u64 expression functions (1..10 args), a compare/cast
    /// mix and a loop kernel — codegen + peephole + assembler + linker
    /// end-to-end, natively executed.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(160))]
        #[test]
        fn p1p2_e2e_codegen_and_opt(
            fns in prop::collection::vec(fn_spec_arb(), 1..3),
            loop_n in any::<u64>(), loop_k in any::<u64>(),
        ) {
            e2e_case(&fns, loop_n, loop_k)?;
        }
    }

    /// P3: double-precision expression functions — exercises the SSE/XMM
    /// codegen paths (double consts, cvt* casts) with %.17g-exact output
    /// comparison against gcc. Domain: no NaN/Inf results (bounded operands,
    /// divisors >= 1.0), so the textual comparison is well-defined.
    fn render_dfn(idx: usize, tokens: &[(u8, i64)]) -> String {
        let mut s = format!("double d{}(double a0, double a1) {{\n", idx);
        let mut stack = vec!["a0".to_string(), "a1".to_string()];
        let mut t = 0;
        for &(k, v) in tokens {
            let name = format!("u{}", t); t += 1;
            let a = stack[(k as usize) % stack.len()].clone();
            let b = stack[(k as usize + 1) % stack.len()].clone();
            let c = ((v.unsigned_abs() % 1000) as f64) / 8.0 + 1.0;
            let stmt = match k % 6 {
                0 => format!("{name} = {a} + {b};"),
                1 => format!("{name} = {a} - {b};"),
                2 => format!("{name} = {a} * {b};"),
                3 => format!("{name} = {a} / {c};"),
                4 => format!("{name} = ({a} < {b}) ? {c} : {a};"),
                _ => format!("{name} = {a} + {c};"),
            };
            s.push_str(&format!("    double {};\n    {}\n", name, stmt));
            stack.push(name);
        }
        let ret = stack.last().cloned().unwrap_or_else(|| "0.0".to_string());
        s.push_str(&format!("    return {};\n}}\n", ret));
        s
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(160))]
        #[test]
        fn p3_e2e_double_codegen(
            f0t in prop::collection::vec(any::<(u8, i64)>(), 2..8),
            f1t in prop::collection::vec(any::<(u8, i64)>(), 2..8),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("d{:04}.c", id));
            let mut prog = String::from("#include <stdio.h>\n");
            prog.push_str(&render_dfn(0, &f0t));
            prog.push_str(&render_dfn(1, &f1t));
            prog.push_str("int main(void) {\n");
            for (x, y) in [(0.0f64, 1.5f64), (3.25f64, -2.0f64), (1e10f64, 7.0f64), (-0.5f64, 0.25f64)] {
                prog.push_str(&format!("    printf(\"%.17g\\n\", d0({}, {}));\n", x, y));
                prog.push_str(&format!("    printf(\"%.17g\\n\", d1({}, {}));\n", x, y));
            }
            prog.push_str("    return 0;\n}\n");
            std::fs::write(&src, prog).unwrap();
            let gcc_out = dir.join(format!("d{:04}_gcc", id));
            let o0 = dir.join(format!("d{:04}_o0", id));
            let o2 = dir.join(format!("d{:04}_o2", id));
            let src_str = src.to_str().unwrap();
            let g = Command::new("gcc").args(["-w", "-O2", "-o", gcc_out.to_str().unwrap(), src_str]).output().unwrap();
            prop_assert!(g.status.success());
            compile_with(&["-o", o0.to_str().unwrap(), src_str])
                .map_err(|e| proptest::test_runner::TestCaseError::fail(format!("ccc -O0 failed: {}", e)))?;
            compile_with(&["-O2", "-o", o2.to_str().unwrap(), src_str])
                .map_err(|e| proptest::test_runner::TestCaseError::fail(format!("ccc -O2 failed: {}", e)))?;
            let r = run_quiet(&gcc_out);
            let a = run_quiet(&o0);
            let b = run_quiet(&o2);
            prop_assert_eq!(&a, &r, "ccc -O0 != gcc on double program d{:04}", id);
            prop_assert_eq!(&b, &a, "ccc -O2 != ccc -O0 on double program d{:04}", id);
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&gcc_out);
            let _ = std::fs::remove_file(&o0); let _ = std::fs::remove_file(&o2);
        }
    }
}

#[cfg(test)]
mod pbt_asm_deep {
    //! Round 15: x86 assembler deep encoder sweep. Oracle: llvm-mc.
    //! Out of scope: ARM NEON encoders (teammate's campaign, #474-#773).
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CASE: AtomicUsize = AtomicUsize::new(0);

    fn run_dir() -> std::path::PathBuf {
        let d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/15_x86_asm_deep/run");
        let _ = std::fs::create_dir_all(&d);
        d
    }

    // ── operand pools ─────────────────────────────────────────────────────
    const G64: [&str; 16] = ["rax","rcx","rdx","rbx","rsp","rbp","rsi","rdi",
        "r8","r9","r10","r11","r12","r13","r14","r15"];
    const G32: [&str; 16] = ["eax","ecx","edx","ebx","esp","ebp","esi","edi",
        "r8d","r9d","r10d","r11d","r12d","r13d","r14d","r15d"];
    const G16: [&str; 16] = ["ax","cx","dx","bx","sp","bp","si","di",
        "r8w","r9w","r10w","r11w","r12w","r13w","r14w","r15w"];
    const G8: [&str; 16] = ["al","cl","dl","bl","spl","bpl","sil","dil",
        "r8b","r9b","r10b","r11b","r12b","r13b","r14b","r15b"];
    const XMM: [&str; 16] = ["xmm0","xmm1","xmm2","xmm3","xmm4","xmm5","xmm6","xmm7",
        "xmm8","xmm9","xmm10","xmm11","xmm12","xmm13","xmm14","xmm15"];
    const YMM: [&str; 16] = ["ymm0","ymm1","ymm2","ymm3","ymm4","ymm5","ymm6","ymm7",
        "ymm8","ymm9","ymm10","ymm11","ymm12","ymm13","ymm14","ymm15"];
    /// imm32-range edges (all valid for addq imm, r64: sign-extended imm32)
    const IMM32: [i64; 12] = [0, 1, -1, 127, -128, 255, 32767, -32768,
        2147483647, -2147483648, 65536, 1000000007];
    const IMM8: [i64; 8] = [0, 1, 7, 63, 127, -1, -128, 64];
    const IMM64: [i64; 6] = [0, 1, -1, 4294967296, 9223372036854775807, -9223372036854775808];

    fn g(i: u8) -> &'static str { G64[(i % 16) as usize] }
    fn g32(i: u8) -> &'static str { G32[(i % 16) as usize] }
    fn x(i: u8) -> &'static str { XMM[(i % 16) as usize] }
    fn y(i: u8) -> &'static str { YMM[(i % 16) as usize] }

    /// Render a VALID form for family `fam` from operand slots (a, b, c, imm).
    fn valid_form(fam: u8, a: u8, b: u8, c: u8, imm: u16) -> String {
        let i = imm as i64;
        let base = match fam % 18 {
            0 => format!("{}q %{}, %{}", ["add","sub","and","or","xor","cmp"][(a % 6) as usize], g(b), g(c)),
            1 => format!("{}b %{}, %{}", ["add","sub","and","or","xor","cmp"][(a % 6) as usize], G8[(b % 16) as usize], G8[(c % 16) as usize]),
            2 => format!("movq %{}, %{}", g(a), g(b)),
            3 => format!("movl %{}, %{}", g32(a), G32[(b % 16) as usize]),
            4 => format!("movabsq ${}, %{}", IMM64[(a % 6) as usize], g(b)),
            5 => format!("addq ${}, %{}", IMM32[(a % 12) as usize], g(b)),
            6 => format!("{}q ${}, %{}", ["shl","shr","sar"][(a % 3) as usize], IMM8[(b % 8) as usize], g(c)),
            7 => format!("{} %{}", ["bswap"][(a % 1) as usize], g(b)),
            8 => match a % 3 { 0 => format!("lzcnt %{}, %{}", g(b), g(c)), 1 => format!("tzcnt %{}, %{}", G32[(b%16) as usize], G32[(c%16) as usize]), _ => format!("popcnt %{}, %{}", g(b), g(c)) },
            9 => format!("{} %{}, %{}", ["movaps","movapd","movdqa"][(a % 3) as usize], x(b), x(c)),
            10 => format!("{} %{}, %{}", ["addps","addpd","paddb","paddq","pxor"][(a % 5) as usize], x(b), x(c)),
            11 => format!("pshufd ${}, %{}, %{}", IMM8[(a % 8) as usize] & 0xff, x(b), x(c)),
            12 => format!("{} %{}, %{}, %{}", ["vaddps","vaddpd","vpaddb","vpxor"][(a % 4) as usize], x(b), x(c), XMM[((b as usize + c as usize) % 16) as usize]),
            13 => format!("vaddps %{}, %{}, %{}", y(a), y(b), y(c)),
            14 => format!("vpbroadcast{} %{}, %{}", ["b","w","d","q"][(a % 4) as usize], x(b), x(c)),
            15 => format!("{} %{}, %{}, %{}", ["bzhi","pdep","pext"][(a % 3) as usize], g(b), g(c), g(a)),
            16 => format!("rorx ${}, %{}, %{}", IMM8[(a % 8) as usize], g(b), g(c)),
            _ => format!("{} {}(%{}), %{}", ["flds","movq","movl"][(a % 3) as usize], i % 1024, g(b), if a % 3 == 0 { G32[(c % 16) as usize] } else { g(c) }),
        };
        // ── sweep families (18..38): documented surfaces the first 18 missed ──
        if fam >= 18 {
            return match fam % 20 {
                0 => format!("leaq {}(%{},%{},{}), %{}", i % 64, g(a), g(b), [1u8,2,4,8][(c % 4) as usize], g((a.wrapping_add(1)) % 16)),
                1 => format!("testq %{}, %{}", g(a), g(b)),
                2 => format!("xchgq %{}, %{}", g(a), g(b)),
                3 => format!("cmov{} %{}, %{}", ["ne","e","l","g","le","ge","a","b"][(a % 8) as usize], g(b), g(c)),
                4 => format!("set{} %{}", ["e","ne","l","g","le","ge"][(a % 6) as usize], G8[(b % 16) as usize]),
                5 => format!("imulq %{}, %{}", g(a), g(b)),
                6 => format!("imulq ${}, %{}, %{}", i % 128, g(a), g(b)),
                7 => format!("idivq %{}", g(a)),
                8 => format!("mulq %{}", g(a)),
                9 => format!("adcq %{}, %{}", g(a), g(b)),
                10 => format!("movss %{}, %{}", x(a), x(b)),
                11 => format!("movsd %{}, %{}", x(a), x(b)),
                12 => format!("cvtss2sd %{}, %{}", x(a), x(b)),
                13 => format!("cvtsd2ss %{}, %{}", x(a), x(b)),
                14 => format!("cvtsi2sdq %{}, %{}", g(a), x(b)),
                15 => format!("psrlq ${}, %{}", i % 64, x(a)),
                16 => format!("vpermd %{}, %{}, %{}", y(a), y(b), y(c)),
                17 => format!("crc32b %{}, %{}", G8[(a % 4) as usize], G32[(b % 16) as usize]),
                18 => match a % 4 { 0 => "rdtsc".to_string(), 1 => "cpuid".to_string(), 2 => "cqo".to_string(), _ => "cdq".to_string() },
                _ => format!("{} %{}, %{}", ["minps","maxps","minpd","maxpd","minss","maxss","minsd","maxsd"][(a % 8) as usize], x(b), x(c)),
            };
        }
        base
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

    /// Returns Ok(bytes) if ccc assembled, Err(msg) if it rejected.
    fn ccc_assemble(src: &std::path::Path, out: &std::path::Path) -> Result<String, String> {
        let argv: Vec<String> = vec![
            "ccc".into(), "-c".into(), "-o".into(), out.to_str().unwrap().into(), src.to_str().unwrap().into(),
        ];
        let mut d = Driver::new();
        let parsed = d.parse_cli_args(&argv)?;
        if parsed { return Err("flag consumed".into()); }
        d.run()?;
        if !out.exists() { return Err("no output".into()); }
        Ok(text_hex(out))
    }

    fn mc_assemble(src: &std::path::Path, out: &std::path::Path) -> Result<String, String> {
        let o = Command::new("llvm-mc")
            .args(["-triple=x86_64", "-filetype=obj", "-o", out.to_str().unwrap(), src.to_str().unwrap()])
            .output().unwrap();
        if !o.status.success() { return Err(String::from_utf8_lossy(&o.stderr).to_string()); }
        Ok(text_hex(out))
    }

    /// P1: valid forms — both assemblers accept ⇒ bytes equal.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]
        #[test]
        fn p1_x86_asm_valid_forms_match_llvm_mc(
            fam in 0u8..18, a in any::<u8>(), b in any::<u8>(), c in any::<u8>(), imm in any::<u16>(),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("v{:04}.s", id));
            let form = valid_form(fam, a, b, c, imm);
            std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
            let ccc_o = dir.join(format!("v{:04}_c.o", id));
            let mc_o = dir.join(format!("v{:04}_m.o", id));
            let mc = mc_assemble(&src, &mc_o);
            let cc = ccc_assemble(&src, &ccc_o);
            // Collect DISTINCT failure signatures per run (mnemonic-level
            // dedup) so one sweep reveals every gap class; assert at the end.
            use std::sync::Mutex;
            static GAPS: Mutex<Option<std::collections::BTreeSet<String>>> = Mutex::new(None);
            match (&mc, &cc) {
                (Ok(m), Ok(c)) => {
                    if c != m {
                        prop_assert_eq!(c, m, "byte mismatch for: {}", form);
                    }
                }
                (Err(_), Ok(c)) => {
                    // llvm-mc rejects but ccc accepted: only a bug if GNU as
                    // (the second reference) also rejects — validated in triage;
                    // record the signature now.
                    let mut g = GAPS.lock().unwrap();
                    g.get_or_insert_with(Default::default)
                        .insert(format!("SILENT-ACCEPT {} (bytes {})", form, &c[..c.len().min(24)]));
                }
                (Ok(_), Err(e)) => {
                    // double-check with GNU as before calling it a gap
                    let gas = Command::new("as").args(["-o", dir.join(format!("v{:04}_g.o", id)).to_str().unwrap(), src.to_str().unwrap()]).output().unwrap();
                    let gas_ok = gas.status.success();
                    let mut g = GAPS.lock().unwrap();
                    g.get_or_insert_with(Default::default)
                        .insert(format!("REJECT-VALID(as_ok={}) {} — {}", gas_ok, form, e.chars().take(60).collect::<String>()));
                }
                (Err(_), Err(_)) => {}
            }
            // final assertion on accumulated gaps (empty ⇒ green)
            {
                let g = GAPS.lock().unwrap();
                if let Some(set) = g.as_ref() {
                    prop_assert!(set.is_empty(), "distinct failure classes ({}): {:?}", set.len(), set);
                }
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o);
        }
    }

    /// P2: invalid mutations — llvm-mc rejects ⇒ ccc must reject too.
    fn mutate(form: &str, m: u8, slot: u8) -> String {
        // token-level mutations on the rendered form
        let words: Vec<&str> = form.split_whitespace().collect();
        let mnem = words[0];
        let joined = if words.len() > 1 { words[1..].join(" ") } else { String::new() };
        let ops: Vec<String> = if words.len() > 1 { joined.split(", ").map(|s| s.to_string()).collect() } else { vec![] };
        let n = ops.len().max(1);
        let at = (slot as usize) % n;
        match m % 4 {
            0 => {
                // width/class swap on one register operand
                let mut o = ops.clone();
                let t = o[at].clone();
                let swapped = if t.starts_with("%ymm") { t.replace("ymm", "xmm") }
                    else if t.starts_with("%xmm") { t.replace("xmm", "ymm") }
                    else if t.starts_with("%e") || t.ends_with('d') { t.trim_end_matches('d').to_string() + "w" }
                    else { t.to_string() + "d" };
                o[at] = swapped;
                format!("{} {}", mnem, o.join(", "))
            }
            1 => {
                // immediate overflow: bump any $imm out of range
                let mut o = ops.clone();
                let mut done = false;
                for i in 0..o.len() {
                    if o[i].starts_with('$') && !done {
                        let v: i128 = o[i][1..].parse().unwrap_or(0);
                        o[i] = format!("${}", (v as i128).unsigned_abs() as i128 + 4294967296i128);
                        done = true;
                    }
                }
                if !done { format!("{} $4294967296, {}", mnem, ops.join(", ")) } else { format!("{} {}", mnem, o.join(", ")) }
            }
            2 => {
                // register-class swap: %rXX <-> %xmmN
                let mut o: Vec<String> = ops.iter().map(|s| s.to_string()).collect();
                let t = o[at].clone();
                if t.starts_with('%') && !t.contains('(') {
                    o[at] = if t.contains("xmm") || t.contains("ymm") { "%rax".to_string() } else { "%xmm0".to_string() };
                }
                format!("{} {}", mnem, o.join(", "))
            }
            _ => {
                // extra operand
                let sep = if ops.is_empty() { "" } else { ", " };
                format!("{} {}{}%r11", mnem, ops.join(", "), sep)
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]
        #[test]
        fn p2_x86_asm_invalid_forms_rejected(
            fam in 0u8..18, a in any::<u8>(), b in any::<u8>(), c in any::<u8>(),
            imm in any::<u16>(), m in 0u8..4, slot in any::<u8>(),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let base = valid_form(fam, a, b, c, imm);
            let form = mutate(&base, m, slot);
            use std::sync::Mutex;
            static GAPS: Mutex<Option<std::collections::BTreeSet<String>>> = Mutex::new(None);
            let src = dir.join(format!("i{:04}.s", id));
            std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
            let ccc_o = dir.join(format!("i{:04}_c.o", id));
            let mc_o = dir.join(format!("i{:04}_m.o", id));
            let mc = mc_assemble(&src, &mc_o);
            let cc = ccc_assemble(&src, &ccc_o);
            if mc.is_err() {
                // Both references must agree it's invalid before we demand
                // rejection: GNU as gets the final word (llvm-mc alone may be
                // stricter than the GNU convention).
                let gas = Command::new("as").args(["-o", dir.join(format!("i{:04}_g.o", id)).to_str().unwrap(), src.to_str().unwrap()]).output().unwrap();
                if cc.is_ok() && !gas.status.success() {
                    let mut g = GAPS.lock().unwrap();
                    g.get_or_insert_with(Default::default)
                        .insert(format!("SILENT-ACCEPT(mutation {}) {} (base: {})", m % 4, form, base));
                }
            }
            {
                let g = GAPS.lock().unwrap();
                if let Some(set) = g.as_ref() {
                    prop_assert!(set.is_empty(), "distinct invalid-accept classes ({}): {:?}", set.len(), set);
                }
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o);
        }
    }

    /// RED WITNESSES — round 15 (issues #774-#777), all double-reference
    /// confirmed (GNU as AND llvm-mc agree; ccc disagrees).
    #[test]
    fn test_x86asm_regression_gpr_width_rejected() {
        // #774: width mismatch must be rejected, not silently coerced.
        let dir = run_dir();
        let src = dir.join("reg774.s");
        std::fs::write(&src, ".text\nmovl %eax, %r8w\n").unwrap();
        let out = dir.join("reg774.o");
        assert!(ccc_assemble(&src, &out).is_err(), "#774: movl %eax, %r8w silently assembled");
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn test_x86asm_regression_simd_width_rejected() {
        // #775: xmm operand in a ymm form must be rejected.
        let dir = run_dir();
        let src = dir.join("reg775.s");
        std::fs::write(&src, ".text\nvaddps %ymm0, %ymm1, %xmm2\n").unwrap();
        let out = dir.join("reg775.o");
        assert!(ccc_assemble(&src, &out).is_err(), "#775: vaddps ymm/ymm/xmm silently assembled");
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn test_x86asm_regression_imm32_overflow_rejected() {
        // #776: out-of-range immediate must be rejected, not truncated to 0.
        let dir = run_dir();
        let src = dir.join("reg776.s");
        std::fs::write(&src, ".text\naddq $4294967296, %rax\n").unwrap();
        let out = dir.join("reg776.o");
        assert!(ccc_assemble(&src, &out).is_err(), "#776: addq $2^32 silently truncated to $0");
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }

    #[test]
    fn test_x86asm_regression_bare_popcnt_assembles() {
        // #777: suffix-less popcnt/lzcnt/tzcnt are valid (as + llvm-mc both
        // assemble them) — ccc must infer the width, not reject.
        let dir = run_dir();
        let src = dir.join("reg777.s");
        std::fs::write(&src, ".text\npopcnt %rax, %rbx\nlzcnt %eax, %ecx\ntzcnt %rax, %rbx\n").unwrap();
        let out = dir.join("reg777.o");
        assert!(ccc_assemble(&src, &out).is_ok(), "#777: bare popcnt/lzcnt/tzcnt rejected");
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }

    /// P3 (sweep round): documented surfaces the first 18 families missed —
    /// lea/test/xchg/cmov/setcc/imul/idiv/mul/adc, scalar SSE movs and
    /// conversions, psrlq, vpermd, crc32, rdtsc/cpuid/cqo/cdq, and the
    /// min/max family. Same protocol as p1.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]
        #[test]
        fn p3_x86_asm_sweep_families(
            fam in 18u8..38, a in any::<u8>(), b in any::<u8>(), c in any::<u8>(), imm in any::<u16>(),
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = run_dir();
            let src = dir.join(format!("w{:04}.s", id));
            let form = valid_form(fam, a, b, c, imm);
            std::fs::write(&src, format!(".text\n{}\n", form)).unwrap();
            let ccc_o = dir.join(format!("w{:04}_c.o", id));
            let mc_o = dir.join(format!("w{:04}_m.o", id));
            let mc = mc_assemble(&src, &mc_o);
            let cc = ccc_assemble(&src, &ccc_o);
            use std::sync::Mutex;
            static GAPS3: Mutex<Option<std::collections::BTreeSet<String>>> = Mutex::new(None);
            match (&mc, &cc) {
                (Ok(m), Ok(c)) => prop_assert_eq!(c, m, "byte mismatch for: {}", form),
                (Err(_), Ok(c)) => {
                    let gas = Command::new("as").args(["-o", dir.join(format!("w{:04}_g.o", id)).to_str().unwrap(), src.to_str().unwrap()]).output().unwrap();
                    if !gas.status.success() {
                        let mut g = GAPS3.lock().unwrap();
                        g.get_or_insert_with(Default::default).insert(format!("SILENT-ACCEPT {}", form));
                    }
                }
                (Ok(_), Err(_)) => {
                    let gas = Command::new("as").args(["-o", dir.join(format!("w{:04}_g.o", id)).to_str().unwrap(), src.to_str().unwrap()]).output().unwrap();
                    if gas.status.success() {
                        let mut g = GAPS3.lock().unwrap();
                        g.get_or_insert_with(Default::default).insert(format!("REJECT-VALID {}", form));
                    }
                }
                (Err(_), Err(_)) => {}
            }
            {
                let g = GAPS3.lock().unwrap();
                if let Some(set) = g.as_ref() {
                    prop_assert!(set.is_empty(), "sweep failure classes ({}): {:?}", set.len(), set);
                }
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&ccc_o); let _ = std::fs::remove_file(&mc_o);
        }
    }

    /// RED WITNESS — round 15 sweep (issue #782): packed min/max missing.
    #[test]
    fn test_x86asm_regression_packed_minmax_assembles() {
        let dir = run_dir();
        let src = dir.join("reg782.s");
        std::fs::write(&src, ".text\nminps %xmm0, %xmm1\nmaxps %xmm2, %xmm3\nminpd %xmm4, %xmm5\nmaxpd %xmm6, %xmm7\n").unwrap();
        let out = dir.join("reg782.o");
        assert!(ccc_assemble(&src, &out).is_ok(), "#782: packed min/max family rejected");
        let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
    }
}
