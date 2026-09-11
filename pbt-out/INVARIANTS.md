# Confirmed invariants (encode_adc)

- Same-width GPR ADC/ADCS (x0–x30/xzr and w0–w30/wzr, both S values) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_adc(ops, true) XOR encode_adc(ops, false) = 1<<29 (ARM ARM S bit).
- Success-path word: Rd at [4:0], Rn at [9:5], Rm at [20:16], sf at 31, S at 29, op at 30 = 0, bits [28:21] = 0b11010000, bits [15:10] = 0.
- Fewer than 3 operands always Err.
- Non-register (Imm/Mem/Shift/Symbol/Cond) in any of the three slots always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1) always Err.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `adc sp, ...`).
- Known-answer: `adc x0, x1, x2` encodes as 0x9a020020.

## Quirks

- encode_adc does not inspect operands beyond index 2, so a trailing Shift is silently dropped (see bugs).
- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_add_sub)

- Immediate-form ADD/SUB/ADDS/SUBS with a valid imm12 or auto-shift (N<<12, N in 1..=0xFFF), including negative-imm alias, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Shifted-register form (LSL/LSR/ASR, Rd/Rn not SP, amount in range) matches llvm-mc.
- NEON vector ADD/SUB Vd.T, Vn.T, Vm.T for T in {8b,16b,4h,8h,2s,4s,2d} matches llvm-mc.
- Fewer than 3 operands always returns Err containing "requires 3 operands".
- encode_add_sub([Rd,Rn,Imm(-N)], is_sub, s) equals encode_add_sub([Rd,Rn,Imm(N)], !is_sub, s) for valid positive N.
- :lo12: Modifier and ModifierOffset produce WordWithReloc { AddAbsLo12, symbol, addend } with imm12 field 0 and ADD-immediate opcode bits (1000 cases).
- :tprel_lo12_nc: / :tprel_hi12: produce TlsLeAddTprelLo12 / TlsLeAddTprelHi12 with sh bit 0 / 1 (1000 cases).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Immediate form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `add Rd, XZR, #imm`).
- ADDS/SUBS Rd cannot be SP/WSP (llvm-mc rejects `adds sp, ...`).
- Known-answer: `add x0, x1, #42` encodes as 0x9100a820.

## Quirks

- llvm-mc may disassemble `add w0, wsp, #0` as `mov w0, wsp`; the encoding word still matches.
- llvm-mc may rewrite `add x0, x1, #4096, lsl #0` as `add x0, x1, #1, lsl #12`.
- proptest `prop_assert_eq!` format strings cannot use implicit captures (`{asm}`); use `{}` + args.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (FP regs, ADDS Rd=SP, tprel modifiers).
- explicit_shift is true only for lsl#12; other immediate-form shifts are ignored (see bugs).
- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- ADDS/SUBS with Rd=SP encodes register 31 as XZR (see bugs).

---

# Confirmed invariants (IrConst::cast_float_to_target)

- F64 identity: `cast_float_to_target(fv, F64)` is `Some(F64(fv))` with bit-identical payload (NaN payload and signed zero preserved); 1000 random bit patterns.
- Signed in-range truncation toward zero: for I8/I16/I32/I64, the integer payload equals trunc_toward_zero(fv) when that integer is in range (seed: 3.125 → I32(3)).
- IrType::Void always returns None.
- U8 values in 128..=255 are not saturated to i8::MAX (127); the 8-bit pattern equals n as u8. (Storage form is still I8, so to_i64() sign-extends — see bugs.)
- F32 preserves sign of finite-nonzero and infinite inputs; infinities stay infinite.
- Ptr agrees with from_i64(n, Ptr) / ptr_int for in-range exact integers (default LP64 → I64).

## Environment

- Default target_ptr_size is 8 (LP64). IrConst does not implement PartialEq; tests compare via variant match / to_bits().
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit.

## Quirks

- `from_i64` stores U8/U16/U32 as I64; `cast_float_to_target` stores U8 as I8 and U16 as I16 (U32 already I64). `zero()`/`one()` also use I8 for U8.
- F128 arm calls `long_double` → `f64_to_f128_bytes_lossless`, which panics on f64 subnormals (biased_exp=0, mantissa≠0) via `u128` subtraction underflow.

---

# Confirmed invariants (classify_cast_with_f128)

- Identity: classify(ty, ty, native) = Noop for every IrType and both native flags (1000 cases).
- Non-native F128 reduction: classify(from, to, false) = classify(F128↦F64(from), F128↦F64(to), false).
- native flag is a no-op when neither endpoint is F128.
- Native F32/F64 ↔ F128 is FloatToF128 / F128ToFloat with the from_f32 / to_f32 flag.
- Integer-to-integer casts match size/signedness (IntWiden / IntNarrow / SignedToUnsignedSameSize / UnsignedToSignedSameSize / Noop).
- F32→F64 is FloatToFloat { widen: true }; F64→F32 is FloatToFloat { widen: false }.
- f128_is_native=false never returns SignedToF128 / UnsignedToF128 / F128ToSigned / F128ToUnsigned / FloatToF128 / F128ToFloat.
- Ptr ↔ pointer-width integer is Noop (I32/U32 on ILP32, I64/U64 on LP64).

## Environment

- Default target_ptr_size is 8 (LP64). Tests that exercise ILP32 use set_target_ptr_size(4) with a Drop guard so the thread-local is restored.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit.

## Quirks

- Ptr normalization (Ptr ≡ U64/U32) is applied only when neither endpoint is float. Float/F128 ↔ Ptr skips it: Ptr→float is SignedToFloat / SignedToF128, and float→Ptr always sets to_u64=true. See bug_reports/classify_cast_ptr_not_normalized_for_float.md.

---

# Confirmed invariants (encode_adr)

- Immediate-form ADR with Xd (x0–x30/xzr) and imm in [-1048576, 1048575] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases, bounds forced).
- Success-path word: bit 31 (op) = 0, bits [28:24] = 0b10000, Rd at [4:0], SignExtend21(immlo[30:29] | immhi[23:5]<<2) = imm.
- Changing Rd does not change opcode/imm fields; changing imm does not change Rd.
- Symbol / Label / SymbolOffset produce WordWithReloc { AdrPrelLo21, symbol, addend } with word = 0x10000000|rd and imm fields 0 (1000 cases).
- Empty operands, Imm-only, Rd-only, Mem second operand, and invalid name x32 always Err.
- Parser-misclassified Reg/Cond/Barrier names at operand 1 are treated as symbols (get_symbol workaround) and emit AdrPrelLo21.
- Known-answer: `adr x0, #0` encodes as 0x10000000; `adr x0, #1` encodes as 0x30000000.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADR register 31 is XZR, never SP (llvm-mc rejects `adr sp, ...`).
- ADR takes Xd only (llvm-mc rejects `adr w0, ...` and `adr d0, ...`).
- 21-bit signed range: [-1048576, 1048575]; llvm-mc rejects #1048576 and #-1048577.

## Quirks

- encode_adr ignores the is_64 flag from get_reg, so W and FP names encode as Xd with the same register number (see bugs).
- parse_reg_num maps sp to 31, so `adr sp, #imm` encodes as `adr xzr, #imm` (see bugs).
- Out-of-range immediates are truncated to 21 bits via `imm as u32` (see bugs). TODO at load_store.rs:697 notes the missing check.
- get_symbol accepts Modifier / ModifierOffset, so `:lo12:` / `:got:` produce AdrPrelLo21 instead of Err (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier + ModifierOffset).

---

# Confirmed invariants (encode_bic)

- Same-width GPR BIC register form (x0–x30/xzr and w0–w30/wzr, optional lsl/lsr/asr/ror in range) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid BIC-immediate (inverted value is an AArch64 bitmask), including Rd=SP/WSP, matches llvm-mc (1000 cases). Encodes as AND with #~imm.
- NEON BIC Vd.T, Vn.T, Vm.T for T in {8b, 16b} matches llvm-mc (1000 cases).
- encode_bic([Rd, Rn, Imm(imm)]) equals encode_logical([Rd, Rn, Imm(~imm)], opc=00) for valid bitmasks (1000 cases).
- Fewer than 3 operands always Err.
- Operand 2 that is Mem/Symbol/Cond/Label/Barrier always Err.
- Invalid Rm names (x32, w32, empty, foo, r0, x) always Err.
- Immediates llvm-mc rejects as non-bitmasks (#0, all-ones, #5, #9, #0x11) are also rejected by encode_bic.
- Known-answer: `bic x0, x1, x2` encodes as 0x8a220020; `bic x0, x1, #1` as 0x927ff820; `bic v0.16b, v1.16b, v2.16b` as 0x4e621c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Register form: register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `bic sp, ...`).
- Immediate form: Rd of 31 is SP/WSP, not XZR (llvm-mc rejects `bic xzr, x0, #1`; accepts `bic sp, x0, #1`).
- NEON three-same T is 8B or 16B only.
- Shift amount: W-form [0, 31], X-form [0, 63]. Bound+1 (32 / 64) is rejected by llvm-mc.

## Quirks

- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp/wsp to 31, so register-form SP encodes as XZR (see bugs).
- Immediate-form XZR/WZR encodes as SP/WSP (see bugs).
- Shift amount is masked with 0x3F; 32-bit lsl #32 is accepted (see bugs).
- encode_neon_bic sets Q only for 16b; 8h/4s/2d encode as 8b (see bugs).
- Unknown shift kinds fall through to LSL (`_ => 0b00`) (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid bitmask, unsupported third operand, invalid rm, unknown shift kind).

---

# Confirmed invariants (encode_neon_three_diff_narrow)

- Valid ADDHN/RADDHN/SUBHN/RSUBHN (+2) with mandated (Ta,Tb) pairs matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Ta∈{8h,4s,2d}; Tb is 8b/16b, 4h/8h, 2s/4s according to is_high.
- encode(..., is_high=true) XOR encode(..., is_high=false) = 1<<30 (ARM ARM Q bit).
- encode(..., u=1) XOR encode(..., u=0) = 1<<29 (ARM ARM U bit).
- Success-path word: bit 31 = 0, bits [28:24] = 0b01110, bit 21 = 1, bits [11:10] = 00, Rd at [4:0], Rn at [9:5], Rm at [20:16], opcode at [15:12], size at [23:22] from Ta (8h=00, 4s=01, 2d=10).
- Fewer than 3 operands always Err.
- Unsupported source Ta (not 8h/4s/2d) always Err.
- Non-register (Imm/Mem/Symbol/Shift/Cond/Label) in any of the three slots always Err.
- Invalid NEON register names (v32, v99, foo, empty, v, v-1) always Err.
- Known-answer: `addhn v0.8b, v1.8h, v2.8h` encodes as 0x0e224020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADDHN2/RADDHN2/SUBHN2/RSUBHN2 set Q=1 (upper half).
- U=1 is the rounding form (RADDHN/RSUBHN); opcode 0b0100 add-family, 0b0110 sub-family.

## Quirks

- Dest arrangement Tb is ignored (see bugs).
- Rm arrangement is ignored; size comes only from operand 1 (see bugs).
- Extra operands beyond 3 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b, so GPR/FP dest encodes as Vd (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid register names via get_neon_reg).

