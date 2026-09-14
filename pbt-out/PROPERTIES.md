# Properties: encode_neon_tbl

## encode_neon_tbl_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler of the same GNU-style TBL text). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree TBL decoder. encode_neon_tbx rejected (same-job gate: TBX / op=1). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping operands <-> `tbl Vd.Ta, {Vn.16B, ...}, Vm.Ta`.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, Ta ∈ {8b,16b}, n ∈ {1,2,3,4}. let table = [(rn+i) mod 32 | i < n]. encode_neon_tbl([Vd.Ta, {Vn.16B..}, Vm.Ta]) = Word(v) ∧ llvm-mc(-triple=aarch64, "tbl Vd.Ta, {Vn.16B, ...}, Vm.Ta") = v
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_tbl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ta: {8b,16b}, n: 1..4 }
  relation:
    op: eq
    lhs: encode_neon_tbl(ops)
    rhs: llvm_mc_word("tbl Vd.Ta, {Vn.16B, ...}, Vm.Ta")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: src/backend/arm/assembler/README.md:14 same textual assembly as gas; README.md:233 tbl under NEON permute; neon.rs:767 Encode NEON TBL; ARM ARM Advanced SIMD table lookup TBL
```

## encode_neon_tbl_metamorphic_q
- Tier: 4c
- Rationale: ARM ARM Q is bit 30 of Advanced SIMD table lookup; Ta=8B (Q=0) vs Ta=16B (Q=1) at equal Rd/Rn/Rm/len must differ only in Q. Stronger differential already used on the valid domain; this is an independent field metamorphic (required metamorphic/differential companion).
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, n ∈ {1,2,3,4}. encode(Ta=8b) XOR encode(Ta=16b) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_tbl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, n]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, n: 1..4 }
  relation:
    op: eq
    lhs: encode_neon_tbl(ops_8b) XOR encode_neon_tbl(ops_16b)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: ARM ARM Advanced SIMD table lookup Q at bit 30; neon.rs:776 q = 1 iff arr_d == 16b
```

## encode_neon_tbl_invariant_arm_fields
- Tier: 4d
- Rationale: ARM ARM field layout of TBL is an exact structural predicate on every success-path word. Stronger differential already covers value equality vs llvm-mc; this pins each field independently.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, Ta ∈ {8b,16b}, n ∈ {1,2,3,4}. word bit31=0 ∧ Q=(Ta==16b) ∧ bits[29:24]=001110 ∧ bits[23:21]=0 ∧ Rm=rm ∧ bit15=0 ∧ len=n-1 at [14:13] ∧ op=0 at 12 ∧ bits[11:10]=0 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_tbl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n]
  domain: { ta: {8b,16b}, n: 1..4 }
  relation:
    op: holds
    expr: arm_tbl_fields(word)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: ARM ARM Advanced SIMD table lookup TBL 0 Q 00 1110 00 0 Rm 0 len 0 00 Rn Rd; neon.rs:792-796
```

## encode_neon_tbl_metamorphic_len
- Tier: 4c
- Rationale: ARM ARM len occupies bits [14:13] as nregs-1; changing only the table length at equal Rd/Rn/Rm/Ta must differ only in those two bits. Documented bound nregs in {1,2,3,4} is pinned exactly.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, Ta ∈ {8b,16b}, n1, n2 ∈ {1,2,3,4}. (encode(n1) XOR encode(n2)) & ~(0b11 << 13) = 0 ∧ ((encode(n1) >> 13) & 0b11) = n1-1 ∧ ((encode(n2) >> 13) & 0b11) = n2-1
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_tbl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n1, n2]
  domain: { n1: 1..4, n2: 1..4 }
  relation:
    op: holds
    expr: (w1 XOR w2) & ~(0b11 << 13) == 0
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n1: { gen: int, min: 1, max: 4, type: u32 }
  n2: { gen: int, min: 1, max: 4, type: u32 }
evidence: ARM ARM len at bits [14:13]; neon.rs:791-792 len field 1 reg -> 00, 2 -> 01, 3 -> 10, 4 -> 11
```

## encode_neon_tbl_neg_extra_operand
- Tier: 4e
- Rationale: GNU-style TBL takes exactly three operands (Vd.Ta, register list, Vm.Ta). llvm-mc rejects a fourth operand. Documented arity at neon.rs:772-773 (requires 3 operands). Extra operands must Err, not be ignored.
- Seed: (none)
- Formal: ∀ rd, rn, rm, extra ∈ {0..31}, Ta ∈ {8b,16b}, n ∈ {1,2,3,4}. encode_neon_tbl([Vd.Ta, list, Vm.Ta, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, ta="8b", n=1 — tbl v0.8b, {v0.16b}, v0.8b, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_tbl_extra_operand.md

```property
function: encode_neon_tbl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, ta, n]
  domain: { extra: extra_operand }
  relation:
    op: throws
    expr: encode_neon_tbl(ops_plus_extra)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
expected_error: String
evidence: neon.rs:772-773 tbl requires 3 operands; llvm-mc rejects a fourth operand
```

## encode_neon_tbl_neg_invalid_ta
- Tier: 4e
- Rationale: ARM ARM Ta is only 8B or 16B. llvm-mc rejects 4h/8h/2s/4s/2d/1d. Documented T in neon.rs:769-771 / ARM ARM. Invalid Ta must Err, not encode Q=0.
- Seed: (none)
- Formal: ∀ rd, rn, rm ∈ {0..31}, Ta ∉ {8b,16b} ∧ Ta ∈ {4h,8h,2s,4s,2d,1d,8h,4s}, n ∈ {1,2,3,4}. encode_neon_tbl([Vd.Ta, {Vn.16B..}, Vm.Ta]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ta="4h", n=1 — tbl v0.4h, {v0.16b}, v0.4h
- Bug report: pbt-out/bug_reports/encode_neon_tbl_invalid_ta.md

```property
function: encode_neon_tbl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n]
  domain: { ta: {4h,8h,2s,4s,2d,1d} }
  relation:
    op: throws
    expr: encode_neon_tbl(ops)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["4h", "8h", "2s", "4s", "2d", "1d"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
expected_error: String
evidence: ARM ARM Ta in {8B,16B}; llvm-mc rejects other arrangements; neon.rs:776 Q only for 16b
```

## encode_neon_tbl_neg_table_contract
- Tier: 4e
- Rationale: ARM ARM / llvm-mc require the table to be 1–4 consecutive `.16B` registers (wrapping at 31). Non-16B arrangement, non-sequential numbers, 0 registers, and 5+ registers must Err (llvm-mc: invalid operand / registers must be sequential / invalid number of vectors). Documented len 1..4 at neon.rs:791.
- Seed: (none)
- Formal: ∀ invalid table list L ∈ {empty, n=5..8, non-16B arr, non-consecutive numbers}. encode_neon_tbl([Vd.8b, L, Vm.8b]) = Err (must not panic)
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, kind=0 — Operand::RegList(vec![]) panics at regs[0]. Related witnesses: n=5 wraps len; {v0.16b,v2.16b} encodes as sequential; {v0.8b} table accepted.
- Bug report: pbt-out/bug_reports/encode_neon_tbl_empty_list_panic.md

```property
function: encode_neon_tbl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rm, list]
  domain: { list: invalid_tbl_reglist }
  relation:
    op: throws
    expr: encode_neon_tbl([Vd.8b, list, Vm.8b])
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM table is 1-4 consecutive .16B; llvm-mc rejects empty / 5+ / non-16b / non-sequential; neon.rs:791 len 1..4
```

## encode_neon_tbl_neg_arity_kinds
- Tier: 4e
- Rationale: TBL requires three operands: Vd.Ta (NEON arrangement), a register list, Vm.Ta matching Vd. Fewer than 3 operands, missing RegList, GPR/FP dest, mismatched Vd/Vm T, and invalid dest names are rejected by llvm-mc and must Err.
- Seed: (none)
- Formal: ∀ arity < 3 ∨ dest ∈ GPR/FP/invalid ∨ second operand not RegList ∨ Vd.Ta ≠ Vm.Ta. encode_neon_tbl(ops) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=0, prefix="x", dest_n=0 — tbl x0, {v0.16b}, v0.8b encodes. Related witness: tbl v0.8b, {v0.16b}, v0.16b encodes (Vm arrangement ignored). Too-few operands, missing list, and invalid dest names correctly Err.
- Bug report: pbt-out/bug_reports/encode_neon_tbl_gpr_dest.md

```property
function: encode_neon_tbl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, dest, kind]
  domain: { n: 0..2, dest: gpr_fp_invalid, kind: non_reglist }
  relation:
    op: throws
    expr: encode_neon_tbl(ops)
    error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  dest_n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:772-773 / 787-788; llvm-mc rejects too few operands, missing list, GPR dest, mismatched T
```

## encode_neon_tbl_neg_list_and_vm_kinds
- Tier: 4e
- Rationale: Coverage-sweep (manual arm audit; coverage_gaps had no profraw). Documented branches at neon.rs:780-784 (list[0] must be a register) and get_neon_reg on Vm; ARM ARM table is Vn.16B and Vm.Ta. llvm-mc rejects `{v1}` (no arrangement), `{x1}`, and a GPR/bare Vm.
- Seed: (none)
- Formal: ∀ list[0] ∈ {Reg(vN), Imm, invalid name} ∨ Vm ∈ {Reg(prefixN), invalid name}. encode_neon_tbl([Vd.8b, list, Vm]) = Err (must not panic)
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, kind=0 — RegList([Reg("v0")]) encodes as Word(0x0e000000). Related witness: Vm=Reg("x0") encodes (get_neon_reg accepts Operand::Reg). Imm-in-list and invalid names correctly Err.
- Bug report: pbt-out/bug_reports/encode_neon_tbl_bare_list_reg.md

```property
function: encode_neon_tbl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind]
  domain: { kind: {bare_list_reg, imm_in_list, bad_list_name, bare_vm, bad_vm_name} }
  relation:
    op: throws
    expr: encode_neon_tbl(ops)
    error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:780-784 expected register in list; ARM ARM table .16B and Vm.Ta; llvm-mc rejects {v1} / GPR Vm
```
