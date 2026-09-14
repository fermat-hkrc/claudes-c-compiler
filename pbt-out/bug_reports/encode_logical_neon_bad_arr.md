# Bug: encode_logical accepts NEON arrangements other than 8b/16b
**Law:** Advanced SIMD logical three-same is defined only for T in {8B, 16B}. llvm-mc rejects `and v0.4s, v0.4s, v0.4s`.
**Impact:** `and v0.4s, ...` is encoded as the 8B form (Q=0 because arrangement != "16b"), producing the wrong instruction.
**Function:** encode_logical (via encode_neon_logical)
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands three `RegArrangement{v0, "4s"}`, opc=0 (`and v0.4s, v0.4s, v0.4s`)
**Expected:** Err
**Actual:** Ok(Word) — Q is `arr_d == "16b"` with no other T check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_neon_bad_arr
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
