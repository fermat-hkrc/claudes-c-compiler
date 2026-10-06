# Bug: encode_mov WSP form is not ADD (S bit / 10001 layout)
**Law:** MOV to/from WSP must use ADD (immediate) #0: op=0, S=0, bits[28:24]=10001, imm12=0
**Impact:** The ARM ADD-SP layout does not hold for `wsp`, so 32-bit SP moves are encoded as logical ORR
**Function:** encode_mov
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:129
**Detected by:** Algebraic — Invariant (ARM ADD vs ORR field layout)
**Minimal input:** encode_mov([Reg("wsp"), Reg("w0")])
**Expected:** ADD layout with S=0
**Actual:** ORR layout (S bit reads as 1 in the ADD-field unpack)
**Severity:** high
**Root cause:** data_processing.rs:129 compares only the exact name `sp`, so `wsp` falls through to ORR
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:129`
```rust
        if rd_name.to_lowercase() == "sp" || rm_name.to_lowercase() == "sp" {
            let sf = sf_bit(is_64);
            // ADD Xd, Xn, #0: sf 0 0 10001 00 imm12=0 Rn Rd
            let word = ((sf << 31) | (0b10001 << 24)) | (rm << 5) | rd;
            return Ok(EncodeResult::Word(word));
        }
```
**Suggested fix:** Treat `wsp` the same as `sp`.
```rust
        let rd_l = rd_name.to_lowercase();
        let rm_l = rm_name.to_lowercase();
        if rd_l == "sp" || rd_l == "wsp" || rm_l == "sp" || rm_l == "wsp" {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_invariant_arm_fields -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `1`,
 right: `0`: ADD S
minimal failing input: rd = 31, rm = 0, is_64 = false, rd_sp = true, rm_sp = false
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_mov_pbt.rs
