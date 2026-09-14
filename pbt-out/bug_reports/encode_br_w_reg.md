# Bug: encode_br accepts 32-bit W registers as BR Rn
**Law:** ARM BR encoding takes Xn only (64-bit). llvm-mc rejects `br w0` / `br wzr` / `br wsp`. encode_br must Err when Rn is a W/WSP/WZR register.
**Impact:** `br w0` is assembled as `br x0` (encoding 0xd61f0000). The object file contains a 64-bit indirect jump instead of rejecting illegal 32-bit operands.
**Function:** encode_br
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0")]
**Expected:** Err (invalid operand; BR takes Xn only)
**Actual:** Ok(Word(0xd61f0000)) — same encoding as `br x0`. encode_br discards the is_64 flag from get_reg.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_br_pbt::test_encode_br_regression_w_reg
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_br_neg_w_reg -- --test-threads=1)
