# Bug: ADDS/SUBS with Rd=SP/WSP encodes as XZR/WZR (CMP/CMN)
**Law:** ARM ARM ADDS/SUBS (immediate) Rd is Wd/Xd, not SP/WSP. llvm-mc rejects `adds sp, x0, #0` and `adds wsp, w0, #0`. Encoding register 31 with S=1 means XZR/WZR, i.e. CMP/CMN, a different instruction.
**Impact:** `adds wsp, w0, #0` is assembled as `adds wzr, w0, #0` (`cmn w0, #0`). Stack-pointer destination is lost.
**Function:** encode_add_sub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0"), Imm(0)], is_sub=false, set_flags=true
**Expected:** Err
**Actual:** Ok(Word) — Rd=31 is accepted for the flag-setting form
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_adds_sp_rd
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_add_sub_neg_adds_sp_rd -- --test-threads=1`
