# Properties: encode_neon_fcvtl

## encode_neon_fcvtl_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FCVTL decoder). Sibling encode_neon_fcvtn rejected (same-job gate: narrowing vs widening). encode_neon_xtl / encode_neon_two_misc rejected (different encoding class). README.md:12 claims gas-compatible textual assembly; encoder/mod.rs:3 claims 32-bit AArch64 words. llvm-mc is the independent reference for that contract.
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (ta,tb,is_high) ∈ {(4s,4h,false),(4s,8h,true),(2d,2s,false),(2d,4s,true)}. encode_neon_fcvtl([Vd.ta, Vn.tb], is_high) = llvm-mc("fcvtl{2} Vd.ta, Vn.tb")
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, is_high]
  domain: { rd: v0..v31, rn: v0..v31, (ta,tb,is_high): ARM FCVTL pairs }
  relation:
    op: eq
    lhs: "encode_neon_fcvtl([arr(rd,ta), arr(rn,tb)], is_high)"
    rhs: "llvm_mc(asm2(rd, rn, ta, tb, is_high))"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtl_meta_rd_rn_q
- Tier: 4
- Rationale: Metamorphic isolation of Rd/Rn/Q. Stronger differential is the primary property; this checks field packing independently of llvm-mc. Changing only Rd (resp. Rn, is_high) must XOR only bits[4:0] (resp. bits[9:5], bit 30).
- Doc contract: neon.rs:1639 "Format: 0 Q 0 01110 0 sz 10000 10111 10 Rn Rd" — asserted fingerprint fe9f2a54
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_meta_rd_rn_q_u
- Formal: ∀ rd,rd2,rn,rn2 ∈ {0..31}, ∀ valid (ta,tb,is_high). let w = encode_neon_fcvtl([Vd.ta,Vn.tb], is_high). w ⊕ encode([Vd2.ta,Vn.tb], is_high) = rd ⊕ rd2 ∧ w ⊕ encode([Vd.ta,Vn2.tb], is_high) = (rn ⊕ rn2)<<5 ∧ w ⊕ encode([Vd.ta,Vn.tb'], !is_high) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rd2, rn, rn2, ta, tb, is_high]
  domain: { rd,rd2,rn,rn2: v0..v31, (ta,tb,is_high): ARM FCVTL pairs }
  relation:
    op: eq
    lhs: "sut_word(&ops2(rd, rn, ta, tb), is_high).unwrap() ^ sut_word(&ops2(rd2, rn, ta, tb), is_high).unwrap()"
    rhs: "rd ^ rd2"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
evidence: neon.rs:1639
```

## encode_neon_fcvtl_inv_arm_layout
- Tier: 4
- Rationale: ARM two-misc layout invariant from neon.rs:1639. Weaker than differential; pins bit fields even if llvm-mc is unavailable.
- Doc contract: neon.rs:1639 "Format: 0 Q 0 01110 0 sz 10000 10111 10 Rn Rd" — asserted fingerprint fe9f2a54
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_inv_arm_layout
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (ta,tb,is_high). let w = encode_neon_fcvtl(...). bit31(w)=0 ∧ Q(w)=is_high ∧ U(w)=0 ∧ bits[28:24]=01110 ∧ bit23=0 ∧ sz(w)=(ta==2d) ∧ bits[21:17]=10000 ∧ bits[16:12]=10111 ∧ bits[11:10]=10 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, is_high]
  domain: { rd,rn: v0..v31, (ta,tb,is_high): ARM FCVTL pairs }
  relation:
    op: holds
    expr: "arm_fcvtl_layout(sut_word(&ops2(rd, rn, ta, tb), is_high).unwrap(), rd, rn, ta, is_high)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
evidence: neon.rs:1639
```

## encode_neon_fcvtl_neg_arity
- Tier: 4
- Rationale: FCVTL is a two-operand instruction (Vd, Vn). llvm-mc and gas reject 0 or 1 operand. get_neon_reg on a missing index returns Err. Negative/error contract from ARM/gas arity.
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_neg_arity
- Formal: ∀ n ∈ {0,1}, ∀ rd ∈ {0..31}, ∀ ta ∈ arrangements, ∀ is_high ∈ {false,true}. encode_neon_fcvtl(ops[0..n], is_high) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, ta, is_high]
  domain: { n: 0..1, rd: v0..v31, ta: arrangements, is_high: bool }
  relation:
    op: throws
    expr: "encode_neon_fcvtl(&ops_of_len(n, rd, ta), is_high)"
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtl_neg_extra_operand
- Tier: 4
- Rationale: gas/llvm-mc reject a third operand on FCVTL. README.md:12 gas-compatibility implies Err, not silent ignore. The function has no maximum-arity check (only get_neon_reg of indices 0 and 1).
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, ∀ valid (ta,tb,is_high). llvm-mc rejects "fcvtl{2} Vd.ta, Vn.tb, Vextra.ta" ⇒ encode_neon_fcvtl([Vd.ta,Vn.tb,Vextra.ta], is_high) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: failing
- Counterexample: encode_neon_fcvtl([v0.4s, v0.4h, v0.4s], is_high=false)
- Bug report: bug_reports/encode_neon_fcvtl_extra_operand.md

```property
function: encoder.encode_neon_fcvtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ta, tb, is_high]
  domain: { rd,rn,extra: v0..v31, (ta,tb,is_high): ARM FCVTL pairs }
  relation:
    op: throws
    expr: "encode_neon_fcvtl(&[arr(rd,ta), arr(rn,tb), arr(extra,ta)], is_high)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtl_neg_mismatched_ta_tb
- Tier: 4
- Rationale: ARM FCVTL dest is {4S,2D} with matching source {4H/8H, 2S/4S}. Dest 2S is not a widening dest (half→single dest is 4S). llvm-mc rejects every pair outside the four ARM combinations. The SUT derives sz from dest only and discards source arrangement, so this property is expected to fail on dest 2s and on source mismatches.
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_neg_mismatched_ta_tb
- Formal: ∀ rd,rn ∈ {0..31}, ∀ ta,tb ∈ arrangements, ∀ is_high ∈ {false,true}. (ta,tb,is_high) ∉ ARM FCVTL pairs ⇒ llvm-mc rejects the asm ∧ encode_neon_fcvtl([Vd.ta,Vn.tb], is_high) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: failing
- Counterexample: encode_neon_fcvtl([v0.2s, v0.8b], is_high=false)
- Bug report: bug_reports/encode_neon_fcvtl_mismatched_ta_tb.md

```property
function: encoder.encode_neon_fcvtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, is_high]
  domain: { rd,rn: v0..v31, ta,tb: arrangements, is_high: bool, filter: not ARM FCVTL pair }
  relation:
    op: throws
    expr: "encode_neon_fcvtl(&ops2(rd, rn, ta, tb), is_high)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
expected_error: String
evidence: neon.rs:1638
```

## encode_neon_fcvtl_neg_gpr_or_bare
- Tier: 4
- Rationale: gas/llvm-mc require Vd.Ta / Vn.Tb. Operand::Reg dest (bare v/x/w/d/s/q), x-prefixed RegArrangement, and bare source must Err. parse_reg_num accepts x/w/d/s/q/h/b prefixes, so x-prefixed arrangement dest may encode as V.
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ {0..31}, ∀ kind ∈ {bare-fp-dest, bare-v-src, bare-v-dest, x-prefixed-dest-arr, x-src}. llvm-mc rejects the corresponding asm ⇒ encode_neon_fcvtl(ops, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: failing
- Counterexample: encode_neon_fcvtl([RegArrangement{reg:"x0", arrangement:"4s"}, v0.4h], is_high=false)
- Bug report: bug_reports/encode_neon_fcvtl_gpr_dest.md

```property
function: encoder.encode_neon_fcvtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind]
  domain: { rd,rn: v0..v31, kind: 0..4 }
  relation:
    op: throws
    expr: "encode_neon_fcvtl(&gpr_or_bare_ops(kind, rd, rn), false)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtl_diff_alt_spellings
- Tier: 5
- Rationale: Differential vs llvm-mc on uppercase mnemonic and V-register spellings (gas is case-insensitive). parse_reg_num lowercases the prefix. Complements the lowercase valid-domain differential.
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, ∀ valid (ta,tb,is_high). encode_neon_fcvtl([V{rd}.ta, V{rn}.tb], is_high) = llvm-mc("FCVTL{2} V{rd}.{TA}, V{rn}.{TB}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, is_high]
  domain: { rd,rn: v0..v31, (ta,tb,is_high): ARM FCVTL pairs }
  relation:
    op: eq
    lhs: "encode_neon_fcvtl([arr_named(format!(\"V{}\", rd), ta), arr_named(format!(\"V{}\", rn), tb)], is_high)"
    rhs: "llvm_mc(&format!(\"FCVTL{} V{}.{TA}, V{}.{TB}\", if is_high {\"2\"} else {\"\"}, rd, ta.to_uppercase(), rn, tb.to_uppercase()))"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_high: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_fcvtl_neg_nonreg
- Tier: 4
- Rationale: Sweep property for the documented get_neon_reg error path (expected NEON register). Imm/Mem/Label in either slot must Err. llvm-mc rejects dest #0 / [xN] / L0.
- Doc contract: neon.rs:1638 "FCVTL: half→single or single→double widening float convert" — asserted fingerprint 876e03f6
- Seed: encode_neon_xtl_pbt.rs:encode_neon_xtl_neg_nonreg
- Formal: ∀ rd,rn ∈ {0..31}, ∀ kind ∈ {Imm,Mem,Label}, ∀ slot ∈ {0,1}. encode_neon_fcvtl(ops with slot replaced by non-register, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_fcvtl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_fcvtl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd,rn: v0..v31, kind: 0..2, slot: 0..1 }
  relation:
    op: throws
    expr: "encode_neon_fcvtl(&ops_with_nonreg(kind, slot, rd, rn), false)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```
