# Bug: encode_uxtb emits 64-bit UBFM for an X destination
**Law:** UXTB is the 32-bit-only alias of UBFM Wd, Wn, #0, #7. An X destination must encode the same 32-bit word as the W form (llvm-mc/gas canonicalize `uxtb x0, w0` to `uxtb w0, w0`).
**Impact:** `uxtb x0, w0` assembles as `ubfx x0, x0, #0, #8` (word 0xD3401C00) instead of `uxtb w0, w0` (word 0x53001C00). Object files diverge from GNU as / llvm-mc; disassembly no longer shows UXTB.
**Function:** encode_uxtb
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:900
**Detected by:** Differential — llvm-mc AArch64 assembler
**Minimal input:** `[Reg("x0"), Reg("w0")]` (shrunk; dest64=true, rd=0, rn=0)
**Expected:** `Ok(Word(0x53001C00))` — same encoding as `uxtb w0, w0`
**Actual:** `Ok(Word(0xD3401C00))` — 64-bit UBFM with sf=1 N=1
**Severity:** medium
**Root cause:** data_processing.rs:901-904 takes sf and N from Rd's width, so an X destination sets the 64-bit UBFM form. ARM C6 UXTB has no 64-bit alias (a W write already zero-extends to 64 bits).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:901`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0 };
```
**Suggested fix:** Always encode the 32-bit UXTB form; do not take sf from Rd.
```rust
    let (rd, _is_64) = get_reg(operands, 0)?;
    let (rn, rn_is_64) = get_reg(operands, 1)?;
    if rn_is_64 {
        return Err("uxtb: source must be a W register".to_string());
    }
    let word = (0b10u32 << 29) | (0b100110 << 23) | (7 << 10) | (rn << 5) | rd;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxtb_regression_x_dest -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_uxtb_pbt::encode_uxtb_diff_valid_gpr stdout ----
Test failed: assertion failed: `(left == right)`
  left: `3544194048`,
 right: `1392516096`: UXTB mismatch for uxtb x0, w0 at src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs:233.
minimal failing input: rd = 0, rn = 0, dest64 = true, use_lr = false
	successes: 1
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
