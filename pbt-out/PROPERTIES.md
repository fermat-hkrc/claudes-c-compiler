# Properties: encode_neon_xtl

## encode_neon_xtl_diff_llvm_mc
- Tier: 2
- Rationale: README.md:12 requires GNU-style gas compatibility; llvm-mc `-triple=aarch64 -show-encoding` is an independent assembler of the same AArch64 UXTL/SXTL encoding. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree UXTL decoder). Differential vs encode_neon_shll rejected as primary (independence gate: same encoding family). encode_neon_shl rejected (same-job: same-width SHL).
- Doc contract: neon.rs:159 "Encode NEON UXTL/SXTL (unsigned/signed extend long)." — asserted fingerprint 070d01c2
- Seed: neon.rs:7655 encode_neon_shll_alias_xtl
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (ta,tb,is_high) ∈ mandated UXTL pairs, ∀ u_bit ∈ {0,1}. encode_neon_xtl([Vd.ta, Vn.tb], u_bit, is_high) = llvm-mc("{uxtl|sxtl}{2?} Vd.ta, Vn.tb")
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_xtl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, u_bit, is_high]
  domain: { rd: "v0..v31", rn: "v0..v31", (ta,tb,is_high): mandated_uxtl_pairs }
  relation:
    op: eq
    lhs: encode_neon_xtl([RegArrangement(v{rd},ta), RegArrangement(v{rn},tb)], u_bit, is_high)
    rhs: llvm_mc("{uxtl|sxtl}{2?} v{rd}.{ta}, v{rn}.{tb}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
  ta: { gen: string }
  tb: { gen: string }
evidence: neon.rs:159
```

## encode_neon_xtl_meta_rd_rn_q_u
- Tier: 3
- Rationale: ARM encoding isolates Rd at bits[4:0], Rn at bits[9:5], Q at bit 30, U at bit 29. Metamorphic field isolation is weaker than llvm-mc differential but independently checks the documented format. Stronger differential is the sibling property.
- Doc contract: neon.rs:162 "Format: 0 Q U 011110 immh immb 10100 1 Rn Rd" — asserted fingerprint 9a879603
- Seed: encode_cnt_pbt.rs Rd/Rn isolation; neon.rs encode_neon_shll_metamorphic_q/u
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (ta,tb), ∀ u_bit,is_high. encode(rd') ⊕ encode(rd) differs only in bits[4:0]; encode(rn') ⊕ encode(rn) differs only in bits[9:5]; flipping u_bit toggles only bit 29; flipping is_high with the matching Tb toggles only bit 30.
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_xtl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, u_bit, is_high]
  domain: { (ta,tb,is_high): mandated_uxtl_pairs }
  body: xor_only_rd_rn_q_u(encode_neon_xtl)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
  ta: { gen: string }
  tb: { gen: string }
evidence: neon.rs:162
```

## encode_neon_xtl_inv_arm_layout
- Tier: 3
- Rationale: ARM Advanced SIMD shift-by-immediate UXTL/SXTL (USHLL/SSHLL #0) fixes bit31=0, bits[28:23]=011110, immh from source esize, immb=000, bits[15:10]=101001. Invariant is weaker than llvm-mc equality but cites the documented format comment.
- Doc contract: neon.rs:162 "Format: 0 Q U 011110 immh immb 10100 1 Rn Rd" — asserted fingerprint 9a879603
- Seed: neon.rs encode_neon_shll_invariant_arm_fields
- Formal: ∀ valid UXTL inputs. word bit31=0 ∧ Q=is_high ∧ U=u_bit ∧ bits[28:23]=011110 ∧ immh=immh(tb) ∧ immb=0 ∧ bits[15:10]=101001 ∧ bits[9:5]=rn ∧ bits[4:0]=rd. immh(8b|16b)=0001, immh(4h|8h)=0010, immh(2s|4s)=0100.
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_xtl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, u_bit, is_high]
  domain: { (ta,tb,is_high): mandated_uxtl_pairs }
  body: arm_uxtl_fields(encode_neon_xtl(...))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
  ta: { gen: string }
  tb: { gen: string }
evidence: neon.rs:162
```

## encode_neon_xtl_neg_arity
- Tier: 4
- Rationale: neon.rs:164-166 requires 2 operands; llvm-mc rejects 0- and 1-operand UXTL. Negative/error contract for documented arity. Stronger oracles do not apply to the empty/short operand vector.
- Doc contract: neon.rs:165 "NEON uxtl/sxtl requires 2 operands" — asserted fingerprint 4250e4ad
- Seed: encode_cnt_pbt.rs arity; encode_neon_shrn_pbt.rs arity
- Formal: ∀ ops with |ops| ∈ {0,1}, ∀ u_bit ∈ {0,1}, ∀ is_high ∈ {false,true}. encode_neon_xtl(ops, u_bit, is_high) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_xtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, u_bit, is_high]
  domain: { ops: "len 0..1" }
  relation:
    op: throws
    expr: encode_neon_xtl(ops, u_bit, is_high)
generators:
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: neon.rs:165
```

## encode_neon_xtl_neg_extra_operand
- Tier: 4
- Rationale: gas/llvm-mc reject a third operand on UXTL/SXTL (two-operand alias of USHLL #0). README.md:12 gas compatibility requires Err. The SUT only checks len < 2, so extra operands are a documented-error path the success differential never reaches.
- Doc contract: neon.rs:159 "Encode NEON UXTL/SXTL (unsigned/signed extend long)." — asserted fingerprint 070d01c2
- Seed: encode_neon_shrn_pbt.rs encode_neon_shrn_neg_extra_operand
- Formal: ∀ valid 2-operand UXTL inputs, ∀ extra. llvm-mc rejects the 3-operand form ⇒ encode_neon_xtl(ops++[extra], u_bit, is_high) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: failing
- Counterexample: encode_neon_xtl([v0.8h, v0.8b, v0.8h], u_bit=0, is_high=false) = Ok(Word) while llvm-mc rejects `sxtl v0.8h, v0.8b, v0.8h`
- Bug report: bug_reports/encode_neon_xtl_extra_operand.md

```property
function: encode_neon_xtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ta, tb, u_bit, is_high]
  domain: { extra: "v0..v31", (ta,tb,is_high): mandated_uxtl_pairs }
  relation:
    op: throws
    expr: encode_neon_xtl([Vd.ta, Vn.tb, extra], u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
  ta: { gen: string }
  tb: { gen: string }
expected_error: String
evidence: neon.rs:164
```

## encode_neon_xtl_neg_mismatched_ta_tb
- Tier: 4
- Rationale: ARM UXTL requires dest Ta in {8H,4S,2D} paired with source Tb 8B/4H/2S (Q=0) or 16B/8H/4S (Q=1). llvm-mc rejects mismatched pairs (wrong dest, wrong Q-for-Tb, reserved arrangements). The SUT discards dest arrangement and maps 8b|16b to the same immh with Q from is_high only, so this error path is otherwise untested.
- Doc contract: neon.rs:160 "These are aliases for USHLL/SSHLL with shift #0." — asserted fingerprint 3917f631
- Seed: encode_neon_shrn_pbt.rs encode_neon_shrn_neg_invalid_arrangement
- Formal: ∀ rd,rn, ∀ (ta,tb,is_high) not in mandated UXTL pairs. llvm-mc rejects "{uxtl|sxtl}{2?} Vd.ta, Vn.tb" ⇒ encode_neon_xtl = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: failing
- Counterexample: encode_neon_xtl([v0.8b, v0.8b], u_bit=0, is_high=false) = Ok(Word) while llvm-mc rejects `sxtl v0.8b, v0.8b`
- Bug report: bug_reports/encode_neon_xtl_mismatched_ta_tb.md

```property
function: encode_neon_xtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, u_bit, is_high]
  domain: { (ta,tb,is_high): not mandated_uxtl_pairs }
  relation:
    op: throws
    expr: encode_neon_xtl([Vd.ta, Vn.tb], u_bit, is_high)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: string }
  tb: { gen: string }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: neon.rs:160
```

## encode_neon_xtl_neg_gpr_or_bare
- Tier: 4
- Rationale: gas/llvm-mc require Vd.Ta / Vn.Tb SIMD arrangements; GPR (x/w), scalar FP (d/s/q/h/b), SP/WSP, and bare V without arrangement are invalid. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w prefixes, so this is a caller-reachable error path.
- Doc contract: neon.rs:159 "Encode NEON UXTL/SXTL (unsigned/signed extend long)." — asserted fingerprint 070d01c2
- Seed: encode_neon_shrn_pbt.rs encode_neon_shrn_neg_gpr_or_bare
- Formal: ∀ kind ∈ {GPR dest, bare-V dest, bare-V src, x-prefix dest arrangement, GPR src}. llvm-mc rejects the asm ⇒ encode_neon_xtl = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: failing
- Counterexample: encode_neon_xtl([Reg(x0), v0.8b], u_bit=1, is_high=false) = Ok(Word) while llvm-mc rejects `uxtl x0, v0.8b`
- Bug report: bug_reports/encode_neon_xtl_gpr_or_bare.md

```property
function: encode_neon_xtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind]
  domain: { kind: gpr_or_bare_kinds }
  relation:
    op: throws
    expr: encode_neon_xtl(ops(kind), 1, false)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:159
```

## encode_neon_xtl_diff_alt_spellings
- Tier: 2
- Rationale: parse_reg_num lowercases prefixes; the assembler accepts GNU-style uppercase mnemonics and V prefixes. Differential vs llvm-mc on uppercase spellings of otherwise valid UXTL. Complements the lowercase valid-domain differential.
- Doc contract: neon.rs:159 "Encode NEON UXTL/SXTL (unsigned/signed extend long)." — asserted fingerprint 070d01c2
- Seed: encode_neon_shrn_pbt.rs encode_neon_shrn_diff_alt_spellings
- Formal: ∀ valid UXTL inputs. encode_neon_xtl([V{rd}.ta, V{rn}.tb], u_bit, is_high) = llvm-mc("{UXTL|SXTL}{2?} V{rd}.{TA}, V{rn}.{TB}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_xtl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, u_bit, is_high]
  domain: { (ta,tb,is_high): mandated_uxtl_pairs }
  relation:
    op: eq
    lhs: encode_neon_xtl([RegArrangement(V{rd},ta), RegArrangement(V{rn},tb)], u_bit, is_high)
    rhs: llvm_mc("{UXTL|SXTL}{2?} V{rd}.{TA}, V{rn}.{TB}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_high: { gen: bool }
  ta: { gen: string }
  tb: { gen: string }
evidence: neon.rs:159
```

## encode_neon_xtl_neg_nonreg
- Tier: 4
- Rationale: Sweep — get_neon_reg other arm (Imm/Mem/Label) is the documented error for a non-register operand. coverage_gaps had no profraw; this generator drives that branch. llvm-mc rejects Imm/Mem/Label in the dest slot.
- Doc contract: neon.rs:19 "expected NEON register at operand {}, got {:?}" — asserted fingerprint b152f086
- Seed: encode_neon_shrn_pbt.rs encode_neon_shrn_neg_nonreg
- Formal: ∀ kind ∈ {Imm, Mem, Label}, ∀ slot ∈ {0,1}. encode_neon_xtl(ops with slot replaced by non-reg, 1, false) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_xtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_xtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { kind: {Imm, Mem, Label}, slot: {0,1} }
  relation:
    op: throws
    expr: encode_neon_xtl(ops_with_nonreg(slot, kind), 1, false)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: neon.rs:19
```
