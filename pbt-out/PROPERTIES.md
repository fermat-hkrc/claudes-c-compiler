# Properties: encode_smsw (i686)

## encode_smsw_diff_reg16
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc i686. SMSW r16 must emit 66 0F 01 /4. State machine N/A; no decoder for round-trip.
- Doc contract: system.rs:222-224 "Encode SMSW (Store Machine Status Word): 0F 01 /4 … Register form gets a 66h prefix for 16-bit operand size." — asserted fingerprint 7a3c9e12
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_reg16 (generalized to SMSW + 0x66)
- Formal: ∀ r ∈ {ax,bx,cx,dx,sp,bp,si,di}. encode_smsw(%r) = llvm_mc("smsw %r")
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: r16_regs }
  relation:
    op: eq
    lhs: "sut_encode(smsw, Reg(r))"
    rhs: "llvm_mc_bytes(smsw %r)"
generators:
  r: { gen: oneof, values: ["ax","bx","cx","dx","sp","bp","si","di"], type: "&str" }
evidence: system.rs:222-224; Intel SDM SMSW r/m16; llvm-mc -triple=i686
```

## encode_smsw_diff_reg32
- Tier: 5
- Rationale: Intel SDM / llvm-mc accept SMSW r32 (smswl) without 0x66. Doc says "16-bit" incompletely; differential owns the contract.
- Doc contract: system.rs:222-224 (domain incomplete vs Intel r32/m16) — other fingerprint 7a3c9e12
- Seed: (none) — SMSW-specific vs LMSW which rejects r32
- Formal: ∀ r ∈ {eax,ecx,edx,ebx,esp,ebp,esi,edi}. encode_smsw(%r) = llvm_mc("smsw %r") ∧ 0x66 ∉ encode_smsw(%r)
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: r32_regs }
  relation:
    op: eq
    lhs: "sut_encode(smsw, Reg(r))"
    rhs: "llvm_mc_bytes(smsw %r)"
generators:
  r: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: llvm-mc smswl %eax = [0f,01,e0]; Intel SDM SMSW r32/m16
```

## encode_smsw_diff_base_disp
- Tier: 5
- Rationale: Memory form 0F 01 /4 must match llvm-mc across disp8/disp32/esp/ebp edges.
- Doc contract: system.rs:222-223 "Accepts a 16-bit register or memory operand." — asserted fingerprint 7a3c9e12
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_base_disp
- Formal: ∀ base ∈ GP32, d ∈ i32. encode_smsw(mem(base,d)) = llvm_mc("smsw d(%base)")
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [base, d]
  domain: { base: gp32, d: i32 }
  relation:
    op: eq
    lhs: "sut_encode(smsw, Mem(base,d))"
    rhs: "llvm_mc_bytes(att)"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  d: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:241-244; core.rs encode_modrm_mem
```

## encode_smsw_diff_sib
- Tier: 5
- Rationale: SIB forms (index≠esp) must match llvm-mc.
- Doc contract: system.rs:222-223 — asserted fingerprint 7a3c9e12
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_sib
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, d. encode_smsw(SIB) = llvm_mc(SIB)
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, d]
  relation:
    op: eq
    lhs: "sut_encode(smsw, SIB)"
    rhs: "llvm_mc_bytes(att)"
generators:
  scale: { gen: oneof, values: [1,2,4,8], type: u8 }
evidence: core.rs encode_modrm_mem SIB path
```

## encode_smsw_diff_segment
- Tier: 5
- Rationale: Memory with segment override must emit 26/2E/36/3E/64/65 before 0F 01 (core.rs:31-42).
- Doc contract: (none on segment — inferred from emit_segment_prefix + llvm-mc)
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_segment
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, d. encode_smsw(%seg:d(%base)) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: failing
- Counterexample: smsw %es:(%eax) → SUT [0f,01,20] vs llvm-mc [26,0f,01,20]
- Bug report: bug_reports/encode_smsw_missing_segment_prefix.md

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, d]
  relation:
    op: eq
    lhs: "sut_encode(smsw, Mem(seg,base,d))"
    rhs: "llvm_mc_bytes(att)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: core.rs:31-42 emit_segment_prefix; llvm-mc smsw %es:(%eax)
```

## encode_smsw_diff_edges
- Tier: 5
- Rationale: ESP/EBP/abs/SIB edge encodings that historically break ModR/M.
- Doc contract: system.rs:222-223 — asserted fingerprint 7a3c9e12
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_edges
- Formal: ∀ edge ∈ EdgeSet. encode_smsw(edge) = llvm_mc(edge)
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [edge]
  relation:
    op: eq
    lhs: sut
    rhs: llvm_mc
generators:
  edge: { gen: int, min: 0, max: 11, type: u8 }
evidence: Intel ModR/M special cases esp/ebp/abs
```

## encode_smsw_invariant_opcode_ext4
- Tier: 4
- Rationale: Algebraic invariant — every successful encode yields 0F 01 with ModRM.reg=/4; r16 carries 0x66.
- Doc contract: system.rs:222 "0F 01 /4" + "66h prefix for 16-bit" — asserted fingerprint 7a3c9e12
- Seed: encode_lmsw_pbt.rs encode_lmsw_invariant_opcode_ext6
- Formal: ∀ valid ops. let b = strip_prefixes(encode_smsw(ops)) in b[0]=0F ∧ b[1]=01 ∧ ((b[2]>>3)&7)=4 ∧ (r16 ⇒ 0x66 ∈ prefixes)
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ops]
  relation:
    op: holds
    expr: "strip_legacy(encode_smsw(ops)) starts_with [0x0F,0x01] && ((modrm>>3)&7)==4 && (is_r16(ops) => 0x66 in prefixes)"
generators:
  ops: { gen: oneof, values: ["r16","r32","mem"], type: "Operand" }
evidence: system.rs:222-239
```

## encode_smsw_metamorphic_vs_lidt
- Tier: 4
- Rationale: Required metamorphic — same memory → smsw and lidt share mod+rm/SIB/disp; only ModRM.reg differs (4 vs 3).
- Doc contract: system.rs:222 vs encode_system_table lidt /3
- Seed: encode_lmsw_pbt.rs encode_lmsw_metamorphic_vs_lidt
- Formal: ∀ mem. set_modrm_reg(encode_smsw(mem), 3) = strip_seg(encode_lidt(mem))
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem]
  relation:
    op: eq
    lhs: "set_modrm_reg(smsw(mem), 3)"
    rhs: "strip_seg(lidt(mem))"
generators:
  mem: { gen: oneof, values: ["base","sib","abs"], type: "MemoryOperand" }
evidence: system.rs lidt ext=3; smsw ext=4
```

## encode_smsw_neg_arity
- Tier: 3
- Rationale: Negative contract — ops.len()≠1 must Err with "smsw requires 1 operand".
- Doc contract: system.rs:226-228 — asserted fingerprint a1b2c3d4
- Seed: encode_lmsw_pbt.rs encode_lmsw_neg_arity
- Formal: ∀ ops. |ops|≠1 ⇒ encode_smsw(ops) = Err(contains "requires 1 operand")
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len != 1" }
  relation:
    op: throws
    expr: "sut_encode(smsw, ops)"
generators:
  n: { gen: int, min: 0, max: 5, type: usize }
expected_error: "smsw requires 1 operand"
evidence: system.rs:226-228
```

## encode_smsw_neg_bad_operand
- Tier: 3
- Rationale: Imm/label rejected; 8-bit registers rejected by llvm-mc (SMSW is r/m16 or r32/m16, not r8).
- Doc contract: system.rs:245 "smsw requires register or memory operand"; width from Intel/llvm-mc
- Seed: encode_lmsw_pbt.rs encode_lmsw_neg_bad_operand
- Formal: ∀ bad ∈ {Imm, Label, r8}. encode_smsw(bad) = Err
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: failing
- Counterexample: smsw %al → Ok([0f,01,e0]); llvm-mc rejects
- Bug report: bug_reports/encode_smsw_accepts_r8_register.md

```property
function: i686.encoder.encode_smsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  relation:
    op: throws
    expr: "sut_encode(smsw, bad)"
generators:
  bad: { gen: oneof, values: ["imm","label","al","cl","dl","bl","ah","ch","dh","bh"], type: "Operand" }
expected_error: "register or memory | width reject"
evidence: system.rs:245; llvm-mc rejects smsw %al
```

## encode_smsw_diff_abs_disp32
- Tier: 5
- Rationale: Absolute disp32 memory form (strengthening round).
- Doc contract: system.rs:222-223 — asserted fingerprint 7a3c9e12
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_abs_disp32
- Formal: ∀ d ∈ i32. encode_smsw(abs(d)) = llvm_mc("smsw d")
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [d]
  relation:
    op: eq
    lhs: sut
    rhs: llvm_mc
generators:
  d: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: core.rs abs encoding mod=00 rm=101
```

## encode_smsw_diff_segment_sib
- Tier: 5
- Rationale: Segment + SIB combination (strengthening / contract-surface sweep).
- Doc contract: (none on segment — inferred)
- Seed: encode_lmsw_pbt.rs encode_lmsw_diff_segment_sib
- Formal: ∀ seg, base, index≠esp, scale, d. encode_smsw(%seg:SIB) = llvm_mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_smsw_pbt.rs
- Status: failing
- Counterexample: smsw %es:(%eax,%eax,1) → SUT [0f,01,24,00] vs llvm-mc [26,0f,01,24,00]
- Bug report: bug_reports/encode_smsw_missing_segment_prefix_sib.md

```property
function: i686.encoder.encode_smsw
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, index, scale, d]
  relation:
    op: eq
    lhs: sut
    rhs: llvm_mc
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: core.rs emit_segment_prefix + SIB
```
