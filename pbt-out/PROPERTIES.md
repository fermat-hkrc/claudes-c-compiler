# Properties: encode_system_table (i686)

## encode_system_table_diff_base_disp_llvm_mc
- Tier: 4
- Rationale: Strongest applicable is differential vs llvm-mc (trusted i686 assembler). State machine rejected (pure encoder). Round-trip rejected (no in-tree i686 decoder for system-table ops). Same-job sibling x86 path rejected (REX/64-bit ISA).
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs (memory-form differential pattern)
- Formal: ∀ mnem ∈ {sgdt,sidt,lgdt,lidt}, base ∈ GP32, disp ∈ i32. encode_system_table([Mem(base,disp)], mnem) = llvm_mc("{mnem} disp(%base)")
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, base, disp]
  domain: { mnem: system_table_mnemonics, base: gp32, disp: i32 }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Mem(base,disp)])"
    rhs: "llvm_mc_bytes(format!(\"{mnem} {}\", att_mem))"
generators:
  mnem: { gen: oneof, values: ["sgdt","sidt","lgdt","lidt"], type: "&str" }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:169-186; Intel SDM 0F 01 /0../3; llvm-mc -triple=i686
```

## encode_system_table_diff_sib_llvm_mc
- Tier: 4
- Rationale: SIB forms (base+index*scale+disp, index not esp) must match llvm-mc.
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs
- Formal: ∀ mnem, base?, index≠esp, scale∈{1,2,4,8}, disp. SUT bytes = llvm-mc bytes when llvm accepts
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, base, index, scale, disp]
  domain: { mnem: system_table, index: gp32_no_esp, scale: powers_of_two }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Mem(base,index,scale,disp)])"
    rhs: "llvm_mc_bytes(att_form)"
generators:
  mnem: { gen: oneof, values: ["sgdt","sidt","lgdt","lidt"], type: "&str" }
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
evidence: system.rs:183-186; encode_modrm_mem SIB path
```

## encode_system_table_diff_segment_llvm_mc
- Tier: 4
- Rationale: Segment override prefixes (es/cs/ss/ds/fs/gs) must precede 0F 01; core.rs emit_segment_prefix is the intended mechanism used by GP integer encoders. Differential vs llvm-mc.
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs segment property (same defect class)
- Formal: ∀ mnem, seg ∈ SEG, base ∈ GP32, disp. encode_system_table([Mem(seg:base+disp)], mnem) = llvm_mc("%seg:disp(%base)")
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: failing
- Counterexample: mnem="sgdt", seg="es", base="eax", disp=0 — SUT=[0f,01,00] llvm-mc=[26,0f,01,00]
- Bug report: bug_reports/encode_system_table_missing_segment_prefix.md

```property
function: i686.InstructionEncoder.encode_system_table
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, seg, base, disp]
  domain: { seg: segment_regs }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Mem(seg,base,disp)])"
    rhs: "llvm_mc_bytes(att_seg_form)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: core.rs:31-42 emit_segment_prefix; llvm-mc segment forms
```

## encode_system_table_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: ESP forces SIB, EBP forces disp8/disp32 even at 0, absolute disp32 uses mod=00 rm=101 — boundary edges for ModR/M.
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs edges property
- Formal: ∀ edge forms of mem, mnem. SUT = llvm-mc
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: differential
predicate:
  quantifier: forall
  vars: [edge, mnem]
  domain: { edge: esp_ebp_sib_abs_set }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Mem(edge)])"
    rhs: "llvm_mc_bytes(att_edge)"
generators:
  edge: { gen: int, min: 0, max: 11, type: u8 }
evidence: encode_modrm_mem special cases; Intel SDM addressing
```

## encode_system_table_invariant_opcode_ext
- Tier: 3
- Rationale: Algebraic invariant — body after optional segment prefixes is always 0F 01 with ModRM.reg = {sgdt:0,sidt:1,lgdt:2,lidt:3}.
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs invariant_opcode_ext7
- Formal: ∀ mnem, mem. let b = strip_seg(encode(mnem,mem)). b[0]=0x0F ∧ b[1]=0x01 ∧ (b[2]>>3)&7 = ext(mnem)
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnem, mem]
  domain: { mnem: system_table, mem: valid_mem }
  relation:
    op: holds
    expr: "strip_seg(sut)[0]==0x0F && strip_seg(sut)[1]==0x01 && ((strip_seg(sut)[2]>>3)&7)==ext(mnem)"
generators:
  mnem: { gen: oneof, values: ["sgdt","sidt","lgdt","lidt"], type: "&str" }
evidence: system.rs:176-181 reg_ext table
```

## encode_system_table_meta_shared_modrm_across_mnemonics
- Tier: 3
- Rationale: Metamorphic — same memory operand across the four mnemonics must share mod+rm/SIB/disp; only ModRM.reg differs (0/1/2/3).
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs meta vs lidt
- Formal: ∀ mem. for each pair (a,b) of system-table mnemonics: strip(a) opcode equal, mod+rm equal, SIB/disp equal, only reg field differs by ext table
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: valid_mem }
  relation:
    op: holds
    expr: "pairwise_same_mod_rm_sib_disp(sgdt,sidt,lgdt,lidt, mem)"
generators:
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: system.rs:176-186 shared encode path
```

## encode_system_table_meta_l_suffix_equivalent
- Tier: 3
- Rationale: Metamorphic — optional 'l' suffix (lgdtl→lgdt) must not change encoding (doc comment + strip_suffix).
- Doc contract: system.rs:174 "Strip optional 'l' suffix (e.g., \"lgdtl\" -> \"lgdt\")" — asserted fingerprint 35d6412b
- Seed: (none) — from code comment at system.rs:174
- Formal: ∀ mnem ∈ {sgdt,sidt,lgdt,lidt}, mem. encode(mnem,mem) = encode(mnem+"l",mem)
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mnem, mem]
  domain: { mnem: base_forms }
  relation:
    op: eq
    lhs: "sut_encode(mnem, mem)"
    rhs: "sut_encode(mnem + \"l\", mem)"
generators:
  mnem: { gen: oneof, values: ["sgdt","sidt","lgdt","lidt"], type: "&str" }
evidence: system.rs:174-175 strip_suffix l
```

## encode_system_table_neg_arity
- Tier: 2
- Rationale: Negative error — arity ≠ 1 must Err with requires-1-operand message.
- Doc contract: system.rs:171-173 arity guard returns Err — asserted fingerprint 9f0e1d2c
- Seed: encode_invlpg_pbt.rs neg_arity
- Formal: ∀ mnem, ops. len(ops)≠1 ⇒ encode returns Err containing "requires 1 operand"
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, ops]
  domain: { ops: arity_not_1 }
  relation:
    op: throws
    expr: "sut_encode(mnem, ops)"
generators:
  n_extra: { gen: int, min: 0, max: 3, type: usize }
expected_error: "requires 1 operand"
evidence: system.rs:171-173
```

## encode_system_table_neg_non_memory_non_label
- Tier: 2
- Rationale: Negative error — register/imm/indirect operands must Err ("requires memory operand"); Label is accepted (absolute form).
- Doc contract: system.rs:197 "{mnemonic} requires memory operand" — asserted fingerprint 7c8b9a0e
- Seed: encode_invlpg_pbt.rs neg_non_memory (adapted: Label is valid here)
- Formal: ∀ mnem, op ∈ {Reg, Imm, Indirect}. encode([op]) = Err(contains "memory operand")
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, op]
  domain: { op: Reg_or_Imm_or_Indirect }
  relation:
    op: throws
    expr: "sut_encode(mnem, [op])"
generators:
  kind: { gen: int, min: 0, max: 2, type: u8 }
expected_error: "memory operand"
evidence: system.rs:197
```

## encode_system_table_label_form_structure
- Tier: 3
- Rationale: Label form is documented ("Label as absolute memory reference: lgdtl tr_gdt") — must emit 0F 01 + modrm(0,ext,5) + 4 zero disp bytes (relocation fill-in). Differential vs llvm-mc placeholder A,A,A,A.
- Doc contract: system.rs:188 "Label as absolute memory reference: lgdtl tr_gdt" — asserted fingerprint 30014cd5
- Seed: (none) — from code comment
- Formal: ∀ mnem, label. encode([Label(label)]) = [0x0F,0x01, modrm(0,ext,5), 0,0,0,0]
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnem, label]
  domain: { mnem: system_table, label: non_empty_ident }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Label(label)])"
    rhs: "[0x0F, 0x01, modrm(0, ext, 5), 0, 0, 0, 0]"
generators:
  mnem: { gen: oneof, values: ["sgdt","sidt","lgdt","lidt","sgdtl","lgdtl"], type: "&str" }
evidence: system.rs:188-196; llvm-mc bare-label form
```

## encode_system_table_diff_segment_sib (strengthening round)
- Tier: 4
- Rationale: Strengthening — segment override combined with SIB addressing (standard tier extra round after first green batch on non-segment paths).
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs segment+SIB
- Formal: ∀ mnem, seg, base, index≠esp, scale, disp. SUT = llvm-mc for segment+SIB memory
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: failing
- Counterexample: mnem="sgdt", seg="es", base="eax", index="eax", scale=1, disp=0 — SUT=[0f,01,04,00] llvm-mc=[26,0f,01,04,00]
- Bug report: bug_reports/encode_system_table_missing_segment_prefix_sib.md

```property
function: i686.InstructionEncoder.encode_system_table
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, seg, base, index, scale, disp]
  domain: { seg: segment_regs, index: gp32_no_esp }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Mem(seg,base,index,scale,disp)])"
    rhs: "llvm_mc_bytes(att_seg_sib)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
evidence: core.rs:31-42; same missing-prefix root cause
```

## encode_system_table_diff_abs_disp32 (strengthening)
- Tier: 4
- Rationale: Absolute disp32 (no base/index) form — additional differential angle.
- Doc contract: system.rs:169 "Encode SGDT/SIDT/LGDT/LIDT: 0F 01 /N (memory operand)" — asserted fingerprint e040d895
- Seed: encode_invlpg_pbt.rs abs
- Formal: ∀ mnem, disp∈i32. encode([Mem(abs disp)]) = llvm_mc
- Test file: src/backend/i686/assembler/encoder/encode_system_table_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.InstructionEncoder.encode_system_table
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, disp]
  domain: { disp: i32 }
  relation:
    op: eq
    lhs: "sut_encode(mnem, [Mem(abs,disp)])"
    rhs: "llvm_mc_bytes"
generators:
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: encode_modrm_mem direct memory path
```
