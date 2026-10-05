# PROPERTIES — src/frontend/lexer (round 02) — FINAL

All generated properties ran with proptest `ProptestConfig::with_cases(1024)`.
Test file: `src/frontend/lexer/scan.rs` (inline `mod pbt_tests` + `mod pbt_regression` — repo convention).
Run: `cargo test --lib frontend::lexer` from `pbt-out/rounds/02_lexer/run/`. LP64 thread-local default (8).
Final run: 20 passed, 8 failed (all = confirmed SUT bugs B1–B5), 1 ignored (B4 process-abort witness).

## P1 identifier_round_trip
- Tier: 2
- Rationale: identifiers are the purest round-trip surface; keyword/synthetic-pragma exclusions are documented (token.rs from_keyword; scan.rs try_pragma_pack_token doc comment).
- Doc contract: scan.rs:892 "Check for wide/unicode char/string prefixes: L'x', L\"...\", u\"...\"..." — other fingerprint 1a2b3c4d
- Seed: (none)
- Formal: ∀ s ∈ Ident (first ∈ [A-Za-z_$], rest ∈ [A-Za-z0-9_$], |s| ≤ 16, s ∉ Keywords, not __ccc_pack_/__ccc_visibility_ prefixed): tokenize(s) = [Identifier(s)@span(0,len), Eof@span(len,len)]
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::lex_identifier
oracle: algebraic.round_trip
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: identifier_chars, len(s) in 1..=16, s not keyword/synthetic }
  relation:
    op: eq
    lhs: tokenize(s)[0]
    rhs: Identifier(s)
generators:
  s: { gen: string, regex: "[A-Za-z_$][A-Za-z0-9_$]{0,15}", filter: not_keyword_and_not_synthetic }
evidence: token.rs from_keyword returns None for non-keywords; scan.rs lex_identifier
```

## P2 keyword_table
- Tier: 3
- Rationale: from_keyword + Display must agree with the C11 §6.4.1 keyword list and the documented gnu_extensions flag contract; alias spellings display canonically (token.rs:200-203).
- Doc contract: token.rs:367 "When `gnu_extensions` is false (strict C standard mode, e.g. -std=c99), bare GNU keywords like `typeof` and `asm` are treated as identifiers. The double-underscore forms are always keywords." — asserted fingerprint 5e6f7a8b
- Seed: (none)
- Formal: ∀ (t,kw) ∈ keyword table: tokenize(t,gnu=true)[0]=kw ∧ Display(kw)="'<canonical t>'" ∧ strict mode keeps C keywords, rejects bare GNU words
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::TokenKind::from_keyword
oracle: reference
predicate:
  quantifier: forall
  vars: [t]
  domain: { t: keyword_spellings_table }
  relation:
    op: eq
    lhs: from_keyword(t, true)
    rhs: expected_kind(t)
generators:
  t: { gen: const, sample: keyword_table }
evidence: C11 6.4.1 keyword list; token.rs:367 doc comment on gnu_extensions
```

## P3 int_literal_round_trip
- Tier: 3
- Rationale: integer token round-trip via canonical rendering; expected variant from the documented C11 §6.4.4.1 promotion table (README:218-226) under LP64.
- Doc contract: README:218 "make_int_token implements the C11 §6.4.4.1 integer promotion rules: Decimal literals without suffix: int -> long -> long long..." — asserted fingerprint 9c0d1e2f
- Seed: (none)
- Formal: ∀ v ∈ u64, suf ∈ {none,u,l,ll,ul,ull}, base ∈ {dec,hex,oct,bin}: tokenize(render(v,suf,base))[0] = promote(v,suf,base) ∧ span = (0,len)
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::make_int_token
oracle: algebraic.round_trip
predicate:
  quantifier: forall
  vars: [v, suffix, base]
  domain: { v: u64 (signed render restricted to v <= i64::MAX), suffix: oneof, base: oneof }
  relation:
    op: eq
    lhs: tokenize(render(v, suffix, base))[0].kind
    rhs: c11_promotion(v, suffix, base)
generators:
  v: { gen: int, min: 0, max: 18446744073709551615, type: u64 }
  suffix: { gen: oneof, of: ["", "u", "l", "ll", "ul", "ull"] }
  base: { gen: oneof, of: [dec, hex, oct, bin] }
evidence: README:218-226 promotion rules; C11 6.4.4.1
```

## P4 promotion_boundaries
- Tier: 4
- Rationale: every boundary of the documented promotion table sampled exactly (exhaustive closed matrix: 7 boundaries × 6 suffixes × 4 bases).
- Doc contract: README:222 "Hex/octal literals without suffix: int -> unsigned int -> long -> unsigned long. The unsigned intermediate step is the key difference from decimal." — asserted fingerprint 3a4b5c6d
- Seed: (none)
- Formal: ∀ v ∈ {i32::MAX,+1, u32::MAX,+1, i64::MAX,+1, u64::MAX}, suf, base: variant(tokenize(render(v,suf,base))) = table(v,suf,base)
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::make_int_token
oracle: reference
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: fixed boundary set x suffix x base (exhaustive) }
  relation:
    op: eq
    lhs: variant_of(tokenize(render(v))[0])
    rhs: promotion_table(v)
generators:
  v: { gen: const, sample: [2147483647, 2147483648, 4294967295, 4294967296, 9223372036854775807, 9223372036854775808, 18446744073709551615] }
evidence: README:218-226
```

## P5 hex_float_exact_value
- Tier: 4
- Rationale: README:181 documents value = (int_part + frac_part)·2^exp; in the exact domain (≤8 int + ≤4 frac hex digits, |exp| ≤ 60) both reference and SUT arithmetic are exact → bitwise equality is sound. KAT gate (0x1.8p1=3.0, 0x10p2=64.0, 0x1p-2=0.25) passed before PBT.
- Doc contract: README:181 "Hex floats follow the C99 format 0x<int>.<frac>p<exp> and are converted via: value = (int_part + frac_part) * 2^exp" — asserted fingerprint 7d8e9f0a
- Seed: (none)
- Formal: ∀ lit in exact domain: bits(FloatLiteral(tokenize(lit))) = bits(mant · 2^(exp−4·frac_len)), mant = int‖frac as u64
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::lex_hex_float
oracle: reference
predicate:
  quantifier: forall
  vars: [int_digits, frac_digits, exp]
  domain: { int_digits: hex 1..8, frac_digits: hex 0..4, exp: -60..=60 }
  relation:
    op: eq
    lhs: f64_bits(value_of(tokenize(hex_float_lit(int_digits, frac_digits, exp))[0]))
    rhs: f64_bits(mantissa * pow2(exp - 4*frac_len))
generators:
  int_digits: { gen: string, regex: "[0-9a-fA-F]{1,8}" }
  frac_digits: { gen: string, regex: "[0-9a-fA-F]{0,4}" }
  exp: { gen: int, min: -60, max: 60, type: i32 }
evidence: README:181 formula
```

## P6a hex_float_no_value_collapse
- Tier: 5
- Rationale: hunt for silent value collapse from u64::from_str_radix(...).unwrap_or(0) (scan.rs:242,244). A nonzero hex-float literal with |exp| ≤ 1000 can never evaluate to ±0 (min representable 2^-1074 < 2^-1000). Witness is a valid C99 literal, exactly representable, accepted by GCC without warning.
- Doc contract: README:181 "value = (int_part + frac_part) * 2^exp" — asserted (formula must hold for all accepted literals) fingerprint 7d8e9f0a
- Seed: (none)
- Formal: ∀ lit = 0x<hex 1..20 digits, ≥1 nonzero>p<−1000..1000>: value(lit) ≠ 0.0 ∧ ¬NaN
- Test file: src/frontend/lexer/scan.rs
- Status: failing
- Counterexample: `0x10000000000000000p0` → FloatLiteral(0.0), expected 18446744073709551616.0 (2^64)
- Bug report: bug_reports/lex_hex_float_wide_mantissa_zero.md

```property
function: frontend::lexer::Lexer::lex_hex_float
oracle: reference
predicate:
  quantifier: forall
  vars: [int_digits, exp]
  domain: { int_digits: hex 1..20 digits (>=1 nonzero), exp: -1000..=1000 }
  relation:
    op: holds
    expr: value != 0.0 && !value.is_nan()
generators:
  int_digits: { gen: string, regex: "[0-9a-fA-F]{1,20}", filter: at_least_one_nonzero }
  exp: { gen: int, min: -1000, max: 1000, type: i32 }
evidence: README:181 formula; C99 6.4.4.2 (GCC accepts 0x10000000000000000p0 = 2^64, exactly representable)
```

## P6b hex_float_huge_exponent
- Tier: 5
- Rationale: exponent is parsed as i64 (unwrap_or(0), scan.rs:235) then cast `exp as i32` (scan.rs:249); exponents ≥ 2^31 wrap, >19-digit exponents become 0 — value aliases to 1.0 instead of inf/0.0.
- Doc contract: README:181 "value = (int_part + frac_part) * 2^exp" — asserted fingerprint 7d8e9f0a
- Seed: (none)
- Formal: ∀ e ∈ {2^31, 2^32, 2^64, 10^20, −…}: value("0x1p"+e) = inf for positive e, = 0.0 for negative e
- Test file: src/frontend/lexer/scan.rs
- Status: failing
- Counterexample: `0x1p4294967296` → FloatLiteral(1.0), expected inf; `0x1p-4294967296` → 1.0, expected 0.0
- Bug report: bug_reports/lex_hex_float_exponent_truncation.md

```property
function: frontend::lexer::Lexer::lex_hex_float
oracle: reference
predicate:
  quantifier: forall
  vars: [e]
  domain: { e: "huge exponent spellings: 2147483648 | 4294967296 | 18446744073709551616 | 99999999999999999999 | negations thereof" }
  relation:
    op: holds
    expr: "e > 0 ? value.is_infinite() : value == 0.0"
generators:
  e: { gen: const, sample: [2147483648, 4294967296, 18446744073709551616, 99999999999999999999, -2147483648, -4294967296] }
evidence: IEEE-754 overflow/underflow semantics of README:181 formula; GCC warns and yields inf/0
```

## P7 integer_overflow_wraparound
- Tier: 5
- Rationale: hex/binary/octal literals longer than 64 bits: GCC's accumulate loop wraps mod 2^64 with a diagnostic; ccc's unwrap_or(0) (scan.rs:199,286,314,371) silently yields 0 — value collapse on an accepted literal, wrong under every candidate convention.
- Doc contract: README:191 "Binary Integers ... parses via u64::from_str_radix(s, 2)" — other (implementation note; no overflow behavior documented anywhere) fingerprint 0f1a2b3c
- Seed: (none)
- Formal: ∀ base ∈ {hex,bin,oct}, digits 1..20: u64_payload(tokenize(render)) = u128(digits) mod 2^64; ∀ decimal ≤ u64::MAX: payload = value exactly
- Test file: src/frontend/lexer/scan.rs
- Status: failing
- Counterexample: `0x10000000000000001` → IntLiteral(0), expected IntLiteral(1) (mod 2^64; GCC warns and keeps 1); decimal probes 18446744073709551616→0, 99999999999999999999→0
- Bug report: bug_reports/lex_int_u64_overflow_zero.md

```property
function: frontend::lexer::Lexer::lex_hex_number
oracle: reference
predicate:
  quantifier: forall
  vars: [digits, base]
  domain: { digits: hex 1..20 (also binary/octal renders), base: hex|bin|oct|dec }
  relation:
    op: eq
    lhs: u64_payload(tokenize(render(digits, base))[0])
    rhs: u128_parse(digits) % 2^64
generators:
  digits: { gen: string, regex: "[0-9a-fA-F]{1,20}" }
  base: { gen: oneof, of: [hex, bin, oct, dec] }
evidence: inferred (GCC/Clang wraparound accumulation for out-of-range constants; ccc lexer has no diagnostic channel, so 0 is silent data loss)
```

## P8 escape_sequences_reference
- Tier: 4
- Rationale: escape table documented verbatim (README:275-283, C11 6.4.4.4); octal/hex truncation-to-byte, \u/\U code points, multichar packing with int-typed (sign-extended) semantics (C11 6.4.4.4p10), UTF-8 packing for narrow '\uXXXX' (README:258).
- Doc contract: README:275 "lex_escape_char supports the full set: \n \t \r \\ \' \" \a \b \f \v, \e \E = 0x1B GNU, octal 1-3 digits truncated to byte, \xNN... truncated to byte, \uNNNN \UNNNNNNNN" — asserted fingerprint 4c5d6e7f
- Seed: (none)
- Formal: ∀ documented escape (simple | octal 1..3 digits | hex | \u/\U cp): char/string value = independent table value; multichar 'AB' = 0x4142 (int-typed)
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::lex_escape_char
oracle: reference
predicate:
  quantifier: forall
  vars: [esc]
  domain: { esc: simple | octal(1..3 digits) | hex(1..3 digits) | unicode(cp) }
  relation:
    op: eq
    lhs: char_value_of(tokenize("'<esc>'")[0])
    rhs: c11_escape_value(esc)
generators:
  esc: { gen: oneof, of: [const table, octal_digits, hex_digits, unicode_cp] }
evidence: README:275-283 escape table; C11 6.4.4.4 (+ 6.4.4.4p10 int-typed multichar)
```

## P9 whitespace_invariance (metamorphic)
- Tier: 3
- Rationale: inserting/replacing inter-token separators (spaces/tabs/newlines/comments) must not change the token stream — the metamorphic transform for a lexer; ≥1 separator always present so tokens cannot fuse.
- Doc contract: README:158 "Ignored by the lexer: ASCII whitespace bytes, GCC-style line markers, line comments, block comments" — asserted fingerprint 8a9b0c1d
- Seed: (none)
- Formal: ∀ safe token list T, separator assignments S1,S2 ∈ Sep⁺: kinds(tokenize(render(T,S1))) = kinds(tokenize(render(T,S2))) = T ++ [Eof]
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::skip_whitespace_and_comments
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [tokens, sep1, sep2]
  domain: { tokens: list of safe-render tokens len 1..8, sep1/sep2: per-gap separator choices }
  relation:
    op: eq
    lhs: kinds(tokenize(render(tokens, sep1)))
    rhs: kinds(tokenize(render(tokens, sep2)))
generators:
  tokens: { gen: list, elem: safe_token, minLen: 1, maxLen: 8 }
evidence: README:158 whitespace/comment skipping contract
```

## P10 span_invariants_bounded_input
- Tier: 2
- Rationale: crash-only justified — Lexer has no error channel (tokenize returns Vec<Token>); for bounded arbitrary input the statable contracts are terminal Eof + monotone in-bounds spans + no panic. TERMINATION AT ARBITRARY DEPTH IS A SEPARATE LAW carried by P13, which is FAILING (stack exhaustion at ~4000 unknown chars, bug B4): P10's ≤64-char domain was fixed in the proposed IR before the first run (performance bound, not a post-failure narrowing) and its assertion is byte-identical to the first-run assertion, which passed on the first execution; no clause was narrowed or re-allowed after any failure.
- Doc contract: scan.rs unknown-char branch "Non-ASCII or unknown character: skip any remaining bytes of a multi-byte UTF-8 sequence ... and continue tokenizing." — limitation (recursive per char → termination falsified at depth by P13/B4) fingerprint 2e3f4a5b
- Seed: (none)
- Formal: ∀ s ∈ UTF-8* with |s| ≤ 64: tokenize(s) yields a terminal Eof@span(len,len) and spans that are monotone non-overlapping and within [0,len]
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none — the termination law itself is red in P13 → bug_reports/lex_punctuation_stack_overflow.md)

```property
function: frontend::lexer::Lexer::tokenize
oracle: crash_only
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: arbitrary unicode string len 0..=64 }
  relation:
    op: holds
    expr: spans_monotone_inbounds_and_terminal_eof(tokenize(s), s)
generators:
  s: { gen: string, maxLen: 64 }
evidence: "rejection chain: no reference/algebraic oracle exists for malformed input; API returns Vec<Token> with no error variant — span sanity + terminal Eof is the only statable contract at this bound (unbounded termination is P13, failing)"
```

## P11 imaginary_suffix_combinations (sweep round)
- Tier: 3
- Rationale: coverage-gaps sweep found the documented suffix grammar (README:213-217 integer i/I/j/J; README:245-251 float imaginary before/after type suffix) had no property; closed the gap.
- Doc contract: README:249 "The i/I suffix can appear before or after the type suffix (1.0fi, 1.0if, 1.0Li, 1.0iL). The j/J suffix can appear after the type suffix or standalone." — asserted fingerprint 6f7a8b9c
- Seed: (none)
- Formal: ∀ documented suffix combos: variant + value as documented; ∀ random u^a·l^b(·j) composition: classification per the u/l/ll matrix, trailing j → ImaginaryLiteral
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::parse_int_suffix
oracle: reference
predicate:
  quantifier: forall
  vars: [us, ls, tail]
  domain: { us: 0..=2, ls: 0..=3, tail: none|j }
  relation:
    op: eq
    lhs: variant_of(tokenize(format(v, us*u, ls*l, tail))[0])
    rhs: suffix_matrix(us, ls, tail)
generators:
  us: { gen: int, min: 0, max: 2, type: usize }
  ls: { gen: int, min: 0, max: 3, type: usize }
  tail: { gen: oneof, of: ["", "j"] }
evidence: README:213-217, 245-251 suffix grammar
```

## P12 surrogate_fallback_and_string_families (sweep round)
- Tier: 3
- Rationale: coverage-gaps sweep found README:283 (U+FFFD fallback for invalid code points) and README:248-254 (wide/u8/u16 string content rules) uncovered; closed the gap.
- Doc contract: README:283 "Invalid Unicode code points (e.g., surrogates) fall back to U+FFFD (replacement character)." — asserted fingerprint 1b2c3d4e
- Seed: (none)
- Formal: '\ud800' → U+FFFD (narrow packed 0xEFBFBD, wide 0xFFFD); L"\uXXXX" stores code point directly; u8"..." ≡ narrow "..."; u"..." → Char16StringLiteral; u'x'/u8'x' → IntLiteral(cp)
- Test file: src/frontend/lexer/scan.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: frontend::lexer::Lexer::lex_unicode_escape
oracle: reference
predicate:
  quantifier: forall
  vars: [cp]
  domain: { cp: 0xD800..=0xDFFF (surrogates) }
  relation:
    op: eq
    lhs: value_of(tokenize("'\\ud800'")[0])
    rhs: U+FFFD
generators:
  cp: { gen: const, sample: [0xD800, 0xDFFF] }
evidence: README:283 fallback rule
```

## P13 unknown_char_run_terminates (derived from P10 by depth escalation)
- Tier: 4
- Rationale: P10 bounds input at 64 chars; escalating a single unknown multibyte char to ~4000 repetitions falsifies the termination contract via unbounded recursion in lex_punctuation's unknown-char branch (scan.rs:1173). Test #[ignore]d because it aborts the process (stack overflow cannot be caught); threshold measured empirically (2000 pass / 4000 crash on a 2 MiB thread stack).
- Doc contract: scan.rs:1167 "Non-ASCII or unknown character: skip any remaining bytes of a multi-byte UTF-8 sequence (including PUA-encoded bytes ...) and continue tokenizing." — limitation (recursion instead of continuation) fingerprint 2e3f4a5b
- Seed: (none)
- Formal: ∀ n ≥ 1: tokenize("ÿ"×n) terminates and ends with Eof
- Test file: src/frontend/lexer/scan.rs
- Status: failing
- Counterexample: "ÿ"×200000 (crashes at 4000) → "fatal runtime error: stack overflow, aborting" (SIGABRT)
- Bug report: bug_reports/lex_punctuation_stack_overflow.md

```property
function: frontend::lexer::Lexer::lex_punctuation
oracle: crash_only
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 1..=200000 }
  relation:
    op: holds
    expr: tokenize("\u{00FF}".repeat(n)) terminates_with_eof
generators:
  n: { gen: int, min: 1, max: 200000, type: usize }
evidence: "rejection chain: no value oracle for invalid input; termination is the contract (same as P10); depth escalation falsifies it — measured threshold 4000 crash, 2000 pass"
```

## T-series deterministic documented-behavior tests
- T1 line markers (README:162): Status: passing (t1_line_markers)
- T2 unterminated comment consumed to EOF (README:158-160): Status: failing → B5 (t2_unterminated_comment; witness `int /* gone` → [Int, Identifier("e"), Eof]); Bug report: bug_reports/lex_comment_unterminated_leaks_last_byte.md
- T3 synthetic pragma tokens: Status: passing (t3_pragma_synthetic_tokens)
- T4 octal + ellipsis special case (README:205): Status: passing (t4_octal_ellipsis)

## Test-bug fixes during triage (not SUT bugs)
- p2: Display asserted for alias spellings — Display is canonical-only (token.rs:200-203) → skip aliases. Fixed, passing.
- p8c: reference packed multichar value into i64 without int-typed sign extension (C11 6.4.4.4p10) → fixed, passing.
- p9: expected list missing terminal Eof → fixed, passing.
