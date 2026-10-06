# PROPERTIES.md — round 09 (src/backend/x86)

The oracle is the REAL behavior of the reference toolchain: every generated
program is compiled by BOTH ccc (in-process via driver::Driver) and the
system gcc, executed natively, and stdout must match exactly. Optimization
preservation compares ccc -O2 against ccc -O0. All generated arithmetic is
in unsigned long long (defined wrap semantics) or int bitwise/compare forms
(no signed-overflow UB; div/rem divisors are nonzero constants; shift
amounts are constants < 64).

## p1_e2e_codegen_matches_gcc
- Re-verified: one generator defect fixed en route (K&R implicit-int params — a1<<32 was UB; params now explicitly u64). After the fix: 160 programs, ccc -O0 == gcc on native execution.
- Tier: 1
- Rationale: the backend's contract is that the produced executable computes
  the program's C11 semantics; gcc on the same host is the strongest available
  reference (native execution, same libc). Covers codegen + peephole + builtin
  assembler + builtin linker end-to-end at -O0.
- Doc contract: src/backend/x86/README.md:1-3 "covers the full pipeline from IR to ELF executable" (the pipeline contract)
- Formal: ∀ P ∈ generated C programs (expr fns over u64, bitwise/cmp/casts, 1..10-arg calls, loop kernel): stdout(run(ccc -O0, P)) == stdout(run(gcc, P)).
- Test file: src/backend/x86/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::x86 (full pipeline via driver::Driver)
oracle: differential
predicate:
  quantifier: forall
  vars: [program]
  domain: { program: gen_c_program (n_fns 1..3, ops = u64 wrap-arith + bitwise + cmp + casts, arg counts 1..10, loop kernel) }
  relation:
    op: eq
    lhs: stdout(execute(compile_ccc(program, O0)))
    rhs: stdout(execute(compile_gcc(program)))
generators:
  program: { gen: c_prog_spec, fns: int(1..=3), ops_per_fn: int(2..=8), seeds: list(tuple(u8,u8,i64), 1..=14) }
evidence: src/backend/x86/README.md pipeline overview (IR -> asm -> .o -> executable)
```

## p2_e2e_o2_preserves_o0
- Re-verified: same run as p1 (joint harness): ccc -O2 == ccc -O0 on all 160 programs.
- Tier: 1
- Rationale: the optimizer's contract (run_passes, peephole 15 passes/7 phases,
  regalloc) is value preservation; -O2 must not change observable output.
- Doc contract: src/backend/x86/codegen/peephole/README.md (7-phase optimizer over generated asm; preservation is the optimizer's premise)
- Formal: ∀ P: stdout(run(ccc -O2, P)) == stdout(run(ccc -O0, P)).
- Test file: src/backend/x86/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::x86 (optimized pipeline)
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [program]
  domain: { program: gen_c_program (as p1) }
  relation:
    op: eq
    lhs: stdout(execute(compile_ccc(program, O2)))
    rhs: stdout(execute(compile_ccc(program, O0)))
generators:
  program: { gen: c_prog_spec, fns: int(1..=3), ops_per_fn: int(2..=8), seeds: list(tuple(u8,u8,i64), 1..=14) }
evidence: src/passes/README.md "Every pass preserves the program's observable behavior"
```

## p3_e2e_double_codegen
- Tier: 1
- Rationale: double-precision expression functions exercise the SSE/XMM paths
  (double consts, conversions, compare/select) with %.17g-exact comparison.
- Doc contract: src/backend/x86/README.md "8 XMM argument registers" (SSE paths)
- Formal: ∀ P ∈ double expr programs (bounded operands, no NaN/Inf): stdout(run(ccc -O0/O2, P)) == stdout(run(gcc, P)).
- Test file: src/backend/x86/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::x86 (SSE paths via driver::Driver)
oracle: differential
predicate:
  quantifier: forall
  vars: [program]
  domain: { program: double_expr_program (2 fns × 2..7 ops, +,-,*,/,select; bounded; NaN-free) }
  relation:
    op: eq
    lhs: stdout(execute(compile_ccc(program, O0))) /\ stdout(execute(compile_ccc(program, O2)))
    rhs: stdout(execute(compile_gcc(program)))
generators:
  program: { gen: tuple, elems: [list(tuple(u8,i64), 2..=7), list(tuple(u8,i64), 2..=7)] }
evidence: src/backend/x86/README.md "Assembler ... SSE through SSE4.1, AVX/AVX2"
```
