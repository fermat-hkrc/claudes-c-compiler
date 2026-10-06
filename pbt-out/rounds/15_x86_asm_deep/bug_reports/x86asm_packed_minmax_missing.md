# Bug: x86 assembler lacks the packed min/max family (minps/maxps/minpd/maxpd rejected; scalar minss/maxsd work)
> **Filed as [issue #782](https://github.com/fermat-hkrc/claudes-c-compiler/issues/782)**

**Law:** the assembler's documented SSE coverage includes the packed min/max instructions; `minps %xmm0, %xmm1` is a standard SSE form that both GNU as and llvm-mc assemble.

**Impact:** any external .s file using packed min/max fails to assemble ("unhandled instruction"). The scalar forms (minss/maxss/minsd/maxsd) ARE supported, so the omission looks accidental rather than deliberate. ccc's internal C pipeline is unaffected (round-09 E2E green; codegen currently never emits packed min/max).

**Function:** x86 assembler SSE encoder/mnemonic table
**Source location:** src/backend/x86/assembler/encoder/sse.rs (no minps/maxps/minpd/maxpd arms; scalar min/max present)
**Detected by:** round-15 contract-surface sweep (P3 family probe), double-reference confirmed (GNU as exit 0 AND llvm-mc exit 0 on all four; ccc exit 1 on all four)
**Minimal input:** `.text\nminps %xmm0, %xmm1` (also maxps, minpd, maxpd — all four)
**Expected:** assembles (SSE packed min/max, 0F 5F/0F 5F 66/66 0F 5D... encodings)
**Actual:** ccc: error: unhandled instruction
**Severity:** low (completeness gap, no wrong code; same class as #777)
**Contract evidence:** inferred (double-reference acceptance; x86/README documents "SSE through SSE4.1" coverage for the builtin assembler)
**Fix:** add minps/maxps/minpd/maxpd (0F 5F/5E + 66 0F 5F/5E with the packed forms' prefix rules) beside the existing scalar min/max arms.
**Regression test:** src/backend/x86/mod.rs pbt_asm_deep::test_x86asm_regression_packed_minmax_assembles
