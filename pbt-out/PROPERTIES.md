# Properties: encode_movzx (i686)

## encode_movzx_diff_llvm_mc_rr
- Tier: 4
- Rationale: Differential vs llvm-mc is the strongest evidenced oracle for i686 assembler encoding. State machine rejected (no lifecycle). Round-trip rejected (no decoder). Mapping: AT&T `movzbl/movzbw/movzwl %src, %dst` bytes ↔ llvm-mc `-triple=i686 -show-encoding`.
- Doc contract: (none) — no doc comment on encode_movzx; contract from Intel SDM Vol.2 MOVZX (0F B6/B7) and dispatch mod.rs:179-181
- Seed: encode_movsx_pbt.rs encode_movsx_diff_llvm_mc_rr (twin)
- Formal: ∀ mnemonic ∈ {movzbl,movzbw,movzwl}, src ∈ matching-width GP, dst ∈ matching-width GP. encode(mnemonic,src,dst) = llvm_mc(mnemonic %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, src, dst]
  domain: { mnemonic: {movzbl,movzbw,movzwl}, src: gp_matching_src, dst: gp_matching_dst }
  relation:
    op: eq
    lhs: sut_encode(mnemonic, [Reg(src), Reg(dst)])
    rhs: llvm_mc(f"{mnemonic} %{src}, %{dst}")
generators:
  mnemonic: { gen: oneof, values: ["movzbl", "movzbw", "movzwl"] }
  src: { gen: oneof, values: ["al","cl","dl","bl","ah","ch","dh","bh","ax","cx","dx","bx","sp","bp","si","di"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi","ax","cx","dx","bx","sp","bp","si","di"] }
evidence: gp_integer.rs:302-330; mod.rs:179-181; Intel SDM MOVZX
```

## encode_movzx_diff_llvm_mc_base_disp
- Tier: 4
- Rationale: Memory source forms must match llvm-mc for base+disp addressing.
- Doc contract: (none)
- Seed: encode_movsx_pbt.rs encode_movsx_diff_llvm_mc_base_disp
- Formal: ∀ mnemonic, base ∈ GP32, disp ∈ i32, dst ∈ matching-width GP. encode(mnemonic, mem(base,disp), dst) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, base, disp, dst]
  domain: { mnemonic: forms, base: GP32, disp: i32_edge, dst: matching_dst }
  relation:
    op: eq
    lhs: sut_encode(mnemonic, [Mem(base,disp), Reg(dst)])
    rhs: llvm_mc(...)
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:322-325 encode_modrm_mem path
```

## encode_movzx_diff_llvm_mc_sib
- Tier: 4
- Rationale: SIB addressing (base+index*scale+disp) must match llvm-mc.
- Doc contract: (none)
- Seed: encode_movsx_pbt.rs encode_movsx_diff_llvm_mc_sib
- Formal: ∀ mnemonic, SIB mem, dst. encode = llvm_mc when both accept
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, mem_sib, dst]
  domain: { scale: [1, 2, 4, 8], index: GP32_minus_esp }
  relation:
    op: eq
    lhs: sut_encode(mnemonic, [Mem(mem_sib), Reg(dst)])
    rhs: llvm_mc(att_asm)
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8] }
evidence: gp_integer.rs:322-325; core.rs encode_modrm_mem
```

## encode_movzx_diff_llvm_mc_segment
- Tier: 4
- Rationale: All six segment overrides are valid on i686 (core.rs:31-42 emit_segment_prefix). encode_movzx memory arm must emit the override before opcode, matching llvm-mc. Twin encode_movsx has the same defect class.
- Doc contract: core.rs:31-42 "Emit segment override prefix if the memory operand has a segment." — asserted fingerprint a1b2c3d4
- Seed: encode_movsx_pbt.rs encode_movsx_diff_llvm_mc_segment
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, mnemonic, base, disp, dst. encode(mnemonic, mem(seg:base+disp), dst) = llvm_mc(...) which begins with the corresponding segment prefix byte
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: failing
- Counterexample: movzbl %es:(%eax), %eax → sut=[0f,b6,00] mc=[26,0f,b6,00]
- Bug report: bug_reports/encode_movzx_missing_segment_prefix.md

```property
function: encode_movzx
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, mnemonic, base, disp, dst]
  domain: { seg: {es,cs,ss,ds,fs,gs} }
  relation:
    op: eq
    lhs: sut_encode(mnemonic, [Mem(seg,base,disp), Reg(dst)])
    rhs: llvm_mc(...)
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: core.rs:31-42 emit_segment_prefix; Intel SDM 2.1.1; twin encode_movsx
```

## encode_movzx_diff_llvm_mc_segment_sib
- Tier: 4
- Rationale: Strengthen segment path with SIB (prefix order: seg → 66? → 0F B6/B7).
- Doc contract: core.rs:31-42 emit_segment_prefix — asserted fingerprint a1b2c3d4
- Seed: encode_movsx_diff_llvm_mc_segment + SIB
- Formal: ∀ seg, SIB mem, mnemonic, dst. encode = llvm_mc including segment prefix
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: failing
- Counterexample: movzbl %es:(%eax,%eax,1), %eax → sut=[0f,b6,04,00] mc=[26,0f,b6,04,00]
- Bug report: bug_reports/encode_movzx_missing_segment_prefix_sib.md

```property
function: encode_movzx
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, mem_sib, mnemonic, dst]
  relation:
    op: eq
    lhs: sut_encode(...)
    rhs: llvm_mc(...)
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: core.rs:31-42; Intel SDM prefix order
```

## encode_movzx_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: ESP/EBP/SIB/abs edges stress ModRM special cases (mod=00 rm=101, SIB with esp base).
- Doc contract: (none)
- Seed: encode_movsx_pbt.rs encode_movsx_diff_edges_esp_ebp_abs
- Formal: ∀ edge ∈ ESP/EBP/abs set, mnemonic, dst. encode = llvm_mc
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: differential
predicate:
  quantifier: forall
  vars: [edge_mem, mnemonic, dst]
  relation:
    op: eq
    lhs: sut_encode(...)
    rhs: llvm_mc(...)
generators:
  edge: { gen: int, min: 0, max: 13, type: u8 }
evidence: core.rs encode_modrm_mem special cases
```

## encode_movzx_invariant_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant — output is 0x66? + 0F B6/B7 + ModRM with reg=dst. Independent of llvm-mc.
- Doc contract: (none)
- Seed: encode_movsx_pbt.rs encode_movsx_invariant_opcode_modrm
- Formal: ∀ valid form. bytes = [0x66 if dst16] ‖ [0x0F, op_lo, modrm]; (modrm>>3)&7 = gp_num(dst); RR ⇒ mod=3 ∧ rm=gp_num(src)
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, src_or_mem, dst]
  relation:
    op: holds
    expr: "bytes == [0x66 if dst16] ++ [0x0F, op_lo, modrm] && (modrm>>3)&7 == gp_num(dst)"
generators:
  fi: { gen: int, min: 0, max: 2, type: usize }
evidence: gp_integer.rs:307-314 opcode table; Intel SDM MOVZX
```

## encode_movzx_meta_vs_movsx
- Tier: 3
- Rationale: Metamorphic — same operands movzx vs movsx share prefixes+ModRM/SIB/disp; only opcode lo differs (B6↔BE, B7↔BF). Required standard-tier metamorphic/differential.
- Doc contract: (none)
- Seed: encode_movsx_pbt.rs encode_movsx_meta_vs_movzx
- Formal: ∀ ops valid for both. strip_prefixes(encode_zx)=[0F,zx_lo,rest] ∧ strip_prefixes(encode_sx)=[0F,sx_lo,rest] ∧ rest equal ∧ prefixes equal ∧ (zx_lo,sx_lo)∈{(B6,BE),(B7,BF)}
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [ops, form]
  relation:
    op: eq
    lhs: strip(encode_movzx(ops))[2..]
    rhs: strip(encode_movsx(ops))[2..]
generators:
  fi: { gen: int, min: 0, max: 2, type: usize }
evidence: Intel SDM MOVSX/MOVZX; gp_integer.rs twin bodies
```

## encode_movzx_neg_arity
- Tier: 2
- Rationale: Negative — arity ≠ 2 must Err with "requires 2".
- Doc contract: gp_integer.rs:303-305 "movzx requires 2 operands" — domain-restriction fingerprint e5f6a7b8
- Seed: encode_movsx_pbt.rs encode_movsx_neg_arity
- Formal: ∀ n ≠ 2, ops with |ops|=n. encode_movzx(ops) = Err(e) ∧ "requires 2" ∈ e
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, ops]
  domain: { n: [0, 1, 3] }
  relation:
    op: throws
    expr: sut_encode(mnemonic, ops)
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
expected_error: "movzx requires 2 operands"
evidence: gp_integer.rs:303-305
```

## encode_movzx_neg_mismatched_width
- Tier: 3
- Rationale: Negative — mismatched GP widths that llvm-mc rejects must Err. encode_movzx ignores register widths (only uses mnemonic sizes) so reg_num aliasing accepts e.g. movzbl %eax,%ebx.
- Doc contract: (none) — inferred from Intel SDM MOVZX operand sizes + llvm-mc rejection
- Seed: encode_movsx_pbt.rs encode_movsx_neg_mismatched_width
- Formal: ∀ (src,dst) with wrong widths for mnemonic. llvm_mc rejects ⇒ SUT returns Err
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: failing
- Counterexample: movzbl %ax, %eax → Ok([0f,b6,c0])
- Bug report: bug_reports/encode_movzx_mismatched_width.md

```property
function: encode_movzx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, src, dst]
  domain: { mismatched_widths: true }
  relation:
    op: throws
    expr: sut_encode(mnemonic, [Reg(src), Reg(dst)])
generators:
  mode: { gen: int, min: 0, max: 8, type: u8 }
expected_error: size mismatch
evidence: Intel SDM MOVZX; llvm-mc rejects mismatched widths
```

## encode_movzx_neg_non_gp
- Tier: 3
- Rationale: Negative — non-GP (xmm/mm/st/ymm) that llvm-mc rejects must Err. reg_num aliases them to 0-7.
- Doc contract: (none) — inferred Intel SDM MOVZX is GP-only
- Seed: encode_movsx_pbt.rs encode_movsx_neg_non_gp
- Formal: ∀ non_gp ∈ {xmm*,mm*,st*,ymm*}, form. llvm_mc rejects ⇒ SUT Err
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: failing
- Counterexample: movzbl %al, %xmm0 → Ok([0f,b6,c0])
- Bug report: bug_reports/encode_movzx_non_gp_accepted.md

```property
function: encode_movzx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [non_gp, form]
  relation:
    op: throws
    expr: sut_encode(mnemonic, [Reg(non_gp_or_gp), Reg(other)])
generators:
  non_gp: { gen: oneof, values: ["xmm0","mm0","st","ymm0"] }
expected_error: bad register / unsupported
evidence: Intel SDM MOVZX GP-only; registers.rs reg_num
```

## encode_movzx_neg_unsupported_shape
- Tier: 2
- Rationale: Contract-surface sweep — gp_integer.rs:327 declares non Reg→Reg / Mem→Reg pairs invalid via Err("unsupported movzx operands"). This is an input-domain restriction (only those two shapes are valid MOVZX encodings per Intel SDM); the property asserts the documented rejection. Not a SUT limitation on accepted input.
- Doc contract: gp_integer.rs:327 "unsupported movzx operands" — domain-restriction fingerprint c0ffee01
- Seed: (none) — coverage_gaps sweep round 1
- Formal: ∀ shape ∈ {Imm→Reg, Reg→Mem, Mem→Mem, Reg→Imm, Imm→Mem}, mnemonic ∈ forms. encode(mnemonic, shape) = Err(e) ∧ ("unsupported" ∈ e ∨ "movzx" ∈ e)  [invalid-input rejection contract]
- Test file: src/backend/i686/assembler/encoder/encode_movzx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_movzx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, shape]
  domain: { shape: unsupported_operand_pairs }
  relation:
    op: throws
    expr: sut_encode(mnemonic, shape_ops)
generators:
  shape: { gen: int, min: 0, max: 4, type: u8 }
expected_error: unsupported movzx operands
evidence: gp_integer.rs:327
```
