//! Shared property-based-testing support for the preprocessor (round 03).
//!
//! Test-only module (#[cfg(test)] in mod.rs): independent reference
//! implementations written from the documented contracts (C99 6.10.1
//! intmax/uintmax arithmetic, C11 6.10.3.2 stringification), a C token
//! tokenizer for token-stream comparison, and the gcc -E differential runner.

#![cfg(test)]

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

// ---------------------------------------------------------------------------
// C tokenizer (maximal munch) for token-stream comparison.
// ---------------------------------------------------------------------------

const TWO_CHAR_OPS: &[&str] = &[
    "<<=", ">>=", "...",
    "<<", ">>", "<=", ">=", "==", "!=", "&&", "||", "->", "++", "--",
    "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "##",
];

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}
fn is_ident_cont(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Tokenize preprocessed C text into a token stream (whitespace-insensitive).
pub fn tokens(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let len = b.len();
    let mut out = Vec::new();
    let mut i = 0;
    while i < len {
        let c = b[i];
        if c.is_ascii_whitespace() || c == b'\x01' || c == b'\x02' || c == b'\x03' {
            i += 1;
            continue;
        }
        // string / char literals
        if c == b'"' || c == b'\'' {
            let start = i;
            i += 1;
            while i < len {
                if b[i] == b'\\' && i + 1 < len {
                    i += 2;
                } else if b[i] == c {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            out.push(String::from_utf8_lossy(&b[start..i]).into_owned());
            continue;
        }
        // identifiers
        if is_ident_start(c) {
            let start = i;
            i += 1;
            while i < len && is_ident_cont(b[i]) {
                i += 1;
            }
            out.push(String::from_utf8_lossy(&b[start..i]).into_owned());
            continue;
        }
        // pp-numbers: digit, or . digit
        if c.is_ascii_digit() || (c == b'.' && i + 1 < len && b[i + 1].is_ascii_digit()) {
            let start = i;
            i += 1;
            while i < len {
                let d = b[i];
                if d.is_ascii_alphanumeric() || d == b'.' || d == b'_' {
                    i += 1;
                } else if (d == b'+' || d == b'-')
                    && matches!(b[i - 1], b'e' | b'E' | b'p' | b'P')
                {
                    i += 1;
                } else {
                    break;
                }
            }
            out.push(String::from_utf8_lossy(&b[start..i]).into_owned());
            continue;
        }
        // multi-char operators, longest first
        let mut matched = false;
        for op in TWO_CHAR_OPS {
            let ob = op.as_bytes();
            if i + ob.len() <= len && &b[i..i + ob.len()] == ob {
                out.push((*op).to_string());
                i += ob.len();
                matched = true;
                break;
            }
        }
        if matched {
            continue;
        }
        // single char
        out.push((c as char).to_string());
        i += 1;
    }
    out
}

/// Strip GCC-style line markers (`# N "file" ...`) from preprocessor output.
pub fn strip_line_markers(out: &str) -> String {
    out.lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// Independent #if expression reference (C99 6.10.1 / module README).
// Values are (i128 magnitude, is_unsigned); unsigned values are always in
// 0..=u64::MAX and behave mod 2^64; signed values are exact i128 and the
// generator guarantees no i64 overflow (checked by `overflow` flag).
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum Expr {
    Num(u64, bool),
    Ident(String),
    Defined(String),
    Un(&'static str, Box<Expr>),
    Bin(&'static str, Box<Expr>, Box<Expr>),
    Tern(Box<Expr>, Box<Expr>, Box<Expr>),
}

/// Reference evaluation result: (value, is_unsigned). Returns None when the
/// expression leaves the well-defined domain (signed i64 overflow, division
/// by zero, shift count outside 0..64).
pub fn ref_eval(e: &Expr, env: &dyn Fn(&str) -> Option<i64>) -> Option<(i128, bool)> {
    match e {
        Expr::Num(v, u) => Some((*v as i128, *u || *v > i64::MAX as u64)),
        Expr::Ident(n) => Some((env(n)? as i128, false)),
        Expr::Defined(_) => unreachable!("defined() resolved before eval"),
        Expr::Un(op, a) => {
            let (av, au) = ref_eval(a, env)?;
            match *op {
                "!" => Some(((av == 0) as i128, false)),
                "-" => {
                    if au {
                        Some(((av as u64).wrapping_neg() as i128, true))
                    } else {
                        Some((-av, false))
                    }
                }
                "+" => Some((av, au)),
                "~" => Some((!av, au)),
                _ => unreachable!(),
            }
        }
        Expr::Tern(c, t, f) => {
            let (cv, _) = ref_eval(c, env)?;
            if cv != 0 {
                ref_eval(t, env)
            } else {
                ref_eval(f, env)
            }
        }
        Expr::Bin(op, a, b) => {
            let (av, au) = ref_eval(a, env)?;
            let (bv, bu) = ref_eval(b, env)?;
            let u = au || bu;
            match *op {
                "&&" => Some(((av != 0 && bv != 0) as i128, false)),
                "||" => Some(((av != 0 || bv != 0) as i128, false)),
                "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                    let r = if u {
                        let x = av as u64;
                        let y = bv as u64;
                        match *op {
                            "==" => x == y,
                            "!=" => x != y,
                            "<" => x < y,
                            ">" => x > y,
                            "<=" => x <= y,
                            _ => x >= y,
                        }
                    } else {
                        match *op {
                            "==" => av == bv,
                            "!=" => av != bv,
                            "<" => av < bv,
                            ">" => av > bv,
                            "<=" => av <= bv,
                            _ => av >= bv,
                        }
                    };
                    Some((r as i128, false))
                }
                "<<" | ">>" => {
                    if !(0..64).contains(&bv) {
                        return None; // shift count out of range: UB
                    }
                    let amt = bv as u32;
                    // C99 6.5.7: the result type is the promoted LEFT operand's
                    // type — the right operand's signedness does not leak in.
                    if au {
                        let x = av as u64;
                        let r = if *op == "<<" {
                            x.wrapping_shl(amt)
                        } else {
                            x.wrapping_shr(amt)
                        };
                        Some((r as i128, true))
                    } else {
                        // signed shift: value stays exact (no overflow possible
                        // for << only if it fits; check)
                        if *op == "<<" {
                            let r = av.checked_shl(amt)?;
                            if r >= (1i128 << 63) {
                                return None; // signed overflow: UB
                            }
                            Some((r, false))
                        } else {
                            Some((av >> amt, false))
                        }
                    }
                }
                _ => {
                    if u {
                        let x = av as u64;
                        let y = bv as u64;
                        let r = match *op {
                            "+" => x.wrapping_add(y),
                            "-" => x.wrapping_sub(y),
                            "*" => x.wrapping_mul(y),
                            "/" => {
                                if y == 0 {
                                    return None;
                                }
                                x / y
                            }
                            "%" => {
                                if y == 0 {
                                    return None;
                                }
                                x % y
                            }
                            "&" => x & y,
                            "|" => x | y,
                            "^" => x ^ y,
                            _ => unreachable!(),
                        };
                        Some((r as i128, true))
                    } else {
                        let r = match *op {
                            "+" => av.checked_add(bv)?,
                            "-" => av.checked_sub(bv)?,
                            "*" => av.checked_mul(bv)?,
                            "/" => {
                                if bv == 0 {
                                    return None;
                                }
                                av.checked_div(bv)?
                            }
                            "%" => {
                                if bv == 0 {
                                    return None;
                                }
                                av.checked_rem(bv)?
                            }
                            "&" => av & bv,
                            "|" => av | bv,
                            "^" => av ^ bv,
                            _ => unreachable!(),
                        };
                        if r >= (1i128 << 63) || r < -(1i128 << 63) {
                            return None; // signed i64 overflow: UB in #if
                        }
                        Some((r, false))
                    }
                }
            }
        }
    }
}

/// Render an expression to C text with full parenthesization and random spacing.
pub fn render_expr(e: &Expr, rng: &mut dyn FnMut() -> usize) -> String {
    fn spac(n: usize) -> &'static str {
        match n % 3 {
            0 => "",
            1 => " ",
            _ => "  ",
        }
    }
    fn go(e: &Expr, rng: &mut dyn FnMut() -> usize) -> String {
        match e {
            Expr::Num(v, u) => render_num(*v, *u, rng),
            Expr::Ident(n) => n.clone(),
            Expr::Defined(n) => {
                if rng() % 2 == 0 {
                    format!("defined({n})")
                } else {
                    format!("defined {n}")
                }
            }
            Expr::Un(op, a) => format!("{}{}{}", op, spac(rng()), go(a, rng)),
            Expr::Bin(op, a, b) => format!(
                "({}{}{}{}{})",
                go(a, rng),
                spac(rng()),
                op,
                spac(rng()),
                go(b, rng)
            ),
            Expr::Tern(c, t, f) => format!(
                "({}{}?{}{}:{}{})",
                go(c, rng),
                spac(rng()),
                go(t, rng),
                spac(rng()),
                spac(rng()),
                go(f, rng)
            ),
        }
    }
    go(e, rng)
}

/// Render a numeric literal in a random base/suffix combination. The SUT's
/// documented rule (conditionals.rs:265-275, C99 6.4.4.1): hex/octal larger
/// than i64::MAX are unsigned; a U suffix forces unsigned.
pub fn render_num(v: u64, u: bool, rng: &mut dyn FnMut() -> usize) -> String {
    let mut suffix = String::new();
    if u {
        // random u/l combinations
        for _ in 0..(rng() % 3 + 1) {
            suffix.push(if rng() % 2 == 0 { 'u' } else { 'U' });
        }
    }
    if v > i64::MAX as u64 {
        // must render as hex/octal for SUT's unsigned rule; octal needs digits
        // without 8/9 — use hex always for simplicity
        return format!("0x{x:x}{suffix}", x = v);
    }
    let oct_ok = v == 0 || {
        let s = format!("{v:o}");
        s.chars().all(|c| c.is_ascii_digit())
    };
    match rng() % 3 {
        0 => format!("{v}{suffix}"),
        1 => format!("0x{x:x}{suffix}", x = v),
        _ => {
            if oct_ok {
                format!("0{o:o}{suffix}", o = v)
            } else {
                format!("{v}{suffix}")
            }
        }
    }
}

/// Generator: arbitrary well-formed expressions over a set of identifier names.
pub fn arb_expr(depth: u32, idents: Vec<String>) -> BoxedStrategy<Expr> {
    let leaf = prop_oneof![
        4 => (0u64..1000).prop_map(|v| Expr::Num(v, false)),
        1 => (0u64..=u64::MAX).prop_map(|v| Expr::Num(v, false)),
        2 => (0u64..1000, proptest::bool::ANY)
            .prop_map(|(v, u)| Expr::Num(v, u)),
        3 => (0u64..=u64::MAX, proptest::bool::ANY)
            .prop_map(|(v, u)| Expr::Num(v, u)),
    ];
    let base: BoxedStrategy<Expr> = if idents.is_empty() {
        leaf.boxed()
    } else {
        let il = proptest::sample::select(idents.clone())
            .prop_map(Expr::Ident)
            .boxed();
        prop_oneof![7 => leaf, 3 => il].boxed()
    };
    if depth == 0 {
        base
    } else {
        let sub = arb_expr(depth - 1, idents.clone());
        let sub2 = arb_expr(depth - 1, idents);
        let un_ops = proptest::sample::select(vec!["!", "-", "+", "~"]);
        let bin_ops = proptest::sample::select(vec![
            "+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "==", "!=", "<", ">", "<=",
            ">=", "&&", "||",
        ]);
        prop_oneof![
            3 => base,
            2 => (un_ops, sub.clone()).prop_map(|(o, a)| Expr::Un(o, Box::new(a))),
            6 => (bin_ops, sub, sub2.clone()).prop_map(|(o, a, b)| Expr::Bin(o, Box::new(a), Box::new(b))),
            1 => (arb_expr(depth - 1, vec![]), sub2, arb_expr(depth - 1, vec![]))
                .prop_map(|(c, t, f)| Expr::Tern(Box::new(c), Box::new(t), Box::new(f))),
        ]
        .boxed()
    }
}

// ---------------------------------------------------------------------------
// gcc -E differential runner.
// ---------------------------------------------------------------------------

static SCRATCH_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn scratch_dir() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("pbt-out/rounds/03_preprocessor/run/p8scratch");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Run `gcc -E -P` on the given program text and return the token stream.
/// Err on non-zero exit (generated program left the well-formed domain).
pub fn gcc_tokens(program: &str) -> Result<Vec<String>, String> {
    let n = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
    let file = scratch_dir().join(format!("case_{n}.c"));
    std::fs::write(&file, program).expect("write scratch .c");
    let out = std::process::Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(&file)
        .output()
        .expect("spawn gcc");
    if !out.status.success() {
        return Err(format!(
            "gcc -E failed:\n{}\nstderr: {}",
            program,
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(tokens(&String::from_utf8_lossy(&out.stdout)))
}

/// Run the SUT preprocessor and return the token stream (markers stripped).
pub fn sut_tokens(pp: &mut crate::frontend::preprocessor::Preprocessor, program: &str) -> Vec<String> {
    let out = pp.preprocess(program);
    tokens(&strip_line_markers(&out))
}

// ===========================================================================
// PBT round 04 — change-surface obligation: is_ident_start / is_ident_cont
// (changed by commit HEAD). These classify the ASCII subset of C11 6.4.2.1
// nondigits for the round-03 differential ORACLE tokenizer; pin them to the
// standard clause, plus maximal-munch / whitespace-collapse invariance of
// tokens() itself (the oracle's own contract, pbt_support.rs:19).
// ===========================================================================
mod round04_tests {
    use super::*;
    use proptest::prelude::*;

    fn arb_token() -> BoxedStrategy<String> {
        prop_oneof![
            "[a-mo-t][a-z0-9_]{0,5}",
            "[0-9]{1,4}",
            "[+*<=>!&|]{1,2}",
            "[( )]{1,3}",
        ].boxed()
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p10_ident_classification_c11(b in 0u8..=255) {
            // C11 6.4.2.1: identifier-nondigit = _ | letters | ... ; for the
            // ASCII byte range this is exactly [A-Za-z_], and continuation
            // additionally admits decimal digits.
            prop_assert_eq!(is_ident_start(b), b.is_ascii_alphabetic() || b == b'_');
            prop_assert_eq!(is_ident_cont(b), b.is_ascii_alphanumeric() || b == b'_');
        }

        #[test]
        fn p10_tokenizer_ws_collapse_invariance(toks in proptest::collection::vec(arb_token(), 0..8)) {
            let spaced = toks.join("  ");
            let collapsed = toks.join(" ");
            let t1 = tokens(&spaced);
            let t2 = tokens(&collapsed);
            prop_assert_eq!(&t1, &t2, "ws-run width must not change the token stream");
            // Round trip: re-joining the emitted stream with single spaces
            // re-tokenizes to itself (idempotence of the token spelling).
            let rejoined = t1.join(" ");
            prop_assert_eq!(tokens(&rejoined), t1);
        }
    }
}

mod round04_failure_branch {
    use super::*;
    use proptest::prelude::*;

    /// Bytes that must be REJECTED by is_ident_start/is_ident_cont (the
    /// negative/failure branch of the classifier changed by commit HEAD):
    /// printable ASCII separators that are neither letter, digit, nor '_'.
    /// Quotes and backslash are excluded (they open literals/escapes and are
    /// covered by the literal branch of tokens(), not the ident branch).
    const SEPARATOR_BYTES: &[u8] = b"@#%&*+~^|?!<>=,;.:/";
    const IDENT: &str = "ab_z9";

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p10b_rejected_bytes_terminate_identifiers(b in proptest::sample::select(SEPARATOR_BYTES.to_vec())) {
            // Failure branch of the classifier: a non-nondigit byte must
            // return false from both predicates...
            prop_assert!(!is_ident_start(b), "byte {:?} must not start an identifier", b as char);
            prop_assert!(!is_ident_cont(b), "byte {:?} must not continue an identifier", b as char);
            // ...and the tokenizer must therefore SPLIT there: the identifier
            // cannot swallow the separator byte. If the classifier's reject
            // branch regressed to accept, this fails.
            let s = format!("{}{}{}", IDENT, b as char, IDENT);
            let toks = tokens(&s);
            prop_assert!(toks.len() >= 2, "identifier swallowed separator {:?} in {:?}", b as char, toks);
            prop_assert_eq!(&toks[0], IDENT);
            // The separator may form its own operator token, but no token may
            // MIX identifier letters with the separator byte (that would mean
            // the ident classifier's reject branch regressed).
            prop_assert!(!toks.iter().any(|t| t.contains(IDENT) && t.as_bytes().contains(&b)),
                "identifier token swallowed separator byte: {:?}", toks);
        }
    }
}
