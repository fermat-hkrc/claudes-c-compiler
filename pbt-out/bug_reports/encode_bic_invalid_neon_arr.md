# Bug: encode_bic NEON form accepts arrangements other than 8b/16b
**Law:** ARM ARM BIC (vector, three-same) arrangement T is 8B or 16B only. `bic Vd.8h, Vn.8h, Vm.8h` (and 4h/4s/2s/2d) must be rejected.
**Impact:** `bic v0.8h, v1.8h, v2.8h` is encoded as `bic v0.8b, v1.8b, v2.8b` (Q=0). The assembler claims gas-compatible AArch64; llvm-mc rejects this form (it is the NEON-immediate opcode, not three-same). Callers get the wrong vector width with no error.
**Function:** encode_bic (via encode_neon_bic)
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bic v0.8h, v1.8h, v2.8h` (d=0, n=0, m=0, arr=8h)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 255]")
**Actual:** Ok(Word) — `encode_neon_bic` sets `q = if arr_d == "16b" { 1 } else { 0 }`, so every non-16b arrangement is encoded as 8b.
**Severity:** medium
**Regression test:** `test_encode_bic_regression_neon_8h` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bic_neg_invalid_neon_arr -- --test-threads=1`
