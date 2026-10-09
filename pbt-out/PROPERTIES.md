# Properties: encode_imul (i686)

## encode_imul_diff_rr_same_width
- Tier: 4
- Rationale: Differential vs llvm-mc. State machine rejected. Round-trip rejected (no decoder).
- Doc contract: gp_integer.rs:711 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ width∈{2,4}, src,dst∈GP(width). encode(imul{w|l} %src, %dst) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: width=2, src=ax, dst=bx → sut=[0f,af,d8] mc=[66,0f,af,d8]
- Bug report: pbt-out/bug_reports/encode_imul_missing_operand_size_prefix.md

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [width, src, dst]
  domain: { width: {2,4}, src: gp(width), dst: gp(width) }
  relation:
    op: eq
    lhs: "sut_encode(imul_mnem(width), [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(...)"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: gp_integer.rs:716-721
```

## encode_imul_diff_mem_reg
- Tier: 4
- Rationale: Differential Mem→Reg.
- Doc contract: gp_integer.rs:723 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ width∈{2,4}, mem, dst∈GP(width). encode(imul mem, dst) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: width=2, form=0, bi=0, di=0, disp=0
- Bug report: pbt-out/bug_reports/encode_imul_diff_mem_reg_size_or_seg.md

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [width, mem, dst]
  domain: { width: {2,4}, mem: i686_mem_forms, dst: gp(width) }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: gp_integer.rs:723-726
```

## encode_imul_diff_imm_reg
- Tier: 4
- Rationale: Differential Imm→Reg with imm edges.
- Doc contract: gp_integer.rs:728 "imul $imm, %reg  =>  imul $imm, %reg, %reg (dst = src * imm)" — asserted fingerprint 48a11701
- Seed: (none)
- Formal: ∀ width∈{2,4}, dst, imm. encode(imul $imm, %dst) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: width=2, di=0, raw=0, edge=0
- Bug report: pbt-out/bug_reports/encode_imul_diff_imm_reg_size.md

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [width, dst, imm]
  domain: { width: {2,4}, dst: gp(width), imm: i32_edges }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: gp_integer.rs:728-740
```

## encode_imul_diff_imm_reg_reg
- Tier: 4
- Rationale: Differential Imm,Reg,Reg.
- Doc contract: gp_integer.rs:747 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ width∈{2,4}, src,dst,imm. encode(imul $imm, %src, %dst) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: width=2, si=0, di=0, raw=0, edge=0
- Bug report: pbt-out/bug_reports/encode_imul_diff_imm_reg_reg_size.md

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [width, src, dst, imm]
  domain: { width: {2,4} }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: gp_integer.rs:747-758
```

## encode_imul_diff_imm_mem_reg
- Tier: 4
- Rationale: Differential Imm,Mem,Reg including segment.
- Doc contract: gp_integer.rs:761 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ width, mem, dst, imm. encode(imul $imm, mem, %dst) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: imull $5, %es:(%eax), %ebx → sut=[6b,18,05] mc=[26,6b,18,05]
- Bug report: pbt-out/bug_reports/encode_imul_diff_imm_mem_reg_seg.md

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [width, mem, dst, imm]
  domain: { width: {2,4} }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: gp_integer.rs:761-771
```

## encode_imul_diff_unary
- Tier: 4
- Rationale: Differential 1-op via encode_unary_rm /5.
- Doc contract: gp_integer.rs:713 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ width∈{2,4}, op∈{Reg,Mem±seg}. encode(imul op) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: imulw %es:(%eax) → sut=[66,f7,28] mc=[26,66,f7,28]
- Bug report: pbt-out/bug_reports/encode_imul_unary_missing_segment_prefix.md

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [width, op]
  domain: { width: {2,4} }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: gp_integer.rs:713; encode_unary_rm:781
```

## encode_imul_invariant_rr_opcode
- Tier: 3
- Rationale: Algebraic invariant RR shape.
- Doc contract: gp_integer.rs:716 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ width∈{2,4}, src,dst. bytes = [0x66 if width=2] ++ [0x0F,0xAF,modrm(3,dst,src)]
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: width=2, si=0, di=0 → first byte 0x0F not 0x66
- Bug report: pbt-out/bug_reports/encode_imul_invariant_rr_missing_66.md

```property
function: encode_imul
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [width, src, dst]
  domain: { width: {2,4} }
  relation:
    op: holds
    expr: "bytes == [0x66?] ++ [0x0F, 0xAF, modrm(3,dst,src)]"
generators:
  width: { gen: oneof, values: [2, 4], type: u8 }
evidence: Intel SDM IMUL 0F AF /r
```

## encode_imul_metamorphic_seg_prefix
- Tier: 3
- Rationale: Metamorphic seg ‖ bare.
- Doc contract: core.rs:31 "Emit segment override prefix if the memory operand has a segment." — asserted fingerprint 00a663e1
- Seed: encode_test_metamorphic_segment_prefix
- Formal: ∀ seg, mem, dst. encode(seg:mem→dst) = [seg_prefix(seg)] ++ encode(mem→dst)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: seg=es, bare=(%eax), dst=eax → missing 0x26
- Bug report: pbt-out/bug_reports/encode_imul_missing_segment_prefix.md

```property
function: encode_imul
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [seg, mem, dst]
  domain: { seg: sregs }
  relation:
    op: eq
    lhs: "sut(seg:mem,dst)"
    rhs: "[seg_prefix(seg)] ++ sut(mem,dst)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: core.rs:31-42
```

## encode_imul_neg_arity
- Tier: 3
- Rationale: Negative arity.
- Doc contract: gp_integer.rs:777 "imul requires 1-3 operands" — asserted fingerprint c7507a53
- Seed: (none)
- Formal: ∀ n∈{0,4,5,6}. sut_encode(imull, n regs).is_err()
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_imul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: {0,4,5,6} }
  relation:
    op: holds
    expr: "sut_encode(imull, n_regs).is_err()"
generators:
  n: { gen: oneof, values: [0, 4, 5, 6], type: usize }
expected_error: String
evidence: gp_integer.rs:777
```

## encode_imul_neg_unsupported_shape
- Tier: 3
- Rationale: Negative unsupported shapes including non-GP.
- Doc contract: gp_integer.rs:742 "unsupported imul operands" — asserted fingerprint ab40f627
- Seed: (none)
- Formal: ∀ kind∈unsupported_shapes. sut_encode(imull, shape(kind)).is_err()
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: kind=2 xmm0,eax → Ok([0f,af,c0])
- Bug report: pbt-out/bug_reports/encode_imul_accepts_non_gp.md

```property
function: encode_imul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind]
  domain: { kind: {0,1,2,3} }
  relation:
    op: holds
    expr: "sut_encode(imull, unsupported_shape(kind)).is_err()"
generators:
  kind: { gen: int, min: 0, max: 3, type: u8 }
expected_error: String
evidence: gp_integer.rs:742,774
```

## encode_imul_diff_bare32_all_forms
- Tier: 4
- Rationale: Strengthen — bare 32-bit happy path.
- Doc contract: gp_integer.rs:711 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ form∈bare32_forms. encode(imull …) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_imul
oracle: differential
predicate:
  quantifier: forall
  vars: [form]
  domain: { form: bare32_forms }
  relation:
    op: eq
    lhs: "sut_encode(imull, ...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  form: { gen: int, min: 0, max: 5, type: u8 }
evidence: gp_integer.rs:711-777
```

## encode_imul_neg_mismatched_or_non_gp
- Tier: 3
- Rationale: Strengthen — mismatched width / non-GP must Err when llvm-mc rejects.
- Doc contract: gp_integer.rs:717 (none) — other fingerprint 00000000
- Seed: (none)
- Formal: ∀ invalid pair. llvm_mc rejects ⇒ SUT.is_err()
- Test file: src/backend/i686/assembler/encoder/encode_imul_pbt.rs
- Status: failing
- Counterexample: imull %ax, %eax accepted while llvm-mc rejects
- Bug report: pbt-out/bug_reports/encode_imul_neg_mismatched_width.md

```property
function: encode_imul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind]
  domain: { kind: {0,1,2} }
  relation:
    op: holds
    expr: "llvm_mc_rejects(asm) => sut.is_err()"
generators:
  kind: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: Intel SDM IMUL; llvm-mc rejection
```
