# PBT Campaign: src/backend root (round 08)

> Whole-repo module loop, round 08. Rounds 01–07: common, lexer, ledger,
> preprocessor, parser, sema, ir, passes (see rounds/*/ and
> bug_reports/FILED_ISSUES.md — 23 issues filed through #760).

## Scan findings

- **Spec:** `src/backend/README.md` (1271 lines) — backend subsystem design:
  codegen pipeline (pre-scan analysis → stack layout/regalloc → instruction
  selection → peephole → assembler → linker), per-module contracts for
  exactly this round's files: liveness backward dataflow with the textbook
  equations quoted in the code doc ("live_in[B] = gen[B] ∪ (live_out[B] -
  kill[B])", "live_out[B] = ∪ live_in[S]"), three-phase linear-scan regalloc
  (callee-saved for call-spanning, caller-saved otherwise, spillover),
  eligibility whitelist, no interval splitting; call ABI classification with
  the caller/callee-agreement contract ("callers and callees must agree
  exactly on where each argument lives... implemented once in
  classify_args_core"), category table + CallAbiConfig parameterization
  (x86 6 GP + 8 FP, ARM/RISC-V 8+8, i686 0 GP); cast classification table
  (~20 CastKind variants, Ptr→U64 normalization, F128-native distinction);
  SlotAddr/RegCache (state.rs); F128 soft-float orchestration.
- **Test layout:** inline `#[cfg(test)] mod tests` / `mod pbt_tests` modules
  inside source files (repo convention, rounds 01–07); run via
  `cargo test --lib backend::`. proptest 1.11.0 is a declared dev-dependency.
  Existing deterministic tests: asm_preprocess (1 module), liveness (1),
  asm_expr (1) + 190 more across backend subdirs.
- **Buildability probe:** `PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1
  cargo test --lib -- backend::` → **193 passed, 0 failed, 6 ignored** (0.10s;
  log: pbt-out/rounds/08_backend_core/probe.log). Rung 1 confirmed.
- **Harness placement:** rung 1 — extend the repo's inline-test convention.
  New `#[cfg(test)] mod pbt_tests` at the bottom of `src/backend/cast.rs`,
  `call_abi.rs`, `liveness.rs`, `regalloc.rs`, `asm_expr.rs`, plus one shared
  `#[cfg(test)] mod pbt_backend_common;` file (registered by one
  `#[cfg(test)]` line in `src/backend/mod.rs`) holding the naive set-based
  liveness reference, the random-CFG builder (random function shapes with
  phis/params), and expression-tree generators. Runs via the same command
  with the target narrowed (`cargo test --lib backend::cast::pbt_tests` etc.).
- **Candidate modules:** cast.rs (classify_cast_with_f128 table laws — pure
  fn, PartialEq), call_abi.rs (classify_call_args vs classify_params_full
  agreement differential + capacity/skip bookkeeping), liveness.rs
  (compute_live_intervals vs naive textbook dataflow reference + interval
  coverage invariants + call_points + loop depth), regalloc.rs
  (allocate_registers soundness: disjoint intervals per register,
  call-spanning ⇒ callee-saved, eligibility whitelist), asm_expr.rs
  (parse_integer_expr vs independent evaluator over generated expression
  strings).
- **Skipped modules:** `generation.rs` (dispatch glue over ArchCodegen —
  exercised end-to-end by per-arch rounds 09–12), `traits.rs` (trait
  definitions, no logic), `state.rs` (RegCache/SlotAddr — stateful emission
  cache, integration-bound; its consumers are per-arch codegen), `common.rs` /
  `x86_common.rs` / `peephole_common.rs` (shared emission helpers consumed by
  per-arch rounds), `f128_softfloat.rs` (libcall ORCHESTRATION — the numeric
  kernels live in common (round 01) and ir (round 06); emission correctness
  is per-arch), `inline_asm.rs` (requires asm symbol plumbing), `asm_preprocess.rs`
  (text preprocessing feeding the assemblers; its semantic core parse_integer_expr
  sits in asm_expr.rs and IS tested here), `elf_writer_common.rs` (round 13:
  elf + linker_common), `mod.rs` (module wiring).

## Module: backend_core

- [x] Scan: identify targets (spec read; probe green)
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
