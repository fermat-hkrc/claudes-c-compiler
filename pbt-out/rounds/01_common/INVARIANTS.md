# Confirmed Invariants — ccc / src/common (round 01)

Environment quirks:
- Test layout: inline `#[cfg(test)]` modules in source files; modules are `pub(crate)` so `tests/` integration tests cannot reach them.
- proptest 1.11.0 added as [dev-dependencies] by this campaign (`cargo add --dev proptest`). Parameterless fns inside `proptest!{}` break the macro — keep plain `#[test]` fns outside the macro.
- `target_is_32bit()`/`target_long_double_is_f128()` are mutable globals; the 'l' builtins and long-double behavior depend on them — pin or avoid in tests that could run in parallel.
- Run tests from `pbt-out/rounds/01_common/run/` (scratch CWD), serial via `-- --test-threads=1` for triage.

Confirmed invariants (1024-case proptest runs, this campaign):
- bytes_to_string/decode_pua_byte round-trip over ASCII + lone continuation bytes (P1).
- truncate_and_extend_bits == reference mask/sign-extension, idempotent (P2).
- eval_const_binop int path: C99 (a/b)*b + a%b ≡ a (mod 2^w) for all 4 width/signedness combos; Div/Mod by zero → None; comparisons total and consistent (P3/P4/P5).
- eval_const_binop F64 path == native Rust f64 arithmetic bitwise (P6).
- i64 ↔ f128 exact round-trip (P8); f64↔x87 round-trip exact for non-subnormals (P11's passing cases).
- __builtin_clz/ctz/clzll/ctzll/popcount(ll)/parity(ll)/ffs == Rust native bit intrinsics (P12).
- align_up: multiple / ≥ offset / advances < align / idempotent for power-of-two aligns (P13).
- negate/bitnot involutions value-wise incl. MIN values and i8/i16→i32 promotion (P14).
- FxHasher: typed write_uN == write(ne_bytes); 8-aligned splits transparent (P15).
- SymbolTable: innermost binding wins; pop restores outer; global scope survives (P16).

Known-broken invariants (bugs B1–B4 — see REPORT.md; do NOT re-verify as "passing" until fixed):
- f64→f128 lossless for |x| < 1.0 (B1), subnormal f64→x87 (B2), f128→f64 round-to-nearest (B3), bswap32 unsigned representation (B4).

Next-round targets:
- irconst_to_bits float-by-value conversion (Design Caveat in REPORT.md).
- eval_const_binop_i128 widening differential (zero- vs sign-extension per operand signedness).
- is_null_pointer_constant / is_zero_expr AST predicates (parser-bound; construct Expr trees).
- eval_binop_with_types usual-arithmetic-conversion table.
- long_double parse paths (parse_long_double_to_f128_bytes) vs x87 known-value tests.
