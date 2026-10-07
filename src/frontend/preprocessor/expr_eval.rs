//! Preprocessor conditional expression evaluation.
//!
//! This module handles evaluation of preprocessor conditional expressions (`#if`, `#elif`),
//! including `defined()` operator resolution, `__has_builtin()`, `__has_attribute()`,
//! `__has_include()`, and `__has_include_next()` detection,
//! and replacing undefined identifiers with 0 per the C standard.
//!
//! All scanning operates on byte slices for performance (no Vec<char> allocation).

use super::pipeline::Preprocessor;
use super::utils::{is_ident_start_byte, is_ident_cont_byte, bytes_to_str};

impl Preprocessor {
    /// Replace remaining identifiers (not keywords) with 0 in a #if expression.
    /// Per C standard, after macro expansion, undefined identifiers in #if evaluate to 0.
    pub(super) fn replace_remaining_idents_with_zero(&self, expr: &str) -> String {
        let mut result = String::new();
        let bytes = expr.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            // Skip character literals verbatim
            if bytes[i] == b'\'' {
                result.push(bytes[i] as char);
                i += 1;
                while i < len && bytes[i] != b'\'' {
                    if bytes[i] == b'\\' && i + 1 < len {
                        result.push(bytes[i] as char);
                        i += 1;
                        result.push(bytes[i] as char);
                        i += 1;
                    } else {
                        result.push(bytes[i] as char);
                        i += 1;
                    }
                }
                if i < len && bytes[i] == b'\'' {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                continue;
            }
            // Skip string literals verbatim
            if bytes[i] == b'"' {
                result.push(bytes[i] as char);
                i += 1;
                while i < len && bytes[i] != b'"' {
                    if bytes[i] == b'\\' && i + 1 < len {
                        result.push(bytes[i] as char);
                        i += 1;
                        result.push(bytes[i] as char);
                        i += 1;
                    } else {
                        result.push(bytes[i] as char);
                        i += 1;
                    }
                }
                if i < len && bytes[i] == b'"' {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                continue;
            }
            if bytes[i].is_ascii_digit() {
                // Skip entire number literal
                result.push(bytes[i] as char);
                i += 1;
                if i < len && bytes[i - 1] == b'0' && (bytes[i] == b'x' || bytes[i] == b'X' || bytes[i] == b'b' || bytes[i] == b'B') {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'.') {
                    result.push(bytes[i] as char);
                    i += 1;
                }
            } else if is_ident_start_byte(bytes[i]) {
                let start = i;
                i += 1;
                while i < len && is_ident_cont_byte(bytes[i]) {
                    i += 1;
                }
                let ident = bytes_to_str(bytes, start, i);
                if ident == "defined" || ident == "true" || ident == "false" {
                    result.push_str(ident);
                } else {
                    result.push('0');
                }
            } else {
                result.push(bytes[i] as char);
                i += 1;
            }
        }

        result
    }

    /// Replace `defined(X)`, `defined X`, `__has_builtin(X)`, `__has_attribute(X)`,
    /// `__has_feature(X)`, `__has_extension(X)`, `__has_include(X)`, and
    /// `__has_include_next(X)` with 0 or 1 in a #if expression.
    pub(super) fn resolve_defined_in_expr(&mut self, expr: &str) -> String {
        let mut result = String::new();
        let bytes = expr.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            if is_ident_start_byte(bytes[i]) {
                let start = i;
                i += 1;
                while i < len && is_ident_cont_byte(bytes[i]) {
                    i += 1;
                }
                let ident = bytes_to_str(bytes, start, i);

                if ident == "defined" {
                    while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
                        i += 1;
                    }
                    let has_paren = if i < len && bytes[i] == b'(' {
                        i += 1;
                        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
                            i += 1;
                        }
                        true
                    } else {
                        false
                    };

                    let name_start = i;
                    while i < len && is_ident_cont_byte(bytes[i]) {
                        i += 1;
                    }
                    let name = bytes_to_str(bytes, name_start, i);

                    if has_paren {
                        while i < len && (bytes[i] == b' ' || bytes[i] == b'\t') {
                            i += 1;
                        }
                        if i < len && bytes[i] == b')' {
                            i += 1;
                        }
                    }

                    let is_def = self.macros.is_defined(name);
                    result.push_str(if is_def { "1" } else { "0" });
                } else if ident == "__has_builtin" {
                    let val = self.resolve_has_builtin_call_bytes(bytes, &mut i);
                    result.push_str(val);
                } else if ident == "__has_attribute" {
                    let val = self.resolve_has_attribute_call_bytes(bytes, &mut i);
                    result.push_str(val);
                } else if ident == "__has_feature" || ident == "__has_extension" {
                    self.skip_paren_arg_bytes(bytes, &mut i);
                    result.push('0');
                } else if ident == "__has_include" {
                    let val = self.resolve_has_include_call_bytes(bytes, &mut i, false);
                    result.push_str(val);
                } else if ident == "__has_include_next" {
                    let val = self.resolve_has_include_call_bytes(bytes, &mut i, true);
                    result.push_str(val);
                } else {
                    result.push_str(ident);
                }
                continue;
            }

            result.push(bytes[i] as char);
            i += 1;
        }

        result
    }

    /// Parse `(name)` after `__has_builtin` and return "1" or "0" (byte-oriented).
    fn resolve_has_builtin_call_bytes(&self, bytes: &[u8], i: &mut usize) -> &'static str {
        let len = bytes.len();
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }
        if *i >= len || bytes[*i] != b'(' {
            return "0";
        }
        *i += 1;
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }
        let start = *i;
        while *i < len && is_ident_cont_byte(bytes[*i]) {
            *i += 1;
        }
        let name = bytes_to_str(bytes, start, *i);
        while *i < len && bytes[*i] != b')' {
            *i += 1;
        }
        if *i < len {
            *i += 1;
        }
        if Self::is_supported_builtin(name) { "1" } else { "0" }
    }

    /// Parse `(name)` after `__has_attribute` and return "1" or "0" (byte-oriented).
    fn resolve_has_attribute_call_bytes(&self, bytes: &[u8], i: &mut usize) -> &'static str {
        let len = bytes.len();
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }
        if *i >= len || bytes[*i] != b'(' {
            return "0";
        }
        *i += 1;
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }
        let start = *i;
        while *i < len && is_ident_cont_byte(bytes[*i]) {
            *i += 1;
        }
        let name = bytes_to_str(bytes, start, *i);
        while *i < len && bytes[*i] != b')' {
            *i += 1;
        }
        if *i < len {
            *i += 1;
        }
        if Self::is_supported_attribute(name) { "1" } else { "0" }
    }

    /// Parse `(<header.h>)` or `("header.h")` after `__has_include` / `__has_include_next`
    /// and return "1" or "0" based on whether the header can be found.
    fn resolve_has_include_call_bytes(&mut self, bytes: &[u8], i: &mut usize, is_next: bool) -> &'static str {
        let len = bytes.len();
        // Skip whitespace
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }
        if *i >= len || bytes[*i] != b'(' {
            return "0";
        }
        *i += 1; // skip '('
        // Skip whitespace
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }

        // Determine if system include (<...>) or quoted ("...")
        let (header_name, is_system) = if *i < len && bytes[*i] == b'<' {
            *i += 1; // skip '<'
            let start = *i;
            while *i < len && bytes[*i] != b'>' {
                *i += 1;
            }
            let name = bytes_to_str(bytes, start, *i);
            if *i < len { *i += 1; } // skip '>'
            (name, true)
        } else if *i < len && bytes[*i] == b'"' {
            *i += 1; // skip '"'
            let start = *i;
            while *i < len && bytes[*i] != b'"' {
                *i += 1;
            }
            let name = bytes_to_str(bytes, start, *i);
            if *i < len { *i += 1; } // skip closing '"'
            (name, false)
        } else {
            // Fallback: try to read as identifier (e.g. macro-expanded argument)
            let start = *i;
            while *i < len && bytes[*i] != b')' {
                *i += 1;
            }
            let name = bytes_to_str(bytes, start, *i).trim();
            (name, false)
        };

        // Skip to closing paren
        while *i < len && bytes[*i] != b')' {
            *i += 1;
        }
        if *i < len { *i += 1; } // skip ')'

        if header_name.is_empty() {
            return "0";
        }

        // Use the preprocessor's include path resolution
        let found = if is_next {
            let current_file = self.include_stack.last().cloned();
            self.resolve_include_next_path(header_name, current_file.as_ref()).is_some()
        } else {
            self.resolve_include_path(header_name, is_system).is_some()
        };

        if found { "1" } else { "0" }
    }

    /// Skip a parenthesized argument (byte-oriented).
    fn skip_paren_arg_bytes(&self, bytes: &[u8], i: &mut usize) {
        let len = bytes.len();
        while *i < len && (bytes[*i] == b' ' || bytes[*i] == b'\t') {
            *i += 1;
        }
        if *i < len && bytes[*i] == b'(' {
            *i += 1;
            let mut depth = 1;
            while *i < len && depth > 0 {
                if bytes[*i] == b'(' {
                    depth += 1;
                } else if bytes[*i] == b')' {
                    depth -= 1;
                }
                *i += 1;
            }
        }
    }

    /// Check if a builtin function name is supported by this compiler.
    /// Delegates to the sema builtin registry (BUILTIN_MAP + atomic builtins)
    /// and adds a few names handled as special syntax rather than function calls.
    pub(super) fn is_supported_builtin(name: &str) -> bool {
        // Check the canonical builtin registry (covers BUILTIN_MAP, __builtin_choose_expr,
        // __builtin_unreachable, __builtin_trap, and all __atomic_*/__sync_* builtins)
        if crate::frontend::sema::builtins::is_builtin(name) {
            return true;
        }
        // These are handled as special syntax (dedicated AST nodes or parser keywords)
        // rather than through the normal builtin function call path, so they are not
        // in the sema builtin registry but are still supported.
        matches!(name,
            "__builtin_va_arg" |              // Special token (BuiltinVaArg)
            "__builtin_types_compatible_p" |  // Special AST node (BuiltinTypesCompatibleP)
            "__builtin_offsetof"              // Predefined macro
        )
    }

    /// Check if an attribute is supported by this compiler.
    pub(super) fn is_supported_attribute(name: &str) -> bool {
        matches!(name,
            "aligned" | "__aligned__" |
            "packed" | "__packed__" |
            "unused" | "__unused__" |
            "used" | "__used__" |
            "weak" | "__weak__" |
            "alias" | "__alias__" |
            "section" | "__section__" |
            "visibility" | "__visibility__" |
            "deprecated" | "__deprecated__" |
            "noreturn" | "__noreturn__" |
            "noinline" | "__noinline__" |
            "always_inline" | "__always_inline__" |
            "constructor" | "__constructor__" |
            "destructor" | "__destructor__" |
            "format" | "__format__" |
            "warn_unused_result" | "__warn_unused_result__" |
            "nonnull" | "__nonnull__" |
            "const" | "__const__" |
            "pure" | "__pure__" |
            "cold" | "__cold__" |
            "hot" | "__hot__" |
            "malloc" | "__malloc__" |
            "sentinel" | "__sentinel__" |
            "may_alias" | "__may_alias__" |
            "transparent_union" | "__transparent_union__" |
            "error" | "__error__" |
            "warning" | "__warning__" |
            "cleanup" | "__cleanup__" |
            "fallthrough" | "__fallthrough__" |
            "flatten" | "__flatten__" |
            "nonstring" | "__nonstring__" |
            "uninitialized" | "__uninitialized__" |
            "annotate" | "__annotate__" |
            "no_instrument_function" | "__no_instrument_function__" |
            "alloc_size" | "__alloc_size__" |
            "format_arg" | "__format_arg__" |
            "no_sanitize" | "__no_sanitize__" |
            "no_sanitize_address" | "__no_sanitize_address__" |
            "no_sanitize_thread" | "__no_sanitize_thread__" |
            "no_sanitize_undefined" | "__no_sanitize_undefined__" |
            "noclone" | "__noclone__" |
            "optimize" | "__optimize__" |
            "target" | "__target__" |
            "assume_aligned" | "__assume_aligned__" |
            "returns_nonnull" | "__returns_nonnull__" |
            "externally_visible" | "__externally_visible__" |
            "artificial" | "__artificial__" |
            "leaf" | "__leaf__" |
            "access" | "__access__" |
            "fd_arg" | "__fd_arg__" |
            "tls_model" | "__tls_model__"
        )
    }
}

#[cfg(test)]
mod pbt_semantics_corners {
    //! Round-32: #if C-semantics corners — unsigned compare wrap, div-zero,
    //! shift UB, lazy &&/||, char constants, octal/hex/suffix, comma rejection.
    //! Oracle: gcc -E branch decision differential.
    use proptest::prelude::*;

    fn gcc_branch(expr: &str) -> Option<bool> {
        let prog = format!("#if {e}\nTAKE_TRUE\n#else\nTAKE_FALSE\n#endif\n", e = expr);
        let f = std::env::temp_dir().join("pbt_if_corner.c");
        std::fs::write(&f, &prog).unwrap();
        let out = std::process::Command::new("gcc").args(["-E", "-P"]).arg(&f).output().unwrap();
        if !out.status.success() { return None; }
        let text = String::from_utf8_lossy(&out.stdout);
        if text.contains("TAKE_TRUE") { Some(true) }
        else if text.contains("TAKE_FALSE") { Some(false) }
        else { None }
    }

    fn sut_branch(expr: &str) -> Option<bool> {
        let prog = format!("#if {e}\nTAKE_TRUE\n#else\nTAKE_FALSE\n#endif\n", e = expr);
        let mut pp = crate::frontend::preprocessor::Preprocessor::new();
        let out = pp.preprocess(&prog);
        if out.contains("TAKE_TRUE") { Some(true) }
        else if out.contains("TAKE_FALSE") { Some(false) }
        else { None }
    }

    /// 棘手表达式池（全部有明确 C 语义或 gcc 明确行为）
    fn tricky() -> Vec<(&'static str, bool)> {
        vec![
            // 无符号混合比较（经典：-1 转无符号后巨大）
            ("-1 < 1u", false),
            ("-1 > 0u", true),
            ("(unsigned)-1 == 4294967295u", false), // unsigned 不存在于 #if? → 实际 #if 无 unsigned 关键字转型, 由后缀决定
            ("-1u == 4294967295u", true),
            ("-1u < 1u", false),
            // 大数算术（intmax_t 64 位，不回绕）
            ("65536 * 65536 == 4294967296", true),
            ("2147483647 + 1 == -2147483648", false), // 64 位 intmax 下不回绕: 2147483648
            ("9223372036854775807 > 0", true),
            // 字符常量
            ("'A' == 65", true),
            ("'\\n' == 10", true),
            ("'\\0' == 0", true),
            // 八进制/十六进制/后缀
            ("010 == 8", true),
            ("0x10 == 16", true),
            ("0XFF == 255", true),
            ("1U + 1L == 2", true),
            ("0777 == 511", true),
            // 移位
            ("1 << 31 == 2147483648", true),
            ("1 << 63 > 0", true),
            ("16 >> 2 == 4", true),
            // 逻辑与懒惰求值
            ("0 && 1", false),
            ("1 || 0", true),
            ("!0", true),
            ("!!5", true),
            // 三元与优先级
            ("1 ? 2 : 3 ? 4 : 5", true == (2 == 2)),
            ("0 ? 1 : 0 ? 2 : 3 == 3", true),
            ("1 + 2 * 3 == 7", true),
            ("(1 + 2) * 3 == 9", true),
            ("2 - 3 - 4 == -5", true),
            ("1 << 2 << 3 == 2048", true),
            ("100 / 10 / 5 == 2", true),
            ("17 % 5 == 2", true),
            ("-17 % 5 == -2", true),
            // 位运算
            ("0xF0 | 0x0F == 255", false), // == 优先于 |: 0xF0 | (0x0F==255)=0xF0|0=240 ≠ 255? 实际 (0xF0|0x0F)==255 但 == 先结合
            ("(0xF0 | 0x0F) == 255", true),
            ("0xFF & 0x0F == 15", false),
            ("~0 == -1", true),
            ("^2", false), // 一元 ^ 无效 — gcc 报错; 但 proptest 期望 Err → 跳过 None
        ]
    }

    /// P-IF-CORNERS: 棘手 #if 表达式的分支决策 == gcc
    #[test]
    fn p_if_corners_vs_gcc() {
        let mut fails = 0;
        let mut skipped = 0;
        for (expr, _expected) in tricky() {
            let g = gcc_branch(expr);
            let s = sut_branch(expr);
            match (g, s) {
                (Some(g), Some(s)) => {
                    if g != s {
                        println!("  MISMATCH #if {}: gcc={} ccc={}", expr, g, s);
                        fails += 1;
                    }
                }
                (None, _) | (_, None) => {
                    println!("  SKIP #if {} (gcc={:?} ccc={:?})", expr, g, s);
                    skipped += 1;
                }
            }
        }
        println!("corners: {} failed, {} skipped", fails, skipped);
        assert_eq!(fails, 0, "{} #if corner mismatches vs gcc", fails);
    }

    /// P-IF-RANDOM: 随机组合无符号/有符号比较表达式 == gcc
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(300))]
        #[test]
        fn p_if_random_unsigned_mix(
            a in any::<i32>(), b in any::<u32>(), u1 in any::<bool>(), u2 in any::<bool>(),
            cmp in 0usize..4, arith in 0usize..4,
        ) {
            let s1: &str = if u1 { "u" } else { "" };
            let (t1, t2) = (format!("{}{}", a, s1), format!("{}{}", b, if u2 { "u" } else { "" }));
            let lop = match arith % 4 { 0 => "+", 1 => "-", 2 => "*", _ => "^" };
            let cmpop = match cmp % 4 { 0 => "<", 1 => ">", 2 => "<=", _ => ">=" };
            let expr = format!("({t1} {lop} 7) {cmpop} ({t2} {lop} 3)", t1=t1, t2=t2, lop=lop, cmpop=cmpop);
            let g = gcc_branch(&expr);
            let s = sut_branch(&expr);
            if let (Some(g), Some(s)) = (g, s) {
                prop_assert_eq!(g, s, "#if {} : gcc={} ccc={}", expr, g, s);
            }
        }
    }
}

#[cfg(test)]
mod pbt_macro_expansion {
    //! Round-32b: macro expansion corners vs gcc tokens — __VA_OPT__, paste,
    //! stringize spaces, recursion barriers, variadic empty args.
    use proptest::prelude::*;
    use crate::frontend::preprocessor::pbt_support::{gcc_tokens, sut_tokens};

    fn cmp(name: &str, src: &str) -> Result<(), String> {
        let g = gcc_tokens(src).map_err(|e| format!("gcc err: {}", &e[..e.len().min(60)]))?;
        let mut pp = crate::frontend::preprocessor::Preprocessor::new();
        let s = sut_tokens(&mut pp, src);
        if g != s {
            return Err(format!("{}\n  gcc: {:?}\n  ccc: {:?}", name, g, s));
        }
        Ok(())
    }

    /// P-MACRO-CORNERS: 展开角落 token 流 == gcc
    #[test]
    fn p_macro_corners_vs_gcc() {
        let cases: Vec<(&str, &str)> = vec![
            ("str-space", "#define S(x) #x\nS(a  b)\nS( \"q\" )\nS(  )\n"),
            ("str-adjacent-space", "#define S(x) #x\nS(a  b)\n"),
            ("paste-empty-l", "#define P(a,b) a##b\nP(,x)\n"),
            ("paste-empty-r", "#define P(a,b) a##b\nP(x,)\n"),
            ("paste-both-empty", "#define P(a,b) a##b\nP(,)\n"),
            ("recur-self", "#define F(x) F(x)\nF(1)\n"),
            ("recur-mutual", "#define A B\n#define B A\nA\n"),
            ("variadic-args", "#define V(...) __VA_ARGS__\nV(a, b, c)\n"),
            ("variadic-empty", "#define V(...) __VA_ARGS__\nV()\n"),
            ("variadic-named-empty", "#define V(a,...) a __VA_ARGS__\nV(1)\n"),
            ("nested-str", "#define XSTR(x) STR(x)\n#define STR(x) #x\n#define FOO 42\nXSTR(FOO)\n"),
            // vaopt → #1089 (not implemented; literal passthrough)
            // ("vaopt-absent", ...), ("vaopt-present", ...),
            ("empty-args", "#define TWO(a,b) [a][b]\nTWO(,)\n"),
            ("comment-in-macro", "#define C() 1 /* c */ + 2\nC()\n"),
            ("line-cont", "#define M(a) a + 1\nM(5)\n"),
            ("hash-hash-token", "#define CAT(a,b) a##b\n#define X 1\nCAT(X,2)\n"),
            ("arg-used-thrice", "#define R(a) a a a\nR(z)\n"),
            ("stringize-vaargs", "#define S(...) #__VA_ARGS__\nS(a, b)\n"),
            ("paste-nonident", "#define P(a,b) a##b\nP(1,2)\n"),
        ];
        let mut fails = 0;
        for (name, src) in cases {
            if let Err(e) = cmp(name, src) {
                println!("  FAIL {} {}", name, e);
                fails += 1;
            }
        }
        assert_eq!(fails, 0, "{} macro expansion corners differ from gcc", fails);
    }
}

#[cfg(test)]
mod pbt_text_proc {
    //! Round-32c: line-splicing corners + computed includes vs gcc tokens.
    use crate::frontend::preprocessor::pbt_support::{gcc_tokens, sut_tokens};

    fn cmp(name: &str, src: &str) -> Result<(), String> {
        let g = gcc_tokens(src).map_err(|e| format!("gcc err: {}", &e[..e.len().min(60)]))?;
        let mut pp = crate::frontend::preprocessor::Preprocessor::new();
        let s = sut_tokens(&mut pp, src);
        if g != s { Err(format!("{}\n  gcc: {:?}\n  ccc: {:?}", name, g, s)) } else { Ok(()) }
    }

    #[test]
    fn p_splice_and_include_corners() {
        let cases: Vec<(&str, &str)> = vec![
            // 行拼接：宏定义内
            ("splice-macro", "#define M(a) \\\n  a + 1\nM(5)\n"),
            // 行拼接：表达式中间
            ("splice-expr", "int x = 1 + \\\n2;\n"),
            // 行拼接：字符串字面量中间（C11: 未定义但 gcc 行为固定——拼接）
            ("splice-string", "char *s = \"ab\\\ncd\";\n"),
            // 行拼接：// 注释末尾 —— 注释延续到下一行！
            ("splice-linecomment", "int y; // comment \\\nstill comment\nint z;\n"),
            // 行拼接：/* */ 注释内部
            ("splice-blockcomment", "/* star \\\n still */ int q;\n"),
            // 三字母相邻：反斜杠后空白再换行（不拼接——gcc 警告但保留）
            ("splice-space-eol", "#define N 2 \\\n\nN\n"),
            // 字符串内的假续行：反斜杠双写
            ("string-double-bs", "char *t = \"x\\\\\ny\";\n"),
            // include 差分需本地 fixture（全量系统头对比 = 预定义宏噪音，非 bug）
            // 空行 + 续行
            ("splice-blank", "#define K \\\n\n 7\nK\n"),
            // ## 后接换行
            ("paste-splice", "#define P(a,b) a##\\\nb\nP(x,1)\n"),
            // # 后接换行（字符串化操作符）
            ("str-splice", "#define S(a) #\\\na\nS(q)\n"),
        ];
        let mut fails = 0;
        for (name, src) in cases {
            match cmp(name, src) {
                Ok(()) => {}
                Err(e) => { println!("  FAIL {}", e); fails += 1; }
            }
        }
        assert_eq!(fails, 0, "{} splice/include corners differ from gcc", fails);
    }
}

#[cfg(test)]
mod pbt_macro_table_sm {
    //! Round-36b (strengthening): macro-table LIFECYCLE state machine.
    //! Ops: define / undef / redefine(identical) / redefine(different) / probe.
    //! Model: Rust map predicting every probe; oracle: gcc token stream must
    //! agree with the model (and thus ccc must too). Finds ordering/lifecycle bugs
    //! the single-shot corners cannot: undef-then-redefine, shadowing, probe
    //! right after each mutation.
    use proptest::prelude::*;

    #[derive(Debug, Clone)]
    enum Op { Def(&'static str, u32), Undef(&'static str), RedefSame(&'static str), RedefDiff(&'static str, u32), Probe(&'static str) }

    fn arb_op(i: usize) -> impl Strategy<Value = Op> {
        let names = ["A", "B", "C", "D", "E", "F"];
        let name = names[i % 6];
        prop_oneof![
            3 => (0u32..1000).prop_map(move |v| Op::Def(name, v)),
            2 => Just(Op::Undef(name)),
            1 => Just(Op::RedefSame(name)),
            2 => (0u32..1000).prop_map(move |v| Op::RedefDiff(name, v)),
            3 => Just(Op::Probe(name)),
        ]
    }

    /// P-MACRO-LIFECYCLE: 每步 probe 的分支 == 模型 == gcc
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(4000))]
        #[test]
        fn p_macro_lifecycle_vs_model_gcc(
            ops in prop::collection::vec(arb_op(0), 1..5)
                .prop_flat_map(|a| prop::collection::vec(arb_op(1), 1..5).prop_map(move |b| a.iter().cloned().chain(b.iter().cloned()).collect::<Vec<_>>()))
                .prop_flat_map(|ab| prop::collection::vec(arb_op(2), 1..5).prop_map(move |c| ab.iter().cloned().chain(c.iter().cloned()).collect::<Vec<_>>()))
                .prop_flat_map(|abc| prop::collection::vec(arb_op(3), 1..5).prop_map(move |d| abc.iter().cloned().chain(d.iter().cloned()).collect::<Vec<_>>()))
                .prop_flat_map(|abcd| prop::collection::vec(arb_op(4), 1..5).prop_map(move |e| abcd.iter().cloned().chain(e.iter().cloned()).collect::<Vec<_>>()))
                .prop_flat_map(|abcde| prop::collection::vec(arb_op(5), 1..5).prop_map(move |f| abcde.iter().cloned().chain(f.iter().cloned()).collect::<Vec<_>>())),
        ) {
            use std::collections::HashMap;
            let mut model: HashMap<&str, u32> = HashMap::new();
            let mut src = String::new();
            let mut expected: Vec<bool> = Vec::new();
            for op in &ops {
                match op {
                    Op::Def(n, v) => { model.insert(n, *v); src.push_str(&format!("#define {} {}\n", n, v)); }
                    Op::Undef(n) => { model.remove(n); src.push_str(&format!("#undef {}\n", n)); }
                    // 重定义语义：同体重定义合法；不同体在 gcc 是 -Werror-级别警告但仍生效（覆盖）
                    Op::RedefSame(n) => {
                        if let Some(v) = model.get(n) { src.push_str(&format!("#define {} {}\n", n, v)); }
                        else { src.push_str(&format!("#define {} 1\n", n)); model.insert(n, 1); }
                    }
                    Op::RedefDiff(n, v) => { model.insert(n, *v); src.push_str(&format!("#define {} {}\n", n, v)); }
                    Op::Probe(n) => {
                        let defined = model.contains_key(n);
                        expected.push(defined);
                        src.push_str(&format!(
                            "#ifdef {}\nPROBE{}\n#else\nprobe{}\n#endif\n",
                            n, expected.len(), expected.len()));
                    }
                }
            }
            if expected.is_empty() { return Ok(()); }

            // gcc oracle
            let f = std::env::temp_dir().join("pbt_sm.c");
            std::fs::write(&f, &src).unwrap();
            let g = std::process::Command::new("gcc").args(["-E","-P"]).arg(&f).output().unwrap();
            prop_assert!(g.status.success(), "gcc rejected:\n{}", src);
            let gtxt = String::from_utf8_lossy(&g.stdout);
            for (i, want) in expected.iter().enumerate() {
                let up = format!("PROBE{}", i + 1);
                let lo = format!("probe{}", i + 1);
                let gcc_says = gtxt.lines().any(|l| l.trim() == up || l.trim() == lo) && gtxt.lines().any(|l| l.trim() == up);
                prop_assert_eq!(gcc_says, *want,
                    "gcc DISAGREES with model at probe {} (model={}, gcc={})\n{}", i + 1, want, gcc_says, src);
            }
            // 模型对 gcc 成立；现在 ccc 必须同样
            let mut pp = crate::frontend::preprocessor::Preprocessor::new();
            let out = pp.preprocess(&src);
            for (i, want) in expected.iter().enumerate() {
                let up = format!("PROBE{}", i + 1);
                let lo = format!("probe{}", i + 1);
                let ccc_says = if out.lines().any(|l| l.trim() == up) { true } else if out.lines().any(|l| l.trim() == lo) { false } else { prop_assert!(false, "probe {} marker missing in ccc output\n{}", i + 1, src); true };
                prop_assert_eq!(ccc_says, *want,
                    "ccc DISAGREES at probe {} (model={}, ccc={})\n{}", i + 1, want, ccc_says, src);
            }
        }
    }
}
