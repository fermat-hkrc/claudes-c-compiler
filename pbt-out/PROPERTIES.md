# Properties: encode_neon_pmull

## encode_neon_pmull_diff_llvm_mc_1q
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the documented 64-bit polynomial-multiply-long form. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree PMULL decoder). Sibling encode_neon_three_diff / encode_neon_pmul rejected (same-job gate: integer widening vs 8-bit three-same PMUL, different ARM opcodes).
- Doc contract: neon.rs:1138 "PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)" — asserted fingerprint 14f676c6
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}. let Tb = 1d if ¬is_pmull2 else 2d. encode_neon_pmull([Vd.1q, Vn.Tb, Vm.Tb], is_pmull2) = llvm-mc(pmull{2} Vd.1q, Vn.Tb, Vm.Tb) under -triple=aarch64 -mattr=+aes -show-encoding
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_pmull2: bool }
  relation:
    op: eq
    lhs: encode_neon_pmull(ops_1q(rd, rn, rm, is_pmull2), is_pmull2)
    rhs: llvm_mc(asm_1q(rd, rn, rm, is_pmull2))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
evidence: assembler/README.md:11 gas-compatible textual assembly; neon.rs:1138-1139 1q forms; encoder/mod.rs:779-780 pmull/pmull2 dispatch; ARM three-different PMULL size=11
```

## encode_neon_pmull_diff_llvm_mc_8h
- Tier: 2
- Rationale: ARM / gas / llvm-mc also accept the 8-bit polynomial-long form (Vd.8h, Vn.8b/16b, Vm.8b/16b). README.md:230 lists `pmull` under NEON widen/long without restricting Ta to 1Q. The function comment does not declare 8H invalid. Differential vs llvm-mc is the same independent assembler contract as the 1q property. Stronger oracles rejected as above.
- Doc contract: neon.rs:1127 "Encode NEON PMULL/PMULL2 (polynomial multiply long)" — asserted fingerprint 3c262cb2
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}. let Tb = 8b if ¬is_pmull2 else 16b. encode_neon_pmull([Vd.8h, Vn.Tb, Vm.Tb], is_pmull2) = llvm-mc(pmull{2} Vd.8h, Vn.Tb, Vm.Tb) under -triple=aarch64 -mattr=+aes -show-encoding
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: failing
- Counterexample: encode_neon_pmull([v0.8h, v0.8b, v0.8b], false)
- Bug report: bug_reports/encode_neon_pmull_8h_as_64bit.md

```property
function: encoder.encode_neon_pmull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_pmull2: bool }
  relation:
    op: eq
    lhs: encode_neon_pmull(ops_8h(rd, rn, rm, is_pmull2), is_pmull2)
    rhs: llvm_mc(asm_8h(rd, rn, rm, is_pmull2))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
evidence: assembler/README.md:11 gas-compatible; README.md:230 pmull listed under NEON widen/long; ARM three-different PMULL size=00 for Ta=8H
```

## encode_neon_pmull_metamorphic_rd_rn_rm_q
- Tier: 3
- Rationale: ARM three-different packing isolates Rd[4:0], Rn[9:5], Rm[20:16], Q[30]. Changing one field must not disturb the others. Weaker than differential but independent of llvm-mc availability for the field-isolation claim. Round-trip rejected (no decoder).
- Doc contract: neon.rs:1138 "PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)" — asserted fingerprint 14f676c6
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_metamorphic_rd_rn_rm_rk
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. let w(rd,rn,rm,q) = encode_neon_pmull([Vd.1q,Vn.1d,Vm.1d], q). (w(rd1,rn1,rm1,0) ⊕ w(rd2,rn1,rm1,0)) ∧ ¬0x1F = 0 ∧ w(rd2,…)[4:0]=rd2; (w(rd1,rn1,rm1,0) ⊕ w(rd1,rn2,rm1,0)) ∧ ¬(0x1F≪5) = 0; (w(rd1,rn1,rm1,0) ⊕ w(rd1,rn1,rm2,0)) ∧ ¬(0x1F≪16) = 0; (w(rd1,rn1,rm1,0) ⊕ w(rd1,rn1,rm1,1)) ∧ ¬(1≪30) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmull
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, rm1: u32_0_31, rm2: u32_0_31 }
  body: field isolation of Rd bits[4:0], Rn bits[9:5], Rm bits[20:16], Q bit30
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM Advanced SIMD three-different PMULL; neon.rs:1138-1139 field layout
```

## encode_neon_pmull_invariant_arm_64bit_fields
- Tier: 3
- Rationale: ARM three-different PMULL 64-bit word layout is an exact structural predicate on every success-path encoding of the 1q form. Weaker than differential (does not check agreement with llvm-mc on the variable fields jointly) but pins fixed opcode bits independently.
- Doc contract: neon.rs:1138 "PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)" — asserted fingerprint 14f676c6
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}. let w = encode_neon_pmull([Vd.1q,Vn.Tb,Vm.Tb], is_pmull2). bit31(w)=0 ∧ bit30(w)=is_pmull2 ∧ bits[29:24](w)=001110 ∧ bits[23:22](w)=11 ∧ bit21(w)=1 ∧ bits[20:16](w)=rm ∧ bits[15:11](w)=11100 ∧ bit10(w)=0 ∧ bits[9:5](w)=rn ∧ bits[4:0](w)=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmull
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_pmull2: bool }
  body: ARM three-different PMULL size=11 field layout holds
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
evidence: ARM Advanced SIMD three-different PMULL U=0 opcode=1110 size=11; llvm-mc KAT pmull v0.1q,v1.1d,v2.1d = 0x0ee2e020
```

## encode_neon_pmull_neg_arity
- Tier: 4
- Rationale: Documented arity "pmull requires 3 operands" (neon.rs:1130) plus llvm-mc/gas rejection of 0–2 operands. Negative/error contract is the evidenced failure mode. Stronger oracles do not apply to the invalid-arity domain.
- Doc contract: neon.rs:1130 "pmull requires 3 operands" — domain-restriction fingerprint 77eabc5e
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}. encode_neon_pmull(ops[0..n], is_pmull2) is Err ∧ llvm-mc rejects the corresponding truncated assembly
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, is_pmull2]
  domain: { n: 0..2, rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_pmull2: bool }
  relation:
    op: throws
    expr: encode_neon_pmull(take(ops_1q(rd,rn,rm,is_pmull2), n), is_pmull2)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
expected_error: String
evidence: neon.rs:1130 pmull requires 3 operands; llvm-mc rejects arity 0-2
```

## encode_neon_pmull_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc/gas reject a fourth operand. README.md:11 gas-compatible contract. The comment "pmull requires 3 operands" names the arity; extra is not a documented valid input. Negative/error: SUT must Err.
- Doc contract: neon.rs:1130 "pmull requires 3 operands" — domain-restriction fingerprint 77eabc5e
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, is_pmull2 ∈ {false,true}. llvm-mc rejects pmull{2} Vd.1q, Vn.Tb, Vm.Tb, Vextra.Tb ⇒ encode_neon_pmull([Vd.1q,Vn.Tb,Vm.Tb,Vextra.Tb], is_pmull2) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: failing
- Counterexample: encode_neon_pmull([v0.1q, v0.1d, v0.1d, v0.1d], false)
- Bug report: bug_reports/encode_neon_pmull_extra_operand.md

```property
function: encoder.encode_neon_pmull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, is_pmull2]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, extra: u32_0_31, is_pmull2: bool }
  relation:
    op: throws
    expr: encode_neon_pmull(ops_1q(rd,rn,rm,is_pmull2) ++ [Vextra.Tb], is_pmull2)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
expected_error: String
evidence: assembler/README.md:11 gas-compatible; llvm-mc rejects a fourth operand; neon.rs:1130 requires 3 operands
```

## encode_neon_pmull_neg_invalid_t
- Tier: 4
- Rationale: ARM / llvm-mc reject arrangements outside the Ta/Tb pairs (8H←8B/16B, 1Q←1D/2D) and reject PMULL with PMULL2's Tb (and vice versa). Function discards arrangements so this is the documented-assembler error path. Negative/error contract.
- Doc contract: neon.rs:1138 "PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)" — asserted fingerprint 14f676c6
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}, (Td,Tn,Tm) not a valid PMULL{2} Ta/Tb triple. llvm-mc rejects the assembly ⇒ encode_neon_pmull([Vd.Td,Vn.Tn,Vm.Tm], is_pmull2) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: failing
- Counterexample: encode_neon_pmull([v0.8b, v0.8b, v0.8b], false)
- Bug report: bug_reports/encode_neon_pmull_invalid_t.md

```property
function: encoder.encode_neon_pmull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2, td, tn, tm]
  domain: { rd: u32_0_31, invalid Ta/Tb triples (not the four ARM-legal pairs) }
  relation:
    op: throws
    expr: encode_neon_pmull([Vd.td, Vn.tn, Vm.tm], is_pmull2)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
  td: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tm: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
expected_error: String
evidence: ARM PMULL Ta in {8H,1Q} with matching Tb; llvm-mc rejects other T; assembler/README.md:11
```

## encode_neon_pmull_neg_gpr_or_bare
- Tier: 4
- Rationale: llvm-mc/gas require V registers with arrangement specifiers. Operand::Reg (xN, vN without .T, dN, …) is not a valid PMULL operand. get_neon_reg accepts Operand::Reg, so this is the error-path the public assembler must still reject. Negative/error contract.
- Doc contract: neon.rs:1138 "PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)" — asserted fingerprint 14f676c6
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}, dest ∈ {xN, wN, vN-bare, dN, sN, qN, sp}. llvm-mc rejects the assembly ⇒ encode_neon_pmull([dest, Vn.Tb, Vm.Tb], is_pmull2) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: failing
- Counterexample: encode_neon_pmull([Reg("x0"), v0.1d, v0.1d], false)
- Bug report: bug_reports/encode_neon_pmull_gpr_dest.md

```property
function: encoder.encode_neon_pmull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2, kind]
  domain: { rd: u32_0_31, kind: gpr_or_bare }
  relation:
    op: throws
    expr: encode_neon_pmull([non_v_arr_dest, Vn.Tb, Vm.Tb], is_pmull2)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: assembler/README.md:11 gas-compatible; llvm-mc rejects GPR/bare-V dest; ARM requires Vd.<Ta>
```

## encode_neon_pmull_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — parse_reg_num lowercases V/X prefixes and the assembler accepts the same textual assembly as gas, including uppercase mnemonic and V. Differential vs llvm-mc on the documented 1q domain with uppercase spellings.
- Doc contract: neon.rs:1138 "PMULL  Vd.1q, Vn.1d, Vm.1d: 0 0 00 1110 11 1 Rm 11100 0 Rn Rd  (size=11)" — asserted fingerprint 14f676c6
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}. encode_neon_pmull([Vrd.1q, Vrn.Tb, Vrm.Tb], is_pmull2) = llvm-mc(PMULL{2} Vrd.1Q, Vrn.TbU, Vrm.TbU)
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_pmull2: bool }
  relation:
    op: eq
    lhs: encode_neon_pmull(ops_1q_upper_V(rd, rn, rm, is_pmull2), is_pmull2)
    rhs: llvm_mc(asm_1q_upper(rd, rn, rm, is_pmull2))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
evidence: assembler/README.md:11 gas-compatible; parse_reg_num lowercases prefix (encoder/mod.rs:165)
```

## encode_neon_pmull_neg_nonreg
- Tier: 4
- Rationale: Sweep — get_neon_reg returns Err for Imm/Mem/Label ("expected NEON register"). llvm-mc rejects those at the dest slot. Negative/error contract on a documented helper path the SUT always takes.
- Doc contract: neon.rs:1130 "pmull requires 3 operands" — domain-restriction fingerprint 77eabc5e
- Seed: encode_neon_eor3_pbt.rs:encode_neon_eor3_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_pmull2 ∈ {false,true}, slot ∈ {0,1,2}, kind ∈ {Imm,Mem,Label}. encode_neon_pmull(ops with slot replaced, is_pmull2) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmull_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_pmull2, kind, slot]
  domain: { slot: 0..2, kind: Imm|Mem|Label }
  relation:
    op: throws
    expr: encode_neon_pmull(ops_with_nonreg(slot, kind), is_pmull2)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_pmull2: { gen: bool }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: neon.rs:19 get_neon_reg Err for non-Reg; llvm-mc rejects Imm/Mem/Label dest
```
