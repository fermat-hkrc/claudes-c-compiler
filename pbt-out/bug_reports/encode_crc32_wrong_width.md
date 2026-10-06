# Bug: encode_crc32 ignores W vs X register width
**Law:** Every CRC32 form requires Wd and Wn; Rm is Wm for B/H/W and Xm for X. A mismatched width must be rejected.
**Impact:** `crc32b w0, w0, x0` and `crc32x w0, w1, w2` encode as if the width were correct, so invalid GNU-style assembly becomes a well-formed CRC32 word. llvm-mc and gas reject those triples. Callers that pass X as Rd/Rn or W as Rm of crc32x get the wrong instruction class without an error.
**Function:** encode_crc32
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:226
**Detected by:** Negative/Error Contract
**Minimal input:** encode_crc32("crc32b", [Reg("w0"), Reg("w0"), Reg("x0")])
**Expected:** Err
**Actual:** Ok(Word(0x1ac04000)) — Rm width discarded; encoded as crc32b w0, w0, w0
**Severity:** medium
**Root cause:** bitfield.rs:227-229 binds `let (rd, _) = get_reg(...)` (and the same for rn/rm), discarding is_64, then sf/sz come only from the mnemonic.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Keep is_64 and require Wd/Wn plus Wm or Xm by mnemonic.
```rust
    let (rd, rd64) = get_reg(operands, 0)?;
    let (rn, rn64) = get_reg(operands, 1)?;
    let (rm, rm64) = get_reg(operands, 2)?;
    if rd64 || rn64 || rm64 != mnemonic.ends_with('x') {
        return Err("crc32: Rd/Rn must be W; Rm must be W (B/H/W) or X (X)".into());
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_wrong_width -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_crc32_pbt::encode_crc32_neg_wrong_width stdout ----
Test failed: CRC32 crc32b wrong-width rd64=false rn64=false rm64=true must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_crc32_pbt.rs:432.
minimal failing input: m = "crc32b", rd = 0, rn = 0, rm = 0, rd64 = false, rn64 = false, rm64 = true
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
