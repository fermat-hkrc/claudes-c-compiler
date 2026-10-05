//! Text processing utilities for the preprocessor.
//!
//! Low-level text transformations: block/line comment stripping,
//! line continuation (backslash-newline) joining, parenthesis
//! balancing checks, and directive line splitting.

use super::utils::{skip_literal_bytes, copy_literal_bytes_raw};
use super::pipeline::Preprocessor;

/// Mapping from output line numbers to original source line numbers.
///
/// After stripping block comments, output line numbers may differ from
/// source line numbers (multi-line block comments consume source lines
/// without producing output lines). This enum tracks the mapping:
///
/// - `Identity`: output line == source line (no multi-line block comments).
///   This is the common case and avoids allocating a Vec<usize>.
/// - `Mapped(Vec<usize>)`: line_map[output_line] = source_line.
pub(super) enum LineMap {
    /// Output line numbers equal source line numbers (common case).
    Identity,
    /// Explicit mapping from output line number to source line number.
    Mapped(Vec<usize>),
}

impl LineMap {
    /// Look up the source line for a given output line number.
    #[inline]
    pub(super) fn get(&self, output_line: usize) -> usize {
        match self {
            LineMap::Identity => output_line,
            LineMap::Mapped(v) => v.get(output_line).copied().unwrap_or(output_line),
        }
    }
}

/// Quick check for presence of comment markers (`/*` or `//`) in source bytes.
/// Scans for `/` bytes first, then checks the following byte. Since `/` is
/// relatively rare in C source (typically only in comments and division),
/// this quickly skips large stretches of non-slash bytes.
fn contains_comment_marker(bytes: &[u8]) -> bool {
    let len = bytes.len();
    if len < 2 {
        return false;
    }
    // Search for '/' bytes, then check what follows
    let mut i = 0;
    while i < len - 1 {
        // Skip non-slash bytes quickly
        if bytes[i] != b'/' {
            i += 1;
            continue;
        }
        let next = bytes[i + 1];
        if next == b'*' || next == b'/' {
            return true;
        }
        i += 1;
    }
    false
}

impl Preprocessor {
    /// Check if a line has unbalanced parentheses, indicating a multi-line
    /// macro invocation that needs to be joined with subsequent lines.
    /// Skips string/char literals and line comments.
    pub(super) fn has_unbalanced_parens(line: &str) -> bool {
        let mut depth: i32 = 0;
        let bytes = line.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            match bytes[i] {
                b'"' | b'\'' => {
                    i = skip_literal_bytes(bytes, i, bytes[i]);
                }
                b'(' => { depth += 1; i += 1; }
                b')' => { depth -= 1; i += 1; }
                b'/' if i + 1 < len && bytes[i + 1] == b'/' => break,
                _ => { i += 1; }
            }
        }

        depth > 0
    }

    /// Strip C-style block comments (/* ... */) and C++ line comments (// ...).
    /// Returns the stripped source and a `LineMap` for correct __LINE__ tracking.
    ///
    /// The `LineMap` is either `Identity` (when no multi-line block comments
    /// shifted line numbering — the common case, avoiding a Vec allocation)
    /// or `Mapped(Vec<usize>)` with an explicit mapping from output line
    /// numbers to original source line numbers.
    ///
    /// Block comments are replaced with a single space (per C11 5.1.1.2 phase 3),
    /// which avoids breaking preprocessor directives that have block comments
    /// between `#` and the directive keyword (e.g., `#/*...\n...*/if 1`).
    /// Uses raw byte operations to preserve UTF-8 sequences in string literals.
    pub(super) fn strip_block_comments(source: &str) -> (std::borrow::Cow<'_, str>, LineMap) {
        let bytes = source.as_bytes();
        let len = bytes.len();

        // Fast path: if no block comment or line comment markers exist, return source as-is.
        // This avoids both the byte-by-byte scan and the Vec<u8> allocation.
        if !contains_comment_marker(bytes) {
            return (std::borrow::Cow::Borrowed(source), LineMap::Identity);
        }

        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        let mut i = 0;
        // Track source line number (0-based) as we scan through input
        let mut src_line: usize = 0;
        // Whether any block comment spans multiple lines (shifts line numbering)
        let mut has_multiline_block_comment = false;
        // For each output line, record which source line it corresponds to
        // Only populated if has_multiline_block_comment becomes true
        let mut line_map: Vec<usize> = Vec::new();
        // Track the source line at the start of the current output line
        let mut current_output_line_src = 0usize;

        while i < len {
            match bytes[i] {
                b'"' | b'\'' => {
                    // Copy string/char literals verbatim (don't strip comments inside)
                    let old_i = i;
                    i = copy_literal_bytes_raw(bytes, i, bytes[i], &mut result);
                    // Count newlines in the literal for source line tracking
                    for &b in &bytes[old_i..i] {
                        if b == b'\n' {
                            src_line += 1;
                            if has_multiline_block_comment {
                                line_map.push(current_output_line_src);
                            }
                            current_output_line_src = src_line;
                        }
                    }
                }
                b'/' if i + 1 < len && bytes[i + 1] == b'*' => {
                    // Block comment - replace entire comment with a single space
                    i += 2;
                    result.push(b' ');
                    let comment_start_line = src_line;
                    while i < len {
                        if i + 1 < len && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                            i += 2;
                            break;
                        }
                        if bytes[i] == b'\n' {
                            src_line += 1;
                        }
                        i += 1;
                    }
                    // If block comment spanned multiple lines, we need the full line map
                    if src_line > comment_start_line && !has_multiline_block_comment {
                        has_multiline_block_comment = true;
                        // Retroactively build the line map for all output lines so far
                        // All previous lines had identity mapping (output line == source line)
                        let output_lines_so_far = result.iter().filter(|&&b| b == b'\n').count();
                        line_map.clear();
                        line_map.reserve(output_lines_so_far + 16);
                        for line_idx in 0..output_lines_so_far {
                            line_map.push(line_idx);
                        }
                    }
                    // Don't emit newlines - the comment is fully replaced by one space
                }
                b'/' if i + 1 < len && bytes[i + 1] == b'/' => {
                    // Line comment - skip to end of line
                    i += 2;
                    while i < len && bytes[i] != b'\n' {
                        i += 1;
                    }
                }
                b'\n' => {
                    result.push(b'\n');
                    if has_multiline_block_comment {
                        line_map.push(current_output_line_src);
                    }
                    src_line += 1;
                    current_output_line_src = src_line;
                    i += 1;
                }
                _ => {
                    result.push(bytes[i]);
                    i += 1;
                }
            }
        }
        // Record the last line
        if has_multiline_block_comment {
            line_map.push(current_output_line_src);
        }

        // Input was valid UTF-8, and we only removed/replaced ASCII characters
        // (comments with spaces/newlines), so result is still valid UTF-8.
        let text = String::from_utf8(result)
            .expect("comment stripping produced non-UTF8 (input was valid UTF-8)");
        if has_multiline_block_comment {
            (std::borrow::Cow::Owned(text), LineMap::Mapped(line_map))
        } else {
            (std::borrow::Cow::Owned(text), LineMap::Identity)
        }
    }

    /// Join lines that end with backslash (line continuation).
    /// Also handles backslash followed by trailing whitespace before newline,
    /// matching GCC/Clang behavior (GCC warns: "backslash and newline separated by space").
    ///
    /// Returns `Cow::Borrowed` when the source has no continuation backslashes
    /// (the common case), avoiding a String allocation entirely.
    pub(super) fn join_continued_lines<'a>(&self, source: &'a str) -> std::borrow::Cow<'a, str> {
        // Fast path: if the source contains no backslashes at all, there can't
        // be any line continuations. We check for '\' rather than "\\\n" because
        // GCC/Clang treat backslash + whitespace + newline as a continuation too.
        if !source.contains('\\') {
            return std::borrow::Cow::Borrowed(source);
        }

        let mut result = String::with_capacity(source.len());
        let mut continuation = false;

        for line in source.lines() {
            let bs_pos = Self::find_continuation_backslash(line);
            if continuation {
                // This line continues from the previous
                if let Some(pos) = bs_pos {
                    result.push_str(&line[..pos]);
                    // Still continuing
                } else {
                    result.push_str(line);
                    result.push('\n');
                    continuation = false;
                }
            } else if let Some(pos) = bs_pos {
                result.push_str(&line[..pos]);
                continuation = true;
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        std::borrow::Cow::Owned(result)
    }

    /// Check if a line ends with a continuation backslash (optionally followed by whitespace).
    /// Returns the byte position of the backslash if found, None otherwise.
    pub(super) fn find_continuation_backslash(line: &str) -> Option<usize> {
        let trimmed = line.trim_end();
        if trimmed.ends_with('\\') {
            Some(trimmed.len() - 1)
        } else {
            None
        }
    }
}

/// Strip a // comment from a directive line, but not inside string literals.
/// Returns a `Cow<str>` to avoid allocation when no comment is found (the
/// common case). Only allocates a new String when a `//` comment is present
/// and needs to be stripped.
pub(super) fn strip_line_comment(line: &str) -> std::borrow::Cow<'_, str> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b'"' | b'\'' => {
                i = skip_literal_bytes(bytes, i, bytes[i]);
            }
            b'/' if i + 1 < len && bytes[i + 1] == b'/' => {
                return std::borrow::Cow::Owned(line[..i].trim_end().to_string());
            }
            _ => i += 1,
        }
    }

    std::borrow::Cow::Borrowed(line)
}

/// Split a string into the first word and the rest.
/// For preprocessor directives, '(' is also a word boundary so that
/// `#if(expr)` is correctly parsed as keyword="if", rest="(expr)".
pub(super) fn split_first_word(s: &str) -> (&str, &str) {
    let s = s.trim();
    if let Some(pos) = s.find(|c: char| c.is_whitespace() || c == '(') {
        if s.as_bytes()[pos] == b'(' {
            // Don't trim the '(' - it's part of the rest
            (&s[..pos], &s[pos..])
        } else {
            (&s[..pos], s[pos..].trim())
        }
    } else {
        (s, "")
    }
}
#[cfg(test)]
mod pbt_tests {
    use super::*;
    use proptest::prelude::*;

    // ------------------------------------------------------------------
    // P4: join_continued_lines vs independent GCC phase-2 splicer.
    // ------------------------------------------------------------------

    fn arb_line_body() -> impl Strategy<Value = String> {
        proptest::collection::vec(
            prop_oneof![
                4 => "[A-Za-z0-9_]{1,6}",
                2 => "( +|\t)",
                1 => "[/]{1,2}",
            ],
            0..3,
        )
        .prop_map(|v| v.join(""))
    }

    fn arb_line_ending() -> impl Strategy<Value = String> {
        prop_oneof![
            7 => Just(String::new()),
            2 => Just(String::from("\\")),
            1 => Just(String::from("\\ \t")),
            1 => Just(String::from("x\\")),
        ]
    }

    /// Independent reference for phase 2 (the splice rule documented on the
    /// fn): a line whose trailing-whitespace-stripped form ends with `\` is
    /// spliced to the next line, dropping the backslash, any whitespace after
    /// it, and the newline. Other lines pass through plus a newline.
    fn splice_ref(src: &str, lines: &[String]) -> String {
        // documented fast path: no backslash anywhere -> identity
        if !src.contains('\\') {
            return src.to_string();
        }
        let mut out = String::new();
        let mut pending = false;
        for line in lines {
            let trimmed = line.trim_end();
            if trimmed.ends_with('\\') {
                out.push_str(&line[..trimmed.len() - 1]);
                pending = true;
            } else {
                out.push_str(line);
                out.push('\n');
                pending = false;
            }
        }
        out
    }

    #[test]
    fn p4_join_continued_lines_matches_reference_and_idempotent() {
        proptest!(ProptestConfig::with_cases(1024), |(
            body in arb_line_body(),
            end in arb_line_ending(),
            rest in proptest::collection::vec(
                (arb_line_body(), arb_line_ending()), 0..4),
        )| {
            let mut lines = vec![format!("{}{}", body, end)];
            for (b, e) in &rest {
                lines.push(format!("{}{}", b, e));
            }
            let src = lines.join("\n");
            let pp = Preprocessor::new();
            let joined = pp.join_continued_lines(&src);
            let src_lines: Vec<String> = src.lines().map(String::from).collect();
            let expected = splice_ref(&src, &src_lines);
            prop_assert_eq!(&joined, &expected);
            let joined2 = pp.join_continued_lines(&joined);
            prop_assert_eq!(&joined2, &joined);
        });
    }

    // ------------------------------------------------------------------
    // P5: strip_block_comments vs independent C11 phase-3 stripper.
    // ------------------------------------------------------------------

    fn comment_chunk() -> impl Strategy<Value = String> {
        prop_oneof![
            4 => "[A-Za-z0-9_]{1,5}",
            2 => " ",
            1 => "\n",
            1 => "\"[a-z]{0,5}\"",
            1 => "'[*]'",
            1 => "/*c*/",
            1 => "/*\nc*/",
            1 => "/**/",
            1 => "//c\n",
            1 => "// it's quoted\n",
            1 => "/",
            1 => "[*]",
        ]
        .prop_map(|s: String| s)
    }

    /// Independent reference for C11 5.1.1.2 phase 3 (README contract):
    /// each block comment becomes a single space; line comments are removed
    /// through (not including) the newline; string/char literals are copied
    /// verbatim; unterminated comments/literals scan to end of input.
    fn strip_ref(s: &str) -> String {
        let b = s.as_bytes();
        let len = b.len();
        let mut out = String::new();
        let mut i = 0;
        while i < len {
            match b[i] {
                b'"' | b'\'' => {
                    let q = b[i];
                    let start = i;
                    i += 1;
                    while i < len {
                        if b[i] == b'\\' && i + 1 < len {
                            i += 2;
                        } else if b[i] == q {
                            i += 1;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    out.push_str(&s[start..i]);
                }
                b'/' if i + 1 < len && b[i + 1] == b'*' => {
                    i += 2;
                    while i + 1 < len && !(b[i] == b'*' && b[i + 1] == b'/') {
                        i += 1;
                    }
                    i = (i + 2).min(len);
                    out.push(' ');
                }
                b'/' if i + 1 < len && b[i + 1] == b'/' => {
                    i += 2;
                    while i < len && b[i] != b'\n' {
                        i += 1;
                    }
                }
                c => {
                    out.push(c as char);
                    i += 1;
                }
            }
        }
        out
    }

    #[test]
    fn p5_strip_block_comments_matches_reference_and_idempotent() {
        proptest!(ProptestConfig::with_cases(1024), |(
            chunks in proptest::collection::vec(comment_chunk(), 0..12)
        )| {
            let src: String = chunks.join("");
            let (stripped, _) = Preprocessor::strip_block_comments(&src);
            let expected = strip_ref(&src);
            prop_assert_eq!(&stripped, &expected);
            let (stripped2, _) = Preprocessor::strip_block_comments(&stripped);
            prop_assert_eq!(&stripped2, &stripped);
        });
    }

    /// P5b: literal-preservation law — every generated string literal in the
    /// input appears verbatim in the stripped output.
    #[test]
    fn p5b_literals_preserved_verbatim() {
        proptest!(ProptestConfig::with_cases(512), |(
            pre in comment_chunk(),
            lit_body in "[a-z ]{0,6}",
            post in comment_chunk(),
        )| {
            let src = format!("{}\n\"{}\"{}", pre, lit_body, post);
            let (stripped, _) = Preprocessor::strip_block_comments(&src);
            prop_assert!(
                stripped.contains(&format!("\"{}\"", lit_body)),
                "src: {:?} out: {:?}",
                src,
                stripped
            );
        });
    }
}

// ===========================================================================
// PBT round 04 — change-surface obligation: split_first_word (changed by
// commit HEAD). Oracle: the function's own documented contract
// (text_processing.rs:284-287) — first word / rest split, '(' is also a word
// boundary so `#if(expr)` parses as keyword="if", rest="(expr)".
// ===========================================================================
#[cfg(test)]
mod pbt_round04 {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p9_split_first_word_contract(
            word in "[A-Za-z_][A-Za-z0-9_]{0,7}",
            gap in prop_oneof![Just(" "), Just("  "), Just("\t"), Just("")],
            rest in prop_oneof![
                3 => "[A-Za-z0-9_]{0,4}",
                2 => "(\\([A-Za-z0-9_]{0,4})",
                2 => "[A-Za-z0-9_]{1,4}( [A-Za-z0-9_]{1,4}){1,2}",
            ],
        ) {
            let s = format!("{word}{gap}{rest}");
            let t = s.trim().to_string();
            let (w, r) = split_first_word(&s);

            // Reconstruction: w is a prefix of the trimmed input, and r is
            // exactly what follows (leading ws skipped at a ws split; the '('
            // kept at a '(' split).
            prop_assert!(t.starts_with(w), "w={:?} not a prefix of t={:?}", w, t);
            let after = &t[w.len()..];
            if after.starts_with('(') {
                prop_assert_eq!(r, after);
            } else {
                prop_assert_eq!(r, after.trim_start());
            }
            // Word shape: a non-empty first word contains no whitespace and
            // no '(' (both are word boundaries per the doc comment).
            if !w.is_empty() {
                prop_assert!(!w.contains(|c: char| c.is_whitespace() || c == '('),
                    "word {:?} contains a boundary char", w);
            }
            // Maximality: the char after a non-empty w (when any) is a boundary.
            if !w.is_empty() && t.len() > w.len() {
                let c = t.as_bytes()[w.len()] as char;
                prop_assert!(c.is_whitespace() || c == '(',
                    "split before boundary-free char {:?} in {:?}", c, t);
            }

            // Documented KATs (text_processing.rs:286-287).
            prop_assert_eq!(split_first_word("#if(x)"), ("#if", "(x)"));
            prop_assert_eq!(split_first_word("define F(x) x"), ("define", "F(x) x"));
            prop_assert_eq!(split_first_word("endif"), ("endif", ""));
        }
    }
}
