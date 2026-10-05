# Bug: encode_neon_tbx panics on an empty register list
**Law:** A Result-returning encoder must return Err for an empty table list, not panic; encode_neon_tbx([Vd.8b, RegList([]), Vm.8b]) = Err
**Impact:** An empty `{ }` table operand crashes the assembler instead of reporting "tbx: expected register in list".
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, RegList([]), v0.8b])
**Expected:** Err
**Actual:** panic `index out of bounds: the len is 0 but the index is 0` at neon.rs:812
**Severity:** medium
**Root cause:** neon.rs:812 indexes `regs[0]` without checking `regs.is_empty()`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:812`
```rust
            let first_reg = match &regs[0] {
```
**Suggested fix:** Reject an empty list before indexing.
```rust
            if regs.is_empty() {
                return Err("tbx: expected register in list".to_string());
            }
            let first_reg = match &regs[0] {
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_empty_list -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_empty_list' panicked at src/backend/arm/assembler/encoder/neon.rs:812:40:
index out of bounds: the len is 0 but the index is 0
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_empty_list' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:603:19:
empty table list must Err, not panic on regs[0]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
