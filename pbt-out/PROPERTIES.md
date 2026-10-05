# Properties: encode_neon_mvni

## encode_neon_mvni_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:12 gas-compatible assembler; ARM AdvSIMD modified immediate MVNI). State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MVNI decoder). encode_neon_movi rejected as same-job sibling (MOVI is the inverted-immediate dual with op=0 and extra 8B/16B/2D forms; not interchangeable). Domain is ARM-valid 4H/8H (imm 0-255, no shift), 2S/4S with LSL 0/8/16/24.
- Doc contract: neon.rs:1331 "Encode NEON MVNI Vd.T, #imm (move bitwise NOT immediate to vector)." — asserted fingerprint 9cd14282
- Seed: encode_neon_movi_pbt.rs llvm-mc differential
- Formal: ∀ rd ∈ [0,31], T ∈ {4h,8h,2s,4s}, imm8 ∈ [0,255] (and LSL amount in {0,8,16,24} when T ∈ {2s,4s}). encode_neon_mvni([Vd.T, #imm8 {, lsl #n}]) = llvm-mc("mvni Vd.T, #imm8 {, lsl #n}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mvni
oracle: differential
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: ARM-valid 4h/8h/2s-lsl/4s-lsl mvni }
  relation:
    op: eq
    lhs: encode_neon_mvni(v.ops())
    rhs: llvm_mc(v.asm())
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
evidence: README.md:12 README.md:234 encoder/mod.rs:961 neon.rs:1331
```

## encode_neon_mvni_diff_h_lsl8
- Tier: 2
- Rationale: ARM and llvm-mc/gas accept `mvni Vd.{4h,8h}, #imm8, lsl #8` (cmode=1010). neon.rs:1374 only describes the no-shift encoding; it does not declare LSL #8 invalid. Parser emits Operand::Shift { kind: "lsl", amount: 8 }, so the form is caller-reachable.
- Doc contract: neon.rs:1374 "MVNI 16-bit: cmode=1000, op=1" — other fingerprint ada06ff1
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_diff_h_lsl8
- Formal: ∀ rd ∈ [0,31], T ∈ {4h,8h}, imm8 ∈ [0,255]. encode_neon_mvni([Vd.T, #imm8, lsl #8]) = llvm-mc("mvni Vd.T, #imm8, lsl #8")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: failing
- Counterexample: encode_neon_mvni([v0.4h, #0, lsl #8])
- Bug report: pbt-out/bug_reports/encode_neon_mvni_h_lsl8.md

```property
function: encoder.encode_neon_mvni
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, t, imm8]
  domain: { rd: 0..31, t: {4h,8h}, imm8: 0..255 }
  relation:
    op: eq
    lhs: encode_neon_mvni([Vd.T, #imm8, lsl #8])
    rhs: llvm_mc("mvni Vd.T, #imm8, lsl #8")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
evidence: README.md:12 ARM AdvSIMD MVNI cmode=1010 for H LSL #8 neon.rs:1374
```

## encode_neon_mvni_diff_s_msl
- Tier: 2
- Rationale: ARM and llvm-mc/gas accept `mvni Vd.{2s,4s}, #imm8, msl #{8,16}` (cmode=1100/1101). neon.rs:1358-1360 maps MSL #8 to cmode=1100 and MSL #16 to cmode=1101. Amounts other than {8,16} are a documented domain restriction (neon.rs:1361) and are not in this generator. Differential vs llvm-mc is the same-job reference used for the valid LSL domain.
- Doc contract: neon.rs:1331 "Encode NEON MVNI Vd.T, #imm (move bitwise NOT immediate to vector)." — asserted fingerprint 9cd14282
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_diff_s_msl
- Formal: ∀ rd ∈ [0,31], T ∈ {2s,4s}, imm8 ∈ [0,255], n ∈ {8,16}. encode_neon_mvni([Vd.T, #imm8, msl #n]) = llvm-mc("mvni Vd.T, #imm8, msl #n")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mvni
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, t, imm8, n]
  domain: { rd: 0..31, t: {2s,4s}, imm8: 0..255, n: {8,16} }
  relation:
    op: eq
    lhs: encode_neon_mvni([Vd.T, #imm8, msl #n])
    rhs: llvm_mc("mvni Vd.T, #imm8, msl #n")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
  n: { gen: int, min: 8, max: 16, type: u32 }
evidence: README.md:12 ARM AdvSIMD MVNI MSL neon.rs:1358
```

## encode_neon_mvni_metamorphic_rd
- Tier: 3
- Rationale: ARM encoding places Rd in bits[4:0]; changing only the destination register must differ only in that field. Metamorphic over the valid domain. Stronger differential already covers the same inputs against llvm-mc.
- Doc contract: neon.rs:1367 "MVNI: 0 Q 1 0 1111 00 abc cmode 01 defgh Rd  (op=1)" — asserted fingerprint ae056cdf
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_metamorphic_rd
- Formal: ∀ v valid MVNI, rd2 ∈ [0,31]. (encode_neon_mvni(v with Rd=rd1) ⊕ encode_neon_mvni(v with Rd=rd2)) & ~0x1F = 0 ∧ encode_neon_mvni(v with Rd=rd2) & 0x1F = rd2
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mvni
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [v, rd2]
  domain: { v: ARM-valid mvni, rd2: 0..31 }
  relation:
    op: eq
    lhs: (encode_neon_mvni(v.ops()) ^ encode_neon_mvni(v.with_rd(rd2).ops())) & !0x1F
    rhs: 0
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1367 ARM AdvSIMD Rd field bits[4:0]
```

## encode_neon_mvni_invariant_arm_fields
- Tier: 4
- Rationale: ARM AdvSIMD modified-immediate layout is an exact structural predicate on the success-path word: bit31=0, Q at bit30, op=1 at bit29, bits[28:24]=01111, abc/cmode/o2/bit10/defgh/Rd as specified. Weaker than differential; kept as an independent field-level check.
- Doc contract: neon.rs:1367 "MVNI: 0 Q 1 0 1111 00 abc cmode 01 defgh Rd  (op=1)" — asserted fingerprint ae056cdf
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_invariant_arm_fields
- Formal: ∀ v valid MVNI. let w = encode_neon_mvni(v). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=1 ∧ w[28:24]=01111 ∧ w[23]=0 ∧ w[22:19]=0 ∧ w[18:16]=abc ∧ w[15:12]=cmode(T,shift) ∧ w[11]=0 ∧ w[10]=1 ∧ w[9:5]=defgh ∧ w[4:0]=Rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mvni
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: ARM-valid mvni }
  relation:
    op: holds
    expr: arm_mvni_fields(encode_neon_mvni(v.ops()), v)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm8: { gen: int, min: 0, max: 255, type: u32 }
evidence: neon.rs:1367 ARM AdvSIMD modified immediate MVNI
```

## encode_neon_mvni_neg_arity
- Tier: 4
- Rationale: Documented arity contract neon.rs:1335 "mvni requires 2 operands". llvm-mc also rejects arity 0 and 1. Negative/error contract on the documented minimum.
- Doc contract: neon.rs:1335 "mvni requires 2 operands" — domain-restriction fingerprint 2da05c6c
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_neg_arity
- Formal: ∀ n ∈ {0,1}, ops with |ops|=n. encode_neon_mvni(ops) is Err ∧ llvm-mc rejects the corresponding assembly
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mvni
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t, imm8]
  domain: { n: 0..1, rd: 0..31, t: 4h_8h_2s_4s, imm8: 0..255 }
  relation:
    op: throws
    expr: encode_neon_mvni(ops_prefix(n))
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1335 README.md:12
```

## encode_neon_mvni_neg_extra_and_illegal_shift
- Tier: 4
- Rationale: README.md:12 claims gas-compatible assembly. llvm-mc/gas reject a third non-shift operand, LSL amounts outside the ARM set, LSR, and a trailing extra after a valid shift. The function documents illegal 2s/4s LSL/MSL amounts (neon.rs:1355, 1361) but is silent on extra operands and on 4h/8h shifts other than the no-shift form.
- Doc contract: neon.rs:1331 "Encode NEON MVNI Vd.T, #imm (move bitwise NOT immediate to vector)." — asserted fingerprint 9cd14282
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_neg_extra_and_illegal_shift
- Formal: ∀ rd ∈ [0,31], T ∈ {4h,8h,2s,4s}, extra-or-illegal-shift operand. llvm-mc rejects asm ⇒ encode_neon_mvni(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: failing
- Counterexample: encode_neon_mvni([v0.4h, #0, v0.4h]) is Ok
- Bug report: pbt-out/bug_reports/encode_neon_mvni_extra_operand.md

```property
function: encoder.encode_neon_mvni
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, extra, kind]
  domain: { rd: 0..31, t: 4h_8h_2s_4s, extra: extra_or_illegal_shift }
  relation:
    op: throws
    expr: encode_neon_mvni(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 llvm-mc rejects extra/illegal shift
```

## encode_neon_mvni_neg_imm_oor_invalid_t
- Tier: 4
- Rationale: llvm-mc/gas require imm8 in [0,255] (error: immediate must be an integer in range [0, 255]) and T in {4H,8H,2S,4S}. neon.rs:1379 documents unsupported arrangement as Err. README.md:12 gas-compat is the contract for the immediate range: out-of-range immediates must be rejected, not encoded.
- Doc contract: neon.rs:1379 "mvni: unsupported arrangement" — domain-restriction fingerprint 77064454
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_neg_imm_oor_invalid_t
- Formal: ∀ rd ∈ [0,31]. (imm ∉ [0,255] on a valid T, or T ∉ {4h,8h,2s,4s}). llvm-mc rejects asm ⇒ encode_neon_mvni(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: failing
- Counterexample: encode_neon_mvni([v0.4h, #256]) is Ok
- Bug report: pbt-out/bug_reports/encode_neon_mvni_imm_oor.md

```property
function: encoder.encode_neon_mvni
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, kind]
  domain: { rd: 0..31, kind: imm_oor_or_invalid_t }
  relation:
    op: throws
    expr: encode_neon_mvni(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 neon.rs:1379 llvm-mc immediate range [0,255]
```

## encode_neon_mvni_neg_documented_errors
- Tier: 4
- Rationale: Sweep of documented error contracts the first batch did not keep executing after shrinking: neon.rs:1379 unsupported arrangement, neon.rs:1355 unsupported 2s/4s LSL amount, neon.rs:1361 unsupported MSL amount. These comments declare the inputs invalid (domain restriction). llvm-mc also rejects them.
- Doc contract: neon.rs:1379 "mvni: unsupported arrangement" — domain-restriction fingerprint 77064454
- Seed: encode_neon_movi_pbt.rs encode_neon_movi_neg_documented_errors
- Formal: ∀ rd ∈ [0,31]. (T ∉ {4h,8h,2s,4s}) ∨ (T ∈ {2s,4s} ∧ LSL amount ∉ {0,8,16,24}) ∨ (T ∈ {2s,4s} ∧ MSL amount ∉ {8,16}). encode_neon_mvni(ops) is Err ∧ llvm-mc rejects asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mvni
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, kind]
  domain: { rd: 0..31, kind: invalid_t_or_illegal_s_shift }
  relation:
    op: throws
    expr: encode_neon_mvni(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
evidence: neon.rs:1379 neon.rs:1355 neon.rs:1361
```

## encode_neon_mvni_neg_illegal_h_shift
- Tier: 4
- Rationale: llvm-mc/gas reject LSL amounts other than 0 or 8 on MVNI .4h/.8h. The 4h/8h path never inspects the shift operand. Isolated from the extra-operand shrink of encode_neon_mvni_neg_extra_and_illegal_shift.
- Doc contract: neon.rs:1374 "MVNI 16-bit: cmode=1000, op=1" — other fingerprint ada06ff1
- Seed: encode_neon_mvni_pbt.rs encode_neon_mvni_neg_extra_and_illegal_shift kind=1
- Formal: ∀ rd ∈ [0,31], T ∈ {4h,8h}, imm8 ∈ [0,255], amt ∉ {0,8}. encode_neon_mvni([Vd.T, #imm8, lsl #amt]) is Err ∧ llvm-mc rejects asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: failing
- Counterexample: encode_neon_mvni([v0.4h, #0, lsl #32])
- Bug report: pbt-out/bug_reports/encode_neon_mvni_illegal_h_shift.md

```property
function: encoder.encode_neon_mvni
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, imm8, amt]
  domain: { rd: 0..31, t: 4h_8h, imm8: 0..255, amt: illegal_h_lsl }
  relation:
    op: throws
    expr: encode_neon_mvni(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 llvm-mc rejects illegal H LSL
```

## encode_neon_mvni_neg_lsr
- Tier: 4
- Rationale: ARM MVNI shift kinds are LSL and MSL only. llvm-mc/gas reject LSR. The 2s/4s path maps non-lsl/non-msl to cmode=0000.
- Doc contract: neon.rs:1331 "Encode NEON MVNI Vd.T, #imm (move bitwise NOT immediate to vector)." — asserted fingerprint 9cd14282
- Seed: encode_neon_mvni_pbt.rs encode_neon_mvni_neg_extra_and_illegal_shift kind=2
- Formal: ∀ rd ∈ [0,31], T ∈ {2s,4s}, imm8 ∈ [0,255]. encode_neon_mvni([Vd.T, #imm8, lsr #8]) is Err ∧ llvm-mc rejects asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_mvni_pbt.rs
- Status: failing
- Counterexample: encode_neon_mvni([v0.4s, #0, lsr #8])
- Bug report: pbt-out/bug_reports/encode_neon_mvni_lsr.md

```property
function: encoder.encode_neon_mvni
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, imm8]
  domain: { rd: 0..31, t: 2s_4s, imm8: 0..255 }
  relation:
    op: throws
    expr: encode_neon_mvni(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 llvm-mc rejects LSR on MVNI
```
