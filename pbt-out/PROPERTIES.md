# Properties: encode_neon_three_diff

## encode_neon_three_diff_diff_llvm_mc_long
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the GNU-style assembly the assembler claims to accept (README.md:12). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree three-diff decoder). Sibling encode_neon_three_diff_narrow / encode_neon_pmull rejected (same-job gate: narrowing vs widening; polynomial vs integer). LONG form is the function's own documented job (neon.rs:81-83).
- Doc contract: neon.rs:81 "Encode NEON three-different instructions: USUBL, SSUBL, UADDL, SADDL, etc." — asserted fingerprint 6c009a01
- Seed: encode_neon_three_same_pbt.rs:202 (llvm-mc differential on matching T); encode_neon_pmull_pbt.rs (widening Ta/Tb)
- Formal: ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, (U,opc,mnem) ∈ LONG_TABLE. Let Ta = widen(Tb), is_high = (Tb ∈ {16b,8h,4s}). encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tb], U, opc, is_high) = llvm-mc("mnem Vd.Ta, Vn.Tb, Vm.Tb") where mnem is saddl2/uaddl2/… when is_high else saddl/uaddl/…
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, tb: long_tb, insn: long_table }
  relation:
    op: eq
    lhs: encode_neon_three_diff([Vd.widen(tb), Vn.tb, Vm.tb], insn.U, insn.opc, is_high(tb))
    rhs: llvm_mc(insn.mnem2(is_high(tb)) + " Vd.Ta, Vn.Tb, Vm.Tb")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  insn: { gen: oneof, values: ["saddl", "uaddl", "ssubl", "usubl", "smull", "umull", "sabal", "uabal", "sabdl", "uabdl", "smlal", "umlal", "smlsl", "umlsl"] }
evidence: neon.rs:81 encoder/mod.rs:815-905
```

## encode_neon_three_diff_diff_llvm_mc_wide
- Tier: 2
- Rationale: README.md:230 lists saddw/uaddw/ssubw/usubw (+2) as NEON widen/long; dispatcher routes them through encode_neon_three_diff (mod.rs:819-832). Same llvm-mc differential as LONG; WIDE uses Vd.Ta, Vn.Ta, Vm.Tb.
- Doc contract: neon.rs:99 "Size is determined from the source (narrow) arrangement" — asserted fingerprint 09885f0f
- Seed: encode_neon_pmull_pbt.rs (Ta/Tb pair); encoder/mod.rs:819-832 saddw/uaddw/ssubw/usubw
- Formal: ∀ rd,rn,rm ∈ {0..31}, (Ta,Tb,is_high) ∈ WIDE_PAIRS, (U,opc,mnem) ∈ WIDE_TABLE. encode_neon_three_diff([Vd.Ta, Vn.Ta, Vm.Tb], U, opc, is_high) = llvm-mc("mnem{2} Vd.Ta, Vn.Ta, Vm.Tb")
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_diff([v0.8h, v0.8h, v0.8b], U=0, opc=0b0001, is_high=false) = 0x4e601000, llvm-mc("saddw v0.8h, v0.8h, v0.8b") = 0x0e201000
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_wide_vn_size.md

```property
function: encoder.neon.encode_neon_three_diff
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, pair, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, pair: wide_pairs, insn: wide_table }
  relation:
    op: eq
    lhs: encode_neon_three_diff([Vd.Ta, Vn.Ta, Vm.Tb], insn.U, insn.opc, pair.is_high)
    rhs: llvm_mc(insn.mnem2(pair.is_high) + " Vd.Ta, Vn.Ta, Vm.Tb")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, values: [["8h","8b",false], ["8h","16b",true], ["4s","4h",false], ["4s","8h",true], ["2d","2s",false], ["2d","4s",true]] }
  insn: { gen: oneof, values: ["saddw", "uaddw", "ssubw", "usubw"] }
evidence: neon.rs:99 encoder/mod.rs:819-832
```

## encode_neon_three_diff_invariant_arm_fields
- Tier: 4
- Rationale: neon.rs:84 documents the ASIMDDIFF layout. Weaker than differential; kept as a structural invariant over the full opcode/U domain (not only named mnemonics).
- Doc contract: neon.rs:84 "Format: 0 Q U 01110 size 1 Rm opcode 00 Rn Rd" — asserted fingerprint 3f1ac3c4
- Seed: encode_neon_three_same_pbt.rs:237 (ARM field invariant)
- Formal: ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, U ∈ {0,1}, opc ∈ {0..15}. Let w = encode_neon_three_diff([Vd.widen(Tb), Vn.Tb, Vm.Tb], U, opc, is_high(Tb)). Then w[31]=0 ∧ w[30]=Q(Tb) ∧ w[29]=U ∧ w[28:24]=01110 ∧ w[23:22]=size(Tb) ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:12]=opc ∧ w[11:10]=00 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, u, opc]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, tb: long_tb, u: bit, opc: u4 }
  relation:
    op: holds
    expr: arm_asimddiff_fields(encode_neon_three_diff([Vd.widen(tb), Vn.tb, Vm.tb], u, opc, is_high(tb)), rd, rn, rm, tb, u, opc)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opc: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:84
```

## encode_neon_three_diff_metamorphic_u_opcode_q
- Tier: 4
- Rationale: Documented U (bit 29), 4-bit opcode (bits 15-12), and is_high Q (bit 30) are independent fields. Changing one must differ only in that field.
- Doc contract: neon.rs:88 "`is_high`: true for the \"2\" variant (upper half, Q=1)" — asserted fingerprint 0bc028dd
- Seed: encode_neon_three_same_pbt.rs:256 (U/opcode/Q metamorphic)
- Formal: ∀ rd,rn,rm ∈ {0..31}, opc1,opc2 ∈ {0..15}. Let w0 = encode(..., U=0, opc1, is_high=false) on 8b; w1 = encode(..., U=1, opc1, is_high=false); w2 = encode(..., U=0, opc2, is_high=false); wQ = encode(..., U=0, opc1, is_high=true). Then (w0 ⊕ w1) = 1<<29 ∧ (w0 ⊕ w2) masked off bits[15:12] = 0 ∧ (w0 ⊕ wQ) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, opc1, opc2]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, opc1: u4, opc2: u4 }
  relation:
    op: holds
    expr: u_opcode_q_isolated(rd, rn, rm, opc1, opc2)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  opc1: { gen: int, min: 0, max: 15, type: u32 }
  opc2: { gen: int, min: 0, max: 15, type: u32 }
evidence: neon.rs:84-88
```

## encode_neon_three_diff_neg_arity
- Tier: 4
- Rationale: neon.rs:90-92 returns Err when operands.len() < 3; llvm-mc rejects too-few-operand SADDL. Documented error contract.
- Doc contract: neon.rs:91 "NEON three-different requires 3 operands" — asserted fingerprint ee6ec996
- Seed: encode_neon_three_same_pbt.rs:294 (arity 0-2)
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}, (U,opc,mnem) ∈ LONG_TABLE. encode_neon_three_diff(ops[0..n], U, opc, false) is Err ∧ llvm-mc(mnem with n operands) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, insn]
  domain: { n: 0..2, rd: v0_v31, rn: v0_v31, rm: v0_v31, insn: long_table }
  relation:
    op: holds
    expr: encode_neon_three_diff(ops.take(n), insn.U, insn.opc, false).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  insn: { gen: oneof, values: ["saddl", "uaddl", "ssubl", "usubl"] }
evidence: neon.rs:90-92
```

## encode_neon_three_diff_neg_extra_operand
- Tier: 4
- Rationale: GNU gas / llvm-mc reject a fourth operand on SADDL. neon.rs:91 says the instruction requires 3 operands. Function must Err, not ignore extras.
- Doc contract: neon.rs:91 "NEON three-different requires 3 operands" — asserted fingerprint ee6ec996
- Seed: encode_neon_three_same_pbt.rs:322 (extra operand)
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(mnem Vd.Ta, Vn.Tb, Vm.Tb, Vextra.Tb) is Err ∧ encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tb, Vextra.Tb], U, opc, is_high(Tb)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_diff([v0.8h, v0.8b, v0.8b, v0.8b], U=0, opc=0, is_high=false) is Ok
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_extra_operand.md

```property
function: encoder.neon.encode_neon_three_diff
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, tb, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, extra: v0_v31, tb: long_tb, insn: long_table }
  relation:
    op: holds
    expr: encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tb, Vextra.Tb], insn.U, insn.opc, is_high(tb)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  insn: { gen: oneof, values: ["saddl", "uaddl", "ssubl", "usubl"] }
evidence: neon.rs:91
```

## encode_neon_three_diff_neg_dest_ta_mismatch
- Tier: 4
- Rationale: neon.rs:83 asserts a wider destination than source. ARM/llvm-mc require Ta = widen(Tb). A dest arrangement that is not that widening must Err.
- Doc contract: neon.rs:83 "These instructions have wider destination than source operands." — asserted fingerprint ab17a147
- Seed: encode_neon_three_same_pbt.rs:351 (mismatched T)
- Formal: ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, Td ∈ ARR, Td ≠ widen(Tb), (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(mnem Vd.Td, Vn.Tb, Vm.Tb) is Err ∧ encode_neon_three_diff([Vd.Td, Vn.Tb, Vm.Tb], U, opc, is_high(Tb)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_diff([v0.8b, v0.8b, v0.8b], U=0, opc=0, is_high=false) is Ok
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_dest_ta_mismatch.md

```property
function: encoder.neon.encode_neon_three_diff
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, td, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, tb: long_tb, td: any_t_neq_widen(tb), insn: long_table }
  relation:
    op: holds
    expr: encode_neon_three_diff([Vd.td, Vn.tb, Vm.tb], insn.U, insn.opc, is_high(tb)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  td: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"] }
  insn: { gen: oneof, values: ["saddl", "uaddl"] }
evidence: neon.rs:83
```

## encode_neon_three_diff_neg_gpr_or_bare
- Tier: 4
- Rationale: GNU gas / llvm-mc require Vd.Ta / Vn.Tb / Vm.Tb. GPR dest, bare V without arrangement, and x/w prefixes must Err.
- Doc contract: neon.rs:81 "Encode NEON three-different instructions: USUBL, SSUBL, UADDL, SADDL, etc." — asserted fingerprint 6c009a01
- Seed: encode_neon_three_same_pbt.rs:384 (GPR/bare)
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr_dest, bare_dest, gpr_arr_dest, gpr_src}, (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(asm(kind)) is Err ∧ encode_neon_three_diff(ops(kind), U, opc, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_diff([v0.8h, v0.8b, x0], U=0, opc=0, is_high=false) is Ok
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_gpr_or_bare.md

```property
function: encoder.neon.encode_neon_three_diff
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, kind: gpr_or_bare, insn: long_table }
  relation:
    op: holds
    expr: encode_neon_three_diff(ops(kind), insn.U, insn.opc, false).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
  insn: { gen: oneof, values: ["saddl", "uaddl"] }
evidence: neon.rs:81
```

## encode_neon_three_diff_neg_unsupported_src
- Tier: 4
- Rationale: Sweep of the documented invalid-domain arm. neon.rs:99-106 enumerates the valid source arrangements {8b,16b,4h,8h,2s,4s} and returns Err for any other; ARM size:Q=11:x is reserved; llvm-mc rejects 1d/2d/1q as Tb. This property asserts that documented rejection, not a limitation on valid input.
- Doc contract: neon.rs:106 "unsupported source arrangement for three-diff" — domain-restriction fingerprint c30def5e
- Seed: encode_neon_three_diff_narrow_unsupported_src
- Formal: ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {1d,2d,1q,ε}, (U,opc) ∈ LONG_TABLE. Tb ∉ {8b,16b,4h,8h,2s,4s} ⇒ encode_neon_three_diff([Vd.8h, Vn.Tb, Vm.Tb], U, opc, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_three_diff
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, tb: unsupported_tb, insn: long_table }
  relation:
    op: holds
    expr: encode_neon_three_diff([Vd.8h, Vn.tb, Vm.tb], insn.U, insn.opc, false).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["1d", "2d", "1q", ""] }
  insn: { gen: oneof, values: ["saddl", "uaddl"] }
evidence: neon.rs:106
```

## encode_neon_three_diff_neg_rm_tb_mismatch
- Tier: 4
- Rationale: Sweep — ARM LONG form requires Vm.Tb = Vn.Tb. llvm-mc rejects mismatched Rm. Function discards `_arr_m`.
- Doc contract: neon.rs:83 "These instructions have wider destination than source operands." — asserted fingerprint ab17a147
- Seed: encode_neon_three_diff_narrow_rm_ta_must_match
- Formal: ∀ rd,rn,rm ∈ {0..31}, Tb ∈ {8b,16b,4h,8h,2s,4s}, Tm ≠ Tb, (U,opc,mnem) ∈ LONG_TABLE. llvm-mc(mnem Vd.Ta, Vn.Tb, Vm.Tm) is Err ∧ encode_neon_three_diff([Vd.Ta, Vn.Tb, Vm.Tm], U, opc, is_high(Tb)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_diff_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_diff([v0.8h, v0.8b, v0.16b], U=0, opc=0, is_high=false) is Ok
- Bug report: pbt-out/bug_reports/encode_neon_three_diff_rm_tb_mismatch.md

```property
function: encoder.neon.encode_neon_three_diff
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, tb, tm, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, tb: long_tb, tm: any_t_neq_tb, insn: long_table }
  relation:
    op: holds
    expr: encode_neon_three_diff([Vd.Ta, Vn.tb, Vm.tm], insn.U, insn.opc, is_high(tb)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  tm: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"] }
  insn: { gen: oneof, values: ["saddl", "uaddl"] }
evidence: neon.rs:95
```
