# Properties: encode_neon_sri

## encode_neon_sri_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc AArch64 assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SRI decoder). Sibling encode_neon_sli rejected (same-job gate: SLI / left-insert / opcode 010101). encode_neon_ushr rejected (same-job gate: USHR / opcode 000001). encode_neon_sshr rejected (SSHR / U=0). encode_neon_shift_right rejected (independence gate: generic right-shift helper). SUT-boundary: internal-helper of GNU-style assembler; encode_instruction dispatches `"sri"` through to encode_neon_sri with operands unchanged (encoder/mod.rs:706).
- Doc contract: neon.rs:1283 "Encode SRI (Shift Right and Insert) immediate." — asserted fingerprint c010c071
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_diff_llvm_mc (same shift domain [1, esize], SRI opcode 010001)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_sri([Vd.T, Vn.T, #shift]) = llvm-mc("sri Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sri
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..esize(t) }
  relation:
    op: eq
    lhs: encode_neon_sri([RegArrangement(v{rd},t), RegArrangement(v{rn},t), Imm(shift)])
    rhs: llvm_mc("sri v{rd}.{t}, v{rn}.{t}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 1, max: 64, type: i64 }] }
evidence: encoder/mod.rs:706 "sri" => encode_neon_sri(operands); README.md:12 gas-compatible; README.md:228 sri in NEON shifts; ARM SRI T in {8B,16B,4H,8H,2S,4S,2D} shift in [1, esize]
```

## encode_neon_sri_metamorphic_rd_rn_q
- Tier: 4
- Rationale: Weaker than differential. Changing only Rd / only Rn / only Q (8b vs 16b, same esize and shift) must affect only bits[4:0] / bits[9:5] / bit 30. Evidence: neon.rs:1284 layout "0 Q 1 0 11110 immh:immb 010001 Rn Rd".
- Doc contract: neon.rs:1284 "SRI Vd.T, Vn.T, #shift: 0 Q 1 0 11110 immh:immb 010001 Rn Rd  (U=1)" — asserted fingerprint ebb84f2e
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_metamorphic_rd_rn_q
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}. let w11=SRI(rd1,rn1,8b,#1), w21=SRI(rd2,rn1,8b,#1), w12=SRI(rd1,rn2,8b,#1), w16=SRI(rd1,rn1,16b,#1). (w11⊕w21)∧¬0x1F = 0 ∧ (w11⊕w12)∧¬(0x1F≪5) = 0 ∧ (w11⊕w16)∧¬(1≪30) = 0 ∧ Q(w11)=0 ∧ Q(w16)=1
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sri
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: v0..v31, rd2: v0..v31, rn1: v0..v31, rn2: v0..v31 }
  body: ((w11 xor w21) and not 0x1F) == 0 and ((w11 xor w12) and not (0x1F << 5)) == 0 and ((w11 xor w16) and not (1 << 30)) == 0
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1284 0 Q 1 0 11110 immh:immb 010001 Rn Rd
```

## encode_neon_sri_invariant_arm_fields
- Tier: 4
- Rationale: ARM Advanced SIMD shift-right-and-insert field layout. Weaker than differential. U=1, opcode 010001, immh:immb = 2*esize - shift, Q from T. Evidence: neon.rs:1284 and neon.rs:1295.
- Doc contract: neon.rs:1295 "immh:immb = (2*esize - shift) for right shift" — asserted fingerprint 4cbcb73e
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_invariant_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. let w = encode_neon_sri(...). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=1 ∧ w[28:23]=011110 ∧ w[22:16]=2*esize(T)-shift ∧ w[15:10]=010001 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sri
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..esize(t) }
  body: fields(w) match ARM SRI encoding with U=1 opcode=010001 immh:immb=2*esize-shift
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 1, max: 64, type: i64 }] }
evidence: neon.rs:1284 ARM SRI 0 Q 1 0 11110 immh:immb 010001 Rn Rd; neon.rs:1295 immh:immb = (2*esize - shift)
```

## encode_neon_sri_neg_arity
- Tier: 4
- Rationale: Negative/error contract. GNU/llvm-mc reject SRI with fewer than 3 operands. README.md:12 gas-compatible. Expected: Err.
- Doc contract: neon.rs:1284 "SRI Vd.T, Vn.T, #shift: 0 Q 1 0 11110 immh:immb 010001 Rn Rd  (U=1)" — asserted fingerprint ebb84f2e
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn ∈ {0..31}. encode_neon_sri(first n of [Vd.8b, Vn.8b, #1]) = Err ∧ llvm-mc rejects the matching arity-n asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn]
  domain: { n: 0..2, rd: v0..v31, rn: v0..v31 }
  relation:
    op: throws
    expr: encode_neon_sri(ops.take(n))
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1284 SRI Vd.T, Vn.T, #shift; README.md:12 gas-compatible; llvm-mc rejects arity < 3
```

## encode_neon_sri_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. gas/llvm-mc reject a fourth operand. README.md:12. The SUT uses `len() < 3` (extra ignored) — that is the candidate defect, not a domain restriction. neon.rs:1284 names exactly three operands.
- Doc contract: neon.rs:1284 "SRI Vd.T, Vn.T, #shift: 0 Q 1 0 11110 immh:immb 010001 Rn Rd  (U=1)" — asserted fingerprint ebb84f2e
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_sri([Vd.T, Vn.T, #shift, Vextra.T]) = Err ∧ llvm-mc rejects the matching 4-operand asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: failing
- Counterexample: encode_neon_sri([v0.8b, v0.8b, #1, v0.8b]) → Ok(Word) (llvm-mc rejects)
- Bug report: pbt-out/bug_reports/encode_neon_sri_extra_operand.md

```property
function: encoder.neon.encode_neon_sri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, extra: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..esize(t) }
  relation:
    op: throws
    expr: encode_neon_sri([Vd.T, Vn.T, #shift, Vextra.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 1, max: 64, type: i64 }] }
expected_error: String
evidence: neon.rs:1284 SRI Vd.T, Vn.T, #shift (exactly 3 operands); README.md:12 gas-compatible; llvm-mc rejects a fourth operand
```

## encode_neon_sri_neg_invalid_t
- Tier: 4
- Rationale: Negative/error contract. ARM SRI allows only T in {8B,16B,4H,8H,2S,4S,2D} with matching dest/src arrangements; 1D is reserved (immh != 0000 selects the element size, not a 1D form). llvm-mc rejects reserved/mismatched T. Source arrangement is discarded by the SUT (`let (rn, _)`) — that is the candidate defect, not a domain restriction.
- Doc contract: neon.rs:1284 "SRI Vd.T, Vn.T, #shift: 0 Q 1 0 11110 immh:immb 010001 Rn Rd  (U=1)" — asserted fingerprint ebb84f2e
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, Td,Tn arrangements, shift ∈ [1,64]. (Td ≠ Tn ∨ Td ∉ {8b,16b,4h,8h,2s,4s,2d}) ⇒ encode_neon_sri([Vd.Td, Vn.Tn, #shift]) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: failing
- Counterexample: encode_neon_sri([v0.2s, v0.8b, #1]) → Ok(Word) (llvm-mc rejects mismatched T)
- Bug report: pbt-out/bug_reports/encode_neon_sri_mismatched_t.md

```property
function: encoder.neon.encode_neon_sri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, shift]
  domain: { rd: v0..v31, rn: v0..v31, td: arrangements including 1d/1q, tn: same, shift: 1..64 }
  relation:
    op: throws
    expr: encode_neon_sri([Vd.Td, Vn.Tn, #shift])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
expected_error: String
evidence: neon.rs:1284 SRI Vd.T, Vn.T (matching T); ARM SRI T in {8B,16B,4H,8H,2S,4S,2D}; llvm-mc rejects 1d and mismatched T
```

## encode_neon_sri_neg_shift_oob
- Tier: 4
- Rationale: Negative/error contract. ARM/llvm-mc require shift in [1, esize(T)]; llvm-mc: "immediate must be an integer in range [1, 8]" for .8b. The SUT masks `2*esize - shift` with no range check — that is the candidate defect. Documented bound sampled at 0, esize+1, and negatives.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_neg_shift_oob
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∉ [1, esize(T)]. encode_neon_sri([Vd.T, Vn.T, #shift]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: failing
- Counterexample: encode_neon_sri([v0.8b, v0.8b, #0]) → Ok(Word) with reserved immh=0000 (llvm-mc: immediate must be in [1, 8])
- Bug report: pbt-out/bug_reports/encode_neon_sri_shift_oob.md

```property
function: encoder.neon.encode_neon_sri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: {0, esize+1, -1, 2*esize} }
  relation:
    op: throws
    expr: encode_neon_sri([Vd.T, Vn.T, #shift])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: -1, max: 128, type: i64 }] }
expected_error: String
evidence: README.md:12 gas-compatible; ARM SRI shift in [1, esize]; llvm-mc rejects sri v0.8b, v0.8b, #0
```

## encode_neon_sri_neg_gpr_or_bare
- Tier: 4
- Rationale: Negative/error contract. SRI requires NEON Vd.T / Vn.T arrangement operands. llvm-mc rejects GPR dest, bare V without arrangement, and xN.8b. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w prefixes — that is the candidate defect, not a domain restriction.
- Doc contract: neon.rs:1284 "SRI Vd.T, Vn.T, #shift: 0 Q 1 0 11110 immh:immb 010001 Rn Rd  (U=1)" — asserted fingerprint ebb84f2e
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {Reg dest, bare V src, bare V dest, xN.8b dest, xN src}. encode_neon_sri(kind) = Err ∧ llvm-mc rejects the matching asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: failing
- Counterexample: encode_neon_sri([v0.8b, Reg(v0), #1]) → Ok(Word) (llvm-mc rejects sri v0.8b, v0, #1)
- Bug report: pbt-out/bug_reports/encode_neon_sri_gpr_or_bare.md

```property
function: encoder.neon.encode_neon_sri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind]
  domain: { rd: v0..v31, rn: v0..v31, kind: {gpr-dest, bare-v-src, bare-v-dest, x-arr-dest, x-src} }
  relation:
    op: throws
    expr: encode_neon_sri(kind)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:1284 SRI Vd.T, Vn.T; README.md:12 gas-compatible; llvm-mc rejects GPR/bare operands
```

## encode_neon_sri_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — uppercase mnemonic/V prefix with lowercase T must still match llvm-mc. parse_reg_num lowercases V. Documented gas-compatible assembler (README.md:12).
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_sri([V{rd}.T, V{rn}.T, #shift]) = llvm-mc("SRI V{rd}.{T}, V{rn}.{T}, #{shift}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sri
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..esize(t) }
  relation:
    op: eq
    lhs: encode_neon_sri([RegArrangement(V{rd},t), RegArrangement(V{rn},t), Imm(shift)])
    rhs: llvm_mc("SRI V{rd}.{T}, V{rn}.{T}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 1, max: 64, type: i64 }] }
evidence: README.md:12 gas-compatible; parse_reg_num lowercases V prefix; llvm-mc accepts SRI V0.8B
```

## encode_neon_sri_neg_nonreg
- Tier: 4
- Rationale: Sweep — Imm/Mem/Label in dest or src slot must Err. get_neon_reg's `other` arm. README.md:12 gas-compatible.
- Doc contract: neon.rs:7 "Helper to extract register number from a RegArrangement operand" — other fingerprint ab58aab7
- Seed: encode_neon_ushr_pbt.rs encode_neon_ushr_neg_nonreg
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {Imm, Mem, Label}, slot ∈ {0,1}. encode_neon_sri(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: v0..v31, rn: v0..v31, kind: {Imm, Mem, Label}, slot: {0,1} }
  relation:
    op: throws
    expr: encode_neon_sri(ops_with_nonreg)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: neon.rs:19 other => Err expected NEON register; README.md:12 gas-compatible
```
