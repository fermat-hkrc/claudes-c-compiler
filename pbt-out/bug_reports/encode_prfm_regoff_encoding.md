# Bug: encode_prfm register-offset form uses the wrong opcode bits
**Law:** ARM ARM PRFM (register) is `11 111 0 00 10 1 Rm option S 10 Rn Rt` (base 0xF8A00800). The SUT comment at load_store.rs:764 states the same encoding. llvm-mc emits 0xF8A16800 for `prfm pldl1keep, [x0, x1]`.
**Impact:** Every PRFM (register) instruction is encoded as 0xF92xxxxx instead of 0xF8Axxxxx — bits 24:22 are `100` rather than `010`. Disassemblers will not decode this as PRFM (register); the prefetch is wrong machine code.
**Function:** encode_prfm
**Detected by:** Differential vs llvm-mc (and ARM ARM bitfields)
**Minimal input:** `prfm pldl1keep, [x0, x1]` (also shrunk `[x0, x0]`)
**Expected:** 0xF8A16800 (llvm-mc / ARM ARM)
**Actual:** 0xF9216800 — `(0b10 << 23)` was used instead of `(0b10 << 22)`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_prfm_pbt::test_encode_prfm_regression_regoff_encoding
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_prfm_diff_regoff_llvm_mc and encode_prfm_kat_llvm_mc_regoff_x0_x1)
