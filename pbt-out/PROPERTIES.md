# Properties: encode_neon_scalar_two_misc

## encode_neon_scalar_two_misc_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the GNU-style assembler contract (README.md:12). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree scalar SQABS/SQNEG decoder). Sibling encode_neon_two_misc rejected (same-job gate: vector Vd.T vs scalar Bd/Hd/Sd/Dd).
- Doc contract: neon.rs:1818 "NEON scalar two-reg misc: SQABS/SQNEG Hd,Hn / Sd,Sn / Dd,Dn" — asserted fingerprint 83faae33
- Seed: encode_neon_scalar_three_same_pbt.rs:188 (diff_llvm_mc)
- Formal: ∀ rd, rn ∈ {0..31}, pfx ∈ {b,h,s,d}, is_neg ∈ {false,true}. encode_neon_scalar_two_misc([Reg(pfx∥rd), Reg(pfx∥rn)], U=is_neg, opcode=00111) = llvm-mc("sqabs|sqneg pfx rd, pfx rn")
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, pfx, is_neg]
  domain: { rd: 0..31, rn: 0..31, pfx: {b,h,s,d}, is_neg: bool }
  relation:
    op: eq
    lhs: encode_neon_scalar_two_misc([Reg(pfx+rd), Reg(pfx+rn)], u=is_neg, opcode=0b00111)
    rhs: llvm_mc(sqabs_or_sqneg(is_neg) + " " + pfx+rd + ", " + pfx+rn)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
  is_neg: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_scalar_two_misc_meta_rd_rn_u_opcode_size
- Tier: 3
- Rationale: Algebraic metamorphic field isolation from the ARM encoding comment. Stronger differential already used as primary. Round-trip rejected (no decoder).
- Doc contract: neon.rs:1828 "01 U 11110 size 10000 opcode 10 Rn Rd" — asserted fingerprint 8c4b5163
- Seed: encode_neon_scalar_three_same_pbt.rs:203 (meta_rd_rn_rm_u)
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, u1,u2 ∈ {0,1}, opc1,opc2 ∈ {0..31}, pfx1,pfx2 ∈ {b,h,s,d}. Changing only Rd (resp. Rn, U, opcode, dest prefix/size) differs only in bits[4:0] (resp. [9:5], bit29, [16:12], [23:22]).
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, u1, u2, opc1, opc2, pfx1, pfx2]
  domain: { rd1: 0..31, rd2: 0..31, rn1: 0..31, rn2: 0..31, u1: 0..1, u2: 0..1, opc1: 0..31, opc2: 0..31, pfx1: {b,h,s,d}, pfx2: {b,h,s,d} }
  relation:
    op: holds
    expr: field_isolation(Rd,Rn,U,opcode,size)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  u1: { gen: int, min: 0, max: 1, type: u32 }
  u2: { gen: int, min: 0, max: 1, type: u32 }
  opc1: { gen: int, min: 0, max: 31, type: u32 }
  opc2: { gen: int, min: 0, max: 31, type: u32 }
  pfx1: { gen: oneof, args: ["b", "h", "s", "d"] }
  pfx2: { gen: oneof, args: ["b", "h", "s", "d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:1828
```

## encode_neon_scalar_two_misc_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the documented SISD two-misc bit template. Stronger differential and metamorphic already present.
- Doc contract: neon.rs:1828 "01 U 11110 size 10000 opcode 10 Rn Rd" — asserted fingerprint 8c4b5163
- Seed: encode_neon_scalar_three_same_pbt.rs:248 (invariant_arm_fields)
- Formal: ∀ rd, rn ∈ {0..31}, u ∈ {0,1}, opcode ∈ {0..31}, pfx ∈ {b,h,s,d}. word bits[31:30]=01 ∧ U=u ∧ bits[28:24]=11110 ∧ size=size(pfx) ∧ bits[21:17]=10000 ∧ opcode at [16:12] ∧ bits[11:10]=10 ∧ Rn at [9:5] ∧ Rd at [4:0]
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, u, opcode, pfx]
  domain: { rd: 0..31, rn: 0..31, u: 0..1, opcode: 0..31, pfx: {b,h,s,d} }
  relation:
    op: holds
    expr: arm_sisd_two_misc_layout(word)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
  pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:1828
```

## encode_neon_scalar_two_misc_neg_arity
- Tier: 4
- Rationale: Documented min-arity error plus llvm-mc rejection of 0- and 1-operand forms. Domain restriction at neon.rs:1820.
- Doc contract: neon.rs:1820 "scalar two-misc requires 2 operands" — domain-restriction fingerprint 0b5a5e03
- Seed: encode_neon_scalar_three_same_pbt.rs:275 (neg_arity)
- Formal: ∀ n ∈ {0,1}, rd, rn ∈ {0..31}, pfx ∈ {b,h,s,d}, is_neg ∈ bool. llvm-mc rejects arity-n ⇒ encode_neon_scalar_two_misc(ops[:n], U, 00111) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, pfx, is_neg]
  domain: { n: 0..1, rd: 0..31, rn: 0..31, pfx: {b,h,s,d}, is_neg: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_two_misc(ops.take(n), u, 0b00111).is_err()
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
  is_neg: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1820
```

## encode_neon_scalar_two_misc_neg_extra_operand
- Tier: 4
- Rationale: README.md:12 GNU-style assembler contract; llvm-mc/gas reject a third operand. Function only checks len < 2.
- Doc contract: neon.rs:1820 "scalar two-misc requires 2 operands" — domain-restriction fingerprint 0b5a5e03 (min arity only; extra-operand rejection inferred from README.md:12 + llvm-mc)
- Seed: encode_neon_scalar_three_same_pbt.rs:303 (neg_extra_operand)
- Formal: ∀ rd, rn, extra ∈ {0..31}, pfx ∈ {b,h,s,d}, is_neg ∈ bool. llvm-mc rejects three-operand form ⇒ encode_neon_scalar_two_misc([Rd,Rn,extra], U, 00111) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_two_misc([Reg("b0"), Reg("b0"), Reg("b0")], 0, 0b00111)
- Bug report: pbt-out/bug_reports/encode_neon_scalar_two_misc_extra_operand.md

```property
function: encoder.encode_neon_scalar_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, pfx, is_neg]
  domain: { rd: 0..31, rn: 0..31, extra: 0..31, pfx: {b,h,s,d}, is_neg: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_two_misc([Rd,Rn,extra], u, 0b00111).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
  is_neg: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_scalar_two_misc_neg_wrong_reg_class
- Tier: 4
- Rationale: Comment names matching Hd,Hn / Sd,Sn / Dd,Dn pairs; ARM/llvm-mc require the same B/H/S/D class on both operands and reject GPR/Q/V/SP. Function derives size from dest prefix only and does not check source class.
- Doc contract: neon.rs:1818 "NEON scalar two-reg misc: SQABS/SQNEG Hd,Hn / Sd,Sn / Dd,Dn" — asserted fingerprint 83faae33
- Seed: encode_neon_scalar_three_same_pbt.rs:327 (neg_wrong_reg_class)
- Formal: ∀ rd, rn ∈ {0..31}, dest_pfx ∈ {b,h,s,d}, bad_pfx ∉ {dest_pfx} ∪ {valid matching}, is_neg ∈ bool, slot ∈ {dest, src}. llvm-mc rejects the mismatched-class form ⇒ encode_neon_scalar_two_misc = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_two_misc([Reg("h0"), Reg("b0")], 0, 0b00111)
- Bug report: pbt-out/bug_reports/encode_neon_scalar_two_misc_wrong_reg_class.md

```property
function: encoder.encode_neon_scalar_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest_pfx, bad_pfx, is_neg, slot]
  domain: { rd: 0..31, rn: 0..31, dest_pfx: {b,h,s,d}, bad_pfx: {b,h,s,d,x,w,q,v,sp,xzr,wsp,wzr,lr}, is_neg: bool, slot: {0,1} }
  relation:
    op: holds
    expr: encode_neon_scalar_two_misc(mismatched_ops, u, 0b00111).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
  bad_pfx: { gen: oneof, args: ["b", "h", "s", "d", "x", "w", "q", "v", "sp", "xzr", "wsp", "wzr", "lr"] }
  is_neg: { gen: bool }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1818
```

## encode_neon_scalar_two_misc_neg_nonreg
- Tier: 4
- Rationale: Function requires Operand::Reg in both slots ("expected register"); llvm-mc rejects Imm/Mem/Label.
- Doc contract: neon.rs:1821 "expected register" — asserted fingerprint 551b0369
- Seed: encode_neon_scalar_three_same_pbt.rs:365 (neg_nonreg)
- Formal: ∀ rd, rn ∈ {0..31}, pfx ∈ {b,h,s,d}, kind ∈ {Imm,Mem,Label}, slot ∈ {0,1}. encode_neon_scalar_two_misc(ops with non-Reg at slot) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, pfx, kind, slot]
  domain: { rd: 0..31, rn: 0..31, pfx: {b,h,s,d}, kind: {0,1,2}, slot: {0,1} }
  relation:
    op: holds
    expr: encode_neon_scalar_two_misc(ops_with_nonreg, 0, 0b00111).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1821
```

## encode_neon_scalar_two_misc_diff_alt_spellings
- Tier: 2
- Rationale: parse_reg_num lowercases prefixes; llvm-mc accepts uppercase SQABS/SQNEG and B/H/S/D. Differential on the same GNU-style contract.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_neon_scalar_three_same_pbt.rs:409 (diff_alt_spellings)
- Formal: ∀ rd, rn ∈ {0..31}, pfx ∈ {B,H,S,D}, is_neg ∈ bool. encode_neon_scalar_two_misc([Reg(pfx∥rd), Reg(pfx∥rn)], U=is_neg, opcode=00111) = llvm-mc("SQABS|SQNEG pfx rd, pfx rn")
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, pfx, is_neg]
  domain: { rd: 0..31, rn: 0..31, pfx: {B,H,S,D}, is_neg: bool }
  relation:
    op: eq
    lhs: encode_neon_scalar_two_misc([Reg(pfx+rd), Reg(pfx+rn)], u=is_neg, opcode=0b00111)
    rhs: llvm_mc(SQABS_or_SQNEG + " " + pfx+rd + ", " + pfx+rn)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pfx: { gen: oneof, args: ["B", "H", "S", "D"] }
  is_neg: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_scalar_two_misc_neg_unsupported_dest
- Tier: 4
- Rationale: Negative/error contract for dest prefixes x/w/q/v, which neon.rs:1827 names as unsupported register type. Stronger differential does not apply on this invalid domain. SIMD dest/src mismatch is a separate failing property (neg_wrong_reg_class), not this domain.
- Doc contract: neon.rs:1827 "scalar two-misc: unsupported register type" — asserted fingerprint ee928df0
- Seed: (none) — coverage_gaps sweep (documented error branch)
- Formal: ∀ rd, rn ∈ {0..31}, dest_pfx ∈ {x,w,q,v}, src_pfx ∈ {b,h,s,d}, is_neg ∈ bool. llvm-mc rejects ∧ encode_neon_scalar_two_misc([Reg(dest_pfx∥rd), Reg(src_pfx∥rn)], U, 00111) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_scalar_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest_pfx, src_pfx, is_neg]
  domain: { rd: 0..31, rn: 0..31, dest_pfx: {x,w,q,v}, src_pfx: {b,h,s,d}, is_neg: bool }
  relation:
    op: holds
    expr: encode_neon_scalar_two_misc([Reg(dest_pfx+rd), Reg(src_pfx+rn)], u, 0b00111).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_pfx: { gen: oneof, args: ["x", "w", "q", "v"] }
  src_pfx: { gen: oneof, args: ["b", "h", "s", "d"] }
  is_neg: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1827
```
