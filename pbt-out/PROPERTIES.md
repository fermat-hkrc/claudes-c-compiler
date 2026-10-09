# Properties: encode_invlpg

## encode_invlpg_diff_llvm_mc_base_disp
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc i686 (independent assembler). State machine N/A (pure encode). Round-trip N/A (no i686 decoder). Reference KAT used as gate only.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none) — generalized from sibling encode_prefetch_pbt differential
- Formal: ∀ base ∈ GP32, disp ∈ i32. llvm_mc("invlpg disp(%base)") = encode(invlpg, Mem(base,disp))
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp]
  domain: { base: gp32_regs, disp: i32_edge }
  relation:
    op: eq
    lhs: encode("invlpg", Memory(base, disp))
    rhs: llvm_mc("invlpg " + att(base, disp))
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:109-120; Intel SDM INVLPG 0F 01 /7; llvm-mc -triple=i686
```

## encode_invlpg_diff_llvm_mc_sib
- Tier: 4
- Rationale: SIB forms (index≠esp, scales 1/2/4/8, optional base) must match llvm-mc.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp. llvm_mc(sib form) = encode(invlpg, Mem(sib))
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp]
  domain: { index: gp32_no_esp, scale: {1,2,4,8}, disp: i32_edge }
  relation:
    op: eq
    lhs: encode("invlpg", Memory(sib))
    rhs: llvm_mc(att_sib)
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
  disp: { gen: int, min: -512, max: 512, type: i64 }
evidence: system.rs:109-120; core.rs encode_modrm_mem SIB path
```

## encode_invlpg_diff_llvm_mc_segment
- Tier: 4
- Rationale: Segment override must precede 0F 01 (fs→64, gs→65, …). Sibling gp_integer callers call emit_segment_prefix; encode_invlpg does not — differential catches.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: encode_prefetch_diff_segment_prefix (prior campaign)
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, disp. encode(invlpg, Mem(seg:base+disp)) = llvm_mc("invlpg %seg:…")
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0 → SUT=[0f,01,38] llvm-mc=[26,0f,01,38]
- Bug report: bug_reports/encode_invlpg_missing_segment_prefix.md

```property
function: encode_invlpg
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp]
  domain: { seg: segment_regs, base: gp32, disp: i32_small }
  relation:
    op: eq
    lhs: encode("invlpg", Memory(seg, base, disp))
    rhs: llvm_mc("invlpg %seg:disp(%base)")
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: core.rs:31-42 emit_segment_prefix; x86 encode_mem_only emit_rex_rm; SDM segment overrides
```

## encode_invlpg_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: ESP needs SIB, EBP needs disp even when 0, absolute disp32 (mod=00 rm=101). Boundary edges.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ edge ∈ {esp, ebp, abs, sib-esp, disp±128}. encode = llvm_mc
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: differential
predicate:
  quantifier: forall
  vars: [edge]
  domain: { edge: fixed_edge_set }
  relation:
    op: eq
    lhs: encode("invlpg", edge_mem)
    rhs: llvm_mc(att(edge_mem))
generators:
  edge: { gen: int, min: 0, max: 11, type: u8 }
evidence: core.rs:45-143 encode_modrm_mem; Intel SDM ModR/M tables
```

## encode_invlpg_diff_abs_disp32
- Tier: 4
- Rationale: Absolute memory (no base/index) uses mod=00 rm=101 + disp32; must match llvm-mc.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ disp ∈ i32. encode(invlpg, Abs(disp)) = llvm_mc("invlpg disp")
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: differential
predicate:
  quantifier: forall
  vars: [disp]
  domain: { disp: i32 }
  relation:
    op: eq
    lhs: encode("invlpg", Abs(disp))
    rhs: llvm_mc("invlpg " + disp)
generators:
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: core.rs:69-77 direct memory reference path
```

## encode_invlpg_diff_segment_sib
- Tier: 4
- Rationale: Strengthening round — segment override combined with SIB addressing. Same missing-prefix root cause as base+disp segment property; kept failing as an independent SIB witness.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ seg, base, index≠esp, scale, disp. encode(invlpg, Seg:SIB) = llvm_mc
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", index="eax", scale=1, disp=0 → SUT=[0f,01,3c,00] llvm-mc=[26,0f,01,3c,00]
- Bug report: bug_reports/encode_invlpg_missing_segment_prefix_sib.md

```property
function: encode_invlpg
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, index, scale, disp]
  domain: { seg: segment_regs, index: gp32_no_esp }
  relation:
    op: eq
    lhs: encode("invlpg", Memory(seg, sib))
    rhs: llvm_mc(att_seg_sib)
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
evidence: core.rs:31-42; system.rs:109-120
```

## encode_invlpg_invariant_opcode_ext7
- Tier: 3
- Rationale: Algebraic invariant — successful encode must contain 0F 01 and ModRM.reg = 7.
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ valid Mem m. let b = encode(invlpg, m) in strip_seg(b) starts with 0F 01 and (ModRM>>3)&7 = 7
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: valid_i686_mem }
  relation:
    op: holds
    expr: "let b = strip_seg(encode(\"invlpg\", mem)); b[0]==0x0F && b[1]==0x01 && ((b[2]>>3)&7)==7"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: system.rs:109-118
```

## encode_invlpg_meta_same_as_system_table_lidt_modrm_rm
- Tier: 3
- Rationale: Metamorphic — INVLPG uses /7 on 0F 01; LIDT uses /3 on same opcode family. For identical memory, Mod+RM and SIB/disp must match; only reg field differs (7 vs 3).
- Doc contract: system.rs:109 "Encode INVLPG: 0F 01 /7 (memory operand)" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ m. let a=encode(invlpg,m), b=encode(lidt,m) in a[0..2]==b[0..2]==[0F,01] ∧ a[2]&0xC7==b[2]&0xC7 ∧ (a[2]>>3)&7==7 ∧ (b[2]>>3)&7==3 ∧ a[3..]==b[3..]
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: valid_i686_mem_no_seg }
  relation:
    op: holds
    expr: "let a=encode(\"invlpg\",mem); let b=encode(\"lidt\",mem); a[0..2]==b[0..2] && (a[2]&0xC7)==(b[2]&0xC7) && ((a[2]>>3)&7)==7 && ((b[2]>>3)&7)==3 && a[3..]==b[3..]"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: system.rs:109-120; system.rs:169-198 encode_system_table lidt /3
```

## encode_invlpg_neg_arity
- Tier: 3
- Rationale: Negative contract — arity ≠ 1 must Err with documented message.
- Doc contract: system.rs:109-112 "invlpg requires 1 operand" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ ops. |ops|≠1 ⇒ encode(invlpg,ops) = Err(contains "requires 1 operand")
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: arity_ne_1 }
  relation:
    op: holds
    expr: "encode(\"invlpg\", ops).is_err() && encode(\"invlpg\", ops).unwrap_err().contains(\"requires 1 operand\")"
expected_error: "invlpg requires 1 operand"
generators:
  n: { gen: int, min: 0, max: 4, type: usize }
evidence: system.rs:111-112
```

## encode_invlpg_neg_non_memory
- Tier: 3
- Rationale: Negative contract — register/imm/label/indirect must Err ("memory operand").
- Doc contract: system.rs:119 "invlpg requires memory operand" — asserted fingerprint b897d65e
- Seed: (none)
- Formal: ∀ op ∉ Memory. encode(invlpg,[op]) = Err(contains "memory operand")
- Test file: src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_invlpg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: non_memory }
  relation:
    op: holds
    expr: "encode(\"invlpg\", [op]).is_err() && encode(\"invlpg\", [op]).unwrap_err().contains(\"memory operand\")"
expected_error: "invlpg requires memory operand"
generators:
  kind: { gen: int, min: 0, max: 3, type: u8 }
evidence: system.rs:114-119
```
