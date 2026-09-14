# Bug: encode_umulh encodes FP/SIMD register names as GPRs
**Law:** UMULH operands are general-purpose X registers, not FP/SIMD (d/s/q/v/h/b).
**Impact:** `umulh d0, x1, x2` is encoded as `umulh x0, x1, x2`. The same happens for s/q/v/h/b prefixes. gas / llvm-mc reject these as invalid operands.
**Function:** encode_umulh
**Detected by:** Negative/Error Contract
**Minimal input:** which=0, prefix="d", n=0 — `umulh d0, x1, x2` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) encoding Rd=0 — parse_reg_num accepts 'd'|'s'|'q'|'v'|'h'|'b' prefixes and returns the numeric suffix
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_umulh_regression_fp
