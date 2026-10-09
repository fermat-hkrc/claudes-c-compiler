# PROPERTIES — encode_lmsw (i686)

## encode_lmsw_diff_reg16
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc i686. LMSW register form is r/m16 (Intel SDM); llvm-mc emits `0F 01 /6` with mod=11. Round-trip rejected (no decoder). State machine rejected (pure encode).
- Doc contract: system.rs:201-202 "Encode LMSW (Load Machine Status Word): 0F 01 /6 Accepts a 16-bit register or memory operand." — asserted fingerprint 9826e63a
- Seed: (none) — sibling pattern from encode_verw_pbt.rs
- Formal: ∀ r ∈ {ax,cx,dx,bx,sp,bp,si,di}. encode_lmsw([Reg(r)]) = llvm_mc("lmsw %r")
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: r16_regs }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Reg(r)])"
    rhs: "llvm_mc(\"lmsw %\" ++ r)"
generators:
  r: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"], type: "&str" }
evidence: system.rs:201-202; Intel SDM LMSW r/m16; llvm-mc -triple=i686
```

## encode_lmsw_diff_base_disp
- Tier: 4
- Rationale: Differential memory base+disp forms vs llvm-mc; covers mod/disp8/disp32 and ESP/EBP special cases via random base.
- Doc contract: system.rs:201-202 "Accepts a 16-bit register or memory operand." — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_diff_llvm_mc_base_disp
- Formal: ∀ base ∈ GP32, disp ∈ i32. encode_lmsw([Mem(base,disp)]) = llvm_mc(att("lmsw", mem))
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp]
  domain: { base: gp32, disp: i32_disp_edges }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Mem(base,disp)])"
    rhs: "llvm_mc(att_lmsw_mem(base,disp))"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:214-216; Intel SDM LMSW m16; llvm-mc
```

## encode_lmsw_diff_sib
- Tier: 4
- Rationale: Differential SIB forms (index ≠ esp) vs llvm-mc.
- Doc contract: system.rs:201-202 — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_diff_llvm_mc_sib
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp. encode_lmsw([Mem SIB]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp]
  domain: { index: gp32_minus_esp, scale: {1,2,4,8} }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Mem SIB])"
    rhs: "llvm_mc(att)"
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
evidence: system.rs:214-216; encode_modrm_mem SIB path
```

## encode_lmsw_diff_segment
- Tier: 4
- Rationale: Differential segment-override forms. i686 core.rs provides emit_segment_prefix; x86 sibling emit_rex_rm emits segment before opcode. Documented memory operand must carry segment prefix bytes matching llvm-mc.
- Doc contract: system.rs:201-202 — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_diff_llvm_mc_segment
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, disp. encode_lmsw([Mem seg:base+disp]) = llvm_mc(att with %seg:)
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0 → SUT=[0f,01,30] llvm-mc=[26,0f,01,30]
- Bug report: bug_reports/encode_lmsw_missing_segment_prefix.md

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp]
  domain: { seg: segment_regs }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Mem(seg,base,disp)])"
    rhs: "llvm_mc(att)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: core.rs:31-42 emit_segment_prefix; x86 system.rs:256 emit_rex_rm; llvm-mc
```

## encode_lmsw_diff_edges
- Tier: 4
- Rationale: Differential ESP/EBP/abs/SIB edge encodings that hit special ModR/M cases.
- Doc contract: system.rs:201-202 — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_diff_edges_esp_ebp_abs
- Formal: ∀ edge ∈ ESP/EBP/abs/SIB edge set. encode_lmsw([Mem edge]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [edge]
  domain: { edge: esp_ebp_abs_sib_set }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Mem edge])"
    rhs: "llvm_mc(att)"
generators:
  edge: { gen: int, min: 0, max: 11, type: u8 }
evidence: encode_modrm_mem ESP/EBP/abs special cases
```

## encode_lmsw_invariant_opcode_ext6
- Tier: 3
- Rationale: Algebraic invariant from doc "0F 01 /6" — every valid encoding's body starts 0F 01 and ModRM.reg == 6.
- Doc contract: system.rs:201 "0F 01 /6" — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_invariant_opcode_ext5
- Formal: ∀ valid ops. let b = strip_seg(encode_lmsw(ops)) in b[0]=0x0F ∧ b[1]=0x01 ∧ ((b[2]>>3)&7)=6
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: valid_lmsw_ops }
  relation:
    op: holds
    expr: "let b = strip_seg(encode_lmsw(ops)); b[0]==0x0F && b[1]==0x01 && ((b[2]>>3)&7)==6"
generators:
  ops: { gen: oneof, values: ["r16", "base_disp", "sib"], type: "Vec<Operand>" }
evidence: system.rs:201 "0F 01 /6"
```

## encode_lmsw_metamorphic_vs_lidt
- Tier: 3
- Rationale: Metamorphic — same memory encoding shape as lidt (0F 01 /3); only ModRM.reg differs (6 vs 3). Independent of llvm-mc for structural relation.
- Doc contract: system.rs:201 "0F 01 /6" — asserted fingerprint 9826e63a
- Seed: encode_invlpg_pbt metamorphic vs lidt
- Formal: ∀ mem (no segment). set_modrm_reg(body(encode_lmsw(mem)), 3) = body(encode_lidt(mem))
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: non_seg_memory }
  relation:
    op: eq
    lhs: "set_modrm_reg(body_lmsw, 3)"
    rhs: "body_lidt"
generators:
  mem: { gen: oneof, values: ["base_disp", "sib", "abs"], type: MemoryOperand }
evidence: system.rs:201 /6; system.rs:170-182 lidt /3; shared 0F 01
```

## encode_lmsw_neg_arity
- Tier: 3
- Rationale: Negative contract — doc and body require exactly 1 operand.
- Doc contract: system.rs:201-205 — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_neg_arity
- Formal: ∀ ops. |ops|≠1 ⇒ encode_lmsw(ops) = Err("lmsw requires 1 operand")
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: arity_neq_1 }
  relation:
    op: eq
    lhs: "encode_lmsw(ops).err"
    rhs: "contains \"lmsw requires 1 operand\""
generators:
  n_extra: { gen: int, min: 0, max: 3, type: usize }
expected_error: "lmsw requires 1 operand"
evidence: system.rs:204-206
```

## encode_lmsw_neg_bad_operand
- Tier: 3
- Rationale: Negative — imm/label rejected; non-r16 registers rejected per doc "16-bit register" and Intel LMSW r/m16 (llvm-mc rejects eax/al).
- Doc contract: system.rs:202 "Accepts a 16-bit register or memory operand." — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_neg_bad_operand
- Formal: ∀ bad ∈ {Imm, Label, Reg32, Reg8}. encode_lmsw([bad]) = Err(...)
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: failing
- Counterexample: kind=2, bad_reg="eax" → SUT Ok([0f,01,f0]); llvm-mc rejects; doc requires 16-bit register
- Bug report: bug_reports/encode_lmsw_accepts_non_r16_register.md

```property
function: encode_lmsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: imm_or_label_or_non_r16 }
  relation:
    op: holds
    expr: "sut_encode(\"lmsw\", [bad]).is_err()"
generators:
  bad_reg: { gen: oneof, values: ["eax","al","ah","ebx","bl"], type: "&str" }
expected_error: "lmsw requires register or memory operand | bad width"
evidence: system.rs:202 "16-bit register"; Intel SDM LMSW r/m16; llvm-mc rejects
```

## encode_lmsw_diff_abs_disp32
- Tier: 4
- Rationale: Strengthening — absolute disp32 memory form differential.
- Doc contract: system.rs:201-202 — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_diff_abs_disp32
- Formal: ∀ disp ∈ i32. encode_lmsw([Mem abs disp]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [disp]
  domain: { disp: i32 }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Mem abs disp])"
    rhs: "llvm_mc(att)"
generators:
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: encode_modrm_mem abs path (mod=00 rm=101)
```

## encode_lmsw_diff_segment_sib
- Tier: 4
- Rationale: Strengthening — segment + SIB combined (edge of segment-prefix path). Same root cause as encode_lmsw_diff_segment (B1 missing emit_segment_prefix).
- Doc contract: system.rs:201-202 — asserted fingerprint 9826e63a
- Seed: encode_verw_pbt.rs encode_verw_diff_segment_sib
- Formal: ∀ seg, base, index≠esp, scale, disp. encode_lmsw([Mem seg:SIB]) = llvm_mc(att)
- Test file: src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", index="eax", scale=1, disp=0 → SUT=[0f,01,34,00] llvm-mc=[26,0f,01,34,00]
- Bug report: bug_reports/encode_lmsw_missing_segment_prefix_sib.md

```property
function: encode_lmsw
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, index, scale, disp]
  domain: { seg: segment_regs, index: gp32_minus_esp }
  relation:
    op: eq
    lhs: "sut_encode(\"lmsw\", [Mem seg SIB])"
    rhs: "llvm_mc(att)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: core.rs emit_segment_prefix + encode_modrm_mem SIB
```
