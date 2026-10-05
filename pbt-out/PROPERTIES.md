# Properties: encode_neon_add_sub

## encode_neon_add_sub_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is Differential vs llvm-mc (independent GNU-style assembler of the same textual assembly the SUT claims to accept). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree vector ADD/SUB decoder). Sibling encode_neon_three_same rejected (same-job gate: generic CMEQ/UQSUB encoder; opcode=10000 would copy this formula). encode_neon_scalar_three_same rejected (scalar Dd,Dn,Dm).
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_diff_llvm_mc_1q
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, is_sub ∈ {false,true}. encode_neon_add_sub([Vd.T,Vn.T,Vm.T], is_sub) = llvm-mc("add|sub Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: neon_t, is_sub: bool }
  relation:
    op: eq
    lhs: encode_neon_add_sub([Vd.T, Vn.T, Vm.T], is_sub)
    rhs: llvm_mc(add_or_sub Vd.T, Vn.T, Vm.T)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  is_sub: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```

## encode_neon_add_sub_metamorphic_rd_rn_rm_u
- Tier: 3
- Rationale: ARM three-same layout isolates Rd[4:0], Rn[9:5], Rm[20:16], U[29]. Stronger differential is p1; this metamorphic check does not need llvm-mc and catches field-packing bugs.
- Doc contract: neon.rs:1171 "ADD: 0 Q 0 01110 size 1 Rm 10000 1 Rn Rd" — asserted fingerprint ec6c59b1
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_metamorphic_rd_rn_rm_q
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. changing only Rd (resp. Rn, Rm, is_sub) of encode_neon_add_sub on T=8b differs only in bits[4:0] (resp. [9:5], [20:16], bit 29)
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_add_sub
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, rm1: u32_0_31, rm2: u32_0_31 }
  relation:
    op: holds
    expr: ((w(rd1,rn1,rm1,false) ^ w(rd2,rn1,rm1,false)) & !0x1Fu32) == 0 && ((w(rd1,rn1,rm1,false) ^ w(rd1,rn2,rm1,false)) & !(0x1Fu32 << 5)) == 0 && ((w(rd1,rn1,rm1,false) ^ w(rd1,rn1,rm2,false)) & !(0x1Fu32 << 16)) == 0 && ((w(rd1,rn1,rm1,false) ^ w(rd1,rn1,rm1,true)) & !(1u32 << 29)) == 0
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1171
```

## encode_neon_add_sub_invariant_arm_fields
- Tier: 3
- Rationale: Documented ARM layout on the success path. Weaker than differential; still pins Q/size/U/opcode independently of llvm-mc.
- Doc contract: neon.rs:1171 "ADD: 0 Q 0 01110 size 1 Rm 10000 1 Rn Rd" — asserted fingerprint ec6c59b1; neon.rs:1172 "SUB: 0 Q 1 01110 size 1 Rm 10000 1 Rn Rd" — asserted fingerprint 8620efa7
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_invariant_arm_64bit_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, is_sub ∈ {false,true}. let w = encode_neon_add_sub([Vd.T,Vn.T,Vm.T], is_sub). bit31(w)=0 ∧ Q(w)=q(T) ∧ U(w)=is_sub ∧ bits[28:24]=01110 ∧ size(w)=size(T) ∧ bit21=1 ∧ Rm=rm ∧ bits[15:11]=10000 ∧ bit10=1 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_add_sub
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: neon_t, is_sub: bool }
  relation:
    op: holds
    expr: arm_three_same_add_sub_layout(encode_neon_add_sub([Vd.T,Vn.T,Vm.T], is_sub))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  is_sub: { gen: bool }
evidence: neon.rs:1171
```

## encode_neon_add_sub_neg_arity
- Tier: 4
- Rationale: gas/llvm-mc reject fewer than 3 operands. README.md:11 same textual assembly as gas. Negative-error: arity 0–2 must Err. The function has no explicit arity check; get_neon_reg on a missing slot is the documented failure path.
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}. llvm-mc rejects arity-n ADD/SUB ⇒ encode_neon_add_sub(ops[:n], is_sub) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, is_sub]
  domain: { n: 0..2, rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool }
  relation:
    op: throws
    expr: encode_neon_add_sub(ops.take(n), is_sub)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```

## encode_neon_add_sub_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc/gas reject a fourth operand. README.md:11. Function currently ignores extra operands (no len check).
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, is_sub ∈ {false,true}. llvm-mc rejects 4-operand ADD/SUB ⇒ encode_neon_add_sub([Vd.T,Vn.T,Vm.T,Vextra.T], is_sub) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: failing
- Counterexample: encode_neon_add_sub([v0.8b, v0.8b, v0.8b, v0.8b], is_sub=false)
- Bug report: bug_reports/encode_neon_add_sub_extra_operand.md

```property
function: encoder.encode_neon_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, extra: u32_0_31, t: neon_t, is_sub: bool }
  relation:
    op: throws
    expr: encode_neon_add_sub([Vd.T,Vn.T,Vm.T,Vextra.T], is_sub)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  is_sub: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```

## encode_neon_add_sub_neg_invalid_t
- Tier: 4
- Rationale: ARM integer ADD/SUB T is {8B,16B,4H,8H,2S,4S,2D}; 1D reserved; arrangements must match. llvm-mc rejects reserved/mismatched T. Function uses only dest arrangement and neon_arr_to_q_size accepts 1d.
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}, is_sub ∈ {false,true}. ¬valid(Td,Tn,Tm) ∧ llvm-mc rejects ⇒ encode_neon_add_sub([Vd.Td,Vn.Tn,Vm.Tm], is_sub) = Err. valid iff Td=Tn=Tm ∈ {8b,16b,4h,8h,2s,4s,2d}
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: failing
- Counterexample: encode_neon_add_sub([v0.8b, v0.8b, v0.16b], is_sub=false)
- Bug report: bug_reports/encode_neon_add_sub_invalid_t.md

```property
function: encoder.encode_neon_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, td: any_t, tn: any_t, tm: any_t, is_sub: bool }
  relation:
    op: throws
    expr: encode_neon_add_sub([Vd.Td,Vn.Tn,Vm.Tm], is_sub)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: element, of: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: element, of: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tm: { gen: element, of: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  is_sub: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```

## encode_neon_add_sub_neg_gpr_or_bare
- Tier: 4
- Rationale: gas/llvm-mc require Vd.T / Vn.T / Vm.T. GPR dest, bare V, and xN.T are rejected. parse_reg_num accepts x/w/d/s/q/v/h/b and maps them to 0–31.
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}, kind ∈ {gpr_dest, bare_vn, x_rm, bare_vd, x_vd_arr}. llvm-mc rejects the corresponding asm ⇒ encode_neon_add_sub(ops(kind), is_sub) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: failing
- Counterexample: encode_neon_add_sub([v0.8b, Reg(v0), v0.8b], is_sub=false)
- Bug report: bug_reports/encode_neon_add_sub_bare_src.md

```property
function: encoder.encode_neon_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_sub, kind]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool, kind: 0..4 }
  relation:
    op: throws
    expr: encode_neon_add_sub(ops(kind), is_sub)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```

## encode_neon_add_sub_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc on uppercase mnemonic/V prefix (parser lowercases). Strengthens p1 past the canonical lowercase spelling.
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, is_sub ∈ {false,true}. encode_neon_add_sub([V{rd}.T, V{rn}.T, V{rm}.T], is_sub) = llvm-mc("ADD|SUB Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_add_sub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, is_sub]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: neon_t, is_sub: bool }
  relation:
    op: eq
    lhs: encode_neon_add_sub([Vrd.T, Vrn.T, Vrm.T], is_sub)
    rhs: llvm_mc(ADD_or_SUB Vd.T, Vn.T, Vm.T)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  is_sub: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```

## encode_neon_add_sub_neg_nonreg
- Tier: 4
- Rationale: Sweep — Imm/Mem/Label at any operand slot must Err (get_neon_reg else-arm). llvm-mc rejects Imm/Mem/Label dest. Weaker than the GPR/bare property; covers the documented error path of non-register operands.
- Doc contract: neon.rs:1163 "Encode NEON ADD/SUB (vector integer): ADD/SUB Vd.T, Vn.T, Vm.T" — asserted fingerprint 2c237b39
- Seed: encode_neon_pmull_pbt.rs encode_neon_pmull_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_sub ∈ {false,true}, kind ∈ {Imm,Mem,Label}, slot ∈ {0,1,2}. encode_neon_add_sub(ops with slot replaced by kind, is_sub) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_add_sub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_sub, kind, slot]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_sub: bool, kind: 0..2, slot: 0..2 }
  relation:
    op: throws
    expr: encode_neon_add_sub(ops_with_nonreg(kind, slot), is_sub)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint f00ab438
```
