# Bug: encode_uxtb accepts FP/SIMD registers as GPRs
**Law:** UXTB operands are general-purpose registers. An FP/SIMD register (d/s/q/v/h/b) must return Err, matching llvm-mc (`invalid operand for instruction`).
**Impact:** `uxtb d0, w1` encodes as `uxtb w0, w1` because parse_reg_num accepts the `d` prefix and returns 0. Illegal SIMD assembly becomes a well-formed GPR word.
**Function:** encode_uxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:900
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("w1")]` (shrunk; which=0, prefix="d", n=0)
**Expected:** `Err`
**Actual:** `Ok(Word(0x53001C20))` — d0 parsed as register 0
**Severity:** medium
**Root cause:** data_processing.rs:901 calls get_reg → parse_reg_num, which accepts prefixes d/s/q/v/h/b; encode_uxtb does not reject FP/SIMD names.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:901`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject FP/SIMD register names before encoding.
```rust
    fn is_fp_name(name: &str) -> bool {
        matches!(name.chars().next().unwrap_or(' ').to_ascii_lowercase(), 'd' | 's' | 'q' | 'v' | 'h' | 'b')
    }
    if operands.iter().any(|op| matches!(op, Operand::Reg(n) if is_fp_name(n))) {
        return Err("uxtb: FP/SIMD register is not a valid operand".to_string());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxtb_regression_fp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_uxtb_pbt::encode_uxtb_neg_fp stdout ----
Test failed: FP/SIMD register d0 is not a valid UXTB operand (which=0) at src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs:365.
minimal failing input: which = 0, prefix = "d", n = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
