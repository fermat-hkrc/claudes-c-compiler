# Properties: encode_neon_bitwise_insert

## encode_neon_bitwise_insert_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree BIT/BIF decoder). Sibling encode_neon_bsl / encode_neon_bic rejected (same-job gate: different three-same opcodes). README.md:12 claims gas-compatible textual assembly; encoder/mod.rs:3 claims 32-bit AArch64 words. llvm-mc is the independent reference for that contract.
- Doc contract: neon.rs:1664 "Encodes BIT (size=10) and BIF (size=11) instructions." — asserted fingerprint 2d3adf7e
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}. encode_neon_bitwise_insert([Vd.t, Vn.t, Vm.t], size) = llvm-mc(mnem(size) " Vd.t, Vn.t, Vm.t") where mnem(0b10)=bit and mnem(0b11)=bif
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bitwise_insert
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, size]
  domain: { rd: v0..v31, rn: v0..v31, rm: v0..v31, t: {8b,16b}, size: {0b10,0b11} }
  relation:
    op: eq
    lhs: encode_neon_bitwise_insert([arr(rd,t), arr(rn,t), arr(rm,t)], size)
    rhs: llvm_mc(mnem(size) + " v" + rd + "." + t + ", v" + rn + "." + t + ", v" + rm + "." + t)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_bitwise_insert_meta_rd_rn_rm_size
- Tier: 4
- Rationale: Metamorphic isolation of Rd/Rn/Rm/size. Stronger differential is the primary property; this checks field packing independently of llvm-mc. Changing only Rd (resp. Rn, Rm, size) must XOR only bits[4:0] (resp. bits[9:5], bits[20:16], bits[23:22]).
- Doc contract: neon.rs:1666 "Format: 0 Q 1 01110 ss 1 Rm 000111 Rn Rd" — asserted fingerprint ee9c3e91
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_meta_rd_rn_rm
- Formal: ∀ rd,rd2,rn,rn2,rm,rm2 ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size,size2 ∈ {0b10,0b11}. let w = encode_neon_bitwise_insert([Vd.t,Vn.t,Vm.t], size). (w ⊕ encode([Vd2.t,Vn.t,Vm.t], size)) & ~0x1F = 0 ∧ (w ⊕ encode([Vd.t,Vn2.t,Vm.t], size)) & ~(0x1F<<5) = 0 ∧ (w ⊕ encode([Vd.t,Vn.t,Vm2.t], size)) & ~(0x1F<<16) = 0 ∧ (w ⊕ encode([Vd.t,Vn.t,Vm.t], size2)) & ~(0b11<<22) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bitwise_insert
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rd2, rn, rn2, rm, rm2, t, size, size2]
  domain: { rd,rd2,rn,rn2,rm,rm2: v0..v31, t: {8b,16b}, size,size2: {0b10,0b11} }
  relation:
    op: holds
    expr: "((sut_word(ops3(rd,rn,rm,t), size) ^ sut_word(ops3(rd2,rn,rm,t), size)) & !0x1F) == 0"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
  size2: { gen: int, min: 2, max: 3, type: u32 }
evidence: neon.rs:1666
```

## encode_neon_bitwise_insert_inv_arm_layout
- Tier: 4
- Rationale: ARM Advanced SIMD three-same layout invariant. BIT/BIF share BSL's format with size=10/11 instead of 01. Weaker than differential; pins bit fields even if llvm-mc is unavailable. Format cited at neon.rs:1666.
- Doc contract: neon.rs:1666 "Format: 0 Q 1 01110 ss 1 Rm 000111 Rn Rd" — asserted fingerprint ee9c3e91
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_inv_layout
- Formal: ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}. let w = encode_neon_bitwise_insert(...). bit31(w)=0 ∧ Q(w)=(t==16b) ∧ U(w)=1 ∧ bits[28:24]=01110 ∧ size(w)=size ∧ bit21=1 ∧ Rm=rm ∧ bits[15:10]=000111 ∧ Rn=rn ∧ Rd=rd ∧ (w8 ⊕ w16) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bitwise_insert
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, size]
  domain: { rd,rn,rm: v0..v31, t: {8b,16b}, size: {0b10,0b11} }
  relation:
    op: eq
    lhs: encode_neon_bitwise_insert([arr(rd,t), arr(rn,t), arr(rm,t)], size)
    rhs: arm_bit_bif_word(rd, rn, rm, t, size)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
evidence: neon.rs:1666
```

## encode_neon_bitwise_insert_neg_arity
- Tier: 3
- Rationale: Documented arity floor. neon.rs:1669 returns Err when operands.len() < 3. llvm-mc also rejects fewer than 3 operands. Negative/error contract; stronger oracles do not apply on the invalid-arity domain.
- Doc contract: neon.rs:1669 "bit/bif requires 3 operands" — domain-restriction fingerprint 1f2376df
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_arity
- Formal: ∀ n ∈ {0,1,2}, ∀ rd ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}. encode_neon_bitwise_insert(ops[0..n], size) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bitwise_insert
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t, size]
  domain: { n: 0..2, rd: v0..v31, t: {8b,16b}, size: {0b10,0b11} }
  relation:
    op: holds
    expr: "encode_neon_bitwise_insert(ops_prefix(n, rd, t), size).is_err()"
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
expected_error: String
evidence: neon.rs:1669
```

## encode_neon_bitwise_insert_neg_extra
- Tier: 3
- Rationale: llvm-mc/gas reject a fourth operand for BIT/BIF. README.md:12 gas-compatible assembler. The SUT only checks len < 3 (no maximum). Negative/error vs llvm-mc rejection.
- Doc contract: neon.rs:1664 "Encodes BIT (size=10) and BIF (size=11) instructions." — asserted fingerprint 2d3adf7e
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_extra
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}. llvm-mc(mnem size four-ops) is Err ⇒ encode_neon_bitwise_insert([Vd.t,Vn.t,Vm.t,Vextra.t], size) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t="8b", size=0b10 — encode_neon_bitwise_insert([v0.8b,v0.8b,v0.8b,v0.8b], 0b10) = Ok(Word(0x2ea01c00))
- Bug report: pbt-out/bug_reports/encode_neon_bitwise_insert_extra_operand.md

```property
function: encoder.encode_neon_bitwise_insert
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t, size]
  domain: { rd,rn,rm,extra: v0..v31, t: {8b,16b}, size: {0b10,0b11} }
  relation:
    op: holds
    expr: "encode_neon_bitwise_insert([arr(rd,t), arr(rn,t), arr(rm,t), arr(extra,t)], size).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_bitwise_insert_neg_invalid_t
- Tier: 3
- Rationale: ARM Advanced SIMD three-same BIT/BIF only allow T in {8B,16B}. llvm-mc rejects 4h/8h/2s/4s/2d/1d. README.md:12 gas-compatible. Negative/error vs llvm-mc.
- Doc contract: neon.rs:1664 "Encodes BIT (size=10) and BIF (size=11) instructions." — asserted fingerprint 2d3adf7e
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}, ∀ size ∈ {0b10,0b11}. llvm-mc rejects mnem Vd.t,Vn.t,Vm.t ⇒ encode_neon_bitwise_insert([Vd.t,Vn.t,Vm.t], size) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="4h", size=0b10 — encode_neon_bitwise_insert([v0.4h,v0.4h,v0.4h], 0b10) = Ok(Word) (Q=0 as if 8b)
- Bug report: pbt-out/bug_reports/encode_neon_bitwise_insert_invalid_t.md

```property
function: encoder.encode_neon_bitwise_insert
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, size]
  domain: { rd,rn,rm: v0..v31, t: invalid ARM T, size: {0b10,0b11} }
  relation:
    op: holds
    expr: "encode_neon_bitwise_insert([arr(rd,t), arr(rn,t), arr(rm,t)], size).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:224
```

## encode_neon_bitwise_insert_neg_mismatch_t
- Tier: 3
- Rationale: llvm-mc/gas require matching T on Vd, Vn, Vm. README.md:12 gas-compatible. Negative/error vs llvm-mc.
- Doc contract: neon.rs:1664 "Encodes BIT (size=10) and BIF (size=11) instructions." — asserted fingerprint 2d3adf7e
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_mismatch_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, ∀ td,tn,tm ∈ {8b,16b} not all equal, ∀ size ∈ {0b10,0b11}. llvm-mc rejects mismatched T ⇒ encode_neon_bitwise_insert([Vd.td,Vn.tn,Vm.tm], size) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td="16b", tn="8b", tm="8b", size=0b10 — encode_neon_bitwise_insert([v0.16b,v0.8b,v0.8b], 0b10) = Ok(Word) (Q from dest only)
- Bug report: pbt-out/bug_reports/encode_neon_bitwise_insert_mismatch_t.md

```property
function: encoder.encode_neon_bitwise_insert
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm, size]
  domain: { rd,rn,rm: v0..v31, td,tn,tm: {8b,16b}, size: {0b10,0b11} }
  relation:
    op: holds
    expr: "encode_neon_bitwise_insert([arr(rd,td), arr(rn,tn), arr(rm,tm)], size).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: string }
  tn: { gen: string }
  tm: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_bitwise_insert_neg_gpr_bare_sp
- Tier: 3
- Rationale: llvm-mc/gas require arranged NEON Vd.T / Vn.T / Vm.T. GPR, SP, bare V, scalar FP (d/s/q) are rejected. README.md:12 gas-compatible. Negative/error vs llvm-mc.
- Doc contract: neon.rs:1664 "Encodes BIT (size=10) and BIF (size=11) instructions." — asserted fingerprint 2d3adf7e
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_gpr_bare_sp
- Formal: ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}, ∀ kind ∈ {x-gpr, w-dest, sp, bare-v, d-scalar, s-dest, q-dest}. llvm-mc rejects that form ⇒ encode_neon_bitwise_insert(ops(kind), size) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="8b", size=0b10, kind=0 — encode_neon_bitwise_insert([x0,x0,x0], 0b10) = Ok(Word)
- Bug report: pbt-out/bug_reports/encode_neon_bitwise_insert_gpr_bare_sp.md

```property
function: encoder.encode_neon_bitwise_insert
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, size, kind]
  domain: { rd,rn,rm: v0..v31, t: {8b,16b}, size: {0b10,0b11}, kind: gpr/sp/bare/fp }
  relation:
    op: holds
    expr: "encode_neon_bitwise_insert(ops_kind(kind, rd, rn, rm, t), size).is_err()"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_bitwise_insert_diff_alt_spellings
- Tier: 5
- Rationale: Sweep — GNU as / llvm-mc accept uppercase V register names. parse_reg_num lowercases. Differential vs llvm-mc on uppercase Vd.T. Documented gas-compatible assembly (README.md:12).
- Doc contract: neon.rs:1664 "Encodes BIT (size=10) and BIF (size=11) instructions." — asserted fingerprint 2d3adf7e
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}. encode_neon_bitwise_insert([V{rd}.t, V{rn}.t, V{rm}.t], size) = llvm-mc(mnem(size) " Vd.t, Vn.t, Vm.t")
- Test file: src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bitwise_insert
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, size]
  domain: { rd: v0..v31, rn: v0..v31, rm: v0..v31, t: {8b,16b}, size: {0b10,0b11} }
  relation:
    op: eq
    lhs: encode_neon_bitwise_insert([arr_upper(rd,t), arr_upper(rn,t), arr_upper(rm,t)], size)
    rhs: llvm_mc(mnem(size) + " V" + rd + "." + t + ", V" + rn + "." + t + ", V" + rm + "." + t)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  size: { gen: int, min: 2, max: 3, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```
