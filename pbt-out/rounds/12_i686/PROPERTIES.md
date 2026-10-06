# PROPERTIES.md — round 12 (src/backend/i686)

Oracle: gcc -m32, native i386 execution.

## p1_i686_e2e_matches_gcc_m32
- Bug found during bring-up: issue #770 (i686 target strips the arch-neutral gcc-internal include path — ccc -m32 cannot compile #include <stdio.h> out of the box; harness works around with -I). Filed; the E2E property itself then passed: 160 programs, ccc -m32 -O0 == gcc -m32 AND -O2 == -O0, native i386 execution.
- Tier: 1
- Rationale: the i686 backend's contract is the program's C11 semantics;
  gcc -m32 is the strongest available ILP32 reference (native execution).
  u64 expression functions exercise 64-bit-on-32-bit register-pair paths.
- Doc contract: src/backend/i686 READMEs (full pipeline for ILP32)
- Formal: ∀ P ∈ generated programs (u64 expr fns, 1..10 args, loop kernel, double fns): stdout(ccc -m32 -O0/-O2) == stdout(gcc -m32).
- Test file: src/backend/i686/mod.rs
- Status: passing (with the harness include-path workaround for issue #770 — the bug is in target setup, not the compiled code paths)
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::i686 (full pipeline via driver::Driver -m32)
oracle: differential
predicate:
  quantifier: forall
  vars: [program]
  domain: { program: gen_c_program (u64 exprs + loop kernel + double fns) }
  relation:
    op: eq
    lhs: stdout(execute(compile_ccc_m32(program, O0))) /\ stdout(execute(compile_ccc_m32(program, O2)))
    rhs: stdout(execute(compile_gcc_m32(program)))
generators:
  program: { gen: c_prog_spec, fns: int(1..=3), ops_per_fn: int(2..=8), seeds: list(tuple(u8,u8,i64), 1..=14) }
evidence: native i386 ELF execution verified on this host
```
