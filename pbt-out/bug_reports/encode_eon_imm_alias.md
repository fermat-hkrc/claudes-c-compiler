# Bug: encode_eon rejects the GNU EON-immediate alias
**Law:** Gas/llvm-mc accept `eon Rd, Rn, #imm` as an alias of `eor Rd, Rn, #~imm` when ~imm is a valid AArch64 bitmask. The built-in assembler claims gas-compatible textual assembly (README.md:5-14) and dispatches every `eon` mnemonic to encode_eon.
**Impact:** Valid GNU assembly such as `eon w0, w0, #0xaaaaaaaa` (llvm-mc: `eor w0, w0, #0x55555555`, encoding 0x5200f000) fails to assemble. Sibling `encode_bic` already implements the corresponding AND #~imm alias; EON is missing the EOR counterpart.
**Function:** encode_eon
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** `eon w0, w0, #0xaaaaaaaa` (rd=0, rn=0, is_64=false, seed=0)
**Expected:** Ok(Word(0x5200f000)) matching llvm-mc `eor w0, w0, #0x55555555`
**Actual:** Err("expected register at operand 2, got Some(Imm(2863311530))") — get_reg is applied to operand 2 with no Imm path.
**Severity:** medium
**Regression test:** `test_encode_eon_regression_imm` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_eon_diff_imm -- --test-threads=1`
