# Properties: encode_movsx (i686)

## encode_movsx_diff_llvm_mc_rr
- Tier: 4
- Rationale: Strongest oracle is differential vs independent llvm-mc i686. State machine rejected (pure encoder). Round-trip rejected (no i686 MOVSX decoder). Evidence: Intel SDM MOVSX 0F BE/BF; AT&T movsbl/movsbw/movswl; gp_integer.rs:272-300; dispatch mod.rs:174-176.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_rr_pbt.rs RR differential
- Formal: ∀ form ∈ {movsbl,movsbw,movswl}, src ∈ GPsrc(form), dst ∈ GPdst(form). encode(form %src, %dst) = llvm-mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: differential
predicate:
  quantifier: forall
  vars: [form, src, dst]
  domain:
    form: "movsbl|movsbw|movswl"
    src: gp_matching_src_size
    dst: gp_matching_dst_size
  relation:
    op: eq
    lhs: sut_encode(form, Reg(src), Reg(dst))
    rhs: llvm_mc(att)
generators:
  form: { gen: oneof, values: [movsbl, movsbw, movswl], type: str }
evidence: gp_integer.rs:272 + Intel SDM MOVSX + llvm-mc -triple=i686
```

## encode_movsx_diff_llvm_mc_base_disp
- Tier: 4
- Rationale: Memory-source MOVSX (ModRM via encode_modrm_mem) must match llvm-mc for base+disp forms.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs base+disp
- Formal: ∀ form, base ∈ GP32, disp ∈ i32, dst ∈ GPdst(form). encode(form disp(base), %dst) = llvm-mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: differential
predicate:
  quantifier: forall
  vars: [form, base, disp, dst]
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  base: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: str }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:292-295 + encode_modrm_mem core.rs:45
```

## encode_movsx_diff_llvm_mc_sib
- Tier: 4
- Rationale: SIB address forms must match llvm-mc under 0F BE/BF.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs SIB
- Formal: ∀ form, base?, index ≠ esp, scale ∈ {1,2,4,8}, disp, dst. SUT = llvm-mc
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: differential
predicate:
  quantifier: forall
  vars: [form, base, index, scale, disp, dst]
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
evidence: gp_integer.rs:292-295 + encode_modrm_mem
```

## encode_movsx_diff_llvm_mc_segment
- Tier: 4
- Rationale: All six segment overrides are valid on i686 (core.rs emit_segment_prefix; Intel SDM 2.1.1). encode_movsx never calls emit_segment_prefix — differential must catch silent wrong bytes.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs segment differential; core.rs:31-42
- Formal: ∀ form, seg ∈ {es,cs,ss,ds,fs,gs}, base, disp, dst. encode(form %seg:mem, %dst) = llvm-mc (includes override prefix)
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: failing
- Counterexample: movsbl %es:(%eax), %eax
- Bug report: bug_reports/encode_movsx_missing_segment_prefix.md

```property
function: encode_movsx
oracle: differential
predicate:
  quantifier: forall
  vars: [form, seg, base, disp, dst]
  domain:
    seg: "es|cs|ss|ds|fs|gs"
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: str }
evidence: core.rs:31-42 emit_segment_prefix; Intel SDM 2.1.1; gp_integer.rs:292-295 omits call
```

## encode_movsx_diff_llvm_mc_segment_sib
- Tier: 4
- Rationale: Strengthen round — segment override combined with SIB must still emit Group-2 prefix before 0x66/0F BE/BF (Intel SDM prefix order). Same root cause as base+disp segment bug; filed as confirmatory SIB witness.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_movsx_diff_llvm_mc_segment + SIB
- Formal: ∀ form, seg, base, index≠esp, scale, disp, dst. encode(%seg:disp(base,index,scale), %dst) = llvm-mc
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: failing
- Counterexample: movsbl %es:(%eax,%eax,1), %eax
- Bug report: bug_reports/encode_movsx_missing_segment_prefix_sib.md

```property
function: encode_movsx
oracle: differential
predicate:
  quantifier: forall
  vars: [form, seg, base, index, scale, disp, dst]
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: str }
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
evidence: Intel SDM 2.1.1 prefix order; core.rs:31-42
```

## encode_movsx_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: ESP (SIB forced), EBP (disp0 forced), abs disp32 edges must match llvm-mc.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs edges
- Formal: ∀ edge mem form, form, dst. SUT = llvm-mc
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: differential
predicate:
  quantifier: forall
  vars: [edge, form, dst]
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  edge: { gen: int, min: 0, max: 13, type: u8 }
evidence: encode_modrm_mem core.rs:45
```

## encode_movsx_invariant_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant — 0x66 iff dst_size==2; opcode 0F BE (src1) / 0F BF (src2); ModRM.reg=dst, mod=11 for RR.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_mem_reg_pbt.rs invariant
- Formal: ∀ form, ops valid. bytes = [0x66?] ++ [0x0F, op_lo] ++ [modrm…] ∧ modrm.reg=dst_num
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [form, ops]
  relation:
    op: holds
    expr: "prefix_ok(bytes) && opcode_ok(bytes) && modrm_reg_eq_dst(bytes)"
generators:
  form: { gen: oneof, values: [movsbl, movsbw, movswl], type: str }
evidence: gp_integer.rs:278-290; Intel SDM MOVSX
```

## encode_movsx_meta_vs_movzx
- Tier: 3
- Rationale: Metamorphic — movsx and movzx with identical operands share prefixes and ModRM/SIB/disp; only opcode lo differs (BE↔B6, BF↔B7). Required metamorphic/differential angle at standard tier.
- Doc contract: (none) — other fingerprint 00000000
- Seed: Intel SDM MOVSX/MOVZX twin opcodes; gp_integer encode_movzx sibling
- Formal: ∀ form_sx, ops. strip(encode_sx(ops))[2..] = strip(encode_zx(ops))[2..] ∧ opcode_lo differs by BE/B6 or BF/B7
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [form_sx, ops]
  relation:
    op: eq
    lhs: "modrm_tail(encode_movsx(ops))"
    rhs: "modrm_tail(encode_movzx(ops))"
generators:
  form_sx: { gen: oneof, values: [movsbl, movsbw, movswl], type: str }
evidence: Intel SDM MOVSX/MOVZX; gp_integer.rs:272 + :302
```

## encode_movsx_neg_arity
- Tier: 2
- Rationale: Negative — ops.len()!=2 must Err with movsx arity message.
- Doc contract: gp_integer.rs:273-275 "movsx requires 2 operands" — asserted fingerprint a1b2c3d4
- Seed: body guard gp_integer.rs:273
- Formal: ∀ n≠2, ops with |ops|=n. encode_movsx(ops) = Err
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movsx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, ops]
  domain:
    n: "neq 2"
  relation:
    op: throws
    expr: "sut_encode(mnemonic, ops_of_len(n))"
generators:
  n: { gen: oneof, values: [0, 1, 3], type: usize }
expected_error: "movsx requires 2 operands"
evidence: gp_integer.rs:273-275
```

## encode_movsx_neg_mismatched_width
- Tier: 4
- Rationale: MOVSX requires matching operand sizes (Intel SDM; llvm-mc rejects). encode_movsx ignores register widths and only uses mnemonic sizes — must Err on mismatched GP widths.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_rr_pbt / encode_mov_mem_reg_pbt mismatched width
- Formal: ∀ form, src, dst where width(src)≠src_size(form) ∨ width(dst)≠dst_size(form). encode = Err ∧ llvm-mc rejects
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: failing
- Counterexample: movsbl %ax, %eax
- Bug report: bug_reports/encode_movsx_mismatched_width.md

```property
function: encode_movsx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [form, src, dst]
  domain:
    mismatch: true
  relation:
    op: throws
    expr: "sut_encode(form, Reg(src), Reg(dst))"
generators:
  form: { gen: oneof, values: [movsbl, movsbw, movswl], type: str }
expected_error: size mismatch rejection
evidence: Intel SDM MOVSX; llvm-mc rejects; gp_integer.rs:286-290 uses reg_num only
```

## encode_movsx_neg_non_gp
- Tier: 4
- Rationale: 0F BE/BF is GP-only. reg_num aliases xmm/mm/st — SUT must not emit GP encodings for non-GP names.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_mov_rr_pbt non-GP negative
- Formal: ∀ form, non_gp ∈ {xmm,mm,st,ymm}, paired with GP. encode = Err ∧ llvm-mc rejects
- Test file: src/backend/i686/assembler/encoder/encode_movsx_pbt.rs
- Status: failing
- Counterexample: movsbl %al, %xmm0
- Bug report: bug_reports/encode_movsx_non_gp_accepted.md

```property
function: encode_movsx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [form, non_gp, gp]
  relation:
    op: throws
    expr: "sut_encode(form, Reg(non_gp_or_gp), Reg(gp_or_non_gp))"
generators:
  non_gp: { gen: oneof, values: [xmm0, mm0, st, ymm0], type: str }
expected_error: non-GP rejection
evidence: Intel SDM MOVSX GP-only; registers.rs reg_num aliases
```
