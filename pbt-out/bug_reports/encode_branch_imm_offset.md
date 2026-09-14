# Bug: encode_branch rejects immediate PC-offset form `b #imm`
**Law:** For every 4-byte-aligned offset in the ARM ARM B range [-2^27, 2^27-4], encode_branch([Imm(imm)]) must equal Word(llvm-mc("b #imm")).
**Impact:** GNU-style assembly that uses an explicit PC offset (`b #0`, `b #4`, `b #-134217728`) fails to assemble. The built-in assembler claims gas compatibility, so this is a missing encoding path; codegen currently emits `b <label>` so the hole is latent for compiler output but user/hand-written `.s` files are rejected.
**Function:** encode_branch
**Detected by:** Differential — llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** Imm(-134217728)  (`b #-134217728`); also Imm(0) and Imm(4)
**Expected:** Word matching llvm-mc: `b #0` → 0x14000000, `b #4` → 0x14000001, `b #-134217728` → 0x16000000
**Actual:** Err("expected symbol at operand 0, got Some(Imm(...))") because encode_branch only calls get_symbol and never encodes the ARM ARM imm26 field.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_branch_regression_imm_offset` (and KATs `encode_branch_kat_llvm_mc_b_imm0`, `encode_branch_kat_llvm_mc_b_imm4`)
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_branch_pbt -- --test-threads=1 reproduced the failure.
