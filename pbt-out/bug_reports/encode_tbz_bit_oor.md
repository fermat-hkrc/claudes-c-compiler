# Bug: encode_tbz masks out-of-range bit numbers instead of rejecting them
**Law:** ARM ARM and llvm-mc require the bit immediate in [0, 31] for W registers and [0, 63] for X registers. encode_tbz([Reg(rt), Imm(bit), Symbol(s)], is_nz) must be Err when bit is outside that range (including −1, 32 on W, 64 on X).
**Impact:** `tbz w0, #-1, L` encodes as `tbz x0, #63, L` (bit −1 wraps to 0xFFFFFFFF, b5=1, b40=31). `tbz w0, #32, L` encodes as `tbz x0, #32, L`. llvm-mc rejects both. A 32-bit test-and-branch can silently become a 64-bit one.
**Function:** encode_tbz
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:254
**Detected by:** Negative/Error Contract
**Minimal input:** encode_tbz([Reg("w0"), Imm(-1), Symbol("L")], false); also Imm(32) on W
**Expected:** Err
**Actual:** Ok(WordWithReloc { word: 0xb6f80000, reloc: TstBr14 symbol=L addend=0 }) for bit=-1; Ok(word: 0xb6000000) for bit=32 on W
**Severity:** medium
**Root cause:** compare_branch.rs:258-259 casts bit to u32 and masks to 6 bits (`>> 5 & 1` and `& 0x1F`) with no range check; width from get_reg is discarded so W vs X cannot be enforced.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:258`
```rust
    let b5 = ((bit as u32) >> 5) & 1;
```
**Suggested fix:** Reject out-of-range bits using the register width.
```rust
    let (rt, is_64) = get_reg(operands, 0)?;
    let bit = get_imm(operands, 1)?;
    let max_bit = if is_64 { 63 } else { 31 };
    if bit < 0 || bit > max_bit {
        return Err(format!("tbz bit {} out of range 0..{}", bit, max_bit));
    }
    let b5 = ((bit as u32) >> 5) & 1;
    let b40 = (bit as u32) & 0x1F;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_bit_oor -- --test-threads=1
```
**Raw output:**
```text
---- backend::arm::assembler::encoder::encode_tbz_pbt::encode_tbz_neg_bit_oor stdout ----
Test failed: tbz w0, #-1, L is out of bit range and must Err at src/backend/arm/assembler/encoder/encode_tbz_pbt.rs:591.
minimal failing input: n = 0, is_64 = false, is_nz = false, which = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_tbz_pbt.rs
