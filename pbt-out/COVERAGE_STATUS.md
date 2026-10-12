# Coverage Status — encode_bsr_bsf campaign

**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — no line-level .gcda/.profraw produced (Rust instrumentation not yielding usable gaps for this binary); `coverage_gaps` reported false NOT LINKED due to Rust name mangling.

**Proof of execution:**
- `nm target/debug/deps/ccc-70d56e2a1978a8d3` contains `InstructionEncoder::encode_bsr_bsf`
- KAT tests call `InstructionEncoder::encode` with bsfl/bsrl/bsf and assert byte equality
- Differential properties ran 1000 cases each against live SUT

**Documented behaviors vs properties:**

| Behavior | Property | Status |
|----------|----------|--------|
| r32 RR vs llvm-mc | encode_bsr_bsf_diff_rr | passing |
| mem (no seg) vs llvm-mc | encode_bsr_bsf_diff_mem | passing |
| mem + segment vs llvm-mc | encode_bsr_bsf_diff_mem_segment | failing (bug) |
| 0F opc /r layout | encode_bsr_bsf_invariant_rr | passing |
| bare ≡ *l metamorphic | encode_bsr_bsf_metamorphic_bare_vs_l | passing |
| arity ≠ 2 → Err | encode_bsr_bsf_neg_arity | passing |
| r16/r8 → Err | encode_bsr_bsf_neg_wrong_width | failing (bug) |
| unsupported ops → Err | encode_bsr_bsf_neg_unsupported | passing |

**Sweep (standard, 1 round):** coverage_gaps → false NOT LINKED; nm + KAT prove link/execution. All documented behaviors have properties. No additional gap properties required.
