# Bug: encode_neon_add_sub ignores a fourth operand
**Law:** Vector ADD/SUB take exactly three operands; a fourth operand must be rejected
**Impact:** The assembler silently encodes `add Vd.T, Vn.T, Vm.T, extra` as `add Vd.T, Vn.T, Vm.T`, dropping the extra operand instead of diagnosing invalid assembly
**Function:** encode_neon_add_sub
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1164
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_add_sub([v0.8b, v0.8b, v0.8b, v0.8b], is_sub=false)
**Expected:** Err (llvm-mc/gas reject a fourth operand)
**Actual:** Ok(Word(0x0e208400)) — same as `add v0.8b, v0.8b, v0.8b`
**Severity:** medium
**Root cause:** neon.rs:1164-1167 reads only operands 0..2 via get_neon_reg and never checks `operands.len()`, so any extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1164`
```rust
pub(crate) fn encode_neon_add_sub(operands: &[Operand], is_sub: bool) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("add/sub requires 3 operands".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_add_sub_regression_extra_operand -- --test-threads=1 --exact
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_add_sub_pbt::encode_neon_add_sub_neg_extra_operand' (2332177) panicked at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:221:1:
Test failed: 4 operands must Err (llvm-mc rejects add v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs:341.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", is_sub = false
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_add_sub_pbt.rs
