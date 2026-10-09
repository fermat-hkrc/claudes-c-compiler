# Properties: encode_test (i686)

## encode_test_diff_rr_same_width
- Tier: 4
- Rationale: Differential vs llvm-mc (independent assembler). SM/RT rejected (pure encode; no TEST decoder). Intel SDM TEST r/m,r opcodes 84/85 + optional 0x66.
- Doc contract: src/backend/i686/assembler/encoder/mod.rs:214 `"testl" | "testw" | "testb" | "test" => self.encode_test(ops, mnemonic),` — asserted fingerprint 38d6be59
- Seed: encode_alu_pbt.rs encode_alu_diff_rr_same_width
- Formal: ∀ width∈{1,2,4}, src,dst ∈ GP(width). bytes(encode_test(test{b,w,l}, %src, %dst)) = llvm-mc_i686("test{b,w,l} %src, %dst")
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_test
oracle: differential
predicate:
  quantifier: forall
  vars: [width, src, dst]
  domain: { width: {1,2,4}, src: GP(width), dst: GP(width) }
  relation:
    op: eq
    lhs: "sut_encode(mnem(width), [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_i686(format('test{b,w,l} %{}, %{}', src, dst))"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  src_i: { gen: int, min: 0, max: 7, type: usize }
  dst_i: { gen: int, min: 0, max: 7, type: usize }
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:645
```

## encode_test_diff_imm_reg
- Tier: 4
- Rationale: Differential vs llvm-mc for imm→reg including AL/AX/EAX short forms (A8/A9) and F6/F7 /0.
- Doc contract: src/backend/i686/assembler/encoder/gp_integer.rs:662 `(Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(dst))` — asserted fingerprint 1c047588
- Seed: encode_alu_pbt.rs encode_alu_diff_imm_reg
- Formal: ∀ width∈{1,2,4}, dst∈GP(width), imm in width-domain. bytes(encode_test(test*, $imm, %dst)) = llvm-mc_i686(...)
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_test
oracle: differential
predicate:
  quantifier: forall
  vars: [width, dst, imm]
  domain: { width: {1,2,4}, dst: GP(width), imm: imm_for_width(width) }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Imm(imm), Reg(dst)])"
    rhs: "llvm_mc_i686(format('test* ${}, %{}', imm, dst))"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  dst_i: { gen: int, min: 0, max: 7, type: usize }
  imm: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:662
```

## encode_test_diff_imm_mem_bare
- Tier: 4
- Rationale: Differential Imm→Mem bare (F6/F7 /0 + modrm). Intel TEST r/m, imm; AT&T `test $imm, mem`.
- Doc contract: gp_integer.rs:689 `(Operand::Immediate(...), Operand::Memory(mem))` — asserted fingerprint 844b3611
- Seed: encode_alu_pbt.rs mem forms
- Formal: ∀ width, base∈GP32, disp, imm. bytes(encode_test(test*, $imm, disp(base))) = llvm-mc_i686(...)
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_test
oracle: differential
predicate:
  quantifier: forall
  vars: [width, base, disp, imm]
  domain: { width: {1,2,4}, base: GP32, disp: edge_disps, imm: imm_for_width }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Imm(imm), Mem(base,disp)])"
    rhs: "llvm_mc_i686(...)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  base_i: { gen: int, min: 0, max: 7, type: usize }
  disp: { gen: oneof, values: [0, 1, -1, 4, 127, 128, -128, 4096], type: i64 }
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:689
```

## encode_test_diff_reg_mem_bare
- Tier: 4
- Rationale: Differential Reg→Mem — Intel TEST r/m,r; AT&T `test %reg, mem`. x86-64 sibling implements this arm; i686 match lacks (Register, Memory).
- Doc contract: src/backend/x86/assembler/encoder/gp_integer.rs:608 `// test %reg, mem -> TEST mem, reg (AT&T: src=reg, dst=mem)` — asserted (sibling contract) fingerprint 41d27915
- Seed: (none — gap vs x86-64 sibling)
- Formal: ∀ width, src∈GP(width), base∈GP32, disp. bytes(encode_test(test*, %src, mem)) = llvm-mc_i686("test* %src, mem")
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: failing
- Counterexample: testb %al, (%eax) → Err("unsupported test operands"); llvm-mc = [84, 00]
- Bug report: bug_reports/encode_test_missing_reg_mem.md

```property
function: i686.encoder.encode_test
oracle: differential
predicate:
  quantifier: forall
  vars: [width, src, base, disp]
  domain: { width: {1,2,4}, src: GP(width), base: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Reg(src), Mem(base,disp)])"
    rhs: "llvm_mc_i686(...)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  src_i: { gen: int, min: 0, max: 7, type: usize }
  base_i: { gen: int, min: 0, max: 7, type: usize }
evidence: src/backend/x86/assembler/encoder/gp_integer.rs:608
```

## encode_test_diff_segment_prefix
- Tier: 4
- Rationale: Differential — segment overrides on Imm→Mem/Reg→Mem must match llvm-mc. core.rs documents emit_segment_prefix; x86-64 encode_test calls it on both mem arms.
- Doc contract: src/backend/i686/assembler/encoder/core.rs:31 `"Emit segment override prefix if the memory operand has a segment."` — asserted fingerprint 00a663e1
- Seed: encode_alu_diff_segment_prefix
- Formal: ∀ seg∈{es,cs,ss,ds,fs,gs}, form∈{imm→mem,reg→mem}. bytes(encode_test(..., seg:mem)) = llvm-mc_i686(...)
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: failing
- Counterexample: testb $5, %es:(%eax) → sut=[f6,00,05] mc=[26,f6,00,05]
- Bug report: bug_reports/encode_test_missing_segment_prefix.md

```property
function: i686.encoder.encode_test
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, form, width, base, reg]
  domain: { seg: SREGS, form: {imm_mem, reg_mem}, width: {1,2,4} }
  relation:
    op: eq
    lhs: "sut_encode(... seg:mem ...)"
    rhs: "llvm_mc_i686(...)"
generators:
  seg_i: { gen: int, min: 0, max: 5, type: usize }
  form: { gen: int, min: 0, max: 1, type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: src/backend/i686/assembler/encoder/core.rs:31
```

## encode_test_invariant_rr_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant — RR encoding structure: optional 0x66, opcode 84/85, mod=3, reg=src, rm=dst.
- Doc contract: gp_integer.rs:654 RR arm — asserted fingerprint 3e8364d7
- Seed: encode_alu_invariant_rr_opcode_modrm
- Formal: ∀ width,src,dst. let b=encode_test(...). (width=2⇒b[0]=0x66) ∧ opcode∈{84,85} ∧ modrm.mod=3 ∧ modrm.reg=num(src) ∧ modrm.rm=num(dst)
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_test
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [width, src, dst]
  domain: { width: {1,2,4}, src: GP(width), dst: GP(width) }
  relation:
    op: holds
    expr: "rr_bytes_match_sdm(sut_encode(mnem,[Reg(src),Reg(dst)]), width, src, dst)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  src_i: { gen: int, min: 0, max: 7, type: usize }
  dst_i: { gen: int, min: 0, max: 7, type: usize }
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:654
```

## encode_test_metamorphic_segment_prefix
- Tier: 4
- Rationale: Metamorphic — segmented Imm→Mem = seg_prefix_byte ‖ bare Imm→Mem.
- Doc contract: core.rs:31 emit_segment_prefix — asserted fingerprint 00a663e1
- Seed: encode_alu_metamorphic_segment_prefix
- Formal: ∀ seg, width, base, imm. encode(test*, $imm, seg:mem) = [seg_prefix(seg)] ‖ encode(test*, $imm, bare_mem)
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: failing
- Counterexample: testb $0 %es:(%eax) → got [f6,00,00] expect [26,f6,00,00]
- Bug report: bug_reports/encode_test_metamorphic_segment_prefix.md

```property
function: i686.encoder.encode_test
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [seg, width, base, imm]
  domain: { seg: SREGS, width: {1,2,4}, base: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(..., Mem(seg,base))"
    rhs: "[seg_prefix(seg)] ++ sut_encode(..., Mem(None,base))"
generators:
  seg_i: { gen: int, min: 0, max: 5, type: usize }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: src/backend/i686/assembler/encoder/core.rs:31
```

## encode_test_neg_arity
- Tier: 3
- Rationale: Negative — arity≠2 must Err per encode_test guard.
- Doc contract: gp_integer.rs:646-648 `if ops.len() != 2 { return Err(... requires 2 operands) }` — domain-restriction fingerprint f35451f6
- Seed: encode_alu_neg_arity
- Formal: ∀ arity≠2, width. encode_test(test*, ops_of_len(arity)) = Err
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_test
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity, width]
  domain: { arity: {0,1,3}, width: {1,2,4} }
  relation:
    op: holds
    expr: "sut_encode(mnem, ops_of_len(arity)).is_err()"
generators:
  arity: { gen: int, min: 0, max: 3, type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
expected_error: String
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:646
```

## encode_test_neg_mismatched_width
- Tier: 3
- Rationale: Negative — size-mismatched GP pairs that llvm-mc rejects must Err. SUT uses reg_num aliasing without width gate (same class as encode_alu/mov_rr).
- Doc contract: gp_integer.rs:654-660 RR arm uses reg_num only (no reg_size check) — other fingerprint 3e8364d7; contract inferred from Intel SDM + llvm-mc rejection
- Seed: encode_alu_neg_mismatched_width
- Formal: ∀ mismatched (src,dst,mnem) where llvm-mc rejects. encode_test → Err
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: failing
- Counterexample: testl %ax, %eax → Ok([85, c0]); llvm-mc rejects
- Bug report: bug_reports/encode_test_accepts_mismatched_width.md

```property
function: i686.encoder.encode_test
oracle: negative_error
predicate:
  quantifier: forall
  vars: [src, dst, mnem]
  domain: { mismatched GP pairs llvm-mc rejects }
  relation:
    op: holds
    expr: "sut_encode(mnem, [Reg(src), Reg(dst)]).is_err()"
generators:
  mode: { gen: int, min: 0, max: 5, type: u8 }
expected_error: String
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:654
```

## encode_test_neg_non_gp
- Tier: 3
- Rationale: Negative — non-GP (xmm/mm/st/ymm) that llvm-mc rejects must Err. reg_num aliases them to 0-7.
- Doc contract: gp_integer.rs:654-660 RR arm — other fingerprint 3e8364d7; contract inferred Intel SDM GP-only TEST
- Seed: encode_alu_neg_non_gp
- Formal: ∀ non_gp∈{xmm*,mm*,st*}, gp∈GP(width). encode_test(test*, non_gp, gp) = Err ∧ encode_test(test*, gp, non_gp) = Err
- Test file: src/backend/i686/assembler/encoder/encode_test_pbt.rs
- Status: failing
- Counterexample: testb %al, %xmm0 → Ok([84, c0]); llvm-mc rejects
- Bug report: bug_reports/encode_test_accepts_non_gp.md

```property
function: i686.encoder.encode_test
oracle: negative_error
predicate:
  quantifier: forall
  vars: [non_gp, gp, width]
  domain: { non_gp: NON_GP, gp: GP(width), width: {1,2,4} }
  relation:
    op: holds
    expr: "sut_encode(mnem, [Reg(a), Reg(b)]).is_err() where one is non_gp"
generators:
  ni: { gen: int, min: 0, max: 9, type: usize }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
expected_error: String
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:654
```
