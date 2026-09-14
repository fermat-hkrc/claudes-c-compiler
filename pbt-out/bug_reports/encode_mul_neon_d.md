# Bug: encode_mul accepts NEON MUL with 64-bit elements (size==11 UNDEFINED)
**Law:** ARM ARM Advanced SIMD MUL (vector) is UNDEFINED when size==11. Arrangements 1D and 2D are not valid T for `mul Vd.T, Vn.T, Vm.T` (gas/llvm-mc "invalid operand").
**Impact:** `mul v0.1d, v0.1d, v0.1d` (and `.2d`) is encoded instead of rejected. The resulting word has size=11, which ARM ARM marks UNDEFINED — hardware may UNDEF. Note: `encode_instruction` currently routes NEON MUL to `encode_neon_three_same` rather than `encode_mul`, so this path is encode_mul's own NEON branch (`encode_neon_mul`), not the public dispatch.
**Function:** encode_mul
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `encode_mul([RegArrangement{v0,1d}, RegArrangement{v0,1d}, RegArrangement{v0,1d}])` i.e. `mul v0.1d, v0.1d, v0.1d`
**Expected:** Err
**Actual:** Ok(Word) with size=11. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_mul_pbt::test_encode_mul_regression_neon_d
