# Properties: encode_neon_scalar_three_same

## encode_neon_scalar_three_same_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the documented scalar domain ADD/SUB Dd, Dn, Dm. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree scalar ADD/SUB decoder). Sibling encode_neon_three_same / encode_neon_add_sub rejected (same-job gate: vector Vd.T vs scalar Dd).
- Doc contract: neon.rs:1790 "Encode scalar NEON three-same: 01 U 11110 size 1 Rm opcode 1 Rn Rd" — asserted fingerprint 7e993fde
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}. encode_neon_scalar_three_same([Dd, Dn, Dm], U=is_sub, opcode=10000, size=11) = Word(w) ∧ w = llvm-mc("{add|sub} Dd, Dn, Dm")
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_three_same
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool }
  relation:
    op: eq
    lhs: encode_neon_scalar_three_same([Reg(d rd), Reg(d rn), Reg(d rm)], u=is_sub, opcode=0b10000, size=0b11)
    rhs: llvm_mc("{add|sub} d{rd}, d{rn}, d{rm}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
evidence: src/backend/arm/assembler/encoder/neon.rs:1790
```

## encode_neon_scalar_three_same_meta_rd_rn_rm_u
- Tier: 3
- Rationale: ARM scalar three-same layout isolates Rd[4:0], Rn[9:5], Rm[20:16], U[29]. Changing one of those must differ only in that field. Weaker than differential (already used for value agreement) but independently checks field placement.
- Doc contract: neon.rs:1790 "Encode scalar NEON three-same: 01 U 11110 size 1 Rm opcode 1 Rn Rd" — asserted fingerprint 7e993fde
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_metamorphic_rd_rn_rm_u
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. let w(rd,rn,rm,u)=encode_neon_scalar_three_same([Dd,Dn,Dm], u, 10000, 11). (w(rd1,rn1,rm1,0) ⊕ w(rd2,rn1,rm1,0)) ∧ ¬0x1F = 0 ∧ Rd=rd; (w ⊕ w_rn2) ∧ ¬(0x1F<<5) = 0; (w ⊕ w_rm2) ∧ ¬(0x1F<<16) = 0; (w ⊕ w_u1) ∧ ¬(1<<29) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_three_same
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, rm1: u32_0_31, rm2: u32_0_31 }
  relation:
    op: holds
    expr: field_isolation(Rd[4:0], Rn[9:5], Rm[20:16], U[29])
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1790
```

## encode_neon_scalar_three_same_invariant_arm_fields
- Tier: 4
- Rationale: Documented ARM scalar three-same word layout must hold on the success path, including opcode at [15:11] and size at [23:22] over the ARM field widths. Weaker than differential.
- Doc contract: neon.rs:1790 "Encode scalar NEON three-same: 01 U 11110 size 1 Rm opcode 1 Rn Rd" — asserted fingerprint 7e993fde
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, u ∈ {0,1}, opcode ∈ {0..31}, size ∈ {0..3}. let w=encode_neon_scalar_three_same([Dd,Dn,Dm], u, opcode, size). w[31:30]=01 ∧ w[29]=u ∧ w[28:24]=11110 ∧ w[23:22]=size ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:11]=opcode ∧ w[10]=1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_three_same
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, u, opcode, size]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, u: {0,1}, opcode: u32_0_31, size: u32_0_3 }
  relation:
    op: holds
    expr: arm_scalar_three_same_layout(w, rd, rn, rm, u, opcode, size)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
  size: { gen: int, min: 0, max: 3, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1790
```

## encode_neon_scalar_three_same_neg_arity
- Tier: 5
- Rationale: Documented "scalar three-same requires 3 operands"; llvm-mc rejects arity 0-2 for add/sub Dd. Negative/error contract.
- Doc contract: neon.rs:1792 "scalar three-same requires 3 operands" — domain-restriction fingerprint cff945ed
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}. llvm-mc rejects arity-n add|sub ∧ encode_neon_scalar_three_same(ops[:n], U, 10000, 11) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, is_sub]
  domain: { n: 0..2, rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_three_same(ops[:n], u, 0b10000, 0b11).is_err()
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1792
```

## encode_neon_scalar_three_same_neg_extra_operand
- Tier: 5
- Rationale: gas/llvm-mc reject a fourth operand on add/sub Dd, Dn, Dm. The assembler claims gas-compatible assembly. encode() passes extra operands through when dest is Dd (is_neon_scalar_d_reg_op only checks dest and len>=3). Negative/error: 4 operands must Err. The body checks only len()<3 so extras are ignored — that is the finding, not a reason to weaken the oracle.
- Doc contract: neon.rs:1792 "scalar three-same requires 3 operands" — domain-restriction fingerprint cff945ed
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, is_sub ∈ {false,true}. llvm-mc rejects "{add|sub} Dd, Dn, Dm, Dx" ∧ encode_neon_scalar_three_same([Dd,Dn,Dm,Dx], U, 10000, 11) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, is_sub=false — encode_neon_scalar_three_same([d0, d0, d0, d0], U=0, opcode=10000, size=11) = Word(0x5ee08400) instead of Err
- Bug report: bug_reports/encode_neon_scalar_three_same_extra_operand.md

```property
function: encoder.encode_neon_scalar_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, extra: u32_0_31, is_sub: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_three_same([Dd, Dn, Dm, Dx], u, 0b10000, 0b11).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1792
```

## encode_neon_scalar_three_same_neg_wrong_reg_class
- Tier: 5
- Rationale: ARM/llvm-mc accept only D registers for integer scalar ADD/SUB (size=11). The function comment names ADD/SUB Dd, Dn, Dm. encode() routes here whenever dest is Dd, so a non-D Rn/Rm is caller-reachable. Negative/error: must Err.
- Doc contract: neon.rs:1789 "NEON scalar three-same: ADD/SUB Dd, Dn, Dm" — asserted fingerprint 630276e4
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}, slot ∈ {1,2}, pfx ∈ {s,h,b,x,w,q,v,sp,xzr}. dest is Dd ∧ source slot is non-D ∧ llvm-mc rejects the asm ∧ encode_neon_scalar_three_same(ops, U, 10000, 11) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_sub=false, slot=1, pfx="s" — encode_neon_scalar_three_same([d0, s0, d0], U=0, opcode=10000, size=11) = Word instead of Err
- Bug report: bug_reports/encode_neon_scalar_three_same_wrong_reg_class.md

```property
function: encoder.encode_neon_scalar_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_sub, slot, pfx]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool, slot: {1,2}, pfx: {s,h,b,x,w,q,v,sp,xzr} }
  relation:
    op: holds
    expr: encode_neon_scalar_three_same(ops_with_non_d_source, u, 0b10000, 0b11).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
  slot: { gen: int, min: 1, max: 2, type: usize }
  pfx: { gen: element, of: ["s", "h", "b", "x", "w", "q", "v", "sp", "xzr"] }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1789
```

## encode_neon_scalar_three_same_neg_nonreg
- Tier: 5
- Rationale: Function returns "expected register" for a non-Reg operand. llvm-mc rejects Imm/Mem/Label in any of the three slots. Negative/error contract.
- Doc contract: neon.rs:1793 "expected register" — domain-restriction fingerprint 551b0369
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}, slot ∈ {0,1,2}, kind ∈ {Imm,Mem,Label}. encode_neon_scalar_three_same(ops with non-Reg at slot, U, 10000, 11) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_sub, slot, kind]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool, slot: 0..2, kind: {Imm,Mem,Label} }
  relation:
    op: holds
    expr: encode_neon_scalar_three_same(ops_with_nonreg_at_slot, u, 0b10000, 0b11).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
  slot: { gen: int, min: 0, max: 2, type: usize }
  kind: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1793
```

## encode_neon_scalar_three_same_diff_alt_spellings
- Tier: 2
- Rationale: gas-compatible assembly accepts uppercase D registers and uppercase ADD/SUB mnemonics. parse_reg_num lowercases prefixes. Sweep/strengthening differential vs llvm-mc.
- Doc contract: neon.rs:1790 "Encode scalar NEON three-same: 01 U 11110 size 1 Rm opcode 1 Rn Rd" — asserted fingerprint 7e993fde
- Seed: encode_neon_add_sub_pbt.rs:encode_neon_add_sub_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}. encode_neon_scalar_three_same([D{rd}, D{rn}, D{rm}], U, 10000, 11) = llvm-mc("{ADD|SUB} D{rd}, D{rn}, D{rm}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_three_same
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool }
  relation:
    op: eq
    lhs: encode_neon_scalar_three_same([Reg(D rd), Reg(D rn), Reg(D rm)], u=is_sub, opcode=0b10000, size=0b11)
    rhs: llvm_mc("{ADD|SUB} D{rd}, D{rn}, D{rm}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
evidence: src/backend/arm/assembler/encoder/neon.rs:1790
```
