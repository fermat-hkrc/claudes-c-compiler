# Bug: encode_crc32 accepts FP/SIMD registers as GPRs
**Law:** CRC32 operands are general-purpose W/X registers; FP/SIMD names (d/s/q/v/h/b) must be rejected.
**Impact:** `crc32b d0, w1, w2` encodes as `crc32b w0, w1, w2`. The assembler treats a SIMD register as GPR number N, so invalid GNU-style assembly becomes a CRC32 word. llvm-mc and gas reject those operands.
**Function:** encode_crc32
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:226
**Detected by:** Negative/Error Contract
**Minimal input:** encode_crc32("crc32b", [Reg("d0"), Reg("w1"), Reg("w2")])
**Expected:** Err
**Actual:** Ok(Word(0x1ac24020)) — d0 parses as register 0
**Severity:** medium
**Root cause:** bitfield.rs:227-229 calls get_reg, and parse_reg_num accepts prefixes d/s/q/v/h/b; encode_crc32 never checks is_fp_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject FP/SIMD names after parsing each operand.
```rust
    fn reject_fp(name: &str) -> Result<(), String> {
        let c = name.chars().next().unwrap_or(' ').to_ascii_lowercase();
        if matches!(c, 'd' | 's' | 'q' | 'v' | 'h' | 'b') {
            Err(format!("crc32: FP/SIMD register {name} is not a valid operand"))
        } else {
            Ok(())
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_fp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_crc32_pbt::encode_crc32_neg_fp stdout ----
Test failed: FP/SIMD register d0 is not a valid CRC32 operand (slot=0) at src/backend/arm/assembler/encoder/encode_crc32_pbt.rs:452.
minimal failing input: m = "crc32b", slot = 0, prefix = "d", n = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
