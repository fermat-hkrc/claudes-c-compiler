# Bug: encode_bl rejects immediate PC-offset form `bl #imm`
**Law:** For every 4-byte-aligned offset in the ARM ARM BL range [-2^27, 2^27-4], encode_bl([Imm(imm)]) must equal Word(llvm-mc("bl #imm")).
**Impact:** GNU-style assembly that uses an explicit PC offset (`bl #0`, `bl #4`, `bl #-134217728`) fails to assemble. The built-in assembler claims gas compatibility, so this is a missing encoding path; codegen currently emits `bl <name>` so the hole is latent for compiler output but user/hand-written `.s` files are rejected.
**Function:** encode_bl
**Detected by:** Differential — llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** Imm(-134217728)  (`bl #-134217728`); also Imm(0) and Imm(4)
**Expected:** Word matching llvm-mc: `bl #0` → 0x94000000, `bl #4` → 0x94000001, `bl #-134217728` → 0x96000000
**Actual:** Err("expected symbol at operand 0, got Some(Imm(...))") because encode_bl only calls get_symbol and never encodes the ARM ARM imm26 field.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_bl_regression_imm_offset` (and KATs `encode_bl_kat_llvm_mc_bl_imm0`, `encode_bl_kat_llvm_mc_bl_imm4`)
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_bl_pbt -- --test-threads=1 reproduced the failure.
