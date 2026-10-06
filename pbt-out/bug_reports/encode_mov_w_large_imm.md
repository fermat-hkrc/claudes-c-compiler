# Bug: encode_mov truncates a 64-bit immediate on a W destination
**Law:** `mov w0, #0x0101010101010101` is invalid; GNU as reports "immediate cannot be moved by a single instruction"
**Impact:** A 64-bit repeating pattern on a 32-bit dest is silently truncated to `mov w0, #0x01010101`, so the assembled constant is not the literal the programmer wrote
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:110
**Detected by:** Negative/error contract — gas/llvm-mc reject 64-bit literals on W
**Minimal input:** encode_mov([Reg("w0"), Imm(0x0101010101010101)])
**Expected:** Err
**Actual:** Ok(Word(0x3200c3e0)) — ORR W0, WZR, #0x01010101
**Severity:** medium
**Root cause:** encode_bitmask_imm is called with `imm as u64` and `is_64=false`, which masks to 32 bits and succeeds on the truncated pattern
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:110`
```rust
        if let Some((n, immr, imms)) = encode_bitmask_imm(imm as u64, is_64) {
            let sf = sf_bit(is_64);
            // ORR Rd, XZR, #imm: sf 01 100100 N immr imms 11111 Rd
            let word = (sf << 31) | (0b01 << 29) | (0b100100 << 23) | (n << 22) | (immr << 16) | (imms << 10) | (0b11111 << 5) | rd;
```
**Suggested fix:** For a W dest, reject immediates whose value is not equal to the 32-bit zero- or sign-extended form gas would accept as a 32-bit literal.
```rust
        if !is_64 {
            let u = imm as u64;
            if u > 0xFFFF_FFFF && (imm as i64) != (imm as i32) as i64 {
                return Err("32-bit mov immediate out of range".to_string());
            }
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_regression_w_large_imm -- --test-threads=1
```
**Raw output:**
```text
W dest with 64-bit imm must Err, got Ok(Word(838910944))
minimal failing input: rd = 0, imm = 72340172838076673
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
