# Bug: encode_logical encodes ANDS on NEON registers
**Law:** ANDS is a GPR flag-setting logical; there is no NEON ANDS. llvm-mc rejects `ands v0.8b, v0.8b, v0.8b`.
**Impact:** opc=11 takes the NEON path and encodes as EOR-like (U=1, size=00), emitting a valid-looking EOR encoding for an invalid mnemonic.
**Function:** encode_logical (via encode_neon_logical)
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** three `RegArrangement{v0, "8b"}`, opc=0b11 (`ands v0.8b, v0.8b, v0.8b`)
**Expected:** Err
**Actual:** Ok(Word) — encode_neon_logical maps opc=0b11 to `(u=1, size=00)` with comment "ANDS - not valid for NEON, fall back"
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_logical_regression_ands_neon
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1
