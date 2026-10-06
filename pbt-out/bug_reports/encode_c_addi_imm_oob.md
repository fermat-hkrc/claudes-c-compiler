# Bug: encode_c_addi truncates out-of-range immediates to 6 bits
**Law:** ∀ rd ∈ GPR, ∀ imm ∉ [-32,31]. encode_c_addi([Reg(rd), Imm(imm)]) = Err
**Impact:** An immediate that cannot be represented in C.ADDI's signed 6-bit field is silently wrapped. `c.addi x0, 32` encodes as C.ADDI of imm=-32 (halfword 0x1001), which subtracts 32 instead of being rejected. llvm-mc requires immediates in [-32, 31] (`immediate must be non-zero in the range [-32, 31]`); the in-tree compressor (compress.rs:64) requires the same signed 6-bit range. A caller that meant to add 32 gets -32.
**Function:** encode_c_addi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:27
**Detected by:** Negative/Error Contract — out-of-range immediate (3)
**Minimal input:** encode_c_addi([Reg("x0"), Imm(32)])
**Expected:** Err (32 is outside signed 6-bit imm; llvm-mc reports `immediate must be non-zero in the range [-32, 31]`)
**Actual:** Ok(Half(0x1001)) — the encoding of `c.addi x0, -32`
**Severity:** high
**Root cause:** compressed.rs:30-32 take bit 5 and bits 4:0 of imm with no range check, so 32 (0b100000) is packed as the 6-bit pattern of -32.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:30`
```rust
    let bit5 = ((imm >> 5) & 1) as u16;
    let bits4_0 = (imm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | (bits4_0 << 2) | ((rd as u16) << 7) | (bit5 << 12)))
```
**Suggested fix:** Reject immediates that do not fit in signed 6 bits before packing.
```rust
if !(-32..=31).contains(&imm) {
    return Err("c.addi: imm out of range".into());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_addi_neg_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_addi_pbt::encode_c_addi_neg_imm_oob' (2844741) panicked at src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs:289:1:
Test failed: oob imm 32 must Err (llvm-mc range [-32, 31]); got Ok(Half(4097)) at src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs:352.
minimal failing input: rd = "x0", imm = 32
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs
