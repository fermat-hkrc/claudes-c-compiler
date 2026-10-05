# Properties: encode_neon_scalar_qshrn

## encode_neon_scalar_qshrn_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree scalar SQSHRN decoder). Sibling encode_neon_qshrn rejected (same-job gate: vector Vd.Tb vs scalar Bd/Hd/Sd). README.md:12 claims gas-compatible textual assembly; llvm-mc -triple=aarch64 -show-encoding is an independent assembler of that contract.
- Doc contract: neon.rs:1834 "// ── NEON scalar SQSHRN: sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift ────────" — asserted fingerprint 1a596ff2
- Seed: encode_neon_scalar_two_misc_pbt.rs:207 (valid-domain llvm-mc agreement)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (vd,vn,esize) ∈ {(b,h,8),(h,s,16),(s,d,32)}, ∀ shift ∈ {1..esize}, ∀ u ∈ {0,1}, ∀ round ∈ {false,true}. encode_neon_scalar_qshrn([Reg(vd||rd), Reg(vn||rn), Imm(shift)], u, round) = Word(llvm-mc(mnem(u,round) || " " || vd||rd || ", " || vn||rn || ", #" || shift))
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1)], u_bit=0, is_rounding=false) → SUT 0x4f0f9400, llvm-mc 0x5f0f9400 (sqshrn b0, h0, #1)
- Bug report: bug_reports/encode_neon_scalar_qshrn_asisdshf_bit28.md

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, pair, shift, u, round]
  domain: { rd: "0..=31", rn: "0..=31", pair: "{(b,h,8),(h,s,16),(s,d,32)}", shift: "1..=esize", u: "0..=1", round: bool }
  relation:
    op: eq
    lhs: encode_neon_scalar_qshrn([Reg(vd||rd), Reg(vn||rn), Imm(shift)], u, round)
    rhs: llvm_mc_word(mnem(u,round) ++ " " ++ vd||rd ++ ", " ++ vn||rn ++ ", #" ++ shift)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, choices: ["b/h/8", "h/s/16", "s/d/32"] }
  shift: { gen: int, min: 1, max: 32, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  round: { gen: bool }
evidence: neon.rs:1834
```

## encode_neon_scalar_qshrn_meta_rd_rn_u_round_immhb
- Tier: 4
- Rationale: Algebraic metamorphic field isolation. Stronger differential is the primary property; this checks that Rd, Rn, U, rounding opcode, and immh:immb each occupy disjoint ARM fields (independent of llvm-mc). State machine / round-trip rejected as above.
- Doc contract: neon.rs:1848 "// 01 U 11110 immh:immb opcode 1 Rn Rd" — asserted fingerprint 90d13af3
- Seed: encode_neon_scalar_two_misc_pbt.rs:228 (Rd/Rn/U isolation)
- Formal: ∀ valid scalar-qshrn operands. changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only U in bit 29; only is_rounding in bits[15:10]; only (dest,shift) in bits[23:16]
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, u1, u2, round1, round2, pair1, pair2, sh1, sh2]
  domain: { rd1: "0..=31", rd2: "0..=31", rn1: "0..=31", rn2: "0..=31", u1: "0..=1", u2: "0..=1", round1: bool, round2: bool, pair1: pairs, pair2: pairs, sh1: "1..=esize1", sh2: "1..=esize2" }
  body: changing only Rd xor-masks to bits[4:0]; only Rn to bits[9:5]; only U to bit 29; only rounding to bits[15:10]; only dest/shift to bits[23:16]
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  u1: { gen: int, min: 0, max: 1, type: u32 }
  u2: { gen: int, min: 0, max: 1, type: u32 }
  round1: { gen: bool }
  round2: { gen: bool }
evidence: neon.rs:1848
```

## encode_neon_scalar_qshrn_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM asisdshf (scalar shift by immediate), confirmed by llvm-mc encodings (sqshrn h0,s1,#1 = 0x5f1f9420 so bits[31:24]=0x5F = 01 0 11111). Stronger differential is primary. The SUT comment's `11110` is the producing packing (vector asimdshf), not independent evidence — the invariant asserts ARM/llvm-mc bits[28:24]=11111.
- Doc contract: neon.rs:1848 "// 01 U 11110 immh:immb opcode 1 Rn Rd" — asserted fingerprint 90d13af3
- Seed: encode_neon_scalar_two_misc_pbt.rs:279 (ARM field layout)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (vd,vn,esize) ∈ {(b,h,8),(h,s,16),(s,d,32)}, ∀ shift ∈ {1..esize}, ∀ u ∈ {0,1}, ∀ round ∈ {false,true}. let w = encode_neon_scalar_qshrn(...). bits[31:30](w)=01 ∧ bit29(w)=u ∧ bits[28:24](w)=11111 ∧ bits[23:16](w)=(2*esize-shift) ∧ bits[15:10](w)=(round?100111:100101) ∧ bits[9:5](w)=rn ∧ bits[4:0](w)=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1)], 0, false) bits[28:24]=15 (0b01111) expected 31 (0b11111)
- Bug report: bug_reports/encode_neon_scalar_qshrn_asisdshf_fields.md

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, pair, shift, u, round]
  domain: { rd: "0..=31", rn: "0..=31", pair: pairs, shift: "1..=esize", u: "0..=1", round: bool }
  body: word bits match ARM asisdshf 01 U 11111 immh:immb opcode Rn Rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  round: { gen: bool }
evidence: neon.rs:1848
```

## encode_neon_scalar_qshrn_neg_arity
- Tier: 4
- Rationale: Negative/error contract. neon.rs:1836 "scalar qshrn requires 3 operands"; llvm-mc rejects arity 0..=2. Domain restriction on arity < 3.
- Doc contract: neon.rs:1836 "scalar qshrn requires 3 operands" — domain-restriction fingerprint 4963ad30
- Seed: encode_neon_scalar_two_misc_pbt.rs:301
- Formal: ∀ n ∈ {0,1,2}, ∀ valid (vd,vn,rd,rn,shift,u,round). encode_neon_scalar_qshrn(ops.take(n), u, round) is Err ∧ llvm-mc rejects the truncated asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, pair, shift, u, round]
  domain: { n: "0..=2", rd: "0..=31", rn: "0..=31", pair: pairs, shift: "1..=esize", u: "0..=1", round: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_qshrn(ops.take(n), u, round).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
evidence: neon.rs:1836
```

## encode_neon_scalar_qshrn_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc/gas reject a fourth operand; neon.rs:1836 documents a 3-operand instruction. SUT checks only len < 3 (no maximum).
- Doc contract: neon.rs:1836 "scalar qshrn requires 3 operands" — domain-restriction fingerprint 4963ad30
- Seed: encode_neon_scalar_two_misc_pbt.rs:331
- Formal: ∀ valid 3-operand scalar-qshrn ops, ∀ extra Operand::Reg. llvm-mc rejects asm with a fourth operand ⇒ encode_neon_scalar_qshrn(ops||[extra], u, round) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1), Reg("b0")], 0, false) → Ok(Word) while llvm-mc rejects `sqshrn b0, h0, #1, b0`
- Bug report: bug_reports/encode_neon_scalar_qshrn_extra_operand.md

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, pair, shift, u, round]
  domain: { rd: "0..=31", rn: "0..=31", extra: "0..=31", pair: pairs, shift: "1..=esize", u: "0..=1", round: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_qshrn(ops ++ [Reg(vd||extra)], u, round).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1836
```

## encode_neon_scalar_qshrn_neg_wrong_reg_class
- Tier: 4
- Rationale: Negative/error contract. ARM/llvm-mc require dest/src pair B<-H, H<-S, S<-D. SUT does not check source class. llvm-mc rejects mismatched pairs (sqshrn h0, d1, #1; sqshrn b0, s1, #1; sqshrn s0, s1, #1; dest D/X).
- Doc contract: neon.rs:1834 "// ── NEON scalar SQSHRN: sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift ────────" — asserted fingerprint 1a596ff2
- Seed: encode_neon_scalar_two_misc_pbt.rs:358
- Formal: ∀ dest ∈ {b,h,s}, ∀ src_pfx such that (dest,src) is not a mandated pair, ∀ rd,rn,shift,u,round. llvm-mc rejects mnem dest||rd, src||rn, #shift ⇒ encode_neon_scalar_qshrn is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_qshrn([Reg("b0"), Reg("s0"), Imm(8)], 0, false) → Ok(Word) while llvm-mc rejects `sqshrn b0, s0, #8`
- Bug report: bug_reports/encode_neon_scalar_qshrn_wrong_reg_class.md

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest, src, shift, u, round]
  domain: { dest: "{b,h,s}", src: "{b,h,s,d,x,w,q,v} minus mandated", shift: "1..=esize(dest)" }
  relation:
    op: holds
    expr: encode_neon_scalar_qshrn([Reg(dest||rd), Reg(src||rn), Imm(shift)], u, round).is_err()
expected_error: String
generators:
  dest: { gen: oneof, choices: ["b", "h", "s"] }
evidence: neon.rs:1834
```

## encode_neon_scalar_qshrn_neg_shift_oob
- Tier: 4
- Rationale: Negative/error contract. neon.rs:1845 rejects shift == 0 or shift > dest_esize. llvm-mc: "immediate must be an integer in range [1, esize]". Bounds 0, 1, esize, esize+1 are sampled exactly.
- Doc contract: neon.rs:1845 "scalar qshrn: shift {} out of range" — domain-restriction fingerprint b7cc8975
- Seed: neon.rs encode_neon_qshrn_neg_shift_oob
- Formal: ∀ valid dest/src pair, ∀ shift ∈ {0, esize+1, -1, 64, 255}, ∀ rd,rn,u,round. llvm-mc rejects #shift ⇒ encode_neon_scalar_qshrn is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, pair, shift, u, round]
  domain: { shift: "{0, esize+1, -1, 64, 255}" }
  relation:
    op: holds
    expr: encode_neon_scalar_qshrn([Reg(vd||rd), Reg(vn||rn), Imm(shift)], u, round).is_err()
expected_error: String
generators:
  shift: { gen: oneof, choices: [0, "esize+1", -1, 64, 255] }
evidence: neon.rs:1845
```

## encode_neon_scalar_qshrn_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc on uppercase mnemonic and register prefixes (parse_reg_num lowercases). Metamorphic of case; still a differential agreement.
- Doc contract: neon.rs:1834 "// ── NEON scalar SQSHRN: sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift ────────" — asserted fingerprint 1a596ff2
- Seed: encode_neon_scalar_two_misc_pbt.rs:447
- Formal: ∀ valid scalar-qshrn inputs. encode_neon_scalar_qshrn([Reg(upper(vd)||rd), Reg(upper(vn)||rn), Imm(shift)], u, round) = Word(llvm-mc(upper(mnem) || " " || upper(vd)||rd || ", " || upper(vn)||rn || ", #" || shift))
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_qshrn([Reg("B0"), Reg("H0"), Imm(1)], 0, false) → SUT 0x4f0f9400, llvm-mc 0x5f0f9400 (SQSHRN B0, H0, #1)
- Bug report: bug_reports/encode_neon_scalar_qshrn_asisdshf_alt_spellings.md

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, pair, shift, u, round]
  domain: { rd: "0..=31", rn: "0..=31", pair: pairs, shift: "1..=esize", u: "0..=1", round: bool }
  relation:
    op: eq
    lhs: encode_neon_scalar_qshrn(uppercase regs, u, round)
    rhs: llvm_mc_word(uppercase asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1834
```

## encode_neon_scalar_qshrn_neg_unsupported_dest
- Tier: 4
- Rationale: Negative/error contract for the documented dest-prefix rejection (neon.rs:1844). Sweep of the x/w/q/v/d dest error path that the first batch did not dedicate a property to. llvm-mc rejects those dests.
- Doc contract: neon.rs:1844 "scalar qshrn: unsupported dest" — asserted fingerprint f30b2d03
- Seed: encode_neon_scalar_two_misc_pbt.rs:418
- Formal: ∀ dest_pfx ∈ {x,w,q,v,d}, ∀ src_pfx ∈ {h,s,d}, ∀ rd,rn,u,round. llvm-mc rejects mnem dest||rd, src||rn, #1 ⇒ encode_neon_scalar_qshrn is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest_pfx, src_pfx, u, round]
  domain: { dest_pfx: "{x,w,q,v,d}", src_pfx: "{h,s,d}" }
  relation:
    op: holds
    expr: encode_neon_scalar_qshrn([Reg(dest_pfx||rd), Reg(src_pfx||rn), Imm(1)], u, round).is_err()
expected_error: String
generators:
  dest_pfx: { gen: oneof, choices: ["x", "w", "q", "v", "d"] }
evidence: neon.rs:1844
```

## encode_neon_scalar_qshrn_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract. neon.rs:1837 "expected register" when dest or src is not Operand::Reg. Sweep of Imm/Mem/Label in either register slot.
- Doc contract: neon.rs:1837 "expected register" — asserted fingerprint 229eb68d
- Seed: encode_neon_scalar_two_misc_pbt.rs:388
- Formal: ∀ slot ∈ {0,1}, ∀ kind ∈ {Imm, Mem, Label}, ∀ valid remaining operands. llvm-mc rejects the asm ⇒ encode_neon_scalar_qshrn is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_scalar_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, pair, shift, u, round, kind, slot]
  domain: { kind: "{Imm,Mem,Label}", slot: "0..=1" }
  relation:
    op: holds
    expr: encode_neon_scalar_qshrn(ops_with_nonreg_at(slot), u, round).is_err()
expected_error: String
generators:
  slot: { gen: int, min: 0, max: 1, type: usize }
evidence: neon.rs:1837
```
