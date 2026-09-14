# Bug: encode_cinv accepts condition codes AL and NV
**Law:** CINV is not a valid alias of CSINV when cond is AL or NV; encode_cinv([Reg(rd), Reg(rn), Cond("al"|"nv")]) must be Err.
**Impact:** `cinv x0, x0, al` encodes as CSINV with inverted cond NV (and `cinv ..., nv` as CSINV with AL). llvm-mc rejects both with "condition codes AL and NV are invalid for this instruction". The assembler README claims gas-compatible input; accepting AL/NV emits a CSINV that cannot be written as CINV.
**Function:** encode_cinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Cond("al")]  (`cinv x0, x0, al`)
**Expected:** Err
**Actual:** Ok(Word) — encode_cond maps al=14 / nv=15 and invert(cond)=cond XOR 1 is applied with no AL/NV guard.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinv_pbt::test_encode_cinv_regression_al
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinv_neg -- --test-threads=1 reproduced encode_cinv_neg_al_nv.
