# Bug: encode_cmp accepts SP as Rm without an extend
**Law:** CMP shifted-register Rm is Xm/Wm (register 31 is XZR/WZR). SP as Rm without an extend must be Err.
**Impact:** `cmp x0, sp` encodes as `cmp x0, xzr` (parse_reg_num maps sp→31). That compares against zero, not SP, and diverges from gas/llvm-mc, which reject SP as Rm without uxtx/sxtx.
**Function:** encode_cmp
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("sp")]  (`cmp x0, sp`)
**Expected:** Err
**Actual:** Ok(Word) encoding `cmp x0, xzr`.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmp_pbt::test_encode_cmp_regression_sp_rm
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmp_pbt -- --test-threads=1 reproduced test_encode_cmp_regression_sp_rm.
