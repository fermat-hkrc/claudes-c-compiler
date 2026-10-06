# Bug: encode_ldnp_stnp masks out-of-range and unaligned offsets instead of rejecting them
**Law:** LDNP/STNP signed offset must be a multiple of 4 in [-256, 252] for Wt or a multiple of 8 in [-512, 504] for Xt (ARM imm7, llvm-mc/gas)
**Impact:** An out-of-range or unaligned offset is shifted and masked into a different in-range imm7, so the assembled instruction accesses the wrong address
**Function:** encode_ldnp_stnp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:518
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldnp_stnp([Reg("w0"), Reg("w0"), Mem{base:"x0", offset:-257}], is_load=false)
**Expected:** Err (llvm-mc: index must be a multiple of 4 in range [-256, 252])
**Actual:** Ok(Word(0x281F0000))
**Severity:** medium
**Root cause:** load_store.rs:533 computes `imm7 = ((*offset >> shift) as i32) & 0x7F` with no range or alignment check, wrapping any i64 into 7 bits
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:533`
```rust
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
```
**Suggested fix:** Require `offset` divisible by the scale and in the ARM range before encoding
```rust
            let scale = 1i64 << shift;
            if offset % scale != 0 || offset < -(64 * scale) || offset > (63 * scale) {
                return Err("ldnp/stnp offset out of range".to_string());
            }
            let imm7 = ((offset / scale) as i32) & 0x7F;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_imm7_range -- --test-threads=1
```
**Raw output:**
```text
Test failed: out-of-range/unaligned offset -257 must Err (llvm-mc range); got Ok(Word(673153024))
minimal failing input: is_load = false, is_64 = false, rt1 = 0, rt2 = 0, rn = 0, which_off = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs
