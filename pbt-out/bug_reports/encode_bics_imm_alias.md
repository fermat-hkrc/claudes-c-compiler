# Bug: encode_bics rejects the GNU BICS-immediate alias
**Law:** Gas/llvm-mc accept `bics Rd, Rn, #imm` as an alias of `ands Rd, Rn, #~imm` when ~imm is a valid AArch64 bitmask. The built-in assembler claims gas-compatible textual assembly (README.md:5-14) and dispatches every `bics` mnemonic to encode_bics.
**Impact:** Valid GNU assembly such as `bics w0, w0, #0xaaaaaaaa` (llvm-mc: `ands w0, w0, #0x55555555`, encoding 0x7200f000) fails to assemble. Sibling `encode_bic` already implements the corresponding AND #~imm alias; BICS is missing the ANDS counterpart.
**Function:** encode_bics
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** `bics w0, w0, #0xaaaaaaaa` (rd=0, rn=0, is_64=false, seed=0)
**Expected:** Ok(Word(0x7200f000)) matching llvm-mc `ands w0, w0, #0x55555555`
**Actual:** Err("expected register at operand 2, got Some(Imm(2863311530))") — get_reg is applied to operand 2 with no Imm path.
**Severity:** medium
**Regression test:** `test_encode_bics_regression_imm` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bics_diff_imm -- --test-threads=1`
