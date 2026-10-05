# PBT Coverage Status — src/frontend/lexer (round 02)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| Lexer::tokenize | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P1, P3, P9, P10 + all) |
| Lexer::next_token | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (via tokenize, all properties) |
| Lexer::skip_whitespace_and_comments | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | FAIL→B5 via P9/T2 (unterminated comment leaks last byte) |
| Lexer::is_line_marker | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (T1 line markers) |
| Lexer::lex_number | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (dispatch via P3/P4/P5/P7) |
| Lexer::lex_hex_number | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | FAIL→B3 via P7 (0x…17 digits → 0) |
| Lexer::lex_hex_float | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | FAIL→B1 (P6a) and B2 (P6b); exact domain passes (P5) |
| Lexer::lex_binary_number | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | FAIL→B3 via P7 (binary arm) |
| Lexer::lex_octal_number | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P3/P4/P7/T4 incl. ellipsis special case) |
| Lexer::finish_int_literal | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P3/P4/P11) |
| Lexer::lex_decimal_number | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | FAIL→B3 (decimal >u64 → 0, probe); ≤u64 passes (p7b) |
| Lexer::parse_float_suffix | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P11 imaginary combos) |
| Lexer::make_float_token | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P5/P8 string-float paths) |
| Lexer::parse_int_suffix | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P3/P4/P11/P11b random suffix compositions) |
| Lexer::make_int_token | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P3 round-trip + P4 exact boundary matrix, LP64) |
| Lexer::lex_string | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P8/P9/P12 escapes, u8 equivalence) |
| Lexer::lex_wide_string | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P12 code-point storage) |
| Lexer::lex_char16_string | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P12) |
| Lexer::lex_wide_char | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P8c/P12 incl. surrogate fallback) |
| Lexer::lex_char | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P8a-d; multichar int-typed packing matches GCC) |
| Lexer::lex_escape_char | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P8a/P8b full documented table) |
| Lexer::lex_unicode_escape | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P8c/P12 incl. U+FFFD fallback) |
| Lexer::lex_identifier | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P1 round-trip) |
| Lexer::lex_punctuation | scan.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | FAIL→B4 via P13 (stack overflow at ~4000 unknown chars; safe depths pass) |
| Display::fmt (TokenKind) | token.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P2 canonical Display check) |
| TokenKind::from_keyword | token.rs | src/frontend/lexer/scan.rs | cargo test --lib frontend::lexer | pass (P2 gnu/strict differential) |

Excluded (see FUNCTION_INDEX.md): Lexer::new (constructor, exercised by every test), set_gnu_extensions (exercised in P2), peek_next, parse_simple_float_suffix (via callers), try_pragma_pack_token / try_pragma_visibility_token (T3 deterministic), hex_digit_val, Token::new, Token::is_eof (trivial).
