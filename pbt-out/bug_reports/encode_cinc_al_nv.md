# Bug: encode_cinc accepts condition codes AL and NV
**Law:** CINC is not a valid alias of CSINC when cond is AL or NV; encode_cinc([Reg(rd), Reg(rn), Cond("al"|"nv")]) must be Err.
**Impact:** `cinc x0, x0, al` encodes as CSINC with inverted cond NV (and `cinc ..., nv` as CSINC with AL). llvm-mc rejects both with "condition codes AL and NV are invalid for this instruction". The assembler README claims gas-compatible input; accepting AL/NV emits a CSINC that cannot be written as CINC.
**Function:** encode_cinc
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Cond("al")]  (`cinc x0, x0, al`)
**Expected:** Err
**Actual:** Ok(Word) — encode_cond maps al=14 / nv=15 and invert(cond)=cond XOR 1 is applied with no AL/NV guard.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinc_pbt::test_encode_cinc_regression_al
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinc_pbt -- --test-threads=1 reproduced encode_cinc_neg_al_nv.
