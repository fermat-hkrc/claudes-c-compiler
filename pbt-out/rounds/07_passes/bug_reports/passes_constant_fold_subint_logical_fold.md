# Bug: constant_fold's 32-bit fast path sign-extends sub-int operands before logical ops (LShr/UDiv/URem at I8/I16 fold wrong)
**Law:** A BinOp typed I8/I16 denotes an operation on the 8/16-bit value domain: `LShr` is a LOGICAL shift of the zero-extended sub-int bit pattern. Folding `LShr((I8)0xB8, 1)` must yield 92.
**Impact:** Any I8/I16-typed logical shift / unsigned div / unsigned rem instruction that reaches constant_fold with a negative constant operand silently folds to the wrong constant (the sign bits dragged down into the result). The pipeline re-runs constant_fold after narrow has moved ops down to sub-int types (narrow Phase 1 targets the narrowing cast's type, including I8/I16), so this is a latent silent-miscompile path in `-O` pipelines.
**Function:** fold_binop (called from try_fold_with_map's `is_32bit` arms)
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/passes/constant_fold.rs:684 (fold_binop; LShr arm at :729, same pattern in UDiv/URem arms)
**Detected by:** Differential — P5 fold-vs-C11-interpreter over random all-constant DAGs at mixed widths
**Minimal input:** one-instruction function `d = LShr(Const(I8(-72)), Const(I8(1)), I8)`; return d. (Also `UDiv(Const(I8(-72)), Const(I8(2)), I8)`.)
**Expected:** 92 (0xB8 >>u 1 = 0x5C; 0xB8 /u 2 = 0x5C)
**Actual:** -36 (0xDC) — `(lhs as u32)` sign-extends the i8 -72 to 0xFFFFFFB8 before the logical shift; 0xFFFFFFB8 >> 1 low byte = 0xDC
**Severity:** medium (IR-level latent miscompile; no C-level repro found — the frontend promotes sub-int ops to I32/I64 before emitting shifts, and straightforward C sources do not emit an I8-typed LShr — but the optimizer's own narrow Phase 1 can create exactly that instruction shape and the next constant_fold iteration folds it wrong)
**Contract evidence:** inferred (the IrType-typed BinOp semantics: `ty` is the operation's value domain — the pass itself truncates operands to `ty` first (`ty.truncate_i64`), so the logical-op arms must also zero-extend within `ty`'s width; the `as u32`/`as i32` casts model the 32-bit types, not I8/I16)
**Fix:** In fold_binop's `is_32bit` logical arms, mask to the operand width before widening: for LShr/UDiv/URem use `((lhs as u64) & mask_of(ty)) as u32`-equivalent (zero-extend the sub-int pattern); AShr's `as i32` sign-extension is promotion-correct and can stay. Same fix needed for the UDiv and URem arms.
**Regression test:** src/passes/constant_fold.rs `pbt_tests::test_constant_fold_regression_subint_lshr_zero_extend` (red witness, also covers the UDiv form)

**Reproduction:**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" RUST_TEST_THREADS=1 cargo test --lib passes::constant_fold::pbt_tests::test_constant_fold_regression_subint_lshr_zero_extend --exact --nocapture
```
**Raw output:**
```
thread 'passes::constant_fold::pbt_tests::test_constant_fold_regression_subint_lshr_zero_extend' panicked at src/passes/constant_fold.rs:1395:13:
  left: `Value(-36)`
 right: `Value(92)`
test result: FAILED. 0 passed; 1 failed
```
