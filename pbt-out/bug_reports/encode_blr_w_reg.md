# Bug: encode_blr accepts 32-bit W registers as BLR Rn
**Law:** ARM BLR encoding takes Xn only (64-bit). llvm-mc rejects `blr w0` / `blr wzr` / `blr wsp`. encode_blr must Err when Rn is a W/WSP/WZR register.
**Impact:** `blr w0` is assembled as `blr x0` (encoding 0xd63f0000). The object file contains a 64-bit indirect call instead of rejecting illegal 32-bit operands.
**Function:** encode_blr
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0")]
**Expected:** Err (invalid operand; BLR takes Xn only)
**Actual:** Ok(Word(0xd63f0000)) — same encoding as `blr x0`. encode_blr discards the is_64 flag from get_reg.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_blr_pbt::test_encode_blr_regression_w_reg
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_blr_neg_w_reg -- --test-threads=1)
