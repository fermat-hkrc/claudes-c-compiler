# Bug: encode_cbz rejects immediate PC-offset form `cbz/cbnz Rt, #imm`
**Law:** For every GPR Rt and every 4-byte-aligned offset in the ARM ARM CBZ/CBNZ range [-2^20, 2^20-4], encode_cbz([Reg(rt), Imm(imm)], is_nz) must equal Word(llvm-mc("cbz/cbnz rt, #imm")).
**Impact:** GNU-style assembly that uses an explicit PC offset (`cbz x0, #0`, `cbnz x0, #4`, `cbz x0, #-1048576`) fails to assemble. The built-in assembler claims gas compatibility, so this is a missing encoding path; codegen currently emits `cbz xN, .Llabel` so the hole is latent for compiler output but user/hand-written `.s` files are rejected.
**Function:** encode_cbz
**Detected by:** Differential — llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** [Reg("x0"), Imm(-1048576)], is_nz=false  (`cbz x0, #-1048576`); also Imm(0) and Imm(4)
**Expected:** Word matching llvm-mc: `cbz x0, #0` → 0xb4000000, `cbz w0, #0` → 0x34000000, `cbnz x0, #4` → 0xb5000020, `cbz x0, #-1048576` → 0xb4800000
**Actual:** Err("expected symbol at operand 1, got Some(Imm(...))") because encode_cbz only calls get_symbol for operand 1 and never encodes the ARM ARM imm19 field.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs `test_encode_cbz_regression_imm_offset` (and KATs `encode_cbz_kat_llvm_mc_cbz_x0_imm0`, `encode_cbz_kat_llvm_mc_cbz_w0_imm0`, `encode_cbz_kat_llvm_mc_cbnz_x0_imm4`)
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cbz -- --test-threads=1 reproduced the failure.
