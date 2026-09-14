# Bug: encode_cmn panics on Imm(i64::MIN)
**Law:** Immediate values that are not an unshifted imm12 and not (imm12<<12) must return Err, including i64::MIN (whose absolute value cannot be formed as a positive add/sub immediate).
**Impact:** `encode_cmn([Reg("x0"), Imm(i64::MIN)])` panics with `attempt to negate with overflow` in encode_add_sub instead of returning Err. A panic in the assembler is a crash on a value the Operand::Imm API accepts.
**Function:** encode_cmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Imm(-9223372036854775808)]  (`cmn x0, #i64::MIN`)
**Expected:** Err
**Actual:** panic at data_processing.rs:314 (`(-imm_signed) as u64` when imm_signed == i64::MIN)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmn_pbt::test_encode_cmn_regression_imm_min_overflow
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmn_neg -- --test-threads=1 reproduced encode_cmn_neg_imm_oor (shrunk CE rn="x0", imm=-9223372036854775808).
