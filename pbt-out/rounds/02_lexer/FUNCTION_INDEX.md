# Function Index — src/frontend/lexer (round 02)

> Total files: 3 (scan.rs, token.rs, mod.rs) | Total functions: 35 | PBT candidates: 28 | Excluded: 7

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| Lexer::new | scan.rs | 14 | method | yes | entry point, constructs SUT |
| Lexer::set_gnu_extensions | scan.rs | 23 | method | yes | strict-mode differential input |
| Lexer::tokenize | scan.rs | 27 | method | yes | primary public API |
| Lexer::next_token | scan.rs | 41 | method | yes | via tokenize |
| Lexer::skip_whitespace_and_comments | scan.rs | 76 | method | yes | whitespace/comment invariance |
| Lexer::is_line_marker | scan.rs | 127 | method | yes | documented line-marker contract (README:162) |
| Lexer::peek_next | scan.rs | 144 | method | no | trivial 1-byte lookahead helper |
| Lexer::lex_number | scan.rs | 152 | method | yes | dispatch, via number properties |
| Lexer::lex_hex_number | scan.rs | 173 | method | yes | hex int + hex float entry |
| Lexer::lex_hex_float | scan.rs | 204 | method | yes | value formula documented (README:181) |
| Lexer::parse_simple_float_suffix | scan.rs | 266 | method | no | covered via lex_hex_float suffixes |
| Lexer::lex_binary_number | scan.rs | 279 | method | yes | 0b literals |
| Lexer::lex_octal_number | scan.rs | 291 | method | yes | octal + backtrack to decimal, ellipsis case documented |
| Lexer::finish_int_literal | scan.rs | 319 | method | yes | suffix composition |
| Lexer::lex_decimal_number | scan.rs | 329 | method | yes | decimal int/float split |
| Lexer::parse_float_suffix | scan.rs | 378 | method | yes | documented suffix table (README:245) |
| Lexer::make_float_token | scan.rs | 418 | method | yes | float token construction |
| Lexer::parse_int_suffix | scan.rs | 447 | method | yes | documented suffix combos (README:213) |
| Lexer::make_int_token | scan.rs | 494 | method | yes | documented C11 promotion table (README:218) |
| Lexer::lex_string | scan.rs | 572 | method | yes | escapes, PUA, UTF-8 encoding |
| Lexer::lex_wide_string | scan.rs | 606 | method | yes | code-point storage contract |
| Lexer::lex_char16_string | scan.rs | 666 | method | yes | via wide-string family property |
| Lexer::lex_wide_char | scan.rs | 719 | method | yes | code-point contract |
| Lexer::lex_char | scan.rs | 772 | method | yes | multi-char packing documented (README:258) |
| Lexer::lex_escape_char | scan.rs | 815 | method | yes | documented escape table (README:275) |
| Lexer::lex_unicode_escape | scan.rs | 877 | method | yes | \u/\U code points, U+FFFD fallback documented |
| Lexer::lex_identifier | scan.rs | 892 | method | yes | identifier round-trip, prefix dispatch |
| Lexer::try_pragma_pack_token | scan.rs | 953 | method | no | synthetic preprocessor tokens, plain prefix+parse |
| Lexer::try_pragma_visibility_token | scan.rs | 983 | method | no | plain prefix match |
| Lexer::lex_punctuation | scan.rs | 1002 | method | yes | operator maximal munch + unknown-char path |
| hex_digit_val | scan.rs | 1181 | function | no | trivial table |
| Token::new | token.rs | 190 | method | no | trivial constructor |
| Token::is_eof | token.rs | 194 | method | no | trivial predicate |
| Display::fmt (TokenKind) | token.rs | 207 | method | yes | documented GCC-style display (token.rs:200) |
| TokenKind::from_keyword | token.rs | 372 | method | yes | keyword table + gnu_extensions contract (token.rs:367) |
