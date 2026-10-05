# Properties: encode_neon_logical

## encode_neon_logical_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the GNU-style assembler contract. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree vector-logical decoder). Sibling encode_neon_bic / encode_neon_bsl / encode_orn rejected (same-job gate: different size/U opcodes).
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a
- Seed: encode_neon_bsl_pbt.rs:167 encode_neon_bsl_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. encode_neon_logical([Vd.T,Vn.T,Vm.T], opc) = llvm-mc(mnemonic Vd.T, Vn.T, Vm.T) where mnemonic = {0b00→and, 0b01→orr, 0b10→eor}
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b}, opc: {0,1,2} }
  relation:
    op: eq
    lhs: encode_neon_logical([arr(rd,t), arr(rn,t), arr(rm,t)], opc)
    rhs: llvm_mc(mnemonic(opc) + " v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
evidence: README.md:12 GNU-style gas contract; neon.rs:296-308 AND/ORR/EOR encodings; encoder/mod.rs:295-297 dispatch
```

## encode_neon_logical_meta_rd_rn_rm
- Tier: 4
- Rationale: Algebraic metamorphic — changing only Rd/Rn/Rm must differ only in bits[4:0]/[9:5]/[20:16]. Stronger differential is P1; this isolates field packing independently of llvm-mc.
- Doc contract: neon.rs:306 "ORR: 0 Q 0 01110 10 1 Rm 000111 Rn Rd  (opc=0b01 -> size=10)" — asserted fingerprint 880d7f42
- Seed: encode_neon_bsl_pbt.rs:182 encode_neon_bsl_meta_rd_rn_rm
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. let w(rd,rn,rm)=encode_neon_logical([Vd.T,Vn.T,Vm.T],opc). (w(rd1,rn1,rm1) ⊕ w(rd2,rn1,rm1)) & ~0x1F = 0 ∧ (w(rd1,rn1,rm1) ⊕ w(rd1,rn2,rm1)) & ~(0x1F<<5) = 0 ∧ (w(rd1,rn1,rm1) ⊕ w(rd1,rn1,rm2)) & ~(0x1F<<16) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2, t, opc]
  domain: { rd1: v0_31, rd2: v0_31, rn1: v0_31, rn2: v0_31, rm1: v0_31, rm2: v0_31, t: {8b,16b}, opc: {0,1,2} }
  body: field isolation of Rd bits[4:0], Rn bits[9:5], Rm bits[20:16]
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
evidence: neon.rs:306-308 Rm/Rn/Rd field comments
```

## encode_neon_logical_inv_layout
- Tier: 4
- Rationale: Algebraic invariant — ARM Advanced SIMD three-same logical layout 0 Q U 01110 size 1 Rm 000111 Rn Rd with (U,size)=(0,00) AND / (0,10) ORR / (1,00) EOR, Q=1 iff T=16b. 8b vs 16b differ only in bit 30.
- Doc contract: neon.rs:307 "AND: 0 Q 0 01110 00 1 Rm 000111 Rn Rd  (opc=0b00 -> size=00)" — asserted fingerprint 8f640cbb
- Seed: encode_neon_bsl_pbt.rs:219 encode_neon_bsl_inv_layout
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. encode_neon_logical([Vd.T,Vn.T,Vm.T], opc) = arm_logical_word(rd,rn,rm,T,opc)
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b}, opc: {0,1,2} }
  relation:
    op: eq
    lhs: encode_neon_logical([arr(rd,t), arr(rn,t), arr(rm,t)], opc)
    rhs: arm_logical_word(rd, rn, rm, t, opc)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
evidence: neon.rs:306-308; ARM ARM ASIMDSAME AND/ORR/EOR
```

## encode_neon_logical_meta_opc
- Tier: 4
- Rationale: Algebraic metamorphic — holding registers and T fixed, AND/ORR/EOR must differ only in U (bit 29) and size (bits 23-22). Required metamorphic/differential for STANDARD tier (P1 is differential; this is the opc transform).
- Doc contract: neon.rs:308 "EOR: 0 Q 1 01110 00 1 Rm 000111 Rn Rd  (opc=0b10 -> size=00, U=1)" — asserted fingerprint d4f469c1
- Seed: encode_neon_bsl_pbt.rs:237 8b vs 16b Q isolation
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}. let a=encode_neon_logical(ops,0b00), o=encode_neon_logical(ops,0b01), e=encode_neon_logical(ops,0b10). (a ⊕ o) & ~(0b11<<22) = 0 ∧ (a ⊕ e) & ~(1<<29) = 0 ∧ (o ⊕ e) has only U and size bits
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b} }
  body: AND vs ORR differ only in size bits[23:22]; AND vs EOR differ only in U bit 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:306-308 U/size as opcode discriminator
```

## encode_neon_logical_neg_extra
- Tier: 3
- Rationale: Negative/error contract — gas/llvm-mc reject a fourth operand. README.md:12 same textual assembly as gas. SUT does not check operands.len() > 3.
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a (three operands; extra is out of the documented form)
- Seed: encode_neon_bsl_pbt.rs:256 encode_neon_bsl_neg_extra
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. llvm-mc rejects mnemonic Vd.T,Vn.T,Vm.T,Vextra.T ⇒ encode_neon_logical([Vd.T,Vn.T,Vm.T,Vextra.T], opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: failing
- Counterexample: encode_neon_logical([v0.8b, v0.8b, v0.8b, v0.8b], opc=0) = Ok(Word) ; llvm-mc rejects `and v0.8b, v0.8b, v0.8b, v0.8b`
- Bug report: pbt-out/bug_reports/encode_neon_logical_extra_operand.md

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, extra: v0_31, t: {8b,16b}, opc: {0,1,2} }
  relation:
    op: throws
    expr: encode_neon_logical([arr(rd,t), arr(rn,t), arr(rm,t), arr(extra,t)], opc)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: README.md:12 gas contract; llvm-mc rejects fourth operand; neon.rs:296 three-operand form
```

## encode_neon_logical_neg_mismatch_t
- Tier: 3
- Rationale: Negative/error — ARM/llvm-mc require matching T on Vd,Vn,Vm. SUT discards arr_n and arr_m.
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a (same T)
- Seed: encode_neon_bsl_pbt.rs:296 encode_neon_bsl_neg_mismatch_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. ¬(Td=Tn=Tm) ⇒ encode_neon_logical([Vd.Td,Vn.Tn,Vm.Tm], opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: failing
- Counterexample: encode_neon_logical([v0.8b, v0.8b, v0.16b], opc=0) = Ok(Word) ; llvm-mc rejects `and v0.8b, v0.8b, v0.16b`
- Bug report: pbt-out/bug_reports/encode_neon_logical_mismatch_t.md

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, td: {8b,16b}, tn: {8b,16b}, tm: {8b,16b}, opc: {0,1,2} }
  body: td!=tn or tn!=tm implies Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, items: ["8b", "16b"] }
  tn: { gen: oneof, items: ["8b", "16b"] }
  tm: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: neon.rs:296 Vd.T, Vn.T, Vm.T same T; ARM ARM AND/ORR/EOR; llvm-mc operand mismatch
```

## encode_neon_logical_neg_invalid_t
- Tier: 3
- Rationale: Negative/error — ARM AND/ORR/EOR accept only T in {8B,16B}. llvm-mc rejects 8h/4s/2d/4h/2s/1d. SUT sets Q=1 iff dest=="16b" else Q=0, so other arrangements silently encode as 8b.
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a; ARM T in {8B,16B}
- Seed: encode_neon_bsl_pbt.rs:274 encode_neon_bsl_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {4h,8h,2s,4s,2d,1d,4b,8d}, opc ∈ {0b00,0b01,0b10}. encode_neon_logical([Vd.T,Vn.T,Vm.T], opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: failing
- Counterexample: encode_neon_logical([v0.4h, v0.4h, v0.4h], opc=0) = Ok(Word) ; llvm-mc rejects `and v0.4h, v0.4h, v0.4h`
- Bug report: pbt-out/bug_reports/encode_neon_logical_invalid_t.md

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {4h,8h,2s,4s,2d,1d,4b,8d}, opc: {0,1,2} }
  relation:
    op: throws
    expr: encode_neon_logical([arr(rd,t), arr(rn,t), arr(rm,t)], opc)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["4h", "8h", "2s", "4s", "2d", "1d", "4b", "8d"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: ARM ARM AND/ORR/EOR T in {8B,16B}; llvm-mc invalid operand for 8h/4s
```

## encode_neon_logical_neg_gpr_bare_sp
- Tier: 3
- Rationale: Negative/error — vector logical requires arranged NEON registers. llvm-mc rejects GPR/SP/bare-v/scalar-fp sources. get_neon_reg accepts Operand::Reg. Caller-reachable: encode_logical dispatches on dest RegArrangement and passes Vn/Vm through.
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a
- Seed: encode_neon_bsl_pbt.rs:321 encode_neon_bsl_neg_gpr_bare_sp
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}, kind ∈ {x-src, w-src, sp-src, bare-v, d-src, s-src, q-src}. encode_neon_logical([Vd.T, non-arranged Vn/Vm], opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: failing
- Counterexample: encode_neon_logical([v0.8b, x0, x0], opc=0) = Ok(Word) ; llvm-mc rejects `and v0.8b, x0, x0`
- Bug report: pbt-out/bug_reports/encode_neon_logical_gpr_src.md

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, opc, kind]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b}, opc: {0,1,2}, kind: {0..6} }
  relation:
    op: throws
    expr: encode_neon_logical(non_arranged_src(kind), opc)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u8 }
expected_error: String
evidence: neon.rs:296 Vd.T arranged form; llvm-mc rejects GPR/bare/SP sources
```

## encode_neon_logical_neg_ands
- Tier: 3
- Rationale: Negative/error — ANDS is not a NEON instruction. llvm-mc rejects `ands v0.8b, ...`. Comment at neon.rs:313 admits "ANDS - not valid for NEON, fall back" on an input the API accepts (encode_logical routes "ands" with opc=0b11). Known limitation; keep in domain.
- Doc contract: neon.rs:313 "ANDS - not valid for NEON, fall back" — limitation fingerprint 3d76eeef
- Seed: (none) — strengthening round on documented ANDS fallback
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}. encode_neon_logical([Vd.T,Vn.T,Vm.T], 0b11) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: failing
- Counterexample: encode_neon_logical([v0.8b, v0.8b, v0.8b], opc=0b11) = Ok(Word) encoding EOR; llvm-mc rejects `ands v0.8b, v0.8b, v0.8b`
- Bug report: pbt-out/bug_reports/encode_neon_logical_ands.md

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_logical([arr(rd,t), arr(rn,t), arr(rm,t)], 0b11)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:313 ANDS not valid for NEON; llvm-mc rejects ands vector form; encoder/mod.rs:298 ands => opc=0b11
```

## encode_neon_logical_neg_arity
- Tier: 3
- Rationale: Negative/error — fewer than 3 operands must Err. get_neon_reg returns Err on missing operand. Strengthening round.
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a
- Seed: encode_neon_bsl_pbt.rs:242 encode_neon_bsl_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. encode_neon_logical(ops[:n], opc) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t, opc]
  domain: { n: {0,1,2}, rd: v0_31, t: {8b,16b}, opc: {0,1,2} }
  relation:
    op: throws
    expr: encode_neon_logical(ops_prefix(n), opc)
    error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: neon.rs:296 three-operand form; get_neon_reg missing-operand Err
```

## encode_neon_logical_neg_unsupported_opc
- Tier: 3
- Rationale: Negative/error contract on the documented invalid opc domain. neon.rs:314 declares opc not in {0,1,2,3} invalid and returns Err("unsupported NEON logical opc"). The property asserts that documented rejection; it is not an exclusion of a valid enumerator.
- Doc contract: neon.rs:314 `_ => return Err("unsupported NEON logical opc")` — domain-restriction fingerprint cd744e09
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, opc ∈ ℕ \ {0,1,2,3}. encode_neon_logical([Vd.T,Vn.T,Vm.T], opc) = Err("unsupported NEON logical opc")
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b}, opc: {4..255} }
  relation:
    op: throws
    expr: encode_neon_logical([arr(rd,t), arr(rn,t), arr(rm,t)], opc)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 4, max: 255, type: u32 }
expected_error: String
evidence: neon.rs:314 unsupported NEON logical opc
```

## encode_neon_logical_diff_alt_spellings
- Tier: 5
- Rationale: Differential vs llvm-mc on uppercase V prefix (parse_reg_num lowercases). Strengthening / sweep of register-name contract.
- Doc contract: neon.rs:296 "Encode NEON logical operations: ORR/AND/EOR Vd.T, Vn.T, Vm.T" — asserted fingerprint ce2d8b1a
- Seed: encode_neon_bsl_pbt.rs:394 encode_neon_bsl_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, opc ∈ {0b00,0b01,0b10}. encode_neon_logical([V{rd}.T, V{rn}.T, V{rm}.T], opc) = llvm-mc(mnemonic V{rd}.T, V{rn}.T, V{rm}.T)
- Test file: src/backend/arm/assembler/encoder/encode_neon_logical_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_logical
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, opc]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, t: {8b,16b}, opc: {0,1,2} }
  relation:
    op: eq
    lhs: encode_neon_logical([Arr(V{rd},t), Arr(V{rn},t), Arr(V{rm},t)], opc)
    rhs: llvm_mc(mnemonic(opc) + " V{rd}.{t}, V{rn}.{t}, V{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
evidence: README.md:12; parse_reg_num lowercases; llvm-mc accepts V0.8b
```
