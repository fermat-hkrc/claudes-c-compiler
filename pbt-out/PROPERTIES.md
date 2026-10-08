# Properties: encode_prefetch_0f0d

## encode_prefetch_0f0d_kat_llvm_mc_eax
- Tier: 5
- Rationale: Reference KAT gate — pins llvm-mc mapping and SUT basic path before PBT. Stronger state-machine N/A (pure encoder). Differential independence established by llvm-mc as external assembler.
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: (none — prior encode_prefetch KAT pattern generalized)
- Formal: ∃ known input prefetchw (%eax). encode(SUT)=llvm-mc= [0F, 0D, 08]
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: reference
predicate:
  quantifier: exists
  vars: [kat]
  domain: { kat: fixed_kat }
  body: sut_encode("prefetchw", mem_base(eax)) == vec![0x0F, 0x0D, 0x08]
generators:
  kat: { gen: const, value: 0 }
evidence: system.rs:24; encoder/mod.rs:746; llvm-mc -triple=i686
```

## encode_prefetch_0f0d_diff_llvm_mc_base_disp
- Tier: 5
- Rationale: Differential vs llvm-mc over base+disp memory forms. Strongest applicable (no in-tree decoder for round-trip; pure function so no state machine).
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_diff_llvm_mc_base_disp
- Formal: ∀ base ∈ GP32, disp ∈ i32. llvm-mc("prefetchw disp(%base)") = SUT.encode(prefetchw, Mem(base,disp))
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: differential
predicate:
  quantifier: forall
  vars: [base, disp]
  domain: { base: gp32_regs, disp: i32 }
  relation:
    op: eq
    lhs: sut_encode("prefetchw", mem_base(base, disp))
    rhs: llvm_mc("prefetchw att_mem(base,disp)")
generators:
  base: { gen: oneof, values: [eax,ecx,edx,ebx,esp,ebp,esi,edi] }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: system.rs:24-35; Intel SDM PREFETCHW 0F 0D /1
```

## encode_prefetch_0f0d_diff_llvm_mc_sib
- Tier: 5
- Rationale: Differential covering SIB addressing (index≠ESP, optional base, scales 1/2/4/8).
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_diff_llvm_mc_sib
- Formal: ∀ base?, index≠esp, scale∈{1,2,4,8}, disp. SUT bytes = llvm-mc bytes for SIB form
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: differential
predicate:
  quantifier: forall
  vars: [base, index, scale, disp]
  domain: { index: gp32_no_esp, scale: scales_1248 }
  relation:
    op: eq
    lhs: sut_encode("prefetchw", mem_sib(base, index, scale, disp))
    rhs: llvm_mc(att_form(base, index, scale, disp))
generators:
  scale: { gen: oneof, values: [1, 2, 4, 8] }
  index: { gen: oneof, values: [eax, ecx, edx, ebx, ebp, esi, edi] }
  disp: { gen: int, min: -512, max: 512, type: i64 }
evidence: system.rs:30-32 encode_modrm_mem path
```

## encode_prefetch_0f0d_hint_modrm_reg
- Tier: 4
- Rationale: Algebraic invariant — successful encode yields opcode 0F 0D and ModRM.reg = 1 (dispatch hardcodes hint=1). Rejected round-trip (no decoder).
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_hint_modrm_reg
- Formal: ∀ valid mem. let b = encode(prefetchw, mem). b[0]=0x0F ∧ b[1]=0x0D ∧ ((b[2]>>3)&7)=1
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: valid_i686_mem }
  relation:
    op: eq
    lhs: "(bytes[2] >> 3) & 7"
    rhs: "1"
generators:
  mem: { gen: mem_operand_i686 }
evidence: system.rs:24; encoder/mod.rs:746 hint=1
```

## encode_prefetch_0f0d_neg_arity
- Tier: 3
- Rationale: Negative/error contract — ops.len()≠1 must Err with "prefetchw requires 1 operand".
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_neg_arity
- Formal: ∀ ops, |ops|≠1. encode(prefetchw, ops) = Err(e) ∧ "requires 1 operand" ∈ e
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_lists_len_ne_1 }
  relation:
    op: throws
    expr: sut_encode("prefetchw", ops)
generators:
  ops: { gen: list, elem: { gen: string }, minLen: 0, maxLen: 5 }
expected_error: "prefetchw requires 1 operand"
evidence: system.rs:26-28
```

## encode_prefetch_0f0d_neg_non_memory
- Tier: 3
- Rationale: Negative/error — single non-memory operand must Err containing "memory operand".
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_neg_non_memory
- Formal: ∀ op ∉ Memory. encode(prefetchw, [op]) = Err(e) ∧ "memory operand" ∈ e
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: non_memory_operand }
  relation:
    op: throws
    expr: sut_encode("prefetchw", vec![op])
generators:
  op: { gen: oneof, values: [0, 1, 2, 3] }
expected_error: "prefetchw requires memory operand"
evidence: system.rs:34
```

## encode_prefetch_0f0d_diff_segment_prefix
- Tier: 5
- Rationale: Differential — segment overrides must appear as prefixes (x86 sibling emit_segment_prefix before 0F 0D; core.rs provides emit_segment_prefix). Metamorphic required at standard tier is also covered by edges property.
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_diff_segment_prefix (same class of bug expected)
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, disp. SUT.encode(prefetchw, Mem(seg:base+disp)) = llvm-mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0 → SUT=[0f,0d,08] llvm-mc=[26,0f,0d,08]
- Bug report: bug_reports/encode_prefetch_0f0d_missing_segment_prefix.md
- Re-verified: cargo test --lib encode_prefetch_0f0d_diff_segment_prefix -- --test-threads=1 → FAIL (serial)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, disp]
  domain: { seg: segment_regs }
  relation:
    op: eq
    lhs: sut_encode("prefetchw", mem_with_seg)
    rhs: llvm_mc(att_form_with_seg)
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs] }
evidence: core.rs:31-42 emit_segment_prefix; x86 encode_sse_mem_only; Intel SDM segment override
```

## encode_prefetch_0f0d_diff_edges_and_abs
- Tier: 5
- Rationale: Differential on ESP/EBP forced-SIB/disp edges and absolute disp32 — metamorphic-adjacent structural coverage of encode_modrm_mem corners through prefetchw.
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: encode_prefetch_pbt.rs encode_prefetch_diff_sib_esp_ebp_edges / abs
- Formal: ∀ edge mem form ∈ {esp, ebp, disp±128 bounds, SIB-esp, abs32}. SUT = llvm-mc
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: differential
predicate:
  quantifier: forall
  vars: [edge]
  domain: { edge: addressing_edge_cases }
  relation:
    op: eq
    lhs: sut_encode("prefetchw", edge)
    rhs: llvm_mc(att_form(edge))
generators:
  edge: { gen: oneof, values: [esp0, ebp0, disp_bounds, sib_esp, abs32] }
evidence: core.rs:45-143 encode_modrm_mem
```

## encode_prefetch_0f0d_meta_opcode_stable
- Tier: 4
- Rationale: Algebraic metamorphic — varying only displacement/base must keep opcode bytes 0F 0D fixed and ModRM.reg=1 (addressing bits may change). Required metamorphic at standard tier.
- Doc contract: system.rs:24 "Encode prefetchw (0F 0D /1)" — asserted fingerprint 332f809b
- Seed: (none)
- Formal: ∀ mem1, mem2 valid. let a=encode(m1), b=encode(m2). a[0..2]=b[0..2]=[0F,0D] ∧ ((a[2]>>3)&7)=((b[2]>>3)&7)=1
- Test file: src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_prefetch_0f0d
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mem1, mem2]
  domain: { mem: valid_i686_mem_no_seg }
  relation:
    op: eq
    lhs: "encode(mem1)[0..2]"
    rhs: "[0x0F, 0x0D]"
generators:
  mem1: { gen: mem_operand_i686 }
  mem2: { gen: mem_operand_i686 }
evidence: system.rs:31 opcode fixed; mod.rs:746 hint fixed
```
