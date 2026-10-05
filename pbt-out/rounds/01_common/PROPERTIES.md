# Property Ledger — src/common

Tier: standard (≥1000 generator runs per property, set explicitly to 1024).

## P1: encoding_pua_roundtrip (bytes_to_string / decode_pua_byte)
- Tier: 2
- Rationale: module doc (encoding.rs:1-8) states the PUA scheme exists so non-UTF-8 bytes "encode … then decode them back to raw bytes in the lexer" — a documented round-trip. Domain: bytes that never form valid multi-byte UTF-8 (ASCII + lone continuation bytes 0x80..0xBF), so every ≥0x80 byte provably maps to PUA and back. Existing test `test_roundtrip_all_bytes` covers single bytes only.
- Doc contract: src/common/encoding.rs:1 "C source files may contain non-UTF-8 bytes … we encode non-UTF-8 bytes using Unicode Private Use Area (PUA) code points, then decode them back to raw bytes in the lexer" — asserted, fingerprint 3f9a1c02
- Seed: src/common/encoding.rs:169 test_roundtrip_all_bytes
- Formal: ∀ v ∈ (ASCII ∪ 0x80..0xBF)*. walk_decode(bytes_to_string(v)) = v
- Test file: src/common/encoding.rs (common::encoding::pbt_tests::pbt_p1_pua_roundtrip, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P2: truncate_and_extend_bits (const_arith)
- Tier: 2
- Rationale: doc (const_arith.rs:264-268) — truncate to `target_width` bits, sign- or zero-extend back to 64. Reference oracle: plain Rust mask/shift bit operations (independent formulation).
- Doc contract: src/common/const_arith.rs:267 "truncates to `target_width` bits and optionally sign-extends back to 64 bits" — asserted, fingerprint 71bd4e33
- Seed: (none)
- Formal: ∀ x:u64, w∈1..=64. (a) unsigned: truncate(x,w,false).0 = x & ((1<<w)-1); (b) signed: truncate(x,w,true).0 = sext_{64←w}(x & mask_w); (c) idempotent in x for fixed (w,s).
- Test file: src/common/const_arith.rs (pbt_p2_truncate_extend, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P3: eval_const_binop_div_rem_c99 (const_arith, int path)
- Tier: 2
- Rationale: C99 6.5.5p6: (a/b)*b + a%b == a, for every (width, signedness) combination reachable via eval_const_binop. Holds mod 2^N (wrapping) which is the representable contract.
- Doc contract: (none — no doc comment on eval_const_binop_int; C99 clause is the contract) — other, fingerprint 00000000
- Seed: (none)
- Formal: ∀ a,b:i64 (b≠0), w∈{32,64}, s∈{signed,unsigned}. (a/b)*b + a%b ≡ a (mod 2^w) via eval_const_binop on IrConst::I64 operands
- Test file: src/common/const_arith.rs (pbt_p3_div_rem_identity, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P4: eval_const_binop_div_zero (const_arith, int path)
- Tier: 2
- Rationale: doc on eval_const_binop_int: "Returns … None for division by zero" — negative/error contract.
- Doc contract: src/common/const_arith.rs:48 "Returns `Some(IrConst::I64(result))` or `None` for division by zero." — asserted, fingerprint 52ca77be
- Seed: (none)
- Formal: ∀ a:i64, w∈{32,64}, s∈{signed,unsigned}. eval(Div,a,0)=None ∧ eval(Mod,a,0)=None
- Test file: src/common/const_arith.rs (pbt_p4_div_by_zero_none, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P5: eval_const_binop_cmp_totality (const_arith, int path)
- Tier: 3
- Rationale: comparisons must form a total order per width/signedness: exactly one of lt/eq/gt, and le = lt∨eq, ge = gt∨eq, ne = ¬eq. Signedness decides the order, totality does not depend on it.
- Doc contract: (none) — other, fingerprint 00000000
- Seed: (none)
- Formal: ∀ a,b:i64, w, s. lt+eq+gt = 1 ∧ le = lt∨eq ∧ ge = gt∨eq ∧ ne = ¬eq ∧ lt(a,b) = gt(b,a)
- Test file: src/common/const_arith.rs (pbt_p5_cmp_totality, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P6: eval_const_binop_float_differential (const_arith, F64 path)
- Tier: 3
- Rationale: differential vs Rust native IEEE-754 f64 ops (independent reference implementation). Domain: finite non-NaN a,b with finite result — the documented float path ("uses native Rust arithmetic").
- Doc contract: src/common/const_arith.rs:104 "For F32/F64, uses native Rust arithmetic." — asserted, fingerprint 9d27f1aa
- Seed: (none)
- Formal: ∀ finite a,b, op∈{+,-,*,/} with finite a op b. to_f64(eval(op,F64(a),F64(b))) = a op b (bitwise)
- Test file: src/common/const_arith.rs (pbt_p6_f64_differential, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P7: f128_lossless_roundtrip (long_double)
- Tier: 2
- Rationale: f64→f128 is exact (52-bit mantissa into 112-bit); name and doc say "lossless". Round-trip must be bitwise-identical including -0.0.
- Doc contract: src/common/long_double.rs:1028 `pub fn f64_to_f128_bytes_lossless(val: f64) -> [u8; 16]` — asserted (name), fingerprint c4a0e5b1
- Seed: (none)
- Formal: ∀ finite x:f64. f128_bytes_to_f64(f64_to_f128_bytes_lossless(x)).to_bits() = x.to_bits()
- Test file: src/common/long_double.rs (pbt_p7_f128_lossless_roundtrip + pbt_p7_p11_boundary_constants)
- Status: failing
- Counterexample: x = 0.5 (biased_exp = 1022 < 1023): `d.biased_exp as u128 - 1023` underflows → panic in debug / garbage exponent in release. Also x = f64::from_bits(1) (subnormal, biased_exp = 0).
- Bug report: bug_reports/f64_to_f128_bytes_lossless_lt1_underflow.md (B1)

## P8: i64_f128_roundtrip (long_double)
- Tier: 2
- Rationale: doc: "f128 has 112-bit mantissa, so all i64 values are representable exactly" — exact round-trip contract.
- Doc contract: src/common/long_double.rs:941 "Convert a signed i64 to f128 bytes with full precision. f128 has 112-bit mantissa, so all i64 values are representable exactly." — asserted, fingerprint 8b13d0e7
- Seed: (none)
- Formal: ∀ v:i64. f128_bytes_to_i64(i64_to_f128_bytes(v)) = Some(v)
- Test file: src/common/long_double.rs (pbt_p8_i64_f128_roundtrip, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P9: f128_arith_single_rounding (long_double) — METAMORPHIC/DIFFERENTIAL
- Tier: 4
- Rationale: exact sum/diff/product/quotient of two f64s needs ≤106 significant bits < 113, so f128 arithmetic on f64-exact operands is EXACT; narrowing once to f64 must therefore agree bitwise with native f64 arithmetic (IEEE single-rounding). Reference: Rust native f64 ops. Suspect: f128_bytes_to_f64 truncates its mantissa (comment "Take top 52 bits", no rounding) while sibling x87_bytes_to_f64 explicitly implements round-to-nearest — same-job converters disagreeing.
- Doc contract: src/common/long_double.rs:906 `pub fn f128_bytes_to_f64(f128_bytes: &[u8; 16]) -> f64` — other (no rounding-mode claim), fingerprint 17c2fe90
- Seed: (none)
- Formal: ∀ finite a,b:f64 with finite normal a op b (op∈{+,-,*,/}). f128_bytes_to_f64(f128_op(f64_to_f128_bytes_lossless(a), f64_to_f128_bytes_lossless(b))).to_bits() = (a op b).to_bits()
- Test file: src/common/long_double.rs (pbt_p9_f128_single_rounding, 1024 cases)
- Status: failing
- Counterexample: any operand pair with |a| < 1.0 or |b| < 1.0 (e.g. a = 0.5) — panics inside f64_to_f128_bytes_lossless (B1). Once B1 is fixed this property also isolates B3 (truncating narrowing).
- Bug report: bug_reports/f64_to_f128_bytes_lossless_lt1_underflow.md (B1)

## P10: x87_f128_bridge_differential (long_double)
- Tier: 4
- Rationale: x87→f128 is exact (same bias, 63→112 mantissa bits, explicit→implicit bit, no precision lost). Two same-job converters (x87_bytes_to_f64 direct; via exact f128 bridge) must agree. x87_bytes_to_f64 implements documented round-to-nearest; disagreement localizes the fault.
- Doc contract: src/common/long_double.rs:655 "Round to nearest: check bit 10 (the first dropped bit)" (x87_bytes_to_f64) — asserted, fingerprint 26e4d1f8
- Seed: (none)
- Formal: ∀ normal x87 bytes b (integer bit set, exp in x87 normal range). f128_bytes_to_f64(x87_bytes_to_f128_bytes(b)).to_bits() = x87_bytes_to_f64(b).to_bits()
- Test file: src/common/long_double.rs (pbt_p10_x87_f128_bridge, 1024 cases)
- Status: failing
- Counterexample: x87 b = mantissa LE bytes [00,BC,77,FA,EE,DB,...], direct = 3.2257192316651656e-151, via_f128 = 3.225719231665165e-151 (1 ulp low). Deterministic witness: x87 2^0 × 1.111...1 (all-ones 64-bit mantissa) → direct 2.0, via_f128 1.9999999999999998.
- Bug report: bug_reports/f128_bytes_to_f64_truncates.md (B3)

## P11: x87_f64_roundtrip (long_double)
- Tier: 2
- Rationale: f64→x87 widening is exact ("zero-fills the extra mantissa bits"); narrowing back to f64 must reproduce the value bitwise.
- Doc contract: src/common/long_double.rs:1137 "This is a widening conversion that zero-fills the extra mantissa bits." — asserted, fingerprint 0d92a6c4
- Seed: (none)
- Formal: ∀ finite x:f64. x87_bytes_to_f64(f64_to_x87_bytes_simple(x)).to_bits() = x.to_bits()
- Test file: src/common/long_double.rs (pbt_p11_x87_f64_roundtrip + pbt_p7_p11_boundary_constants)
- Status: failing
- Counterexample: x = f64::from_bits(1) = 5e-324 (smallest subnormal): encodes as ~2.22e-308 (implicit-bit assumption applied to a subnormal mantissa), decodes to +0.0.
- Bug report: bug_reports/f64_to_x87_bytes_simple_subnormal.md (B2)

## P12: builtin_bitops_differential (const_eval)
- Tier: 3
- Rationale: __builtin_clz/ctz/popcount/parity/ffs must match GCC semantics == Rust native leading_zeros/trailing_zeros/count_ones (independent reference). bswap* are involutions and must equal Rust swap_bytes.
- Doc contract: src/common/const_eval.rs:56 "- __builtin_bswap{16,32,64} … __builtin_clz{,l,ll}, __builtin_ctz{,l,ll} …" — asserted, fingerprint 4d6f2e81
- Seed: (none)
- Formal: ∀ v:u64. builtin(v) = rust_native(v) for clz/ctz/popcount/parity(l/ll on LP64) ∧ bswapN(bswapN(v)) = v ∧ bswapN(v) = rust swap_bytes (zero-extended for bswap32)
- Test file: src/common/const_eval.rs (pbt_p12_builtin_bitops + pbt_p12_ffs_zero: PASSING, 1024 cases; pbt_p12b_bswap_semantics: FAILING)
- Status: failing
- Counterexample: __builtin_bswap32 with v = -5495501120125551105 (low u32 after bswap = 0xFFBBD010 ≥ 2^31): folded value reads back -4380928 (signed I32) instead of 4290586368 (zero-extended).
- Bug report: bug_reports/bswap32_signed_representation.md (B4)

## P13: align_up_laws (types)
- Tier: 2
- Rationale: align-up contract (used for every struct layout): multiple of align, never below offset, advances by < align, idempotent. Power-of-two aligns (the only aligns C types have).
- Doc contract: src/common/types.rs:1176 "Align `offset` up to the next multiple of `align`." — asserted, fingerprint b1f3aa8e
- Seed: (none)
- Formal: ∀ o:usize, a power-of-two ≥1 with o+a-1 ≤ usize::MAX. align_up(o,a)%a=0 ∧ align_up(o,a)≥o ∧ align_up(o,a)-o<a ∧ align_up(align_up(o,a),a)=align_up(o,a)
- Test file: src/common/types.rs (pbt_p13_align_up_laws, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P14: negate_bitnot_involution (const_arith)
- Tier: 2
- Rationale: doc: wrapping negation "to handle MIN values"; -(-x)=x and ~~x=x must hold value-wise for every int variant (C promotes sub-int to i32 — checked value-wise).
- Doc contract: src/common/const_arith.rs:239 "Uses wrapping negation to handle MIN values (e.g. -(-2^63) wraps to -2^63 in C)." — asserted, fingerprint 3a1b7c55
- Seed: (none)
- Formal: ∀ v. val(negate_const(negate_const(v))) = val(v) ∧ bitnot_const(bitnot_const(v)) = v (same variant for I32/I64/I128)
- Test file: src/common/const_arith.rs (pbt_p14_negate_bitnot_involution, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P15: fxhash_write_consistency (fx_hash — sweep round)
- Tier: 3
- Rationale: Hasher trait contract: writing a value via its typed method must equal writing its native-endian bytes; chunk-aligned splits are transparent because write() processes 8-byte chunks.
- Doc contract: (none — std Hasher contract is the oracle) — other, fingerprint 00000000
- Seed: (none)
- Formal: ∀ v. write_uN(v).finish() = write(&v.to_ne_bytes()).finish() (N=8,16,32,64) ∧ ∀ s1,s2 with len(s1)%8=0. write(s1);write(s2) = write(s1||s2)
- Test file: src/common/fx_hash.rs (pbt_p15_fxhash_write_consistency, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## P16: symbol_table_scope_shadowing (symbol_table — sweep round)
- Tier: 3
- Rationale: Scoped symbol table state machine: innermost declaration wins; pop_scope restores outer visibility; the global scope created by new() always survives pops.
- Doc contract: src/common/symbol_table.rs:35 "Scoped symbol table supporting nested lexical scopes." — asserted, fingerprint 5e2f6a19
- Seed: (none)
- Formal: automaton(states: scope stack; ops: push_scope/declare/lookup/pop_scope; invariant: lookup returns innermost binding; after popping scope S, bindings declared in S are gone and outer bindings reappear; global binding always visible)
- Test file: src/common/symbol_table.rs (pbt_p16_scope_shadowing, 1024 cases)
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## Final tally (serial run, cargo test --lib -- --test-threads=1)
- 12 passing: P1, P2, P3, P4, P5, P6, P8, P12 (clz/ctz/popcount/parity/ffs differential + ffs(0)), P13, P14, P15, P16
- 6 failing test entries → 4 SUT bugs: P7 + boundary → B1; P9 → B1 (panic); P10 → B3; P11 + boundary → B2; P12b → B4
- Pre-existing suite unbroken: 493 baseline → all still pass (506 passed total incl. 13 new passing PBT; only the 6 new witness properties fail).
