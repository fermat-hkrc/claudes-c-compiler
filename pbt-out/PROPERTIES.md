# Properties: encode_neon_movi

## encode_neon_movi_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:12 gas-compatible assembler; ARM AdvSIMD modified immediate). State machine rejected (pure function). Round-trip rejected (no in-tree MOVI decoder). encode_neon_mvni rejected (same-job gate: inverted immediate, different op bit, no 8B/16B/2D). Domain is ARM-valid 8B/16B (imm 0-255), 4H/8H no-shift, 2S/4S with LSL 0/8/16/24, 2D byte-mask immediates.
- Doc contract: neon.rs:623 "Encode NEON MOVI (move immediate to vector)" — asserted fingerprint f09ebe41
- Seed: encode_neon_ext_pbt.rs:193 llvm-mc differential
- Formal: ∀ rd ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s,2d}, imm in the ARM-valid set for T (and LSL amount in {0,8,16,24} when T ∈ {2s,4s}). encode_neon_movi([Vd.T, #imm {, lsl #n}]) = llvm-mc("movi Vd.T, #imm {, lsl #n}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_movi
oracle: differential
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: ARM-valid 8b/16b/4h/8h/2s-lsl/2d movi }
  relation:
    op: eq
    lhs: encode_neon_movi(v.ops())
    rhs: llvm_mc(v.asm())
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
evidence: README.md:12 README.md:234 encoder/mod.rs:687 neon.rs:623
```

## encode_neon_movi_diff_h_lsl8
- Tier: 2
- Rationale: ARM and llvm-mc/gas accept `movi Vd.{4h,8h}, #imm8, lsl #8` (cmode=1010). neon.rs:708 only describes the no-shift encoding; it does not declare LSL #8 invalid. Parser emits Operand::Shift { kind: "lsl", amount: 8 }, so the form is caller-reachable.
- Doc contract: neon.rs:708 "cmode=1000 for .4h/.8h with no shift" — other fingerprint 928a5c52
- Seed: (none)
- Formal: ∀ rd ∈ [0,31], T ∈ {4h,8h}, imm8 ∈ [0,255]. encode_neon_movi([Vd.T, #imm8, lsl #8]) = llvm-mc("movi Vd.T, #imm8, lsl #8")
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: failing
- Counterexample: rd=0, t="4h", imm8=0 → SUT 0x0f008400 vs llvm-mc 0x0f00a400
- Bug report: pbt-out/bug_reports/encode_neon_movi_h_lsl8.md

```property
function: encoder.encode_neon_movi
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, t, imm8]
  domain: { rd: v0-v31, t: {4h,8h}, imm8: 0..=255 }
  relation:
    op: eq
    lhs: encode_neon_movi([Vd.T, Imm(imm8), Shift(lsl,8)])
    rhs: llvm_mc("movi v{rd}.{t}, #{imm8}, lsl #8")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
evidence: README.md:12 neon.rs:623 ARM AdvSIMD modified immediate cmode=1010
```

## encode_neon_movi_diff_s_msl
- Tier: 2
- Rationale: ARM and llvm-mc/gas accept `movi Vd.{2s,4s}, #imm8, msl #8|#16` (cmode=1100/1101). Sibling encode_neon_mvni implements MSL. encode_neon_movi only special-cases kind=="lsl".
- Doc contract: neon.rs:675 "Check for optional LSL shift operand" — asserted fingerprint edfc5164
- Seed: encode_neon_mvni neon.rs:1358 msl handling
- Formal: ∀ rd ∈ [0,31], T ∈ {2s,4s}, imm8 ∈ [0,255], n ∈ {8,16}. encode_neon_movi([Vd.T, #imm8, msl #n]) = llvm-mc("movi Vd.T, #imm8, msl #n")
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: failing
- Counterexample: rd=0, t="2s", imm8=0, n=8 → SUT 0x0f000400 vs llvm-mc 0x0f00c400
- Bug report: pbt-out/bug_reports/encode_neon_movi_s_msl.md

```property
function: encoder.encode_neon_movi
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, t, imm8, n]
  domain: { rd: v0-v31, t: {2s,4s}, imm8: 0..=255, n: {8,16} }
  relation:
    op: eq
    lhs: encode_neon_movi([Vd.T, Imm(imm8), Shift(msl,n)])
    rhs: llvm_mc("movi v{rd}.{t}, #{imm8}, msl #{n}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
  n: { gen: int, min: 8, max: 16, type: u32 }
evidence: README.md:12 neon.rs:623 ARM AdvSIMD modified immediate cmode=110x
```

## encode_neon_movi_metamorphic_rd
- Tier: 3
- Rationale: ARM encoding places Rd in bits[4:0]; changing only the destination register must differ only in that field.
- Doc contract: neon.rs:634 "Encoding: 0 Q 00 1111 00000 abc 1110 01 defgh Rd" — asserted fingerprint a29a83e7
- Seed: encode_neon_ext_pbt.rs:209 metamorphic Rd
- Formal: ∀ rd1, rd2 ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s,2d}, imm ARM-valid-for-T. (encode(rd1,T,imm) ⊕ encode(rd2,T,imm)) & ~0x1F = 0 ∧ encode(rd2,T,imm) & 0x1F = rd2
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_movi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [v, rd2]
  domain: { v: ARM-valid movi, rd2: v0-v31 }
  relation:
    op: eq
    lhs: (encode(v) xor encode(v.with_rd(rd2))) and (not 0x1F)
    rhs: 0
generators:
  rd2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:634 ARM AdvSIMD modified immediate Rd at bits[4:0]
```

## encode_neon_movi_invariant_arm_fields
- Tier: 3
- Rationale: ARM AdvSIMD modified immediate layout: bit31=0, Q at 30, op at 29 (1 iff 2d), bits[28:24]=01111, bit23=0, bits[22:19]=0000, abc at [18:16], cmode at [15:12], o2=0 at 11, bit10=1, defgh at [9:5], Rd at [4:0].
- Doc contract: neon.rs:634 "Encoding: 0 Q 00 1111 00000 abc 1110 01 defgh Rd" — asserted fingerprint a29a83e7
- Seed: encode_neon_ext_pbt.rs:247 invariant ARM fields
- Formal: ∀ valid (rd,T,imm,shift). let w = encode_neon_movi(...). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=op(T) ∧ w[28:24]=01111 ∧ w[23]=0 ∧ w[22:19]=0 ∧ w[18:16]=imm8[7:5] ∧ w[15:12]=cmode(T,shift) ∧ w[11]=0 ∧ w[10]=1 ∧ w[9:5]=imm8[4:0] ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_movi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: ARM-valid movi }
  relation:
    op: holds
    expr: arm_modified_imm_fields_hold(encode_neon_movi(v.ops()), v)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:634 neon.rs:662 ARM AdvSIMD modified immediate
```

## encode_neon_movi_neg_arity
- Tier: 4
- Rationale: neon.rs:626 documents "movi requires 2 operands"; llvm-mc/gas reject 0- and 1-operand movi.
- Doc contract: neon.rs:626 "movi requires 2 operands" — domain-restriction fingerprint 859ac28f
- Seed: encode_neon_ext_pbt.rs:303 neg arity
- Formal: ∀ n ∈ {0,1}, ops with n operands. llvm-mc rejects arity-n movi ∧ encode_neon_movi(ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_movi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd]
  domain: { n: {0,1}, rd: v0-v31 }
  relation:
    op: throws
    expr: encode_neon_movi(take(ops, n))
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:626 README.md:12
```

## encode_neon_movi_neg_extra_and_illegal_shift
- Tier: 4
- Rationale: llvm-mc/gas reject a trailing extra register, a shift on 8B/16B/2D, and LSL amounts outside the ARM set. README.md:12 gas-compatible assembler.
- Doc contract: neon.rs:623 "Encode NEON MOVI (move immediate to vector)" — asserted fingerprint f09ebe41
- Seed: encode_neon_ext_pbt.rs:333 neg extra operand
- Formal: ∀ valid 2-operand movi plus (extra RegArrangement OR illegal shift for T). llvm-mc rejects the asm ∧ encode_neon_movi(ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: failing
- Counterexample: rd=0, extra=0, t="8b", imm8=0, kind=0 → SUT Ok, llvm-mc Err for `movi v0.8b, #0, v0.8b`
- Bug report: pbt-out/bug_reports/encode_neon_movi_extra_operand.md

```property
function: encoder.encode_neon_movi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, extra, t, imm8, kind]
  domain: { rd: v0-v31, extra: v0-v31, t: legal-T, imm8: 0..=255, kind: extra-or-illegal-shift }
  relation:
    op: throws
    expr: encode_neon_movi(ops ++ extra)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u8 }
evidence: README.md:12 neon.rs:623
```

## encode_neon_movi_neg_imm_oor_invalid_t
- Tier: 4
- Rationale: llvm-mc rejects imm outside [0,255] for 8B/16B/4H/8H/2S/4S; rejects 2D immediates whose bytes are not 0x00/0xFF; rejects illegal T. SUT masks imm8 with & 0xFF.
- Doc contract: neon.rs:712 "movi: unsupported arrangement" — domain-restriction fingerprint a8ce42b7
- Seed: encode_neon_ext_pbt.rs:356 neg invalid T
- Formal: ∀ rd ∈ [0,31]. (T illegal ∨ imm8 ∉ [0,255] for non-2d ∨ 2d byte not in {0x00,0xFF}). llvm-mc rejects ∧ encode_neon_movi = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: failing
- Counterexample: rd=0, kind=0, t_ok="8b", over=1 → `movi v0.8b, #256` SUT Ok (truncated to 0), llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_neon_movi_imm_oor.md

```property
function: encoder.encode_neon_movi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, kind, t_ok, over]
  domain: { rd: v0-v31, kind: {OOR,neg,bad-T,bad-2d}, t_ok: 8-bit-T, over: 1..=256 }
  relation:
    op: throws
    expr: encode_neon_movi(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u8 }
  over: { gen: int, min: 1, max: 256, type: i64 }
evidence: README.md:12 neon.rs:712
```

## encode_neon_movi_neg_documented_errors
- Tier: 4
- Rationale: Sweep of documented invalid-domain rejections. neon.rs:712 declares illegal T invalid ("movi: unsupported arrangement"); neon.rs:684 declares LSL amounts other than 0/8/16/24 invalid ("movi: unsupported shift amount"); neon.rs:657 declares 2d bytes other than 0x00/0xFF invalid. Negative/error: those inputs must Err, matching llvm-mc. Not a generator exclusion of a mishandled valid input.
- Doc contract: neon.rs:712 "movi: unsupported arrangement" — domain-restriction fingerprint a8ce42b7
- Seed: (none) — coverage_gaps sweep round 1/1
- Formal: ∀ rd ∈ [0,31]. (illegal T ∨ (T∈{2s,4s} ∧ LSL amount ∉ {0,8,16,24}) ∨ 2d byte ∉ {0x00,0xFF}). llvm-mc rejects ∧ encode_neon_movi = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_movi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, kind]
  domain: { rd: v0-v31, kind: {bad-T, bad-lsl-amt, bad-2d-byte} }
  relation:
    op: throws
    expr: encode_neon_movi(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
evidence: neon.rs:712 neon.rs:684 neon.rs:657
```
