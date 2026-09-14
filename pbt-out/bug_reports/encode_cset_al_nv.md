# Bug: encode_cset accepts condition codes AL and NV
**Law:** CSET is not a valid alias of CSINC when cond is AL or NV; encode_cset([Reg(rd), Cond("al"|"nv")]) must be Err.
**Impact:** `cset x0, al` encodes as CSINC with inverted cond NV (0x9a9ff7e0) and `cset x0, nv` as CSINC with AL (0x9a9fe7e0). llvm-mc rejects both with "condition codes AL and NV are invalid for this instruction". The assembler README claims gas-compatible input; accepting AL/NV emits a CSINC that cannot be written as CSET.
**Function:** encode_cset
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Cond("al")]  (`cset x0, al`)
**Expected:** Err
**Actual:** Ok(Word(0x9a9ff7e0)) — encode_cond maps al=14 / nv=15 and invert(cond)=cond XOR 1 is applied with no AL/NV guard.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cset_pbt::test_encode_cset_regression_al
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cset_pbt -- --test-threads=1 reproduced encode_cset_neg_al_nv with rd="x0", which=0.
