# Bug: encode_adr accepts 32-bit W registers as ADR Rd
**Law:** ARM ADR encoding takes Xd only (64-bit). llvm-mc rejects `adr w0, #imm` and `adr wsp, #0`. encode_adr must Err when Rd is a W/WSP/WZR register.
**Impact:** `adr w0, #-1048576` is assembled as `adr x0, #-1048576` (encoding 0x10800000). The object file contains a 64-bit PC-relative address computation instead of rejecting illegal 32-bit operands.
**Function:** encode_adr
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Imm(-1048576)]
**Expected:** Err (invalid operand; ADR takes Xd only)
**Actual:** Ok(Word(0x10800000)) — same encoding as `adr x0, #-1048576`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_adr_pbt::test_encode_adr_regression_w_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adr_neg_w_reg -- --test-threads=1)
