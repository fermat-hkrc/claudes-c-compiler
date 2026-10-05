# Properties: encode_neon_tbx

## encode_neon_tbx_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, which the assembler README claims gas-compatible encodings. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree TBX decoder). Sibling encode_neon_tbl rejected by same-job gate (TBL op=0 vs TBX op=1).
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:6847 encode_neon_tbl_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, ta ∈ {8b,16b}, n ∈ {1,2,3,4}. encode_neon_tbx([Vd.ta, {Vn.16b..Vn+n-1.16b wrap}, Vm.ta]) = llvm-mc("tbx Vd.ta, {Vn.16b, ...}, Vm.ta")
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_tbx
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, ta: {8b,16b}, n: 1_4 }
  relation:
    op: eq
    lhs: encode_neon_tbx(valid_ops(rd, ta, rn, n, rm))
    rhs: llvm_mc_word(tbx_asm(rd, ta, rn, n, rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_metamorphic_q
- Tier: 4
- Rationale: ARM Q bit is 1 iff Ta=16B; 8B vs 16B at equal Rd/Rn/Rm/len must XOR only bit 30.
- Doc contract: neon.rs:824 "TBX: 0 Q 00 1110 000 Rm 0 len 1 00 Rn Rd (op=1 for TBX vs op=0 for TBL)" — asserted fingerprint 9de9df1f
- Seed: neon.rs:6864 encode_neon_tbl_metamorphic_q
- Formal: ∀ rd,rn,rm ∈ {0..31}, n ∈ {1,2,3,4}. encode_neon_tbx(8b) XOR encode_neon_tbx(16b) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_tbx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, n]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, n: 1_4 }
  relation:
    op: eq
    lhs: encode_neon_tbx(valid_ops(rd, "8b", rn, n, rm)) XOR encode_neon_tbx(valid_ops(rd, "16b", rn, n, rm))
    rhs: 1u32 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: neon.rs:824
```

## encode_neon_tbx_invariant_arm_fields
- Tier: 4
- Rationale: ARM Advanced SIMD table lookup TBX field layout; pins op=1 (TBX vs TBL).
- Doc contract: neon.rs:824 "TBX: 0 Q 00 1110 000 Rm 0 len 1 00 Rn Rd (op=1 for TBX vs op=0 for TBL)" — asserted fingerprint 9de9df1f
- Seed: neon.rs:6880 encode_neon_tbl_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, ta ∈ {8b,16b}, n ∈ {1,2,3,4}. let w = encode_neon_tbx(...). w[31]=0 ∧ w[30]=Q(ta) ∧ w[29:24]=001110 ∧ w[23:21]=000 ∧ w[20:16]=rm ∧ w[15]=0 ∧ w[14:13]=n-1 ∧ w[12]=1 ∧ w[11:10]=00 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_tbx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, ta: {8b,16b}, n: 1_4 }
  body: bit31(w)=0 AND Q(w)=Q(ta) AND bits[29:24]=0b001110 AND bits[23:21]=0 AND Rm=rm AND bit15=0 AND len=n-1 AND op=1 AND bits[11:10]=0 AND Rn=rn AND Rd=rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: neon.rs:824
```

## encode_neon_tbx_metamorphic_len
- Tier: 4
- Rationale: Changing only table length must differ only in len bits [14:13]; len = nregs-1 for n in {1,2,3,4}.
- Doc contract: neon.rs:824 "TBX: 0 Q 00 1110 000 Rm 0 len 1 00 Rn Rd (op=1 for TBX vs op=0 for TBL)" — asserted fingerprint 9de9df1f
- Seed: neon.rs:6904 encode_neon_tbl_metamorphic_len
- Formal: ∀ rd,rn,rm ∈ {0..31}, ta ∈ {8b,16b}, n1,n2 ∈ {1,2,3,4}. (w(n1) XOR w(n2)) & ~(0b11<<13) = 0 ∧ w(n).len = n-1
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_tbx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n1, n2]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, ta: {8b,16b}, n1: 1_4, n2: 1_4 }
  body: ((encode_neon_tbx(n1) ^ encode_neon_tbx(n2)) & !(0b11u32 << 13)) == 0 && ((encode_neon_tbx(n1) >> 13) & 0b11) == n1 - 1
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n1: { gen: int, min: 1, max: 4, type: u32 }
  n2: { gen: int, min: 1, max: 4, type: u32 }
evidence: neon.rs:824
```

## encode_neon_tbx_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a fourth operand; README claims gas-compatible assembly so the encoder must Err.
- Doc contract: neon.rs:804 "tbx requires 3 operands" — asserted fingerprint c183c7b5
- Seed: neon.rs:6926 encode_neon_tbl_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, ta ∈ {8b,16b}, n ∈ {1,2,3,4}. llvm-mc rejects 4-operand tbx ⇒ encode_neon_tbx(ops++[Vextra.ta]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, {v0.16b}, v0.8b, v0.8b]) = Ok(Word(0x0e001000))
- Bug report: bug_reports/encode_neon_tbx_extra_operand.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, ta, n]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, extra: v0_31, ta: {8b,16b}, n: 1_4 }
  relation:
    op: throws
    expr: encode_neon_tbx(valid_ops(rd, ta, rn, n, rm) ++ [Vextra.ta])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["8b", "16b"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_invalid_ta
- Tier: 3
- Rationale: ARM TBX Ta is only 8B/16B; llvm-mc rejects 4h/8h/2s/4s/2d/1d.
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:6950 encode_neon_tbl_neg_invalid_ta
- Formal: ∀ rd,rn,rm ∈ {0..31}, ta ∈ {4h,8h,2s,4s,2d,1d}, n ∈ {1,2,3,4}. llvm-mc rejects tbx Vd.ta ⇒ encode_neon_tbx is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.4h, {v0.16b}, v0.4h]) = Ok(Word(0x0e001000))
- Bug report: bug_reports/encode_neon_tbx_invalid_ta.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ta, n]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, ta: {4h,8h,2s,4s,2d,1d}, n: 1_4 }
  relation:
    op: throws
    expr: encode_neon_tbx(valid_ops(rd, ta, rn, n, rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, items: ["4h", "8h", "2s", "4s", "2d", "1d"] }
  n: { gen: int, min: 1, max: 4, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_table_contract
- Tier: 3
- Rationale: ARM table is 1-4 consecutive wrapping .16B registers. Empty list, n>4, non-sequential names, and table arrangement other than .16B are rejected by llvm-mc / ARM.
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:6973 encode_neon_tbl_neg_table_contract
- Formal: ∀ invalid table (empty | n∈{5..8} | non-sequential pair | arrangement ∉ {16b}). encode_neon_tbx is Err (not panic, not Ok)
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, RegList([]), v0.8b]) panics at neon.rs:812 regs[0]
- Bug report: bug_reports/encode_neon_tbx_empty_list_panic.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, n_hi, gap, bad_arr, kind]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, n_hi: 5_8, gap: 2_16, bad_arr: {8b,4h,8h,2s,4s,2d}, kind: 0_3 }
  relation:
    op: throws
    expr: encode_neon_tbx(invalid_table(kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  n_hi: { gen: int, min: 5, max: 8, type: u32 }
  gap: { gen: int, min: 2, max: 16, type: u32 }
  bad_arr: { gen: oneof, items: ["8b", "4h", "8h", "2s", "4s", "2d"] }
  kind: { gen: int, min: 0, max: 3, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_arity_kinds
- Tier: 3
- Rationale: Fewer than 3 operands must Err. GPR/FP dest, invalid names, missing RegList, and mismatched Vd.Ta vs Vm.Ta are rejected by llvm-mc.
- Doc contract: neon.rs:804 "tbx requires 3 operands" — asserted fingerprint c183c7b5
- Seed: neon.rs:7032 encode_neon_tbl_neg_arity_kinds
- Formal: ∀ n∈{0,1,2}. encode_neon_tbx(ops[..n]) is Err. ∀ dest prefix in {x,w,d,s,q,h,b}. encode_neon_tbx([Reg(dest), list, Vm.8b]) is Err. ∀ bad name. encode_neon_tbx is Err. Missing RegList is Err. Vd.8b vs Vm.16b is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([Reg("x0"), {v0.16b}, v0.8b]) = Ok(Word(0x0e001000))
- Bug report: bug_reports/encode_neon_tbx_gpr_dest.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, prefix, dest_n, bad, ta]
  domain: { n: 0_2, rd: v0_31, rn: v0_31, rm: v0_31, prefix: {x,w,d,s,q,h,b}, dest_n: v0_31, bad: {v32,v99,foo,empty,v,v-1}, ta: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_tbx(bad_arity_or_kind)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: oneof, items: ["x", "w", "d", "s", "q", "h", "b"] }
  dest_n: { gen: int, min: 0, max: 31, type: u32 }
  bad: { gen: oneof, items: ["v32", "v99", "foo", "", "v", "v-1"] }
  ta: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:804
```

## encode_neon_tbx_neg_list_and_vm_kinds
- Tier: 3
- Rationale: Coverage sweep of documented list/Vm contracts. list[0] must be Vn.16B; Vm must be Vm.Ta. llvm-mc rejects bare Reg / Imm / GPR / bad names.
- Doc contract: neon.rs:815 "tbx: expected register in list" — asserted fingerprint (none — inline error string)
- Seed: neon.rs:7119 encode_neon_tbl_neg_list_and_vm_kinds
- Formal: ∀ kind∈{bare list Reg, list Imm, bad list name, bare Vm Reg, bad Vm name}. encode_neon_tbx is Err (not panic, not Ok)
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, {v0.16b}, Reg("x0")]) = Ok(Word(0x0e001000))
- Bug report: bug_reports/encode_neon_tbx_bare_vm.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, prefix, n, bad, kind]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, prefix: {x,w,d,s,q,h,b,v}, n: v0_31, bad: {v32,foo,empty,v}, kind: 0_4 }
  relation:
    op: throws
    expr: encode_neon_tbx(list_or_vm_kind)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: oneof, items: ["x", "w", "d", "s", "q", "h", "b", "v"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  bad: { gen: oneof, items: ["v32", "foo", "", "v"] }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_diff_alt_spellings
- Tier: 5
- Rationale: Coverage sweep: llvm-mc accepts uppercase V/T spellings; README claims gas-compatible assembly so the lowercase SUT encoding must match.
- Doc contract: src/backend/arm/assembler/README.md:12 "accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint 950aac70
- Seed: encode_neon_ld_st_multi_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, n ∈ {1,2,3,4}. encode_neon_tbx(lowercase ops) = llvm-mc("tbx Vd.8B, {Vn.16B, ...}, Vm.8B")
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_tbx
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, n]
  domain: { rd: v0_31, rn: v0_31, rm: v0_31, n: 1_4 }
  relation:
    op: eq
    lhs: encode_neon_tbx(valid_ops(rd, "8b", rn, n, rm))
    rhs: llvm_mc_word(uppercase_tbx_asm(rd, rn, n, rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 1, max: 4, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_five_regs
- Tier: 3
- Rationale: ARM TBX allows 1-4 table registers; llvm-mc rejects more. Split from table_contract kind=1 after empty-list shrink.
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:7246 test_encode_neon_tbl_regression_five_regs
- Formal: ∀ n∈{5..8}. encode_neon_tbx with n table registers is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, {v0.16b..v4.16b}, v0.8b])
- Bug report: bug_reports/encode_neon_tbx_five_regs.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 5_8 }
  relation:
    op: throws
    expr: encode_neon_tbx(valid_ops(0, "8b", 0, n, 0))
generators:
  n: { gen: int, min: 5, max: 8, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_nonsequential
- Tier: 3
- Rationale: ARM TBX table registers must be consecutive. Split from table_contract kind=2.
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:7258 test_encode_neon_tbl_regression_nonsequential
- Formal: encode_neon_tbx({v0.16b, v2.16b}) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, {v0.16b, v2.16b}, v0.8b])
- Bug report: bug_reports/encode_neon_tbx_nonsequential.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dummy]
  domain: { dummy: 0 }
  relation:
    op: throws
    expr: encode_neon_tbx([v0.8b, {v0.16b, v2.16b}, v0.8b])
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_table_not_16b
- Tier: 3
- Rationale: ARM TBX table is .16B. Split from table_contract kind=3.
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:7272 test_encode_neon_tbl_regression_table_not_16b
- Formal: encode_neon_tbx with table arrangement 8b is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, {v0.8b}, v0.8b])
- Bug report: bug_reports/encode_neon_tbx_table_not_16b.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dummy]
  domain: { dummy: 0 }
  relation:
    op: throws
    expr: encode_neon_tbx([v0.8b, {v0.8b}, v0.8b])
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_mismatched_t
- Tier: 3
- Rationale: ARM TBX requires Vm.Ta = Vd.Ta. Split from arity_kinds mismatched-T arm.
- Doc contract: neon.rs:802 "Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes)" — asserted fingerprint e8e04215
- Seed: neon.rs:7317 test_encode_neon_tbl_regression_mismatched_t
- Formal: encode_neon_tbx(Vd.8b, Vm.16b) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, {v0.16b}, v0.16b])
- Bug report: bug_reports/encode_neon_tbx_mismatched_t.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dummy]
  domain: { dummy: 0 }
  relation:
    op: throws
    expr: encode_neon_tbx([v0.8b, {v0.16b}, v0.16b])
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_tbx_neg_bare_list_reg
- Tier: 3
- Rationale: Table members must be Vn.16B. Split from list_and_vm_kinds kind=0.
- Doc contract: neon.rs:815 "tbx: expected register in list" — asserted fingerprint (none — inline error string)
- Seed: neon.rs:7286 test_encode_neon_tbl_regression_bare_list_reg
- Formal: encode_neon_tbx with bare Reg in the table list is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
- Status: failing
- Counterexample: encode_neon_tbx([v0.8b, RegList([Reg("v0")]), v0.8b])
- Bug report: bug_reports/encode_neon_tbx_bare_list_reg.md

```property
function: encoder.encode_neon_tbx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dummy]
  domain: { dummy: 0 }
  relation:
    op: throws
    expr: encode_neon_tbx([v0.8b, RegList([Reg("v0")]), v0.8b])
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```
