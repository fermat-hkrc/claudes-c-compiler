pub(crate) mod pipeline;
mod cli;
mod external_tools;
mod file_types;

pub use pipeline::{Driver, CompileMode};

#[cfg(test)]
mod pbt_tests {
    use crate::driver::Driver;
    use proptest::prelude::*;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CASE: AtomicUsize = AtomicUsize::new(0);

    /// P1: mode plumbing — each CompileMode produces its well-formed artifact
    /// for each selectable target (argv0 + -m32), end to end through Driver.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]
        #[test]
        fn p1_driver_mode_plumbing(
            i686 in any::<bool>(),
            mode in 0u8..4,
            ret in 0u8..121,
        ) {
            let id = CASE.fetch_add(1, Ordering::SeqCst);
            let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("pbt-out/rounds/14_driver/run");
            let _ = std::fs::create_dir_all(&dir);
            let src = dir.join(format!("d{:04}.c", id));
            let fname = format!("fn{:04}", id);
            std::fs::write(&src, format!(
                "int {}(int x) {{ return x + {}; }}\nint main(void) {{ return {}(0) - {}; }}\n",
                fname, ret, fname, ret)).unwrap();
            let mut argv: Vec<String> = vec!["ccc".to_string()];
            if i686 {
                argv.push("-m32".to_string());
                argv.push("-I".to_string());
                argv.push("/usr/lib/gcc/x86_64-linux-gnu/10/include".to_string()); // issue #770 workaround
            }
            let out = dir.join(format!("d{:04}_out", id));
            match mode % 4 {
                0 => argv.push("-E".to_string()),
                1 => argv.push("-S".to_string()),
                2 => argv.push("-c".to_string()),
                _ => {}
            }
            argv.push("-o".to_string());
            argv.push(out.to_str().unwrap().to_string());
            argv.push(src.to_str().unwrap().to_string());
            let mut d = Driver::new();
            let parsed = d.parse_cli_args(&argv).map_err(|e| proptest::test_runner::TestCaseError::fail(format!("cli: {}", e)))?;
            prop_assert!(!parsed);
            d.run().map_err(|e| proptest::test_runner::TestCaseError::fail(format!("driver failed: {}", e)))?;
            prop_assert!(out.exists(), "artifact not produced");
            match mode % 4 {
                0 | 1 => {
                    let text = std::fs::read_to_string(&out).unwrap();
                    prop_assert!(text.contains(&fname), "text artifact missing function name");
                }
                2 => {
                    let h = Command::new("readelf").args(["-h", out.to_str().unwrap()]).output().unwrap();
                    let t = String::from_utf8_lossy(&h.stdout);
                    prop_assert!(t.contains("REL (Relocatable file)"), "-c artifact not ET_REL");
                    prop_assert!(t.contains(if i686 { "80386" } else { "X86-64" }), "wrong machine");
                }
                _ => {
                    let st = Command::new(out.to_str().unwrap()).status().unwrap();
                    prop_assert!(st.success(), "executable returned non-zero: {:?}", st.code());
                }
            }
            let _ = std::fs::remove_file(&src); let _ = std::fs::remove_file(&out);
        }
    }
}
