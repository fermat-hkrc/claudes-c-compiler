# Bug: encode_orn accepts NEON ORN arrangements other than 8b/16b
**Law:** ARM ARM Advanced SIMD three-same ORN is a bitwise operation: T ∈ {8B,16B}. llvm-mc rejects `orn v0.8h, v0.8h, v0.8h` and the other non-byte arrangements.
**Impact:** `orn v0.8h, v0.8h, v0.8h` encodes as 8-bit Q=0 ORN (`orn v0.8b, ...`). An illegal arrangement is assembled as a different vector instruction.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn v0.8h, v0.8h, v0.8h` (d=n=m=0, arr="8h")
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — Q is 1 iff arr_d == "16b", else 0; 8h/4h/4s/2s/2d/1d all take the 8b encoding.
**Severity:** medium
**Regression test:** `test_encode_orn_regression_invalid_neon_arr` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_orn_neg -- --test-threads=1`
