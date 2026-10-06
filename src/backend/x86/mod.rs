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
