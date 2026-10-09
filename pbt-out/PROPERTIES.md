# Properties: encode_alu (i686)

## encode_alu_diff_rr_same_width
- Tier: 4
- Rationale: Strongest applicable is differential vs llvm-mc (independent GNU-style assembler). State machine rejected (pure encoder). Round-trip rejected (no in-tree i686 ALU decoder). Intel SDM Vol.2 documents ALU r/m,r forms; mod.rs:204-211 routes add/or/adc/sbb/and/sub/xor/cmp to encode_alu.
- Doc contract: (none) — encode_alu has no doc comment; dispatch at mod.rs:204-211 — other fingerprint 552e4230
- Seed: encode_mov_rr_pbt.rs:encode_mov_rr_diff_same_width_gp
- Formal: ∀ op∈{add,or,adc,sbb,and,sub,xor,cmp}, w∈{1,2,4}, src,dst∈GP_w. encode_alu(op_w, %src, %dst) = llvm_mc(op_w %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_alu
oracle: differential
predicate:
  quantifier: forall
  vars: [op, width, src, dst]
  domain: { op: alu_mnemonic, width: {1,2,4}, src: gp_w, dst: gp_w }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic(op,width), [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(att_rr(op,width,src,dst))"
generators:
  op: { gen: int, min: 0, max: 7, type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: src/backend/i686/assembler/encoder/mod.rs:204-211
```

## encode_alu_diff_imm_reg
- Tier: 4
- Rationale: Imm→reg is the densest ALU path (sign-extended imm8 vs imm16/32, EAX short form, AL short form). Differential vs llvm-mc. Accumulators may use short form (04+op*8 / 05+op*8).
- Doc contract: gp_integer.rs:457 "Short form: op eax, imm32" — asserted fingerprint 8a2518e3
- Seed: (none)
- Formal: ∀ op, w∈{1,2,4}, dst∈GP_w, imm∈imm_domain(w). encode_alu(op_w, $imm, %dst) = llvm_mc(op_w $imm, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: failing
- Counterexample: addb $1, %al
- Bug report: pbt-out/bug_reports/encode_alu_al_imm8_short_form.md

```property
function: encode_alu
oracle: differential
predicate:
  quantifier: forall
  vars: [op, width, dst, imm]
  domain: { op: alu_mnemonic, width: {1,2,4}, dst: gp_w, imm: int_for_width }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic(op,width), [Imm(imm), Reg(dst)])"
    rhs: "llvm_mc_bytes(att_imm_reg(op,width,imm,dst))"
generators:
  op: { gen: int, min: 0, max: 7, type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  imm: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:441-469
```

## encode_alu_diff_mem_forms
- Tier: 4
- Rationale: Three memory shapes (mem→reg, reg→mem, imm→mem) with base+disp, no segment. Differential vs llvm-mc.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_mem_reg_pbt.rs
- Formal: ∀ op, w, base∈GP32, disp, reg∈GP_w, shape∈{m2r,r2m,i2m}. encode_alu(shape) = llvm_mc(att(shape))
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_alu
oracle: differential
predicate:
  quantifier: forall
  vars: [op, width, shape, base, reg, disp, imm]
  domain: { shape: {m2r, r2m, i2m}, base: gp32, reg: gp_w }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  shape: { gen: int, min: 0, max: 2, type: u8 }
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:505-538
```

## encode_alu_diff_segment_prefix
- Tier: 4
- Rationale: core.rs:30 documents emit_segment_prefix for es/cs/ss/ds/fs/gs; x86-64 sibling encode_alu calls it on every memory arm. i686 encode_alu memory arms call encode_modrm_mem without emit_segment_prefix.
- Doc contract: core.rs:30 "Emit segment override prefix if the memory operand has a segment." — asserted fingerprint 00a663e1
- Seed: encode_pop_pbt.rs segment property
- Formal: ∀ op, seg∈{es,cs,ss,ds,fs,gs}, shape∈{m2r,r2m,i2m}. encode_alu(… %seg:(%eax) …) = llvm_mc(…) ∧ bytes[0]=seg_prefix(seg)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: failing
- Counterexample: addl %ebx, %es:(%eax)
- Bug report: pbt-out/bug_reports/encode_alu_missing_segment_prefix.md

```property
function: encode_alu
oracle: differential
predicate:
  quantifier: forall
  vars: [op, seg, shape]
  domain: { seg: {es,cs,ss,ds,fs,gs}, shape: mem_shapes }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: src/backend/i686/assembler/encoder/core.rs:30
```

## encode_alu_invariant_rr_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant from Intel SDM: RR form opcode = (size==1?0x00:0x01)+alu_op*8; optional 0x66 for w=2; ModRM mod=3, reg=src, rm=dst.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_rr_pbt.rs invariant
- Formal: ∀ op,w,src,dst. let b=encode_alu(op_w,%src,%dst) in opcode(b)=base(op,w) ∧ modrm(b).mod=3 ∧ modrm.reg=n(src) ∧ modrm.rm=n(dst) ∧ (w=2 ⇒ b[0]=0x66)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_alu
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, width, src, dst]
  relation:
    op: holds
    expr: "rr_struct_ok(encode_alu(op, width, src, dst))"
generators:
  op: { gen: int, min: 0, max: 7, type: u8 }
evidence: Intel SDM Vol.2 ADD — 01 /r
```

## encode_alu_metamorphic_segment_prefix
- Tier: 3
- Rationale: Metamorphic: segmented encoding must equal seg_prefix ‖ bare encoding (behavior-preserving transform of attaching a segment override).
- Doc contract: core.rs:30 "Emit segment override prefix if the memory operand has a segment." — asserted fingerprint 00a663e1
- Seed: encode_pop_pbt.rs metamorphic segment
- Formal: ∀ op, seg, bare_mem. encode(op, seg:bare) = [seg_prefix(seg)] ++ encode(op, bare)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: failing
- Counterexample: addb %es:(%eax) vs bare → got [0x02,0x00] expect [0x26,0x02,0x00]
- Bug report: pbt-out/bug_reports/encode_alu_missing_segment_prefix_metamorphic.md

```property
function: encode_alu
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op, seg]
  relation:
    op: eq
    lhs: "encode(seg:mem)"
    rhs: "[seg_prefix] ++ encode(mem)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: src/backend/i686/assembler/encoder/core.rs:30
```

## encode_alu_neg_arity
- Tier: 3
- Rationale: Negative/error — arity ≠ 2 must Err (gp_integer.rs:434-436).
- Doc contract: gp_integer.rs:435 "{} requires 2 operands" — asserted fingerprint f363de33
- Seed: encode_mov_rr_pbt.rs neg properties
- Formal: ∀ op, w, n≠2. encode_alu(op_w, ops) with |ops|=n = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_alu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  relation:
    op: throws
    expr: "encode_alu(ops) where len(ops) != 2"
generators:
  arity: { gen: int, min: 0, max: 3, type: u8 }
expected_error: String
evidence: src/backend/i686/assembler/encoder/gp_integer.rs:435
```

## encode_alu_neg_mismatched_width
- Tier: 3
- Rationale: Size-mismatched GP pairs that llvm-mc rejects must Err (reg_num must not silently alias widths).
- Doc contract: (none) — inferred from Intel SDM operand-size rules and llvm-mc reject set — other fingerprint 552e4230
- Seed: encode_mov_rr_pbt.rs encode_mov_rr_neg_mismatched_width
- Formal: ∀ mismatched (src,dst,mnem) that llvm-mc rejects. encode_alu = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: failing
- Counterexample: addl %ax, %ebx
- Bug report: pbt-out/bug_reports/encode_alu_accepts_mismatched_width.md

```property
function: encode_alu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [src, dst, mnem]
  relation:
    op: throws
    expr: "encode_alu(mismatched_width_pair)"
generators:
  mode: { gen: int, min: 0, max: 5, type: u8 }
expected_error: String
evidence: Intel SDM Vol.2; llvm-mc -triple=i686 rejects
```

## encode_alu_neg_non_gp
- Tier: 3
- Rationale: Non-GP names (xmm/mm/st) that alias through reg_num must Err; ALU r/m is GP-only.
- Doc contract: (none) — inferred from Intel SDM GP-only r/m forms — other fingerprint 552e4230
- Seed: encode_mov_rr_pbt.rs encode_mov_rr_neg_non_gp
- Formal: ∀ non_gp∈{xmm,mm,st,…}, gp∈GP_w. encode_alu(op_w, non_gp, gp) = Err(_) ∧ encode_alu(op_w, gp, non_gp) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_alu_pbt.rs
- Status: failing
- Counterexample: addb %al, %xmm0
- Bug report: pbt-out/bug_reports/encode_alu_accepts_non_gp.md

```property
function: encode_alu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [non_gp, gp, width]
  relation:
    op: throws
    expr: "encode_alu(op_w, non_gp_pair)"
generators:
  non_gp: { gen: oneof, values: ["xmm0","mm0","st"] }
expected_error: String
evidence: Intel SDM Vol.2; llvm-mc -triple=i686 rejects
```
