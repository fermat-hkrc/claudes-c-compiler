# Bug: encode_ldrs accepts SP/WSP as the destination
**Law:** LDRSB/LDRSH destination is Wt or Xt (31 = WZR/XZR), never SP or WSP.
**Impact:** `ldrsb sp, [x1]` encodes as `ldrsb xzr, [x1]` (Rt=31). Object code silently targets ZR instead of rejecting the assembly.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("sp"), Mem { base: "x0", offset: 0 }]`, size=0
**Expected:** Err
**Actual:** Ok(Word(0x3980001f)) — parse_reg_num maps sp/wsp to 31 with no SP-vs-ZR check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_sp_dest (and test_encode_ldrs_regression_wsp_dest)
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_invalid_regs -- --test-threads=1`
