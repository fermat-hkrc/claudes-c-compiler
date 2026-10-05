# Properties: encode_cnt

## encode_cnt_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree CNT decoder). Sibling encode_neon_not / encode_neon_rbit rejected (same-job gate: different two-misc opcodes). README.md:12 claims gas-compatible textual assembly; llvm-mc provides the known-answer encoding. Weaker: metamorphic Rd/Rn, ARM-field invariant, negative_error.
- Doc contract: neon.rs:24 "CNT Vd.<T>, Vn.<T>" — asserted fingerprint 6b96d8c8
- Seed: neon.rs encode_neon_rbit KAT (same 8b/16b two-misc shape)
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b}. encode_cnt([Vd.T, Vn.T]) = llvm-mc("cnt Vd.T, Vn.T")
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_cnt
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_cnt([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t)])
    rhs: llvm_mc("cnt v{rd}.{t}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: assembler/README.md:12; assembler/README.md:225; encoder/mod.rs:517; neon.rs:24-26
```

## encode_cnt_meta_rd_rn
- Tier: 3
- Rationale: Metamorphic field isolation: changing only Rd (resp. Rn) must differ only in bits[4:0] (resp. bits[9:5]). Stronger differential covers the full word; this pins the ARM register-field placement independently of llvm-mc availability on a given sample. State machine / round-trip rejected as above.
- Doc contract: neon.rs:25 "Encoding: 0 Q 00 1110 size 10 0000 0101 10 Rn Rd" — asserted fingerprint 43332371
- Seed: encode_neon_mvni_pbt.rs Rd-field metamorphic
- Formal: ∀ rd1, rd2, rn1, rn2 ∈ {0..31}, T ∈ {8b,16b}. (encode_cnt(rd1,rn1,T) ⊕ encode_cnt(rd2,rn1,T)) & ~0x1F = 0 ∧ bits[4:0] equal rd. (encode_cnt(rd1,rn1,T) ⊕ encode_cnt(rd1,rn2,T)) & ~(0x1F<<5) = 0 ∧ bits[9:5] equal rn.
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_cnt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, t]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, t: {8b,16b} }
  relation:
    op: holds
    expr: ((encode_cnt(rd1,rn1,t) xor encode_cnt(rd2,rn1,t)) & ~0x1F) == 0 && ((encode_cnt(rd1,rn1,t) xor encode_cnt(rd1,rn2,t)) & ~(0x1F<<5)) == 0
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:25 ARM two-misc layout; ARM ARM CNT
```

## encode_cnt_inv_layout
- Tier: 3
- Rationale: Algebraic invariant from ARM Advanced SIMD two-register miscellaneous CNT: bit31=0, Q at bit30 = 1 iff T=16b, bits[29:24]=001110, size bits[23:22]=00, bits[21:16]=100000, bits[15:10]=010110, Rn at [9:5], Rd at [4:0]. Stronger differential already compares the whole word; this names the ARM fields.
- Doc contract: neon.rs:25 "Encoding: 0 Q 00 1110 size 10 0000 0101 10 Rn Rd" — asserted fingerprint 43332371
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b}. let w = encode_cnt([Vd.T,Vn.T]). w[31]=0 ∧ w[30]=(T=16b) ∧ w[29:24]=0b001110 ∧ w[23:22]=0 ∧ w[21:16]=0b100000 ∧ w[15:10]=0b010110 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_cnt
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_cnt([Vd.T, Vn.T])
    rhs: 0x0e205800 | (Q<<30) | (rn<<5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:25; ARM ARM CNT two-register miscellaneous
```

## encode_cnt_neg_arity
- Tier: 4
- Rationale: Documented error contract: neon.rs:28 "cnt requires 2 operands". llvm-mc and gas reject too-few-operands. Domain is arity 0 and 1.
- Doc contract: neon.rs:28 "cnt requires 2 operands" — domain-restriction fingerprint 5189a0f0
- Seed: encode_neon_mvni_pbt.rs arity negative
- Formal: ∀ ops. |ops| < 2 ⇒ encode_cnt(ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_cnt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: {0,1} }
  relation:
    op: throws
    expr: encode_cnt(ops_of_len(n))
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: neon.rs:28
```

## encode_cnt_neg_extra
- Tier: 4
- Rationale: gas/llvm-mc reject a third operand (`unexpected characters following instruction` / `invalid operand`). README.md:12 claims gas-compatible assembly. encode_cnt checks `operands.len() < 2` only, so extra is accepted — keep in domain.
- Doc contract: neon.rs:24 "CNT Vd.<T>, Vn.<T>" — asserted fingerprint 6b96d8c8
- Seed: encode_neon_mvni_pbt.rs extra-operand negative
- Formal: ∀ rd, rn, extra ∈ {0..31}, T ∈ {8b,16b}. llvm-mc("cnt Vd.T, Vn.T, Vextra.T") = Err ∧ encode_cnt([Vd.T, Vn.T, Vextra.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b" — encode_cnt([v0.8b, v0.8b, v0.8b]) = Ok(Word(0x0e205800))
- Bug report: pbt-out/bug_reports/encode_cnt_extra_operand.md

```property
function: encoder.encode_cnt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t]
  domain: { rd: u32_0_31, rn: u32_0_31, extra: u32_0_31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_cnt([Vd.T, Vn.T, Vextra.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: assembler/README.md:12; llvm-mc/gas reject third operand
```

## encode_cnt_neg_invalid_t
- Tier: 4
- Rationale: neon.rs:26 asserts "Only valid for .8b (Q=0) and .16b (Q=1)". ARM/llvm-mc/gas reject T in {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}. The comment is an asserted contract the code violates — keep illegal T in the generator.
- Doc contract: neon.rs:26 "Only valid for .8b (Q=0) and .16b (Q=1)" — asserted fingerprint cdd2a4c1
- Seed: encode_neon_rbit arrangement check; encode_neon_mvni invalid T
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}. llvm-mc("cnt Vd.T, Vn.T") = Err ∧ encode_cnt([Vd.T, Vn.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="4h" — encode_cnt([v0.4h, v0.4h]) = Ok(Word(0x0e205800))
- Bug report: pbt-out/bug_reports/encode_cnt_invalid_t.md

```property
function: encoder.encode_cnt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s} }
  relation:
    op: throws
    expr: encode_cnt([Vd.T, Vn.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["4h", "8h", "2s", "4s", "2d", "1d", "4b", "8d", "2h", "1s"] }
expected_error: String
evidence: neon.rs:26; ARM ARM CNT T in {8B,16B}; llvm-mc/gas reject
```

## encode_cnt_neg_mismatch_t
- Tier: 4
- Rationale: ARM/llvm-mc/gas require matching T on Vd and Vn (`operand mismatch`). The comment neon.rs:24 "CNT Vd.<T>, Vn.<T>" names the same T. Source arrangement is ignored (`_arr_n`).
- Doc contract: neon.rs:24 "CNT Vd.<T>, Vn.<T>" — asserted fingerprint 6b96d8c8
- Seed: encode_neon_rbit mismatch arrangement
- Formal: ∀ rd, rn ∈ {0..31}, Td ≠ Tn, {Td,Tn} ⊆ {8b,16b}. llvm-mc("cnt Vd.Td, Vn.Tn") = Err ∧ encode_cnt([Vd.Td, Vn.Tn]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, td="8b" — encode_cnt([v0.8b, v0.16b]) = Ok(Word(0x0e205800))
- Bug report: pbt-out/bug_reports/encode_cnt_mismatch_t.md

```property
function: encoder.encode_cnt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn]
  domain: { rd: u32_0_31, rn: u32_0_31, td: {8b,16b}, tn: {8b,16b}, td != tn }
  relation:
    op: throws
    expr: encode_cnt([Vd.Td, Vn.Tn])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, items: ["8b", "16b"] }
  tn: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:24; llvm-mc/gas operand mismatch
```

## encode_cnt_neg_gpr_bare_sp
- Tier: 4
- Rationale: gas/llvm-mc reject GPR dest/src, SP, bare V (no arrangement), and FP scalar d/s/q. README.md:12 claims gas-compatible assembly. get_neon_reg accepts Operand::Reg via parse_reg_num (x/w/d/s/q/v/h/b/sp).
- Doc contract: neon.rs:24 "CNT Vd.<T>, Vn.<T>" — asserted fingerprint 6b96d8c8
- Seed: encode_neon_rbit_bare_src / encode_neon_aes_sp_as_neon
- Formal: ∀ kind ∈ {gpr-x, gpr-w, sp, bare-v, fp-d, fp-s, fp-q}, rd, rn ∈ {0..31}, T ∈ {8b,16b}. llvm-mc(asm(kind)) = Err ∧ encode_cnt(ops(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", kind=0 — encode_cnt([Reg("x0"), Reg("x0")]) = Ok(Word(0x0e205800))
- Bug report: pbt-out/bug_reports/encode_cnt_gpr_bare_sp.md

```property
function: encoder.encode_cnt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, rd, rn, t]
  domain: { kind: {gpr-x, gpr-w, sp, bare-v, fp-d, fp-s, fp-q}, rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_cnt(ops(kind, rd, rn, t))
generators:
  kind: { gen: int, min: 0, max: 6, type: u8 }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: assembler/README.md:12; llvm-mc/gas reject non-NEON-arranged operands
```

## encode_cnt_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — llvm-mc accepts uppercase `V` register prefix; parse_reg_num lowercases. Differential vs llvm-mc on uppercase V with valid T. Arrangement stays lowercase (parser lowercases T before encode_cnt).
- Doc contract: neon.rs:24 "CNT Vd.<T>, Vn.<T>" — asserted fingerprint 6b96d8c8
- Seed: encode_neon_mvni / encode_neon_ext alt-spellings
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b}. encode_cnt([RegArrangement("V{rd}", T), RegArrangement("V{rn}", T)]) = llvm-mc("cnt V{rd}.T, V{rn}.T")
- Test file: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_cnt
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_cnt([RegArrangement(V{rd}, t), RegArrangement(V{rn}, t)])
    rhs: llvm_mc("cnt V{rd}.{t}, V{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: assembler/README.md:12; parse_reg_num lowercases; llvm-mc accepts V prefix
```
