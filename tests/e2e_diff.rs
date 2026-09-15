//! End-to-end differential property tests: ccc (x86-64) vs gcc on the same host.
//!
//! Oracle: differential — for every generated (UB-free, deterministic) C program,
//! the program compiled by ccc must behave identically to the one compiled by gcc:
//! same stdout AND same exit code.
//!
//! Spec evidence: DESIGN_DOC.md:164 "x86-64 code generation (SysV AMD64 ABI)";
//! README.md "A C compiler ... produces ELF executables".
//!
//! Generators are seeded from proptest; on failure the full C source is printed
//! as the reproducer.

use proptest::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static CASE_ID: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// Infrastructure: temp dir, compile, run, compare
// ---------------------------------------------------------------------------

struct CaseDir(PathBuf);

impl CaseDir {
    fn new(tag: &str) -> Self {
        let id = CASE_ID.fetch_add(1, Ordering::Relaxed);
        let d = std::env::temp_dir().join(format!("ccc_pbt_{}_{}", tag, id));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("mkdir");
        CaseDir(d)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for CaseDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Compile `src` with the given compiler; returns Err(msg) if compile fails.
fn compile(compiler: &str, dir: &CaseDir, src: &str) -> Result<(), String> {
    std::fs::write(dir.path("main.c"), src).unwrap();
    let out = Command::new("timeout")
        .args(["30", compiler, "-w", "main.c", "-o", "a.out"])
        .current_dir(&dir.0)
        .output()
        .map_err(|e| format!("spawn {}: {}", compiler, e))?;
    if !out.status.success() {
        return Err(format!(
            "{} compile failed ({}):\n{}",
            compiler,
            out.status,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(())
}

/// Run ./a.out under `timeout`; returns (stdout, exit_code) or Err on timeout/spawn.
fn run(dir: &CaseDir) -> Result<(String, i32), String> {
    let out = Command::new("timeout")
        .args(["10", "./a.out"])
        .current_dir(&dir.0)
        .output()
        .map_err(|e| format!("spawn timeout: {}", e))?;
    let code = out.status.code().unwrap_or(-1);
    if code == 124 {
        return Err("runtime timeout (10s)".to_string());
    }
    Ok((String::from_utf8_lossy(&out.stdout).into_owned(), code))
}

/// Full differential check for one program. Ok(()) = equivalence holds.
/// Err starting with "gcc" means the generated program was rejected by the
/// reference compiler (test-side problem) — callers discard those.
fn assert_equivalent(src: &str, tag: &str) -> Result<(), String> {
    let gdir = CaseDir::new(tag);
    compile("gcc", &gdir, src)?;
    let want = run(&gdir)?;

    let cdir = CaseDir::new(tag);
    compile(env!("CARGO_BIN_EXE_ccc"), &cdir, src)?;
    let got = run(&cdir)?;

    if got != want {
        return Err(format!(
            "MISMATCH\n--- gcc (stdout, exit): {:?} {}\n--- ccc (stdout, exit): {:?} {}\n--- diff-run first divergence: {}",
            want.0, want.1,
            got.0, got.1,
            first_divergence(&want.0, &got.0),
        ));
    }
    Ok(())
}

fn first_divergence(a: &str, b: &str) -> String {
    for (i, (ca, cb)) in a.bytes().zip(b.bytes()).enumerate() {
        if ca != cb {
            return format!("byte {} of stdout: gcc {:?} vs ccc {:?}", i, ca as char, cb as char);
        }
    }
    if a.len() != b.len() {
        return format!("stdout length: gcc {} vs ccc {}", a.len(), b.len());
    }
    format!("stdout equal; exit codes differ").to_string()
}

/// proptest wrapper: on failure, include the full generated source.
// ---------------------------------------------------------------------------
// Tiny deterministic RNG (SplitMix64) — generator infrastructure only.
// proptest still drives case count, seeds, and shrinking of the seed.
// ---------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn u32r(&mut self, lo: u32, hi: u32) -> u32 {
        if hi <= lo { return lo; }
        lo + (self.next() % ((hi - lo + 1) as u64)) as u32
    }
    fn pick<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[(self.next() % v.len() as u64) as usize]
    }
    fn chance(&mut self, pct: u32) -> bool {
        ((self.next() % 100) as u32) < pct
    }
}

// ---------------------------------------------------------------------------
// P1: arithmetic expressions (UB-free by construction via value-range tracking)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ty { I32, U32, I64, U64 }

impl Ty {
    fn is_unsigned(self) -> bool { matches!(self, Ty::U32 | Ty::U64) }
    fn bits(self) -> u32 { if matches!(self, Ty::I32 | Ty::U32) { 32 } else { 64 } }
    fn min(self) -> i128 {
        match self { Ty::I32 => i32::MIN as i128, Ty::U32 => 0, Ty::I64 => i64::MIN as i128, Ty::U64 => 0 }
    }
    fn max(self) -> i128 {
        match self { Ty::I32 => i32::MAX as i128, Ty::U32 => u32::MAX as i128, Ty::I64 => i64::MAX as i128, Ty::U64 => u64::MAX as i128 }
    }
    fn name(self) -> &'static str {
        match self { Ty::I32 => "int", Ty::U32 => "unsigned", Ty::I64 => "long", Ty::U64 => "unsigned long" }
    }
    fn fmt(self) -> &'static str {
        match self { Ty::I32 => "%d", Ty::U32 => "%u", Ty::I64 => "%ld", Ty::U64 => "%lu" }
    }
}

/// A generated expression with a conservative value range [lo, hi] (i128) so
/// that every emitted operation stays defined per C11 (no signed overflow, no
/// division by zero, shift counts in range, signed shift operands >= 0 —
/// except an explicitly-flagged impl-defined `>>` probe).
#[derive(Clone)]
struct E {
    ty: Ty,
    lo: i128,
    hi: i128,
    code: String,
    impl_defined: bool,
}

fn interesting(ty: Ty) -> Vec<i128> {
    let v = match ty {
        Ty::I32 => vec![0, 1, -1, 2, 7, -7, 127, 128, 255, 1000, -1000, 32767, -32768, 65535, 2147483647, -2147483648, 123456789, -123456789],
        Ty::U32 => vec![0, 1, 2, 7, 127, 128, 255, 1000, 32767, 65535, 2147483647, 2147483648, 4294967295, 4000000000, 123456789],
        Ty::I64 => vec![0, 1, -1, 2, 7, -7, 255, 1000, 65535, 2147483647, -2147483648, 4294967296, 9223372036854775807, -9223372036854775808, 12345678901234567, -12345678901234567],
        Ty::U64 => vec![0, 1, 2, 7, 255, 65535, 2147483648, 4294967295, 4294967296, 9223372036854775807, 18446744073709551615, 12345678901234567],
    };
    v.into_iter().filter(|&x| x >= ty.min() && x <= ty.max()).collect()
}

fn const_code(ty: Ty, v: i128) -> String {
    match ty {
        Ty::I32 => format!("{}", v),
        Ty::U32 => format!("{}u", v),
        Ty::I64 => format!("{}L", v),
        Ty::U64 => {
            if v == u64::MAX as i128 { "18446744073709551615UL".to_string() }
            else { format!("{}UL", v) }
        }
    }
}

fn wrap_u128(v: i128, ty: Ty) -> i128 {
    // reduce into the unsigned domain of ty, then view as the ty range
    let m = (1i128 << ty.bits());
    let u = ((v % m) + m) % m; // unsigned value in [0, 2^w)
    u
}

struct VarPool {
    vars: Vec<(String, Ty, i128, i128)>, // name, ty, lo, hi
}

fn gen_expr(rng: &mut Rng, depth: u32, ty: Ty, pool: &VarPool) -> E {
    if depth == 0 {
        return gen_leaf(rng, ty, pool);
    }
    // occasionally cast a differently-typed subexpression (tests conversions)
    let op = rng.u32r(0, 13);
    let leafish = |rng: &mut Rng| gen_expr(rng, depth - 1, ty, pool);
    let e = match op {
        0 | 1 => { // Add / Sub
            let a = leafish(rng);
            let b = leafish(rng);
            let sub = op == 1;
            let range = if sub {
                a.lo.checked_sub(b.hi).and_then(|lo| a.hi.checked_sub(b.lo).map(|hi| (lo, hi)))
            } else {
                a.lo.checked_add(b.lo).and_then(|lo| a.hi.checked_add(b.hi).map(|hi| (lo, hi)))
            };
            match range {
                Some((lo, hi)) if ty.is_unsigned() || (lo >= ty.min() && hi <= ty.max()) => {
                    let (lo, hi) = if ty.is_unsigned() {
                        (wrap_u128(lo, ty).min(wrap_u128(hi, ty)), wrap_u128(lo, ty).max(wrap_u128(hi, ty)))
                    } else { (lo, hi) };
                    E { ty, lo, hi, code: format!("({} {} {})", a.code, if sub { "-" } else { "+" }, b.code), impl_defined: a.impl_defined || b.impl_defined }
                }
                _ => gen_leaf(rng, ty, pool),
            }
        }
        2 => { // Mul
            let a = leafish(rng);
            let b = leafish(rng);
            let cands = [a.lo.checked_mul(b.lo), a.lo.checked_mul(b.hi), a.hi.checked_mul(b.lo), a.hi.checked_mul(b.hi)];
            if cands.iter().all(|c| c.is_some()) {
                let mut c: Vec<i128> = cands.iter().map(|x| x.unwrap()).collect();
                c.sort();
                let (lo, hi) = (c[0], c[3]);
                if ty.is_unsigned() || (lo >= ty.min() && hi <= ty.max()) {
                    E { ty, lo, hi, code: format!("({} * {})", a.code, b.code), impl_defined: a.impl_defined || b.impl_defined }
                } else { gen_leaf(rng, ty, pool) }
            } else { gen_leaf(rng, ty, pool) }
        }
        3 | 4 => { // Div / Mod by a nonzero constant
            let d = match rng.pick(&[1i64, 2, 3, 7, 10, 100, 1000, -1, -3, -7, -1000]) { &x => x };
            let a = leafish(rng);
            // |result| <= max(|lo|,|hi|)/|d| + 1 ; |rem| < |d|
            let bound = a.lo.abs().max(a.hi.abs()) as i128 / d.abs() as i128 + 1;
            let ok = if !ty.is_unsigned() {
                bound < ty.max() && bound < -ty.min().abs() // fits signed
                    && !(d == -1 && a.lo <= ty.min() && a.hi >= ty.min()) // avoid MIN/-1
            } else { true };
            if !ok { return gen_leaf(rng, ty, pool); }
            let (lo, hi) = if op == 3 { (-bound, bound) } else { (-(d.abs() as i128 - 1), d.abs() as i128 - 1) };
            E { ty, lo, hi, code: format!("({} {} {})", a.code, if op == 3 { "/" } else { "%" }, d), impl_defined: a.impl_defined }
        }
        5 => { // Shl by constant count < width (unsigned wraps, signed range-checked)
            let c = rng.u32r(0, ty.bits() - 1) as u32;
            let a = leafish(rng);
            let mut cands = [a.lo << c, a.hi << c];
            cands.sort();
            if ty.is_unsigned() || (a.lo >= 0 && cands[1] <= ty.max()) {
                E { ty, lo: if ty.is_unsigned() { 0 } else { cands[0] }, hi: if ty.is_unsigned() { ty.max() } else { cands[1] }, code: format!("({} << {})", a.code, c), impl_defined: a.impl_defined }
            } else { gen_leaf(rng, ty, pool) }
        }
        6 => { // Shr by constant count
            let c = rng.u32r(0, ty.bits() - 1) as u32;
            let a = leafish(rng);
            let impl_probe = !ty.is_unsigned() && a.lo < 0 && rng.chance(10); // impl-defined probe (gcc: arithmetic shift)
            if ty.is_unsigned() || a.lo >= 0 || impl_probe {
                let hi = a.hi.max(0) >> c;
                E { ty, lo: if impl_probe { ty.min() } else { a.lo.max(0) >> c }, hi, code: format!("({} >> {})", a.code, c), impl_defined: a.impl_defined || impl_probe }
            } else { gen_leaf(rng, ty, pool) }
        }
        7 | 8 | 9 => { // BitAnd / BitOr / BitXor
            let a = leafish(rng);
            let b = leafish(rng);
            let name = ["&", "|", "^"][(op - 7) as usize];
            E { ty, lo: ty.min(), hi: ty.max(), code: format!("({} {} {})", a.code, name, b.code), impl_defined: a.impl_defined || b.impl_defined }
        }
        10 | 11 | 12 | 13 => { // Comparisons -> cast result to requested ty
            let cty = [Ty::I32, Ty::U32, Ty::I64, Ty::U64][rng.u32r(0, 3) as usize];
            let a = gen_expr(rng, depth - 1, cty, pool);
            let b = gen_expr(rng, depth - 1, cty, pool);
            let name = ["==", "!=", "<", "<=", ">", ">="][(op - 10) as usize % 6];
            let cmp = format!("({} {} {})", a.code, name, b.code);
            if ty == Ty::I32 {
                E { ty: Ty::I32, lo: 0, hi: 1, code: cmp, impl_defined: a.impl_defined || b.impl_defined }
            } else {
                E { ty, lo: 0, hi: 1, code: format!("(({})({}))", ty.name(), cmp), impl_defined: a.impl_defined || b.impl_defined }
            }
        }
        _ => unreachable!(),
    };
    e
}

thread_local! {
    static BOUNDARY_ONLY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn gen_leaf(rng: &mut Rng, ty: Ty, pool: &VarPool) -> E {
    if !pool.vars.is_empty() && rng.chance(40) {
        let compatible: Vec<_> = pool.vars.iter().filter(|v| v.1 == ty).collect();
        if !compatible.is_empty() {
            let v = *rng.pick(&compatible);
            return E { ty, lo: v.2, hi: v.3, code: v.0.clone(), impl_defined: false };
        }
    }
    let pool_v: Vec<i128> = if BOUNDARY_ONLY.get() {
        interesting(ty).into_iter().filter(|x| x.abs() > 30000 || *x == 0 || *x == 1 || *x == -1).collect()
    } else {
        interesting(ty)
    };
    let v = *rng.pick(&pool_v);
    E { ty, lo: v, hi: v, code: const_code(ty, v), impl_defined: false }
}

/// Cast an expression to another type, only when the result is defined
/// (unsigned narrowing = modulo; signed narrowing only when value fits).
fn gen_cast(rng: &mut Rng, depth: u32, ty: Ty, pool: &VarPool) -> E {
    let from = [Ty::I32, Ty::U32, Ty::I64, Ty::U64][rng.u32r(0, 3) as usize];
    let a = gen_expr(rng, depth.saturating_sub(1), from, pool);
    let fits = a.lo >= ty.min() && a.hi <= ty.max();
    if ty.is_unsigned() || fits {
        let (lo, hi) = if ty.is_unsigned() {
            if from.bits() >= ty.bits() {
                (0, ty.max()) // wraps
            } else {
                (a.lo.max(0), a.hi.max(0)) // widening from unsigned or positive
            }
        } else if from.bits() < ty.bits() {
            if from.is_unsigned() { (a.lo.max(0), a.hi.max(0)) } else { (a.lo, a.hi) }
        } else {
            (a.lo, a.hi)
        };
        E { ty, lo, hi, code: format!("(({})({}))", ty.name(), a.code), impl_defined: a.impl_defined }
    } else {
        gen_leaf(rng, ty, pool)
    }
}

fn arith_program(seed: u64) -> String {
    arith_program_mode(seed, false)
}

fn arith_program_mode(seed: u64, boundary_only: bool) -> String {
    BOUNDARY_ONLY.with(|b| b.set(boundary_only));
    let mut rng = Rng(seed ^ 0xA5A5_1234);
    let mut pool = VarPool { vars: vec![] };
    let mut lines = vec![
        "#include <stdio.h>".to_string(),
        "int main(void) {".to_string(),
    ];
    for i in 0..4usize {
        let ty = [Ty::I32, Ty::U32, Ty::I64, Ty::U64][i];
        let v = *rng.pick(&interesting(ty));
        let name = format!("v{}", i);
        lines.push(format!("    {} {} = {};", ty.name(), name, const_code(ty, v)));
        pool.vars.push((name, ty, v, v));
    }
    let n_exprs = rng.u32r(10, 22);
    for i in 0..n_exprs {
        let r: u32 = rng.u32r(0, 99);
        let ty = [Ty::I32, Ty::U32, Ty::I64, Ty::U64][rng.u32r(0, 3) as usize];
        let depth = if boundary_only { 4 } else { 3 };
        let cast_pct = if boundary_only { 30 } else { 15 };
        let e = if r < cast_pct { gen_cast(&mut rng, depth, ty, &pool) } else { gen_expr(&mut rng, depth, ty, &pool) };
        lines.push(format!("    printf(\"{} {}\\n\", {});", i, e.ty.fmt(), e.code));
    }
    let ret_ty = Ty::I64;
    let ret = gen_expr(&mut rng, 2, ret_ty, &pool);
    lines.push(format!("    printf(\"R {}\\n\", {});", ret.ty.fmt(), ret.code));
    lines.push("    return (int)((unsigned long)({}) % 100);".replace("{}", &ret.code));
    lines.push("}".to_string());
    let out = lines.join("\n");
    BOUNDARY_ONLY.with(|b| b.set(false));
    out
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P1: every UB-free integer-expression program behaves identically
    /// under ccc and gcc (stdout + exit code).
    #[test]
    fn e2e_arith_expr(seed in any::<u64>()) {
        let src = arith_program(seed);
        prop_check_helper(src, "arith")?;
    }
}

fn prop_check_helper(src: String, tag: &'static str) -> Result<(), proptest::test_runner::TestCaseError> {
    match assert_equivalent(&src, tag) {
        Ok(()) => Ok(()),
        // gcc rejecting the program is a generator (test-side) problem: discard.
        Err(e) if e.starts_with("gcc compile failed") => Err(proptest::test_runner::TestCaseError::reject("gcc rejected program")),
        Err(e) => Err(proptest::test_runner::TestCaseError::fail(format!("reproducer source:\n{}\n{}", src, e))),
    }
}

// ---------------------------------------------------------------------------
// P2: control flow (bounded nested loops, if/else, break/continue, arrays)
// ---------------------------------------------------------------------------

fn control_flow_program(seed: u64) -> String {
    let mut rng = Rng(seed ^ 0xC0FF_EE01);
    let mut lines = vec![
        "#include <stdio.h>".to_string(),
        "int main(void) {".to_string(),
        "    long acc = 0;".to_string(),
        "    unsigned long uacc = 0;".to_string(),
        "    long arr[8] = {0,0,0,0,0,0,0,0};".to_string(),
    ];
    let depth = rng.u32r(1, 3);
    // statement emitter (manual recursion below)
    fn emit(rng: &mut Rng, lines: &mut Vec<String>, depth: u32, indent: usize, budget: &mut u32) {
        let pad = "    ".repeat(indent);
        while *budget > 0 {
            *budget -= 1;
            let choice = rng.u32r(0, 9);
            match choice {
                0..=3 => { // acc update with small defined arithmetic
                    let d = *rng.pick(&[1i64, 2, 3, 5, -1, -2, -7, 17, -100]);
                    match rng.u32r(0, 3) {
                        0 => lines.push(format!("{}acc = acc % 1000000007 {} {};", pad, if d < 0 { "-" } else { "+" }, d.abs())),
                        1 => lines.push(format!("{}acc += (long)(arr[{}]);", pad, rng.u32r(0, 7))),
                        2 => lines.push(format!("{}acc = acc ^ {};", pad, d & 0xff)),
                        _ => lines.push(format!("{}acc = (acc << 3) | (acc & 7);", pad)),
                    }
                }
                4 => { // unsigned accumulate (wrapping defined)
                    lines.push(format!("{}uacc = uacc * 1103515245UL + 12345UL;", pad));
                }
                5 => { // array element write + read
                    let idx = rng.u32r(0, 7);
                    lines.push(format!("{}arr[{}] = acc % 1009;", pad, idx));
                    lines.push(format!("{}acc += (long)(unsigned long)uacc % 13;", pad));
                }
                6 if depth > 0 => { // nested bounded loop
                    let n = rng.u32r(1, 12);
                    let var = format!("i{}", depth);
                    lines.push(format!("{}for (int {} = 0; {} < {}; {}++) {{", pad, var, var, n, var));
                    emit(rng, lines, depth - 1, indent + 1, budget);
                    lines.push(format!("{}}}", pad));
                }
                7 if depth > 0 => { // if/else over induction-dependent condition
                    let v = rng.u32r(0, 7);
                    lines.push(format!("{}if (arr[{}] % 2 == 0) {{", pad, v));
                    lines.push(format!("{}    acc += 1;", pad));
                    if rng.chance(50) {
                        lines.push(format!("{}}} else {{", pad));
                        lines.push(format!("{}    acc -= 1;", pad));
                        if rng.chance(25) { // bounded continue in else
                            lines.push(format!("{}    if (acc > 1000000) continue;", pad));
                        }
                    }
                    lines.push(format!("{}}}", pad));
                }
                8 if depth > 0 => { // bounded while + break
                    lines.push(format!("{}int guard = 0;", pad));
                    lines.push(format!("{}while (guard < 50) {{", pad));
                    lines.push(format!("{}    guard++;", pad));
                    lines.push(format!("{}    acc += guard % 7;", pad));
                    if rng.chance(50) {
                        lines.push(format!("{}    if (acc % 11 == 3) break;", pad));
                    }
                    lines.push(format!("{}}}", pad));
                }
                _ => { // printf snapshot
                    lines.push(format!("{}printf(\"%ld %lu\\n\", acc, uacc);", pad));
                }
            }
        }
    }

    let mut budget = rng.u32r(12, 30);
    emit(&mut rng, &mut lines, depth, 1, &mut budget);
    lines.push("    for (int k = 0; k < 8; k++) printf(\"%ld\\n\", arr[k]);".to_string());
    lines.push("    printf(\"F %ld %lu\\n\", acc, uacc);".to_string());
    lines.push("    return (int)(acc % 100);".to_string());
    lines.push("}".to_string());
    lines.join("\n")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(400))]

    /// P1s (strengthening round): boundary-value-skewed, deeper (depth-4)
    /// arithmetic expressions with aggressive casts — same differential
    /// contract as P1, aimed at width/sign edges.
    #[test]
    fn e2e_arith_boundary(seed in any::<u64>()) {
        let src = arith_program_mode(seed, true);
        prop_check_helper(src, "arithb")?;
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P2: bounded control-flow programs behave identically under ccc and gcc.
    #[test]
    fn e2e_control_flow(seed in any::<u64>()) {
        let src = control_flow_program(seed);
        prop_check_helper(src, "ctrl")?;
    }
}

// ---------------------------------------------------------------------------
// P3: struct/union/array layout (sizeof, offsets, field values, bitfields)
// ---------------------------------------------------------------------------

fn struct_layout_program(seed: u64) -> String {
    let mut rng = Rng(seed ^ 0xFEED_BEEF);
    let scalar_types = ["char", "signed char", "unsigned char", "short", "unsigned short", "int", "unsigned", "long", "unsigned long", "long long", "float", "double"];
    let init_vals = ["1", "2", "7", "42", "255", "1000", "-3", "12345"];

    let mut lines = vec!["#include <stdio.h>".to_string()];

    // struct A with 3-7 scalar/array fields
    let nfields = rng.u32r(3, 7);
    lines.push("struct A {".to_string());
    let mut a_fields = vec![];
    for i in 0..nfields {
        let t = *rng.pick(&scalar_types);
        let f = format!("f{}", i);
        if rng.chance(30) {
            let n = rng.u32r(2, 9);
            lines.push(format!("    {} {}[{}];", t, f, n));
            a_fields.push((f, Some(n)));
        } else {
            lines.push(format!("    {} {};", t, f));
            a_fields.push((f, None));
        }
    }
    lines.push("};".to_string());

    // struct B: nested A + bitfields
    lines.push("struct B {".to_string());
    lines.push("    struct A a;".to_string());
    lines.push("    unsigned bf0 : 3;".to_string());
    lines.push("    unsigned bf1 : 5;".to_string());
    lines.push("    int bf2 : 6;".to_string());
    let extra = rng.u32r(0, 3);
    for i in 0..extra {
        let t = *rng.pick(&scalar_types);
        lines.push(format!("    {} g{};", t, i));
    }
    lines.push("};".to_string());

    // union U
    lines.push("union U {".to_string());
    lines.push("    char c[16];".to_string());
    lines.push("    int i;".to_string());
    lines.push("    long l;".to_string());
    lines.push("    double d;".to_string());
    lines.push("};".to_string());

    lines.push("int main(void) {".to_string());
    // sizeof
    lines.push("    printf(\"sz %lu %lu %lu\\n\", (unsigned long)sizeof(struct A), (unsigned long)sizeof(struct B), (unsigned long)sizeof(union U));".to_string());
    // struct A field offsets + assignments
    lines.push("    struct A a;".to_string());
    for (f, arr_len) in &a_fields {
        lines.push(format!("    printf(\"off {} %ld\\n\", (long)((char*)&a.{} - (char*)&a));", f, f));
        if let Some(len) = arr_len {
            let n = rng.u32r(0, len - 1); // in bounds
            lines.push(format!("    a.{}[{}] = {};", f, n, *rng.pick(&init_vals)));
            lines.push(format!("    printf(\"val {} %d\\n\", (int)a.{}[{}]);", f, f, n));
        } else {
            lines.push(format!("    a.{} = {};", f, *rng.pick(&init_vals)));
            lines.push(format!("    printf(\"val {} %ld\\n\", (long)a.{});", f, f));
        }
    }
    // struct B: nested offsets + bitfield values (within width: bf0<=7, bf1<=31, bf2 in [-32,31])
    lines.push("    struct B b;".to_string());
    lines.push("    printf(\"off a %ld\\n\", (long)((char*)&b.a - (char*)&b));".to_string());
    lines.push("    b.bf0 = {};".replace("{}", &(rng.u32r(0, 7)).to_string()));
    lines.push("    b.bf1 = {};".replace("{}", &(rng.u32r(0, 31)).to_string()));
    let bf2 = *rng.pick(&[-32i64, -17, -1, 0, 1, 16, 31]);
    lines.push("    b.bf2 = {};".replace("{}", &bf2.to_string()));
    lines.push("    printf(\"bf %u %u %d\\n\", b.bf0, b.bf1, b.bf2);".to_string());
    for i in 0..extra {
        lines.push(format!("    b.g{} = {};", i, *rng.pick(&init_vals)));
        lines.push(format!("    printf(\"g{} %ld\\n\", (long)b.g{});", i, i));
    }
    // union: write each member, dump bytes
    lines.push("    union U u = {0};".to_string());
    lines.push("    u.i = -12345;".to_string());
    lines.push("    for (int i = 0; i < (int)sizeof(u); i++) printf(\"%02x\", (unsigned char)u.c[i]);".to_string());
    lines.push("    printf(\"\\n\");".to_string());
    lines.push("    u.d = 1.5;".to_string());
    lines.push("    for (int i = 0; i < (int)sizeof(u); i++) printf(\"%02x\", (unsigned char)u.c[i]);".to_string());
    lines.push("    printf(\"\\n\");".to_string());
    lines.push("    u.c[0] = 'x'; u.c[1] = 'y'; u.c[2] = 0;".to_string());
    lines.push("    printf(\"c %d %d %ld\\n\", u.c[0], u.c[1], u.l);".to_string());
    lines.push("    return 0;".to_string());
    lines.push("}".to_string());
    lines.join("\n")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P3: struct/union layout observables (sizeof, field offsets, values,
    /// bitfields, object bytes) are identical under ccc and gcc.
    #[test]
    fn e2e_struct_layout(seed in any::<u64>()) {
        let src = struct_layout_program(seed);
        prop_check_helper(src, "layout")?;
    }
}

// ---------------------------------------------------------------------------
// P4: calls & SysV ABI (arg counts 0..10, struct-by-value, recursion, fn ptrs)
// ---------------------------------------------------------------------------

fn calls_abi_program(seed: u64) -> String {
    let mut rng = Rng(seed ^ 0xAB12_CD34);
    let mut lines = vec!["#include <stdio.h>".to_string()];

    // small struct shapes exercising INTEGER/SSE/MEMORY classes
    lines.push("struct P2 { int a; int b; };".to_string());
    lines.push("struct D1 { double x; };".to_string());
    lines.push("struct MIX { long a; double b; };".to_string());
    lines.push("struct F2 { float a; float b; };".to_string());
    lines.push("struct BIG { long a[3]; };".to_string());

    // a k-arg scalar function; args mix types
    let nargs = rng.u32r(0, 10);
    let arg_types = ["int", "unsigned", "long", "unsigned long", "short", "char"];
    let mut params = vec![];
    let mut call_args = vec![];
    for i in 0..nargs {
        let t = *rng.pick(&arg_types);
        let v = (rng.next() % 1000) as i64;
        params.push(format!("{} a{}", t, i));
        call_args.push(format!("({}){}", t, v));
    }
    lines.push(format!("static long f({}) {{", params.join(", ")));
    // safe small-arithmetic body over the args
    lines.push("    long r = 0;".to_string());
    if nargs > 0 {
        for i in 0..nargs {
            match rng.u32r(0, 2) {
                0 => lines.push(format!("    r += (long)a{} % 97;", i)),
                1 => lines.push(format!("    r ^= (long)(unsigned long)(unsigned long)a{} << 3;", i)),
                _ => lines.push(format!("    r = r * 31 + (long)a{};", i)),
            }
        }
    } else {
        lines.push("    r = 5;".to_string());
    }
    lines.push("    return r;".to_string());
    lines.push("}".to_string());

    // struct-by-value helpers
    lines.push("static struct P2 mkp(int x) { struct P2 p; p.a = x; p.b = x + 1; return p; }".to_string());
    lines.push("static long usep(struct P2 p) { return (long)p.a * 3 + p.b; }".to_string());
    lines.push("static struct D1 mkd(double v) { struct D1 d; d.x = v; return d; }".to_string());
    lines.push("static double used(struct D1 d) { return d.x; }".to_string());
    lines.push("static struct MIX mkm(long a, double b) { struct MIX m; m.a = a; m.b = b; return m; }".to_string());
    lines.push("static long usem(struct MIX m) { return m.a + (long)m.b; }".to_string());
    lines.push("static struct F2 mkf(float a, float b) { struct F2 f; f.a = a; f.b = b; return f; }".to_string());
    lines.push("static float usef(struct F2 f) { return f.a - f.b; }".to_string());
    lines.push("static long usebig(struct BIG b) { return b.a[0] + b.a[1] + b.a[2]; }".to_string());

    // bounded recursion
    lines.push("static long rec(long n) { return n < 2 ? n : rec(n - 1) + rec(n - 2); }".to_string());

    // variant function-pointer trio
    lines.push("static long g0(long x) { return x + 1; }".to_string());
    lines.push("static long g1(long x) { return x * 2 - 1; }".to_string());
    lines.push("static long g2(long x) { return x ^ 0x5a; }".to_string());
    lines.push("static long (*fps[3])(long) = { g0, g1, g2 };".to_string());

    lines.push("int main(void) {".to_string());
    lines.push(format!("    printf(\"f %ld\\n\", f({}));", call_args.join(", ")));
    lines.push("    struct P2 p = mkp(41); printf(\"p %ld\\n\", usep(p));".to_string());
    lines.push("    struct P2 q = mkp(-7); printf(\"q %ld\\n\", usep(q));".to_string());
    lines.push("    struct D1 d = mkd(1.5); printf(\"d %.6f\\n\", used(d));".to_string());
    lines.push("    struct D1 e = mkd(-0.25); printf(\"e %.6f\\n\", used(e));".to_string());
    lines.push("    struct MIX m = mkm(9, 2.5); printf(\"m %ld\\n\", usem(m));".to_string());
    lines.push("    struct F2 fv = mkf(3.5f, 1.25f); printf(\"f2 %.6f\\n\", (double)usef(fv));".to_string());
    lines.push("    struct BIG bg; bg.a[0] = 11; bg.a[1] = 22; bg.a[2] = 33; printf(\"big %ld\\n\", usebig(bg));".to_string());
    let n = rng.u32r(2, 18);
    lines.push(format!("    printf(\"rec {} %ld\\n\", rec({}));", n, n));
    lines.push("    for (int i = 0; i < 9; i++) printf(\"fp %d %ld\\n\", i, fps[i % 3]((long)i));".to_string());
    lines.push("    return 0;".to_string());
    lines.push("}".to_string());
    lines.join("\n")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P4: calling-convention behavior (scalar args incl. >6, small struct
    /// by-value in both directions, MEMORY-class by-value, bounded recursion,
    /// function pointers) matches gcc.
    #[test]
    fn e2e_calls_abi(seed in any::<u64>()) {
        let src = calls_abi_program(seed);
        prop_check_helper(src, "abi")?;
    }
}

// ---------------------------------------------------------------------------
// P5: globals/statics/arrays with constant initializers
// ---------------------------------------------------------------------------

fn globals_program(seed: u64) -> String {
    let mut rng = Rng(seed ^ 0x0BADC0DE);
    let mut lines = vec!["#include <stdio.h>".to_string()];

    lines.push("int gi = -12345;".to_string());
    lines.push("unsigned gu = 4000000000u;".to_string());
    lines.push("long gl = -9223372036854775807L - 1L;".to_string()); // LONG_MIN via expression
    lines.push("unsigned long gul = 18446744073709551615UL;".to_string());
    lines.push("static int gs; // zero-initialized .bss".to_string());
    lines.push("long garr[8] = {1, -2, 3, -4};".to_string());
    lines.push("unsigned char gbytes[6] = {200, 100, 50, 25, 12, 6};".to_string());
    lines.push("char gstr[] = \"esc:\\t|\\\\|\\\"|\\x41|\\101|\\n\";".to_string());
    lines.push("int g2d[3][4] = {{1,2},{3},{4,5,6}};".to_string());
    lines.push("struct GS { int a; long b; char c[4]; } gstruct = {42, -9223372036854775807L, {'w','x','y','z'}};".to_string());
    lines.push("union GU { int i; char c[4]; } gunion = {0x41424344};".to_string());
    lines.push("const char *gmsg = \"const ptr string\";".to_string());
    lines.push("long gexpr = 6 * 7 + (1 << 10) - 3;".to_string());
    // extra randomized globals
    let nextra = rng.u32r(2, 6);
    let types = ["int", "unsigned", "long", "unsigned long", "short"];
    for i in 0..nextra {
        let t = *rng.pick(&types);
        let v = (rng.next() % 100000) as i64;
        lines.push(format!("{} gx{} = {};", t, i, v));
    }
    lines.push("static int bump(void) { static int s = 100; s += 7; return s; }".to_string());
    lines.push("static unsigned long fib_state(void) { static unsigned long a = 0, b = 1; unsigned long t = a + b; a = b; b = t; return a; }".to_string());

    lines.push("int main(void) {".to_string());
    lines.push("    printf(\"g %d %u %ld %lu %d\\n\", gi, gu, gl, gul, gs);".to_string());
    lines.push("    for (int i = 0; i < 8; i++) printf(\"a%d %ld\\n\", i, garr[i]);".to_string());
    lines.push("    for (int i = 0; i < 6; i++) printf(\"b%d %u\\n\", i, (unsigned)gbytes[i]);".to_string());
    lines.push("    for (int i = 0; i < 3; i++) for (int j = 0; j < 4; j++) printf(\"d%d%d %d\\n\", i, j, g2d[i][j]);".to_string());
    lines.push("    printf(\"s %d %ld %c%c%c%c\\n\", gstruct.a, gstruct.b, gstruct.c[0], gstruct.c[1], gstruct.c[2], gstruct.c[3]);".to_string());
    lines.push("    for (int i = 0; i < 4; i++) printf(\"u%d %02x\\n\", i, (unsigned char)gunion.c[i]);".to_string());
    lines.push("    printf(\"m %s\\n\", gmsg);".to_string());
    lines.push("    printf(\"e [%s]\\n\", gstr);".to_string());
    lines.push("    printf(\"x %ld\\n\", gexpr);".to_string());
    for i in 0..nextra {
        lines.push(format!("    printf(\"gx{} {}\\n\", gx{});", i, i, i));
    }
    lines.push("    int t1 = bump(); int t2 = bump(); int t3 = bump(); printf(\"s1 %d %d %d\\n\", t1, t2, t3);".to_string());
    lines.push("    for (int i = 0; i < 6; i++) printf(\"fib %lu\\n\", fib_state());".to_string());
    lines.push("    garr[0] = 77; printf(\"m0 %ld\\n\", garr[0]);".to_string());
    lines.push("    return (gi % 100 + 100) % 100;".to_string());
    lines.push("}".to_string());
    lines.join("\n")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P5: global/static data with constant initializers (.data/.bss, arrays,
    /// strings with escapes, nested/partial init, static locals) match gcc.
    #[test]
    fn e2e_globals_init(seed in any::<u64>()) {
        let src = globals_program(seed);
        prop_check_helper(src, "glob")?;
    }
}
