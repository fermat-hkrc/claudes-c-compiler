# Bug: encode_csetm accepts condition codes AL and NV
**Law:** CSETM is not a valid alias of CSINV when cond is AL or NV; encode_csetm([Reg(rd), Cond("al"|"nv")]) must be Err.
**Impact:** `csetm x0, al` encodes as CSINV with inverted cond NV (0xda9ff3e0) and `csetm x0, nv` as CSINV with AL (0xda9fe3e0). llvm-mc rejects both with "condition codes AL and NV are invalid for this instruction". The assembler README claims gas-compatible input; accepting AL/NV emits a CSINV that cannot be written as CSETM.
**Function:** encode_csetm
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Cond("al")]  (`csetm x0, al`)
**Expected:** Err
**Actual:** Ok(Word(0xda9ff3e0)) — encode_cond maps al=14 / nv=15 and invert(cond)=cond XOR 1 is applied with no AL/NV guard.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_csetm_pbt::test_encode_csetm_regression_al
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_csetm_neg -- --test-threads=1 reproduced encode_csetm_neg_al_nv with rd="x0", which=0.
