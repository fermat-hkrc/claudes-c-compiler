# Bug: encode_cneg accepts condition codes AL and NV
**Law:** CNEG is not a valid alias of CSNEG when cond is AL or NV; encode_cneg([Reg(rd), Reg(rn), Cond("al"|"nv")]) must be Err.
**Impact:** `cneg x0, x0, al` encodes as CSNEG with inverted cond NV (and `cneg ..., nv` as CSNEG with AL). llvm-mc rejects both with "condition codes AL and NV are invalid for this instruction". The assembler README claims gas-compatible input; accepting AL/NV emits a CSNEG that cannot be written as CNEG.
**Function:** encode_cneg
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("x0"), Cond("al")]  (`cneg x0, x0, al`)
**Expected:** Err
**Actual:** Ok(Word) — encode_cond maps al=14 / nv=15 and invert(cond)=cond XOR 1 is applied with no AL/NV guard.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cneg_pbt::test_encode_cneg_regression_al
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cneg_neg -- --test-threads=1 reproduced encode_cneg_neg_al_nv.
