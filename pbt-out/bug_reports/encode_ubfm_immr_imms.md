# Bug: encode_ubfm accepts out-of-range immr/imms
**Law:** UBFM immediates must satisfy 0 <= immr,imms < R, where R is 32 for W-form and 64 for X-form. Values outside that range (negative, R, R+1) must be rejected.
**Impact:** `ubfm w0, w0, #-1, #0` is encoded instead of rejected: `(-1i64) as u32` wraps to 0xFFFFFFFF and the unmasked `immr << 16` corrupts bits above the 6-bit immr field. llvm-mc reports `immediate must be an integer in range [0, 31]` (W) or `[0, 63]` (X). The same path encodes immr/imms = R (32/64) rather than Err.
**Function:** encode_ubfm
**Detected by:** Negative/Error Contract
**Minimal input:** is_64=false, rd=0, rn=0, immr=-1, imms=0 — `ubfm w0, w0, #-1, #0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — get_imm returns the i64 and encode_ubfm casts to u32 with no range check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_ubfm_regression_immr_neg
