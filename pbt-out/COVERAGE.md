# PBT Coverage Status — round 08 (src/backend root)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| classify_cast_with_f128 | cast.rs | src/backend/cast.rs (pbt_tests P1-P3) | cargo test --lib backend::cast | pass (Noop laws, directional laws, flag independence; x86 F128-reduction documented) |
| classify_call_args / classify_args_core | call_abi.rs | src/backend/call_abi.rs (P4-P5) | cargo test --lib backend::call_abi | pass (caller/callee agreement ×4 configs; capacity + zero-size-skip bookkeeping) |
| classify_params_full | call_abi.rs | src/backend/call_abi.rs (P4) | cargo test --lib backend::call_abi | pass (agreement differential; variadicity-in-config pairing documented) |
| compute_live_intervals | liveness.rs | src/backend/liveness.rs (P6) | cargo test --lib backend::liveness | pass (exact equality vs naive textbook dataflow, 512 CFGs; call_points exact; depth sizing caveat recorded) |
| allocate_registers | regalloc.rs | src/backend/regalloc.rs (P7) | cargo test --lib backend::regalloc | pass (disjoint/callee-saved/eligibility soundness ×4 pool shapes) |
| parse_integer_expr | asm_expr.rs | src/backend/asm_expr.rs (P8 + red regression) | cargo test --lib backend::asm_expr | P8 pass; bug #765 red witness (arith panic vs Err contract) |
| generation.rs / traits.rs / state.rs / common.rs / x86_common.rs / peephole_common.rs / f128_softfloat.rs / inline_asm.rs / asm_preprocess.rs / elf_writer_common.rs / mod.rs | — | — | — | skipped with per-file reasons (PLAN.md): dispatch/glue exercised by per-arch rounds 09–12; orchestration kernels covered rounds 01/06; elf in round 13 |
