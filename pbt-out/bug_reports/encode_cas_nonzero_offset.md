# Bug: encode_cas ignores a nonzero memory offset
**Law:** ARM ARM optional offset is only `#0`; gas: "the optional immediate offset can only be 0"; llvm-mc rejects any `#imm`.
**Impact:** `cas w0, w0, [x0, #-1]` (and `#8`, `i64::MIN/MAX`) encodes as `cas w0, w0, [x0]` because `Operand::Mem { base, .. }` discards offset. Silent drop of an illegal addressing mode.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** v=0, rs=0, rt=0, rn=0, is_64=false, off=-1 — `cas w0, w0, [x0, #-1]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas w0, w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_nonzero_offset
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1
