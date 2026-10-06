# Bug: encode_crc32 accepts SP/WSP as register 31
**Law:** CRC32 Rd, Rn, and Rm are ZR not SP; sp/wsp in any slot must be rejected.
**Impact:** `crc32b wsp, w1, w2` is encoded as `crc32b wzr, w1, w2`. The assembler accepts GNU-style text that llvm-mc and gas reject, and silently substitutes WZR/XZR for the stack pointer.
**Function:** encode_crc32
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:226
**Detected by:** Negative/Error Contract
**Minimal input:** encode_crc32("crc32b", [Reg("wsp"), Reg("w0"), Reg("w0")])
**Expected:** Err
**Actual:** Ok(Word(0x1ac0401f)) — wsp maps to Rd=31 (WZR)
**Severity:** medium
**Root cause:** bitfield.rs:227-229 calls get_reg, and parse_reg_num maps "sp"|"wsp" to 31 the same as xzr/wzr; encode_crc32 never distinguishes SP from ZR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject SP/WSP after parsing each register name.
```rust
    fn reject_sp(name: &str) -> Result<(), String> {
        let n = name.to_lowercase();
        if n == "sp" || n == "wsp" {
            Err(format!("crc32: SP/WSP is not a valid operand ({name})"))
        } else {
            Ok(())
        }
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_sp -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_crc32_pbt::encode_crc32_neg_sp stdout ----
Test failed: SP/WSP is not a valid CRC32 operand (slot=0 sp=wsp) at src/backend/arm/assembler/encoder/encode_crc32_pbt.rs:407.
minimal failing input: m = "crc32b", slot = 0, sp64 = false, other = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
