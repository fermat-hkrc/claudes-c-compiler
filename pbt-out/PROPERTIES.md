# Properties: encode_lsl (i686)

## encode_lsl_diff_reg32_llvm_mc
- Tier: 4
- Rationale: Strongest applicable is differential vs llvm-mc (trusted assembler). State machine rejected (pure encoder). Round-trip rejected (no i686 LSL decoder). Same-job sibling x86 encode_lsl rejected as differential (REX path, different ISA width).
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: encode_verw_pbt.rs (reg form differential pattern)
- Formal: ∀ src,dst ∈ GP32. encode_lsl([Reg(src), Reg(dst)]) = llvm_mc("lsl %src, %dst")
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_lsl
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst]
  domain: { src: gp32, dst: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"lsl\", [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"lsl %{src}, %{dst}\"))"
generators:
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: system.rs:143-158; Intel SDM LSL r32,r/m16; llvm-mc -triple=i686
```

## encode_lsl_diff_reg16_llvm_mc
- Tier: 4
- Rationale: 16-bit dest form must emit 0x66 operand-size override; differential vs llvm-mc.
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: encode_verw_pbt.rs
- Formal: ∀ src,dst ∈ GP16. encode_lsl([Reg(src), Reg(dst)]) = llvm_mc("lsl %src, %dst")
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_lsl
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst]
  domain: { src: gp16, dst: gp16 }
  relation:
    op: eq
    lhs: "sut_encode(\"lsl\", [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"lsl %{src}, %{dst}\"))"
generators:
  src: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"], type: "&str" }
  dst: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"], type: "&str" }
evidence: system.rs:152-155; Intel SDM LSL r16,r/m16
```

## encode_lsl_diff_mixed_width_dest_drives_osize
- Tier: 4
- Rationale: Operand-size prefix must follow destination width (Intel LSL r16/r32, r/m16), not source. Mixed-width pairs catch wrong is_16 on src.
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: (none) — inferred from Intel SDM + llvm-mc
- Formal: ∀ src∈GP, dst∈GP. encode_lsl([Reg(src),Reg(dst)]) = llvm_mc("lsl %src, %dst") when llvm accepts
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: failing
- Counterexample: src="eax", dst="ax" — SUT=[0f,03,c0] llvm-mc=[66,0f,03,c0]
- Bug report: bug_reports/encode_lsl_osize_from_src_not_dst.md

```property
function: i686.InstructionEncoder.encode_lsl
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst]
  domain: { src: gp16_or_32, dst: gp16_or_32 }
  relation:
    op: eq
    lhs: "sut_encode(\"lsl\", [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(...)"
generators:
  src: { gen: oneof, values: ["eax","ax","ecx","cx","ebx","bx"], type: "&str" }
  dst: { gen: oneof, values: ["eax","ax","ecx","cx","ebx","bx"], type: "&str" }
evidence: Intel SDM LSL; llvm-mc keys osize off dest
```

## encode_lsl_diff_mem_base_disp
- Tier: 4
- Rationale: Memory source form vs llvm-mc over base+disp domain including ESP/EBP edges; includes 16-bit dest.
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: encode_verw_pbt.rs base_disp
- Formal: ∀ base∈GP32, disp∈i32, dst∈GP32∪GP16. encode_lsl([Mem(base,disp), Reg(dst)]) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: failing
- Counterexample: lsl (%eax), %bx — SUT=[0f,03,18] llvm-mc=[66,0f,03,18]
- Bug report: bug_reports/encode_lsl_mem16_missing_66.md

```property
function: i686.InstructionEncoder.encode_lsl
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp, dst]
  domain: { base: gp32, disp: i32_edges, dst: gp16_or_32 }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
  dst: { gen: oneof, values: ["eax","ax","ebx","bx"], type: "&str" }
evidence: system.rs:160-164
```

## encode_lsl_diff_segment_prefix
- Tier: 4
- Rationale: Memory operands with segment overrides must emit 26/2E/36/3E/64/65 before opcode (core.rs emit_segment_prefix contract used by correct encoders).
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: encode_verw_pbt.rs segment / encode_invlpg missing-prefix bug class
- Formal: ∀ seg∈{es,cs,ss,ds,fs,gs}, base∈GP32, dst∈GP32. encode_lsl([Mem(seg:base),Reg(dst)]) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0, dst="eax" — SUT=[0f,03,00] llvm-mc=[26,0f,03,00]
- Bug report: bug_reports/encode_lsl_missing_segment_prefix.md

```property
function: i686.InstructionEncoder.encode_lsl
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, dst]
  domain: { seg: segment_regs, base: gp32, dst: gp32 }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: core.rs:31-42 emit_segment_prefix; llvm-mc segment forms
```

## encode_lsl_diff_sib_and_abs
- Tier: 4
- Rationale: SIB and absolute disp32 memory forms, including 16-bit dest and ESP/EBP edges.
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: encode_verw_pbt.rs sib/abs
- Formal: ∀ valid SIB/abs mem, dst∈GP. encode_lsl = llvm_mc when llvm accepts
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: failing
- Counterexample: base=None, index="eax", scale=1, disp=0, dst="ax" — SUT=[0f,03,04,05,00,00,00,00] llvm-mc=[66,0f,03,04,05,00,00,00,00]
- Bug report: bug_reports/encode_lsl_sib_mem16_missing_66.md

```property
function: i686.InstructionEncoder.encode_lsl
oracle: differential
predicate:
  quantifier: forall
  vars: [mem, dst]
  domain: { mem: sib_or_abs, dst: gp16_or_32 }
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_bytes
generators:
  dst: { gen: oneof, values: ["eax","ax","ebx","bx"], type: "&str" }
evidence: system.rs:160-164; core.rs encode_modrm_mem
```

## encode_lsl_invariant_opcode_0f03
- Tier: 3
- Rationale: Algebraic invariant — successful encodings always contain 0F 03 with ModRM.reg = dest.
- Doc contract: system.rs:143 "Encode LSL (Load Segment Limit): 0F 03 /r" — asserted fingerprint 1177d5b0
- Seed: encode_verw_pbt invariant
- Formal: ∀ valid ops. let b = encode_lsl(ops) in strip_prefixes(b) starts with [0x0F,0x03] ∧ ((modrm>>3)&7)=dst_num
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_lsl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: valid_lsl_ops }
  relation:
    op: holds
    expr: "strip_prefixes(bytes)[0]==0x0F && strip_prefixes(bytes)[1]==0x03 && ((strip_prefixes(bytes)[2]>>3)&7)==dst_num"
generators:
  ops: { gen: custom, type: "Vec<Operand>" }
evidence: system.rs:143 "0F 03 /r"
```

## encode_lsl_neg_arity_and_bad_ops
- Tier: 3
- Rationale: Negative/error — arity ≠ 2 and non (reg/mem, reg) shapes must Err.
- Doc contract: system.rs:145-146 "lsl requires 2 operands" — domain-restriction fingerprint a3e1c9f2
- Seed: encode_verw_pbt neg_arity
- Formal: ∀ ops. |ops|≠2 ∨ bad_shape(ops) ⇒ encode_lsl(ops) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_lsl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: bad_arity_or_shape }
  relation:
    op: throws
    expr: "sut_encode(\"lsl\", ops)"
generators:
  ops: { gen: custom, type: "Vec<Operand>" }
expected_error: String
evidence: system.rs:145-166
```
