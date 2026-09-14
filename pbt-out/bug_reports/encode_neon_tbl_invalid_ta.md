# Bug: encode_neon_tbl accepts Ta outside {8B,16B}
**Law:** ARM ARM Advanced SIMD table lookup TBL allows Ta only in {8B,16B}. Other arrangements (4H, 8H, 2S, 4S, 2D, 1D) must be rejected with Err.
**Impact:** `tbl v0.4h, {v0.16b}, v0.4h` is encoded as Q=0 TBL (same as 8B), producing a well-formed but wrong instruction that gas/llvm-mc reject.
**Function:** encode_neon_tbl
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** rd=0, rn=0, rm=0, ta="4h", n=1 — `tbl v0.4h, {v0.16b}, v0.4h`
**Expected:** Err
**Actual:** Ok(Word(...)) — `q` is `1` only when `arr_d == "16b"`, so every other arrangement including 4h encodes as Q=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_tbl_pbt::test_encode_neon_tbl_regression_invalid_ta
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_tbl -- --test-threads=1`
