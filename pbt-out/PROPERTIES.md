# PROPERTIES: encode_verw

## encode_verw_diff_llvm_mc_reg16
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler claiming Intel SDM VERW 0F 00 /5 r/m16). State machine rejected (pure encoder). Round-trip rejected (no in-tree i686 decoder). Register form uses 16-bit GP names only (Intel r/m16; llvm-mc rejects eax/al).
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs KAT/diff pattern (generalized to verw + reg form)
- Formal: ∀ r ∈ {ax,bx,cx,dx,sp,bp,si,di}. encode_verw([Reg(r)]) = llvm_mc("verw %r")
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [reg]
  domain: { reg: r16_gp }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [Reg(reg)])"
    rhs: "llvm_mc(\"verw %\" ++ reg)"
generators:
  reg: { gen: oneof, values: ["ax","bx","cx","dx","sp","bp","si","di"], type: String }
evidence: system.rs:123 Intel SDM VERW 0F 00 /5 r/m16 llvm-mc -triple=i686
```

## encode_verw_diff_llvm_mc_base_disp
- Tier: 5
- Rationale: Differential memory base+disp against llvm-mc covers ModR/M/disp sizing for /5.
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_diff_llvm_mc_base_disp
- Formal: ∀ base ∈ GP32, disp ∈ i32_range. encode_verw([Mem(base,disp)]) = llvm_mc("verw disp(%base)")
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp]
  domain: { base: gp32, disp: i32 }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [Mem(base,disp)])"
    rhs: "llvm_mc(att_verw_mem(base,disp))"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: String }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:123-132 llvm-mc -triple=i686
```

## encode_verw_diff_llvm_mc_sib
- Tier: 5
- Rationale: SIB forms stress ESP-as-base and scale encodings under /5.
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_diff_llvm_mc_sib
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp. encode_verw([Mem SIB]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp]
  domain: { index: gp32_minus_esp, scale: {1,2,4,8}, disp: small_i32 }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [MemSIB])"
    rhs: "llvm_mc(att)"
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
  disp: { gen: int, min: -512, max: 512, type: i64 }
evidence: system.rs:129-132 core.rs encode_modrm_mem
```

## encode_verw_diff_llvm_mc_segment
- Tier: 5
- Rationale: Segment override must precede opcode (sibling x86 emit_rex_rm / i686 emit_segment_prefix used by gp_integer). Differential catches missing prefix.
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_diff_llvm_mc_segment
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, disp. encode_verw([Mem(seg:base+disp)]) = llvm_mc("verw %seg:…")
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0 → SUT=[0f,00,28] llvm-mc=[26,0f,00,28]
- Bug report: bug_reports/encode_verw_missing_segment_prefix.md

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp]
  domain: { seg: segment_regs, base: gp32, disp: edge_disp }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [Mem(seg,base,disp)])"
    rhs: "llvm_mc(att_with_seg)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: String }
evidence: core.rs:31-42 emit_segment_prefix x86 system.rs:121-124 emit_rex_rm before opcode
```

## encode_verw_diff_edges_and_abs
- Tier: 5
- Rationale: Boundary forms ESP/EBP/abs disp32 and SIB edges vs llvm-mc.
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_diff_edges_esp_ebp_abs
- Formal: ∀ edge_mem ∈ EdgeSet. encode_verw([edge_mem]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [edge]
  domain: { edge: esp_ebp_abs_sib_edges }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [Mem(edge)])"
    rhs: "llvm_mc(att)"
generators:
  edge: { gen: int, min: 0, max: 11, type: u8 }
evidence: system.rs:129-132 Intel SDM ModR/M special cases
```

## encode_verw_invariant_opcode_ext5
- Tier: 4
- Rationale: Algebraic invariant — body after optional seg prefixes is 0F 00 with ModRM.reg=/5. Weaker than differential but pins opcode/extension independently of llvm-mc availability.
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_invariant_opcode_ext7
- Formal: ∀ valid_mem_or_r16. let b = strip_seg(encode_verw(op)) in b[0]=0x0F ∧ b[1]=0x00 ∧ ((b[2]>>3)&7)=5
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: mem_or_r16 }
  relation:
    op: holds
    expr: "let b = strip_seg(encode_verw(op)); b[0]==0x0F && b[1]==0x00 && ((b[2]>>3)&7)==5"
generators:
  op: { gen: oneof, values: ["mem","reg16"] }
evidence: system.rs:123 Encode VERW 0F 00 /5
```

## encode_verw_neg_arity
- Tier: 3
- Rationale: Negative/error contract from source: arity ≠ 1 → "verw requires 1 operand".
- Doc contract: system.rs:125-127 "if ops.len() != 1 { return Err(\"verw requires 1 operand\") }" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_neg_arity
- Formal: ∀ ops. |ops|≠1 ⇒ encode_verw(ops)=Err containing "requires 1 operand"
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: wrong_arity }
  relation:
    op: throws
    expr: "encode_verw(ops)"
generators:
  n: { gen: int, min: 0, max: 4, type: usize }
expected_error: String
evidence: system.rs:125-127
```

## encode_verw_neg_non_r16_register
- Tier: 3
- Rationale: Intel SDM VERW is r/m16; llvm-mc rejects %eax/%al. Imm/label also Err. Property asserts rejection of non-r16 register widths.
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_neg_non_memory
- Formal: ∀ r ∈ {eax…edi, al…bh}. encode_verw([Reg(r)]) = Err; ∀ imm|label. encode_verw([op]) = Err
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: failing
- Counterexample: kind=2, bad_reg="eax", imm=0 → Ok([0f,00,e8])
- Bug report: bug_reports/encode_verw_accepts_non_r16_register.md

```property
function: encode_verw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [reg]
  domain: { reg: non_r16_gp }
  relation:
    op: throws
    expr: "encode_verw([Reg(reg)])"
generators:
  reg: { gen: oneof, values: ["eax","ecx","al","ah"], type: String }
expected_error: String
evidence: Intel SDM VERW r/m16 llvm-mc rejects eax/al
```

## encode_verw_diff_segment_sib
- Tier: 5
- Rationale: Strengthening round — segment + SIB combined differential (same missing-prefix root cause as p4, separate witness).
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_diff_segment_sib
- Formal: ∀ seg, base, index≠esp, scale, disp. encode_verw([Mem seg+SIB]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", index="eax", scale=1, disp=0 → SUT=[0f,00,2c,00] llvm-mc=[26,0f,00,2c,00]
- Bug report: bug_reports/encode_verw_missing_segment_prefix_sib.md

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, index, scale, disp]
  domain: { seg: segment_regs, index: gp32_minus_esp }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [Mem])"
    rhs: "llvm_mc(att)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: String }
evidence: core.rs:31-42 system.rs:129-132
```

## encode_verw_diff_abs_disp32
- Tier: 5
- Rationale: Absolute disp32 memory form vs llvm-mc (strengthening / edge coverage).
- Doc contract: system.rs:123 "Encode VERW: 0F 00 /5" — asserted fingerprint dda8ae15
- Seed: encode_invlpg_pbt.rs encode_invlpg_diff_abs_disp32
- Formal: ∀ disp ∈ i32. encode_verw([Mem(abs,disp)]) = llvm_mc("verw disp")
- Test file: src/backend/i686/assembler/encoder/encode_verw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_verw
oracle: differential
predicate:
  quantifier: forall
  vars: [disp]
  domain: { disp: i32 }
  relation:
    op: eq
    lhs: "sut_encode(\"verw\", [MemAbs(disp)])"
    rhs: "llvm_mc(att_abs)"
generators:
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:129-132 core.rs absolute mod=00 rm=101
```
