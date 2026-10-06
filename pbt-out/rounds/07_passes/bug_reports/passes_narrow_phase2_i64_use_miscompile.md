# Bug: narrow Phase 2 rewrites I64 bitwise ops to I32 without checking the use — I64 consumers get the zero-extended value
**Law:** A rewrite pass must preserve the value of the function it transforms (the pipeline's own contract: every pass preserves interpreted values; README pipeline overview). Narrowing `Or(sext(a), sext(b))` at I64 to `Or` at I32 is only value-preserving when the result is re-sign-extended to I64 at the consumer; with no re-extension the I64 consumer reads the zero-extended 32-bit pattern.
**Impact:** Pipeline-level silent miscompile: `f(a: i32, b: i32) -> i64 { Or((i64)a, (i64)b) }` compiled through run_passes(-O2, x86-64) returns 4294967295 for f(1, -1) instead of -1 (confirmed by interpreting the post-pipeline IR: `Or` survives at I32 feeding an I64 Return with no Cast). And/Xor share the code path (same missing use-check); on x86-64 a 32-bit result register move zero-extends. No C-level repro found with straightforward sources (the frontend emits Or-at-I32 + single cast for `(long long)a|(long long)b`), but any IR producer that emits the widen-widen-bitwise shape (e.g. future frontend paths, other pass interleavings, or LTO-style module input) is miscompiled by the documented pipeline.
**Function:** narrow_binops_without_cast (Phase 2)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/passes/narrow.rs:264 (the rewrite at :346-356 checks use_counts == 1, operand narrowability and target width — never the USE's required type)
**Detected by:** Algebraic invariant — value preservation, property P13 (pipeline-confirmed via run_passes probe)
**Minimal input:** fn f(a: i32, b: i32) -> i64 { %w0 = Cast a I32→I64; %w1 = Cast b I32→I64; %r = Or %w0, %w1, I64; Return %r } — call f(1, -1)
**Expected:** -1 (0xFFFFFFFFFFFFFFFF — sext(1) | sext(-1))
**Actual:** 4294967295 (0x00000000FFFFFFFF — the I32 Or result consumed at I64 with no extension)
**Severity:** medium (latent pipeline miscompile; violates the pass's own README restriction "restricted to I32/U32 output" — the code restricts only the rewrite TARGET type, never the output USE)
**Contract evidence:** documented — src/passes/README.md "Integer Narrowing" Phase 2: "narrows I64 bitwise operations (And, Or, Xor) to I32 when all operands are provably 32-bit values ... restricted to I32/U32 output to avoid sub-int register complications" (the "output" restriction is not enforced); plus the pipeline's value-preservation contract (README pipeline overview)
**Fix:** In narrow_binops_without_cast, before rewriting, verify the single use consumes the value in an I32/U32 context (a BinOp/Cmp at target width, a Cast to ≤32 bits, a Store of ≤32-bit type, ...) — or emit the re-extension (Cast target_ty→I64) after the narrowed op when the use needs the I64 value.
**Regression test:** src/passes/narrow.rs `pbt_tests::test_narrow_regression_phase2_i64_bitwise_i64_use` (red witness, run_passes-level)

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib passes::narrow::pbt_tests::test_narrow_regression_phase2_i64_bitwise_i64_use --exact --nocapture
```
**Raw output:**
```
thread 'passes::narrow::pbt_tests::test_narrow_regression_phase2_i64_bitwise_i64_use' panicked at src/passes/narrow.rs:1123:9:
  left: `Value(4294967295)`
 right: `Value(-1)`
test result: FAILED. 0 passed; 1 failed
```
