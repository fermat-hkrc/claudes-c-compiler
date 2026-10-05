# Bug: encode_neon_tbx wraps table lists longer than 4 into len&3
**Law:** ARM TBX allows 1–4 table registers; llvm-mc rejects more, so encode_neon_tbx with nregs>4 = Err
**Impact:** `tbx v0.8b, {v0.16b, v1.16b, v2.16b, v3.16b, v4.16b}, v0.8b` is assembled as a 1-register table (len=(5-1)&3=0), silently dropping four of five vectors.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, {v0.16b..v4.16b}, v0.8b])
**Expected:** Err (llvm-mc: "invalid number of vectors")
**Actual:** Ok(Word(0x0e001000)) — encoded as 1-register TBX
**Severity:** medium
**Root cause:** neon.rs:823 masks `(num_regs - 1) & 0x3` instead of rejecting nregs outside 1..=4.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:823`
```rust
    let len = (num_regs - 1) & 0x3;
```
**Suggested fix:** Reject nregs outside 1..=4 before encoding len.
```rust
    if !(1..=4).contains(&num_regs) {
        return Err(format!("tbx: invalid number of vectors: {}", num_regs));
    }
    let len = num_regs - 1;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_five_regs -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_five_regs' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:611:5:
tbx v0.8b, {v0.16b..v4.16b}, v0.8b must Err (invalid number of vectors)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
