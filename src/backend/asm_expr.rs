/// Shared assembly expression evaluator for all assembler backends (x86, i686, ARM, RISC-V).
///
/// Supports arithmetic expressions with proper operator precedence:
///   - Parentheses: `(expr)`
///   - Bitwise OR: `|`
///   - Bitwise XOR: `^`
///   - Bitwise AND: `&`
///   - Shifts: `<<`, `>>`
///   - Addition/Subtraction: `+`, `-`
///   - Multiplication/Division/Modulo: `*`, `/`, `%`
///   - Unary: `-`, `+`, `~`, `!` (logical NOT)
///
/// Integer literals: decimal, hex (0x), binary (0b), octal (leading 0),
/// character literals ('c', '\n').
/// Used by all four assembler backends (x86, i686, ARM, RISC-V).
/// Map a C escape character to its ASCII value (e.g., b'n' -> 10 for '\n').
fn char_escape_value(esc: u8) -> i64 {
    match esc {
        b'n' => 10,
        b't' => 9,
        b'r' => 13,
        b'0' => 0,
        b'\\' => b'\\' as i64,
        b'\'' => b'\'' as i64,
        b'"' => b'"' as i64,
        b'a' => 7,
        b'b' => 8,
        b'f' => 12,
        b'v' => 11,
        _ => esc as i64,
    }
}

/// Token type for the expression evaluator.
#[derive(Debug)]
enum ExprToken {
    Num(i64),
    Op(char),
    Op2(&'static str),
}

/// Tokenize an expression string into ExprTokens.
fn tokenize_expr(s: &str) -> Result<Vec<ExprToken>, String> {
    let mut tokens = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c == b'(' || c == b')' || c == b'+' || c == b'-' || c == b'*'
            || c == b'/' || c == b'%' || c == b'&' || c == b'|' || c == b'^' || c == b'~'
            || c == b'!'
        {
            tokens.push(ExprToken::Op(c as char));
            i += 1;
        } else if c == b'<' && i + 1 < bytes.len() && bytes[i + 1] == b'<' {
            tokens.push(ExprToken::Op2("<<"));
            i += 2;
        } else if c == b'>' && i + 1 < bytes.len() && bytes[i + 1] == b'>' {
            tokens.push(ExprToken::Op2(">>"));
            i += 2;
        } else if c == b'\'' {
            // Character literal: 'c' or '\n', '\t', '\\', etc.
            i += 1; // skip opening quote
            if i >= bytes.len() {
                return Err("unterminated character literal".to_string());
            }
            let ch_val = if bytes[i] == b'\\' {
                i += 1;
                if i >= bytes.len() {
                    return Err("unterminated character escape".to_string());
                }
                let esc = bytes[i];
                i += 1;
                char_escape_value(esc)
            } else {
                let val = bytes[i] as i64;
                i += 1;
                val
            };
            // Skip closing quote if present
            if i < bytes.len() && bytes[i] == b'\'' {
                i += 1;
            }
            tokens.push(ExprToken::Num(ch_val));
        } else if c.is_ascii_digit() {
            let start = i;
            if c == b'0' && i + 1 < bytes.len() && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
                i += 2;
                while i < bytes.len() && bytes[i].is_ascii_hexdigit() { i += 1; }
            } else if c == b'0' && i + 1 < bytes.len() && (bytes[i + 1] == b'b' || bytes[i + 1] == b'B') {
                i += 2;
                while i < bytes.len() && (bytes[i] == b'0' || bytes[i] == b'1') { i += 1; }
            } else {
                while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
            }
            let num_str = &s[start..i];
            let val = parse_single_integer(num_str)?;
            tokens.push(ExprToken::Num(val));
        } else {
            return Err(format!("unexpected char '{}' in expression", c as char));
        }
    }
    Ok(tokens)
}

fn eval_tokens(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    eval_or(tokens, pos)
}

fn eval_or(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    let mut val = eval_xor(tokens, pos)?;
    while *pos < tokens.len() {
        if matches!(&tokens[*pos], ExprToken::Op('|')) {
            *pos += 1;
            val |= eval_xor(tokens, pos)?;
        } else {
            break;
        }
    }
    Ok(val)
}

fn eval_xor(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    let mut val = eval_and(tokens, pos)?;
    while *pos < tokens.len() {
        if matches!(&tokens[*pos], ExprToken::Op('^')) {
            *pos += 1;
            val ^= eval_and(tokens, pos)?;
        } else {
            break;
        }
    }
    Ok(val)
}

fn eval_and(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    let mut val = eval_shift(tokens, pos)?;
    while *pos < tokens.len() {
        if matches!(&tokens[*pos], ExprToken::Op('&')) {
            *pos += 1;
            val &= eval_shift(tokens, pos)?;
        } else {
            break;
        }
    }
    Ok(val)
}

fn eval_shift(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    let mut val = eval_add(tokens, pos)?;
    while *pos < tokens.len() {
        match &tokens[*pos] {
            ExprToken::Op2("<<") => { *pos += 1; val <<= eval_add(tokens, pos)?; }
            ExprToken::Op2(">>") => { *pos += 1; val = ((val as u64) >> eval_add(tokens, pos)?) as i64; }
            _ => break,
        }
    }
    Ok(val)
}

fn eval_add(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    let mut val = eval_mul(tokens, pos)?;
    while *pos < tokens.len() {
        match &tokens[*pos] {
            ExprToken::Op('+') => { *pos += 1; val += eval_mul(tokens, pos)?; }
            ExprToken::Op('-') => { *pos += 1; val -= eval_mul(tokens, pos)?; }
            _ => break,
        }
    }
    Ok(val)
}

fn eval_mul(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    let mut val = eval_unary(tokens, pos)?;
    while *pos < tokens.len() {
        match &tokens[*pos] {
            ExprToken::Op('*') => { *pos += 1; val *= eval_unary(tokens, pos)?; }
            ExprToken::Op('/') => {
                *pos += 1;
                let rhs = eval_unary(tokens, pos)?;
                if rhs == 0 { return Err("division by zero".to_string()); }
                val /= rhs;
            }
            ExprToken::Op('%') => {
                *pos += 1;
                let rhs = eval_unary(tokens, pos)?;
                if rhs == 0 { return Err("modulo by zero".to_string()); }
                val %= rhs;
            }
            _ => break,
        }
    }
    Ok(val)
}

fn eval_unary(tokens: &[ExprToken], pos: &mut usize) -> Result<i64, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of expression".to_string());
    }
    match &tokens[*pos] {
        ExprToken::Op('-') => { *pos += 1; Ok(-eval_unary(tokens, pos)?) }
        ExprToken::Op('+') => { *pos += 1; eval_unary(tokens, pos) }
        ExprToken::Op('~') => { *pos += 1; Ok(!eval_unary(tokens, pos)?) }
        ExprToken::Op('!') => { *pos += 1; Ok(if eval_unary(tokens, pos)? == 0 { 1 } else { 0 }) }
        ExprToken::Op('(') => {
            *pos += 1;
            let val = eval_tokens(tokens, pos)?;
            if *pos < tokens.len() && matches!(&tokens[*pos], ExprToken::Op(')')) {
                *pos += 1;
            } else {
                return Err("missing closing parenthesis".to_string());
            }
            Ok(val)
        }
        ExprToken::Num(v) => { let v = *v; *pos += 1; Ok(v) }
        other => Err(format!("unexpected token in expression: {:?}", other)),
    }
}

/// Parse a single integer value (no arithmetic expressions).
/// Supports decimal, hex (0x/0X), binary (0b/0B), octal (leading 0),
/// and character literals ('c', '\n', etc.).
fn parse_single_integer(s: &str) -> Result<i64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty integer".to_string());
    }

    // Character literal: 'c' or '\n' etc. Must be the entire string
    // (e.g., "'!'" or "'\n'"), not part of a larger expression.
    if s.starts_with('\'') {
        let bytes = s.as_bytes();
        let mut i = 1;
        if i >= bytes.len() {
            return Err("unterminated character literal".to_string());
        }
        let val;
        if bytes[i] == b'\\' {
            i += 1;
            if i >= bytes.len() {
                return Err("unterminated character escape".to_string());
            }
            val = char_escape_value(bytes[i]);
            i += 1;
        } else {
            val = bytes[i] as i64;
            i += 1;
        }
        // Skip closing quote
        if i < bytes.len() && bytes[i] == b'\'' {
            i += 1;
        }
        // Only accept if we consumed the whole string
        if i == bytes.len() {
            return Ok(val);
        }
        // Otherwise, fall through to let tokenize_expr handle it as an expression
        return Err("character literal has trailing content".to_string());
    }

    let (negative, s) = if let Some(rest) = s.strip_prefix('-') {
        (true, rest)
    } else {
        (false, s)
    };

    let val = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        let uval = u64::from_str_radix(hex, 16)
            .map_err(|_| format!("bad hex: {}", s))?;
        if negative {
            return Ok((uval as i64).wrapping_neg());
        }
        return Ok(uval as i64);
    } else if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        i64::from_str_radix(bin, 2)
            .map_err(|_| format!("bad binary: {}", s))?
    } else if s.starts_with('0') && s.len() > 1 && s.chars().all(|c| c.is_ascii_digit()) {
        // Octal (must be checked before decimal to handle leading-zero literals)
        i64::from_str_radix(s, 8)
            .map_err(|_| format!("bad octal: {}", s))?
    } else {
        // Try decimal, including u64 range for large unsigned values
        if let Ok(val) = s.parse::<i64>() {
            if negative {
                return Ok(-val);
            }
            return Ok(val);
        }
        if let Ok(uval) = s.parse::<u64>() {
            if negative {
                return Ok((uval as i64).wrapping_neg());
            }
            return Ok(uval as i64);
        }
        return Err(format!("bad integer: {}", s));
    };

    Ok(if negative { val.wrapping_neg() } else { val })
}

/// Parse an integer expression with full operator precedence.
/// Supports: |, ^, &, <<, >>, +, -, *, /, %, ~, parentheses.
/// Falls back to simple integer parsing for plain numbers.
pub fn parse_integer_expr(s: &str) -> Result<i64, String> {
    let s = s.trim();
    if s.is_empty() {
        return Err("empty integer".to_string());
    }
    // Fast path: try simple integer first
    if let Ok(val) = parse_single_integer(s) {
        return Ok(val);
    }
    let tokens = tokenize_expr(s)?;
    if tokens.is_empty() {
        return Err("empty expression".to_string());
    }
    let mut pos = 0;
    let val = eval_tokens(&tokens, &mut pos)?;
    if pos < tokens.len() {
        return Err(format!("unexpected trailing token in expression: {:?}", tokens[pos]));
    }
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_integers() {
        assert_eq!(parse_integer_expr("42").unwrap(), 42);
        assert_eq!(parse_integer_expr("-1").unwrap(), -1);
        assert_eq!(parse_integer_expr("0xFF").unwrap(), 255);
        assert_eq!(parse_integer_expr("0b1010").unwrap(), 10);
        assert_eq!(parse_integer_expr("0644").unwrap(), 420);
    }

    #[test]
    fn test_arithmetic() {
        assert_eq!(parse_integer_expr("8 * 16 + 8 * 8").unwrap(), 192);
        assert_eq!(parse_integer_expr("(8 * 16 + 8 * 8)").unwrap(), 192);
        assert_eq!(parse_integer_expr("8*2 + (8 * 16 + 8 * 8) + 64").unwrap(), 272);
        assert_eq!(parse_integer_expr("-(8 * 8 + 8 * 8 + 16)").unwrap(), -144);
        assert_eq!(parse_integer_expr("-(8 * 8 + 8 * 8 + 16)+0*8").unwrap(), -144);
    }

    #[test]
    fn test_bitwise() {
        assert_eq!(parse_integer_expr("0x100 | 0x4000 | 17").unwrap(), 0x4111);
        assert_eq!(parse_integer_expr("0xFF & 0x0F").unwrap(), 0x0F);
        assert_eq!(parse_integer_expr("1 << 5").unwrap(), 32);
        assert_eq!(parse_integer_expr("~0").unwrap(), -1);
        assert_eq!(parse_integer_expr("0xFF ^ 0x0F").unwrap(), 0xF0);
    }

    #[test]
    fn test_logical_not() {
        // GAS logical NOT: !0 = 1, !1 = 0, !nonzero = 0
        assert_eq!(parse_integer_expr("!0").unwrap(), 1);
        assert_eq!(parse_integer_expr("!1").unwrap(), 0);
        assert_eq!(parse_integer_expr("!42").unwrap(), 0);
        // Used in FFmpeg: 1+!0 = 2
        assert_eq!(parse_integer_expr("1+!0").unwrap(), 2);
        assert_eq!(parse_integer_expr("1+!1").unwrap(), 1);
    }

    #[test]
    fn test_complex() {
        // libffi CALL_CONTEXT_SIZE = (N_V_ARG_REG * 16 + N_X_ARG_REG * 8) where N_V=8, N_X=8
        assert_eq!(parse_integer_expr("(8 * 16 + 8 * 8)").unwrap(), 192);
        // ffi_closure_SYSV_FS = (8*2 + CALL_CONTEXT_SIZE + 64)
        assert_eq!(parse_integer_expr("(8*2 + (8 * 16 + 8 * 8) + 64)").unwrap(), 272);
        // musl vfork: CLONE_VM | CLONE_VFORK | SIGCHLD
        assert_eq!(parse_integer_expr("0x100 | 0x4000 | 17").unwrap(), 0x4111);
    }

    #[test]
    fn test_operator_precedence() {
        // * binds tighter than +
        assert_eq!(parse_integer_expr("2 + 3 * 4").unwrap(), 14);
        assert_eq!(parse_integer_expr("3 * 4 + 2").unwrap(), 14);
        // | binds looser than +
        assert_eq!(parse_integer_expr("1 | 2 + 4").unwrap(), 7); // 1 | (2+4) = 1 | 6 = 7
    }

    #[test]
    fn test_character_literals() {
        // Simple ASCII characters
        assert_eq!(parse_integer_expr("'!'").unwrap(), 33);
        assert_eq!(parse_integer_expr("'A'").unwrap(), 65);
        assert_eq!(parse_integer_expr("'0'").unwrap(), 48);
        assert_eq!(parse_integer_expr("' '").unwrap(), 32);
        // Escape sequences
        assert_eq!(parse_integer_expr("'\\n'").unwrap(), 10);
        assert_eq!(parse_integer_expr("'\\t'").unwrap(), 9);
        assert_eq!(parse_integer_expr("'\\r'").unwrap(), 13);
        assert_eq!(parse_integer_expr("'\\0'").unwrap(), 0);
        assert_eq!(parse_integer_expr("'\\\\").unwrap(), 92);
        assert_eq!(parse_integer_expr("'\\''").unwrap(), 39);
        // Character literals in expressions
        assert_eq!(parse_integer_expr("'A' + 1").unwrap(), 66);
        assert_eq!(parse_integer_expr("'!' | 0x80").unwrap(), 0xA1);
    }

}

#[cfg(test)]
mod pbt_tests2 {
    use super::parse_integer_expr;
    use proptest::prelude::*;

    /// P8: a generated expression tree rendered fully parenthesized must
    /// parse to the tree's independently computed value. Operators per the
    /// documented grammar: | ^ & << >> + - * / % ~ and unary minus.
    #[derive(Clone)]
    enum Tree {
        Lit(i64),
        Bin(&'static str, Box<Tree>, Box<Tree>),
        Neg(Box<Tree>),
        Not(Box<Tree>),
    }

    fn render(t: &Tree) -> String {
        match t {
            Tree::Lit(v) => {
                if *v < 16 && *v >= 0 { format!("0x{:x}", v) } else { format!("{}", v) }
            }
            Tree::Bin(op, a, b) => format!("({} {} {})", render(a), op, render(b)),
            Tree::Neg(a) => format!("-({})", render(a)),
            Tree::Not(a) => format!("~({})", render(a)),
        }
    }

    fn eval(t: &Tree) -> i64 {
        match t {
            Tree::Lit(v) => *v,
            Tree::Bin(op, a, b) => {
                let (x, y) = (eval(a), eval(b));
                match *op {
                    "+" => x.wrapping_add(y),
                    "-" => x.wrapping_sub(y),
                    "*" => x.wrapping_mul(y),
                    "/" => x.wrapping_div(y),
                    "%" => x.wrapping_rem(y),
                    "<<" => x.wrapping_shl((y & 15) as u32),
                    // >> = LOGICAL shift per the reference assemblers:
                    // GNU as and llvm-mc both evaluate (-1)>>1 to
                    // 0x7FFFFFFFFFFFFFFF (unsigned expression semantics).
                    ">>" => ((x as u64).wrapping_shr((y & 15) as u32)) as i64,
                    "&" => x & y,
                    "|" => x | y,
                    "^" => x ^ y,
                    _ => unreachable!(),
                }
            }
            Tree::Neg(a) => eval(a).wrapping_neg(),
            Tree::Not(a) => !eval(a),
        }
    }

    /// Token stream → tree: (kind, payload). kind 0..8 = binary op, 9 = neg,
    /// 10 = not, 11+ = literal. Stack-built (RPN-like).
    fn build_tree(tokens: &[(u8, i64)]) -> Tree {
        let ops = ["+", "-", "*", "/", "%", "<<", ">>", "&", "|", "^"];
        let mut stack: Vec<Tree> = vec![];
        for &(k, v) in tokens {
            let t = match k % 12 {
                0..=9 => {
                    if stack.len() >= 2 {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        Tree::Bin(ops[(k % 12) as usize], Box::new(a), Box::new(b))
                    } else {
                        Tree::Lit(v.abs() % 4096)
                    }
                }
                10 => {
                    let a = if stack.is_empty() { Tree::Lit(1) } else { stack.pop().unwrap() };
                    Tree::Neg(Box::new(a))
                }
                11 => {
                    let a = if stack.is_empty() { Tree::Lit(1) } else { stack.pop().unwrap() };
                    Tree::Not(Box::new(a))
                }
                _ => Tree::Lit(v.abs() % 4096),
            };
            stack.push(t);
        }
        let mut root = if stack.is_empty() { Tree::Lit(0) } else { stack.pop().unwrap() };
        while let Some(a) = stack.pop() {
            root = Tree::Bin("+", Box::new(a), Box::new(root));
        }
        root
    }

    /// Neutralize UB-ish forms at render time: /0 and %0 replaced by /1,
    /// shift amounts masked to 0..15 (rendered text must match eval's domain).
    fn fixup(t: Tree) -> Tree {
        match t {
            Tree::Bin(op, a, b) => {
                let a = Box::new(fixup(*a));
                let mut b = Box::new(fixup(*b));
                if op == "<<" || op == ">>" {
                    let amt = (eval(&b) & 15) as i64;
                    b = Box::new(Tree::Lit(amt));
                }
                if (op == "/" || op == "%") && eval(&b) == 0 {
                    b = Box::new(Tree::Lit(1));
                }
                Tree::Bin(op, a, b)
            }
            Tree::Neg(a) => Tree::Neg(Box::new(fixup(*a))),
            Tree::Not(a) => Tree::Not(Box::new(fixup(*a))),
            l => l,
        }
    }

    /// Keep every node's |value| within i64/4: the SUT's parser uses plain
    /// `*=`/`<<=` (debug-overflow panics — covered by the red regression),
    /// so the SEMANTIC property stays in the defined-evaluation domain by
    /// replacing overflowing `*`/`<<` nodes with the bounded `&`.
    fn bound(t: Tree) -> Tree {
        match t {
            Tree::Bin(op, a, b) => {
                let a = Box::new(bound(*a));
                let b = Box::new(bound(*b));
                let a2 = a.clone();
                let b2 = b.clone();
                let inner = Tree::Bin(op, a, b);
                let v = eval(&inner).wrapping_abs();
                if v > (i64::MAX / 4) as i64 {
                    Tree::Bin("&", a2, b2)
                } else {
                    inner
                }
            }
            Tree::Neg(a) => Tree::Neg(Box::new(bound(*a))),
            Tree::Not(a) => Tree::Not(Box::new(bound(*a))),
            l => l,
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p8_asmexpr_parse_matches_evaluator(
            tokens in prop::collection::vec(any::<(u8, i64)>(), 1..14)
        ) {
            let tree = bound(fixup(build_tree(&tokens)));
            let expected = eval(&tree);
            let s = render(&tree);
            let got = parse_integer_expr(&s).unwrap_or_else(|e| panic!("parse failed for {}: {}", s, e));
            prop_assert_eq!(got, expected, "expr: {}", s);
        }
    }
}

#[cfg(test)]
mod pbt_regression {
    use super::parse_integer_expr;

    /// RED WITNESS — round 08 bug (issue #765): out-of-range shift amounts
    /// PANIC instead of returning Err — `(val as u64) >> n` / `val <<= n` on
    /// i64 debug-panic for n >= 64, and wrap silently in release (1<<64 -> 1).
    /// A Result-returning parser must reject the input.
    #[test]
    fn test_asmexpr_regression_arith_panic_not_err() {
        assert!(parse_integer_expr("1 << 64").is_err());
        assert!(parse_integer_expr("4 >> 64").is_err());
        assert!(parse_integer_expr("1 << 1000").is_err());
        // plain `*=` on i64: debug panic instead of Err
        assert!(parse_integer_expr("9223372036854775807 * 9223372036854775807").is_err());
    }
}
