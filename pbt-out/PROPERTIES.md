# Properties: encode_mov_imm_mem (i686)

## encode_mov_imm_mem_diff_llvm_mc_base_disp
- Tier: 4
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. State machine rejected (pure encoder). Round-trip rejected (no i686 MOV decoder). Evidence: Intel SDM MOV r/m,imm; AT&T `$imm, mem`; gp_integer.rs:238-270; dispatch mod.rs:167-169.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_reg_mem_pbt.rs base+disp differential
- Formal: ∀ base ∈ GP32, disp ∈ i32, width ∈ {1,2,4}, imm ∈ range(width). encode(movb|movw|movl $imm, disp(base)) = llvm-mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp, width, imm]
  domain:
    base: gp32
    disp: i32
    width: "1|2|4"
    imm: signed_for_width
  relation:
    op: eq
    lhs: sut_encode(suffix(width), Imm(imm), Mem(base, disp))
    rhs: llvm_mc(att)
generators:
  base: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: str }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  imm: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:238 + Intel SDM MOV r/m,imm + llvm-mc -triple=i686
```

## encode_mov_imm_mem_diff_llvm_mc_sib
- Tier: 4
- Rationale: SIB address forms (index/scale/esp) must match llvm-mc under C6/C7 /0.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_reg_mem_pbt.rs SIB differential
- Formal: ∀ base?, index ≠ esp, scale ∈ {1,2,4,8}, disp, width, imm. SUT encoding = llvm-mc encoding
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp, width, imm]
  domain:
    index: gp32_minus_esp
    scale: "1|2|4|8"
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8], type: u8 }
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:238 + encode_modrm_mem core.rs:45
```

## encode_mov_imm_mem_diff_llvm_mc_segment
- Tier: 4
- Rationale: All six segment overrides are valid on i686 (core.rs emit_segment_prefix; Intel SDM 2.1.1). x86-64 sibling calls emit_segment_prefix. i686 encode_mov_imm_mem emits no segment prefix — differential must catch silent wrong bytes.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_reg_mem_pbt.rs segment differential; x86-64 gp_integer.rs:180
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, disp, width, imm. encode($imm, %seg:mem) = llvm-mc (includes override prefix)
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0, width=1, imm=0 → movb $0, %es:(%eax); sut=[c6,00,00] mc=[26,c6,00,00]
- Bug report: bug_reports/encode_mov_imm_mem_missing_segment_prefix.md

```property
function: encode_mov_imm_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp, width, imm]
  domain:
    seg: "es|cs|ss|ds|fs|gs"
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: str }
evidence: core.rs:31-42 emit_segment_prefix; x86 gp_integer.rs:180; Intel SDM 2.1.1
```

## encode_mov_imm_mem_diff_edges_esp_ebp_abs
- Tier: 4
- Rationale: ESP (SIB forced), EBP (disp0 forced), abs disp32 edges must match llvm-mc C6/C7 /0 (no moffs for imm→mem).
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_reg_mem_pbt.rs edges
- Formal: ∀ edge mem form, width, imm. SUT = llvm-mc
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [edge, width, imm]
  domain:
    edge: esp_ebp_sib_abs
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  edge: { gen: int, min: 0, max: 13, type: u8 }
evidence: core.rs encode_modrm_mem ESP/EBP special cases
```

## encode_mov_imm_mem_invariant_opcode_modrm_imm
- Tier: 3
- Rationale: Algebraic invariant from Intel SDM: opcode C6 (size1) / C7 (else), optional 0x66 for size2, ModRM.reg=/0, trailing imm width bytes LE.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_mov_reg_mem_pbt.rs opcode invariant
- Formal: ∀ valid base+disp/SIB no-seg forms. bytes = [0x66?] ++ [C6|C7] ++ modrm(reg=0,…) ++ imm_le(width)
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mem, width, imm]
  domain:
    width: "1|2|4"
    segment: none
  relation:
    op: holds
    expr: opcode_modrm_reg0_and_imm_trail_ok(sut)
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: Intel SDM MOV r/m,imm /0; gp_integer.rs:239-254
```

## encode_mov_imm_mem_meta_same_mem_imm_trail
- Tier: 3
- Rationale: Metamorphic — same memory address, two different integer immediates of same width share all prefix+opcode+ModRM+SIB+disp; only trailing imm bytes differ. Required STANDARD metamorphic.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: (none) — derived from C6/C7 /0 layout
- Formal: ∀ mem, width, imm1≠imm2. drop_imm(encode(imm1,mem)) = drop_imm(encode(imm2,mem)) ∧ trail(encode(imm_i)) = le_bytes(imm_i, width)
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem, width, imm1, imm2]
  domain:
    width: "1|2|4"
  relation:
    op: eq
    lhs: prefix_body(encode(imm1))
    rhs: prefix_body(encode(imm2))
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:248-256 Integer arm appends size-dependent LE imm after modrm_mem
```

## encode_mov_imm_mem_diff_symbol_imm32
- Tier: 4
- Rationale: Symbol immediate for movl must emit R_386_32 reloc + 4 zero imm bytes matching llvm-mc A,A,A,A placeholders.
- Doc contract: (none) — other fingerprint 552e4230
- Seed: encode_system_table_pbt.rs label/fixup A parse
- Formal: ∀ base,disp,sym. encode(movl $sym, mem) bytes = llvm-mc placeholders-as-0 ∧ reloc.symbol=sym ∧ reloc_type=R_386_32
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp, sym]
  domain:
    width: 4
    imm: Symbol
  relation:
    op: eq
    lhs: sut_bytes
    rhs: llvm_mc_A_as_0
generators:
  base: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: str }
evidence: gp_integer.rs:259-264; R_386_32
```

## encode_mov_imm_mem_diff_symbol_narrow
- Tier: 4
- Rationale: llvm-mc accepts movb/movw $sym (FK_Data_1/2). AT&T path movb/movw → encode_mov_imm_mem accepts Symbol. Body returns Err with message admitting size==4 only — known limitation on accepted input, not a domain exclusion that retires the property. Keep domain full; expect differential match; file failure as documented-limitation bug.
- Doc contract: gp_integer.rs:265 "symbol immediate only supported for 32-bit mov to memory" — limitation fingerprint 073edab9
- Seed: (none)
- Formal: ∀ width ∈ {1,2}, base, disp. encode(mov{b,w} $sym, mem) = llvm-mc(same)
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: failing
- Counterexample: width=1, base="eax", disp=0 → movb $sym, (%eax); SUT Err("symbol immediate only supported for 32-bit mov to memory"); llvm-mc=[c6,00,00]
- Bug report: bug_reports/encode_mov_imm_mem_narrow_symbol_imm_rejected.md

```property
function: encode_mov_imm_mem
oracle: differential
predicate:
  quantifier: forall
  vars: [width, base, disp]
  domain:
    width: "1|2"
    imm: Symbol
  relation:
    op: eq
    lhs: sut_encode
    rhs: llvm_mc
generators:
  width: { gen: oneof, values: [1, 2], type: u8 }
evidence: llvm-mc FK_Data_1/2; mod.rs:167-169 accepts movb/movw $sym
```

## encode_mov_imm_mem_neg_symbol_mod_diff
- Tier: 2
- Rationale: SymbolMod and SymbolDiff have no supported encoding path — body returns Err("unsupported immediate for mov to memory"). True negative/error contract for unsupported kinds (not symbol-on-narrow).
- Doc contract: gp_integer.rs:267 "unsupported immediate for mov to memory" — domain-restriction fingerprint df6c6ae1
- Seed: (none)
- Formal: ∀ width ∈ {1,2,4}, kind ∈ {SymbolMod, SymbolDiff}. encode = Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_imm_mem
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, width]
  domain:
    kind: SymbolMod_or_SymbolDiff
  body: sut_encode(kind, width).is_err()
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
expected_error: String
evidence: gp_integer.rs:267-268
```
