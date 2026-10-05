# Properties: encode_neon_scalar_addp

## encode_neon_scalar_addp_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree scalar ADDP decoder). Sibling encode_neon_three_same / encode_neon_faddp rejected (same-job gate: vector ADDP and float FADDP are different opcodes). Doc evidence: README.md:12 GNU-style assembly; README.md:237 `addp` (scalar); neon.rs:1801 form `addp Dd, Vn.2d`.
- Doc contract: neon.rs:1801 "NEON scalar ADDP: addp Dd, Vn.2d" — asserted fingerprint d6d5ee84
- Seed: encode_neon_scalar_three_same_pbt.rs:168 llvm-mc differential
- Formal: ∀ rd,rn ∈ {0..31}. encode_neon_scalar_addp([Reg("d{rd}"), RegArrangement("v{rn}","2d")]) = Word(llvm-mc("addp d{rd}, v{rn}.2d"))
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: "0..=31", rn: "0..=31" }
  relation:
    op: eq
    lhs: encode_neon_scalar_addp([Reg(d{rd}), RegArrangement(v{rn}, 2d)])
    rhs: llvm_mc_word("addp d{rd}, v{rn}.2d")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_scalar_addp_meta_rd_rn
- Tier: 3
- Rationale: Weaker than differential; metamorphic field isolation: changing only Rd (resp. Rn) differs only in bits[4:0] (resp. bits[9:5]). Stronger rejected as above for this relation. ARM SISD ADDP layout at neon.rs:1812.
- Doc contract: neon.rs:1812 "Scalar ADDP: 01 0 11110 11 11000 11011 10 Rn Rd" — asserted fingerprint 124148d9
- Seed: encode_neon_scalar_three_same_pbt.rs:204 Rd/Rn isolation
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}. let w11 = f(rd1,rn1); w21 = f(rd2,rn1); w12 = f(rd1,rn2). (w11 ⊕ w21) ∧ ¬0x1F = 0 ∧ (w21 ∧ 0x1F) = rd2 ∧ (w11 ⊕ w12) ∧ ¬(0x1F≪5) = 0 ∧ ((w12≫5) ∧ 0x1F) = rn2
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: "0..=31", rd2: "0..=31", rn1: "0..=31", rn2: "0..=31" }
  relation:
    op: holds
    expr: rd_rn_isolation(rd1, rd2, rn1, rn2)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1812
```

## encode_neon_scalar_addp_invariant_arm_fields
- Tier: 3
- Rationale: Algebraic invariant of the ARM SISD ADDP encoding. Stronger differential covers value equality; this pins every documented field bit. Evidence: neon.rs:1812 encoding comment.
- Doc contract: neon.rs:1812 "Scalar ADDP: 01 0 11110 11 11000 11011 10 Rn Rd" — asserted fingerprint 124148d9
- Seed: encode_neon_scalar_three_same_pbt.rs:250 ARM field layout
- Formal: ∀ rd,rn ∈ {0..31}. let w = encode_neon_scalar_addp([Reg("d{rd}"), RegArrangement("v{rn}","2d")]). w[31:30]=01 ∧ w[29]=0 ∧ w[28:24]=11110 ∧ w[23:22]=11 ∧ w[21:17]=11000 ∧ w[16:12]=11011 ∧ w[11:10]=10 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: "0..=31", rn: "0..=31" }
  relation:
    op: holds
    expr: arm_sisd_addp_fields(encode_neon_scalar_addp([Reg(d{rd}), RegArrangement(v{rn}, 2d)]))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1812
```

## encode_neon_scalar_addp_neg_arity
- Tier: 4
- Rationale: Documented error contract — arity < 2 returns Err. llvm-mc also rejects 0- and 1-operand addp. Domain restriction at neon.rs:1803.
- Doc contract: neon.rs:1803 "scalar addp requires 2 operands" — domain-restriction fingerprint 8c205ae9
- Seed: encode_neon_scalar_three_same_pbt.rs:276 arity
- Formal: ∀ n ∈ {0,1}, rd,rn ∈ {0..31}. llvm-mc rejects arity-n addp ∧ encode_neon_scalar_addp(ops[..n]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn]
  domain: { n: "0..=1", rd: "0..=31", rn: "0..=31" }
  relation:
    op: holds
    expr: encode_neon_scalar_addp(ops.take(n)).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1803
```

## encode_neon_scalar_addp_neg_extra_operand
- Tier: 4
- Rationale: GNU/gas and llvm-mc reject a third operand on scalar ADDP. Function comment "requires 2 operands" plus README GNU-style contract. Function checks only len < 2; extra ignored is a documented-arity violation.
- Doc contract: neon.rs:1803 "scalar addp requires 2 operands" — domain-restriction fingerprint 8c205ae9
- Seed: encode_neon_scalar_three_same_pbt.rs:306 extra operand
- Formal: ∀ rd,rn,extra ∈ {0..31}. llvm-mc("addp d{rd}, v{rn}.2d, d{extra}") fails ∧ encode_neon_scalar_addp([Dd, Vn.2d, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_addp([Reg("d0"), RegArrangement { reg: "v0", arrangement: "2d" }, Reg("d0")])
- Bug report: pbt-out/bug_reports/encode_neon_scalar_addp_extra_operand.md

```property
function: encode_neon_scalar_addp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: "0..=31", rn: "0..=31", extra: "0..=31" }
  relation:
    op: holds
    expr: encode_neon_scalar_addp([Dd, Vn.2d, extra]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1803
```

## encode_neon_scalar_addp_neg_wrong_reg_class
- Tier: 4
- Rationale: ARM/gas/llvm-mc require dest Dd (not s/h/b/x/w/q/v/sp/zr) and source Vn.2D (not x/w/d/s prefix). Comment neon.rs:1801 form `addp Dd, Vn.2d` and error "expected d register". Function accepts any parse_reg_num dest and any prefix on the .2d source.
- Doc contract: neon.rs:1801 "NEON scalar ADDP: addp Dd, Vn.2d" — asserted fingerprint d6d5ee84
- Seed: encode_neon_scalar_three_same_pbt.rs:330 wrong reg class
- Formal: ∀ rd,rn ∈ {0..31}, pfx ∈ {s,h,b,x,w,q,v,sp,xzr,wsp,wzr,lr}. llvm-mc rejects addp with dest pfx (or source pfx.2d) ∧ encode_neon_scalar_addp on that operand vector = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: failing
- Counterexample: encode_neon_scalar_addp([Reg("s0"), RegArrangement { reg: "v0", arrangement: "2d" }])
- Bug report: pbt-out/bug_reports/encode_neon_scalar_addp_wrong_reg_class.md

```property
function: encode_neon_scalar_addp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, slot, pfx]
  domain: { rd: "0..=31", rn: "0..=31", slot: "0..=1", pfx: "non-d dest or non-v source" }
  relation:
    op: holds
    expr: encode_neon_scalar_addp(wrong_class_ops).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  slot: { gen: int, min: 0, max: 1, type: usize }
evidence: src/backend/arm/assembler/encoder/neon.rs:1801
```

## encode_neon_scalar_addp_neg_bad_arrangement
- Tier: 4
- Rationale: Source must be Vn.2d; any other arrangement is rejected by llvm-mc and by the function's own error at neon.rs:1807. Documented bound: arrangement == "2d". Generator covers the closed set of other NEON arrangements.
- Doc contract: neon.rs:1807 "scalar addp requires .2d source, got .{}" — asserted fingerprint 4241c204
- Seed: encode_neon_faddp_pbt.rs invalid T
- Formal: ∀ rd,rn ∈ {0..31}, arr ∈ {8b,16b,4h,8h,2s,4s,1d}. llvm-mc rejects addp d{rd}, v{rn}.{arr} ∧ encode_neon_scalar_addp([Dd, Vn.arr]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, arr]
  domain: { rd: "0..=31", rn: "0..=31", arr: "{8b,16b,4h,8h,2s,4s,1d}" }
  relation:
    op: holds
    expr: encode_neon_scalar_addp([Reg(d{rd}), RegArrangement(v{rn}, arr)]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:1807
```

## encode_neon_scalar_addp_neg_nonreg
- Tier: 4
- Rationale: Imm/Mem/Label in either slot is not a register form. llvm-mc rejects; function should Err. Error paths at neon.rs:1805 (dest) and neon.rs:1810 (source).
- Doc contract: neon.rs:1805 "expected d register" — asserted fingerprint b47dce50
- Seed: encode_neon_scalar_three_same_pbt.rs:365 nonreg
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {Imm,Mem,Label}, slot ∈ {0,1}. encode_neon_scalar_addp(ops with slot=kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: "0..=31", rn: "0..=31", kind: "0..=2", slot: "0..=1" }
  relation:
    op: holds
    expr: encode_neon_scalar_addp(nonreg_ops).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
evidence: src/backend/arm/assembler/encoder/neon.rs:1805
```

## encode_neon_scalar_addp_diff_alt_spellings
- Tier: 2
- Rationale: Sweep / strengthening: uppercase D/V prefixes and ADDP mnemonic must still match llvm-mc. parse_reg_num lowercases; arrangement in the test is the parser-canonical "2d".
- Doc contract: neon.rs:1801 "NEON scalar ADDP: addp Dd, Vn.2d" — asserted fingerprint d6d5ee84
- Seed: encode_neon_scalar_three_same_pbt.rs:410 alt-spellings
- Formal: ∀ rd,rn ∈ {0..31}. encode_neon_scalar_addp([Reg("D{rd}"), RegArrangement("V{rn}","2d")]) = Word(llvm-mc("ADDP D{rd}, V{rn}.2D"))
- Test file: src/backend/arm/assembler/encoder/encode_neon_scalar_addp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_scalar_addp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: "0..=31", rn: "0..=31" }
  relation:
    op: eq
    lhs: encode_neon_scalar_addp([Reg(D{rd}), RegArrangement(V{rn}, 2d)])
    rhs: llvm_mc_word("ADDP D{rd}, V{rn}.2D")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```
