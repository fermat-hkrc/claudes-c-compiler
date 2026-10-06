# Bug: encode_ldp_stp wraps out-of-range and unaligned pair offsets
**Law:** LDP/STP signed offset must be a multiple of the transfer size in the ARM imm7 range: W/S [-256,252] step 4; X/D [-512,504] step 8; Q [-1024,1008] step 16. llvm-mc rejects anything else
**Impact:** `stp w0, w0, [sp, #-257]` is encoded as a different in-range offset (imm7 masked to 7 bits after a shift), so an out-of-range immediate silently becomes another displacement
**Function:** encode_ldp_stp
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:452
**Detected by:** Negative/Error Contract
**Minimal input:** encode_ldp_stp([Reg("w0"), Reg("w0"), Mem{base:"sp", offset:-257}], is_load=false)
**Expected:** Err (llvm-mc: index must be a multiple of 4 in range [-256, 252])
**Actual:** Ok(Word(689930240)) — shifted and masked imm7, not rejected
**Severity:** medium
**Root cause:** load_store.rs:506 computes `(*offset >> shift) as i32 & 0x7F` with no range or alignment check, so out-of-range and unaligned offsets wrap into a 7-bit field
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:502`
```rust
        Some(Operand::Mem { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
```
**Suggested fix:** Require `offset` divisible by `1<<shift` and the scaled imm7 in [-64, 63] before masking
```rust
            if offset & ((1i64 << shift) - 1) != 0 {
                return Err("ldp/stp offset not aligned".to_string());
            }
            let scaled = offset >> shift;
            if scaled < -64 || scaled > 63 {
                return Err("ldp/stp offset out of range".to_string());
            }
            let imm7 = (scaled as i32) & 0x7F;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_imm7_range -- --test-threads=1
```
**Raw output:**
```text
Test failed: out-of-range/unaligned offset -257 form 0 must Err (llvm-mc range); got Ok(Word(689930240))
minimal failing input: is_load = false, is_64 = false, rt1 = 0, rt2 = 0, rn = 0, which_off = 0, form = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
