# PROPERTIES.md — round 15 (x86 assembler deep)

## p1_x86_asm_valid_forms_match_llvm_mc
- Status: failing — 2 classes, double-reference confirmed and filed: REJECT-VALID → #777 (bare popcnt/lzcnt/tzcnt); SILENT-ACCEPT mem-form GPR width → #774. Collection-style: accumulates distinct signatures, GNU as cross-check in-run, asserts at end.
- Counterexample: `popcnt %rsp, %r13` (ccc rejects, as+llvm-mc accept); `movl 596(%rsp), %r13` (ccc coerces r13→r13d, both references reject).
- Bug report: bug_reports/x86asm_bare_popcnt_lzcnt_tzcnt_rejected.md, bug_reports/x86asm_gpr_width_silent_coercion.md
- Tier: 2
- Formal: ∀ T ∈ instruction templates × consistent operand fills: both assemblers accept ⇒ .text bytes equal.
- Test file: src/backend/x86/mod.rs
- Status: (superseded — see status notes above)
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::x86::assembler (builtin, via Driver -c)
oracle: differential
predicate:
  quantifier: forall
  vars: [form]
  domain: { form: template × consistent_fill }
  relation:
    op: holds
    expr: "llvm_mc_accepts(form) implies (ccc_accepts(form) && bytes_ccc(form) == bytes_mc(form))"
generators:
  form: { gen: tuple, elems: [{ gen: int, min: 0, max: 13, type: u8 }, { gen: int, min: 0, max: 63, type: u8 }, { gen: int, min: 0, max: 15, type: u8 }, { gen: int, min: 0, max: 4095, type: u16 }] }
evidence: hand probes (3 validation gaps) + teammate's #577-755 methodology
```

## p3_x86_asm_sweep_families
- Tier: 2
- Status: failing — 1 class: REJECT-VALID packed min/max family → issue #782. Found by the contract-surface sweep round (coverage_gaps had no line-level data; the sweep added 20 documented families).
- Counterexample: `minpd %xmm7, %xmm3` (ccc rejects; GNU as and llvm-mc both assemble).
- Bug report: bug_reports/x86asm_packed_minmax_missing.md
- Formal: ∀ T ∈ sweep families (lea/test/xchg/cmovcc/setcc/imul±imm/idiv/mul/adc, movss/movsd, cvt*, psrlq, vpermd, crc32, rdtsc/cpuid/cqo/cdq, min/max): same double-reference protocol as p1/p2.
- Test file: src/backend/x86/mod.rs

```property
function: backend::x86::assembler (sweep families)
oracle: differential
predicate:
  quantifier: forall
  vars: [fam, a, b, c, imm]
  domain: { fam: 18..38, a/b/c: regs, imm: 0..65535 }
  relation:
    op: holds
    expr: "double-reference protocol over the sweep families"
generators:
  fam: { gen: int, min: 18, max: 37, type: u8 }
evidence: contract-surface sweep; x86/README "SSE through SSE4.1"
```

## p2_x86_asm_invalid_forms_rejected
- Status: failing — 3 classes, all double-reference confirmed: `addq %r10d, %r10` + `movl %r8w, %esp` → #774; `vaddps %xmm4, %ymm10, %xmm14` → #775. imm-overflow (#776) confirmed by hand probe with the same protocol. Harness defect fixed en route: static shadowing made an earlier p2 run vacuously green — hoisted one static per case closure.
- Counterexample: `vaddps %xmm4, %ymm10, %xmm14` (mutation width_swap on `vaddps %xmm4, %xmm10, %xmm14`).
- Bug report: bug_reports/x86asm_gpr_width_silent_coercion.md, bug_reports/x86asm_simd_width_silent_promotion.md, bug_reports/x86asm_imm32_overflow_silent_truncation.md
- Tier: 2
- Formal: ∀ valid form × mutation (width mismatch / out-of-range imm / class swap / extra operand): llvm-mc rejects ⇒ ccc rejects (Err or no output), never silent encode.
- Test file: src/backend/x86/mod.rs
- Status: (superseded — see status notes above)
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::x86::assembler validation
oracle: negative_error
predicate:
  quantifier: forall
  vars: [form, mutation]
  domain: { form: valid template fill, mutation: {width_swap, imm_overflow, class_swap, extra_operand} }
  relation:
    op: holds
    expr: "!llvm_mc_accepts(mutate(form, mutation)) implies !ccc_accepts(mutate(form, mutation))"
generators:
  form: { gen: tuple, elems: [{ gen: int, min: 0, max: 13, type: u8 }, { gen: int, min: 0, max: 63, type: u8 }, { gen: int, min: 0, max: 15, type: u8 }] }
  mutation: { gen: int, min: 0, max: 3, type: u8 }
evidence: three hand-confirmed gaps (imm64 truncation, GPR width coercion, ymm/xmm promotion)
```
