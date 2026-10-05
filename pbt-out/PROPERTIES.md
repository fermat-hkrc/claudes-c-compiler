# Properties: encode_neon_not

## encode_neon_not_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree NOT decoder). Sibling encode_cnt / encode_neon_rbit rejected (same-job gate: different two-misc opcodes). encode_mvn rejected (calls encode_neon_not for vector form; not independent). README.md:12 claims gas-compatible textual assembly; llvm-mc provides the known-answer encoding. Weaker: metamorphic Rd/Rn, ARM-field invariant, negative_error.
- Doc contract: neon.rs:607 "Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T" — asserted fingerprint 806b7b7e
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:171 encode_cnt_diff_llvm_mc
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b}. encode_neon_not([Vd.T, Vn.T]) = llvm-mc("not Vd.T, Vn.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_not
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_not([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t)])
    rhs: llvm_mc("not v{rd}.{t}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:607; assembler/README.md:12; assembler/README.md:225; encoder/mod.rs:692
```

## encode_neon_not_meta_rd_rn
- Tier: 3
- Rationale: Metamorphic field isolation: changing only Rd (resp. Rn) must differ only in bits[4:0] (resp. bits[9:5]). Stronger differential covers the full word; this pins the ARM register-field placement independently of llvm-mc availability on a given sample. State machine / round-trip rejected as above.
- Doc contract: neon.rs:617 "NOT Vd.T, Vn.T (alias of MVN): 0 Q 1 01110 00 10000 00101 10 Rn Rd" — asserted fingerprint 3f129cd4
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:182 encode_cnt_meta_rd_rn
- Formal: ∀ rd1, rd2, rn1, rn2 ∈ {0..31}, T ∈ {8b,16b}. (encode_neon_not(rd1,rn1,T) ⊕ encode_neon_not(rd2,rn1,T)) & ~0x1F = 0 ∧ bits[4:0] equal rd. (encode_neon_not(rd1,rn1,T) ⊕ encode_neon_not(rd1,rn2,T)) & ~(0x1F<<5) = 0 ∧ bits[9:5] equal rn.
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_not
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, t]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, t: {8b,16b} }
  relation:
    op: holds
    expr: ((encode_neon_not(rd1,rn1,t) xor encode_neon_not(rd2,rn1,t)) & ~0x1F) == 0 && ((encode_neon_not(rd1,rn1,t) xor encode_neon_not(rd1,rn2,t)) & ~(0x1F<<5)) == 0
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:617 ARM two-misc layout; ARM ARM NOT
```

## encode_neon_not_inv_layout
- Tier: 3
- Rationale: Algebraic invariant from ARM Advanced SIMD two-register miscellaneous NOT: bit31=0, Q at bit30 = 1 iff T=16b, bits[29:24]=101110, size bits[23:22]=00, bits[21:16]=100000, bits[15:10]=010110, Rn at [9:5], Rd at [4:0]. Stronger differential already compares the whole word; this names the ARM fields. Q-bit metamorphic (8b vs 16b differs only at bit 30) is included.
- Doc contract: neon.rs:617 "NOT Vd.T, Vn.T (alias of MVN): 0 Q 1 01110 00 10000 00101 10 Rn Rd" — asserted fingerprint 3f129cd4
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:209 encode_cnt_inv_layout
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b}. let w = encode_neon_not([Vd.T,Vn.T]). w[31]=0 ∧ w[30]=(T=16b) ∧ w[29:24]=0b101110 ∧ w[23:22]=0 ∧ w[21:16]=0b100000 ∧ w[15:10]=0b010110 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_not
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_not([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t)])
    rhs: 0x2e205800 | ((t==16b)<<30) | (rn<<5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:617; ARM ARM Advanced SIMD two-register miscellaneous NOT
```

## encode_neon_not_neg_arity
- Tier: 4
- Rationale: Documented arity contract: neon.rs:610 returns Err("not requires 2 operands") when operands.len() < 2. Negative/error contract. Stronger oracles do not apply to the empty/short domain.
- Doc contract: neon.rs:610 "not requires 2 operands" — domain-restriction fingerprint 97a6cc10
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:228 encode_cnt_neg_arity
- Formal: ∀ n ∈ {0,1}, ops with |ops|=n. encode_neon_not(ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t]
  domain: { n: {0,1}, rd: u32_0_31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_not(ops_of_len(n))
    error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:609-610
```

## encode_neon_not_neg_extra
- Tier: 4
- Rationale: llvm-mc and gas reject a third operand (`not v0.8b, v1.8b, v2.8b`). neon.rs:607 asserts NOT Vd.T, Vn.T (two operands). README.md:12 claims gas-compatible assembly, so extra operands must Err. The SUT only checks len < 2. Negative/error vs independent assemblers.
- Doc contract: neon.rs:607 "Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T" — asserted fingerprint 806b7b7e
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:241 encode_cnt_neg_extra
- Formal: ∀ rd, rn, extra ∈ {0..31}, T ∈ {8b,16b}. llvm-mc rejects "not Vd.T, Vn.T, Vextra.T" ⇒ encode_neon_not([Vd.T, Vn.T, Vextra.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: failing
- Counterexample: encode_neon_not([v0.8b, v0.8b, v0.8b]) → Ok(Word) instead of Err
- Bug report: bug_reports/encode_neon_not_extra_operand.md

```property
function: encoder.encode_neon_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t]
  domain: { rd: u32_0_31, rn: u32_0_31, extra: u32_0_31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_not([Vd.T, Vn.T, Vextra.T])
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:607; neon.rs:609
```

## encode_neon_not_neg_invalid_t
- Tier: 4
- Rationale: ARM / README.md:225 / llvm-mc / gas restrict NOT to T in {8B,16B}. llvm-mc and gas reject .4h/.8h/.2s/.4s/.2d/etc. The function comment does not declare those T invalid; it asserts NOT Vd.T, Vn.T and the independent assemblers define the domain. Negative/error.
- Doc contract: neon.rs:607 "Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T" — asserted fingerprint 806b7b7e
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:261 encode_cnt_neg_invalid_t
- Formal: ∀ rd, rn ∈ {0..31}, T ∉ {8b,16b}. llvm-mc rejects "not Vd.T, Vn.T" ⇒ encode_neon_not([Vd.T, Vn.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: failing
- Counterexample: encode_neon_not([v0.4h, v0.4h]) → Ok(Word) instead of Err
- Bug report: bug_reports/encode_neon_not_invalid_t.md

```property
function: encoder.encode_neon_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s} }
  relation:
    op: throws
    expr: encode_neon_not([Vd.T, Vn.T])
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["4h", "8h", "2s", "4s", "2d", "1d", "4b", "8d", "2h", "1s"] }
expected_error: String
evidence: neon.rs:607; neon.rs:615
```

## encode_neon_not_neg_mismatch_t
- Tier: 4
- Rationale: llvm-mc and gas reject mismatched arrangements (`not v0.8b, v1.16b`). neon.rs:607 asserts NOT Vd.T, Vn.T (same T). Negative/error.
- Doc contract: neon.rs:607 "Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T" — asserted fingerprint 806b7b7e
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:281 encode_cnt_neg_mismatch_t
- Formal: ∀ rd, rn ∈ {0..31}, Td ≠ Tn ∈ {8b,16b}. llvm-mc rejects "not Vd.Td, Vn.Tn" ⇒ encode_neon_not([Vd.Td, Vn.Tn]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: failing
- Counterexample: encode_neon_not([v0.8b, v0.16b]) → Ok(Word) instead of Err
- Bug report: bug_reports/encode_neon_not_mismatch_t.md

```property
function: encoder.encode_neon_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td]
  domain: { rd: u32_0_31, rn: u32_0_31, td: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_not([Vd.td, Vn.other(td)])
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:607; neon.rs:613
```

## encode_neon_not_neg_gpr_bare_sp
- Tier: 4
- Rationale: llvm-mc/gas reject GPR, SP, bare V (no arrangement), and scalar FP/SIMD (d/s/q) as NOT operands. neon.rs:607 asserts NOT Vd.T, Vn.T. get_neon_reg accepts Operand::Reg via parse_reg_num. Negative/error vs independent assemblers.
- Doc contract: neon.rs:607 "Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T" — asserted fingerprint 806b7b7e
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs:301 encode_cnt_neg_gpr_bare_sp
- Formal: ∀ kind ∈ {x-gpr, w-gpr, sp, bare-v, d-fp, s-fp, q-fp}. llvm-mc rejects the corresponding `not` ⇒ encode_neon_not(ops(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: failing
- Counterexample: encode_neon_not([Reg("x0"), Reg("x0")]) → Ok(Word) instead of Err
- Bug report: bug_reports/encode_neon_not_gpr_bare_sp.md

```property
function: encoder.encode_neon_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, kind]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b}, kind: {x,w,sp,bare_v,d,s,q} }
  relation:
    op: throws
    expr: encode_neon_not(ops(kind, rd, rn, t))
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  kind: { gen: int, min: 0, max: 6, type: u8 }
expected_error: String
evidence: neon.rs:607; neon.rs:612
```

## encode_neon_not_diff_alt_spellings
- Tier: 2
- Rationale: Sweep / strengthening: parse_reg_num lowercases V/X prefixes, so uppercase Vd.T must match llvm-mc. Differential vs llvm-mc on the same valid domain with uppercase V spelling.
- Doc contract: neon.rs:607 "Encode NEON NOT (bitwise NOT): NOT Vd.T, Vn.T" — asserted fingerprint 806b7b7e
- Seed: src/backend/arm/assembler/encoder/encode_cnt_pbt.rs encode_cnt_diff_alt_spellings
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b}. encode_neon_not([V{rd}.T, V{rn}.T]) = llvm-mc("not V{rd}.T, V{rn}.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_not
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: u32_0_31, rn: u32_0_31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_not([RegArrangement(V{rd}, t), RegArrangement(V{rn}, t)])
    rhs: llvm_mc("not V{rd}.{t}, V{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:607; encoder/mod.rs:152 parse_reg_num lowercases
```
