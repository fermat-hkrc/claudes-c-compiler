# Bug: encode_c_lui truncates out-of-range immediates to 6 bits
**Law:** ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∉ [-32,-1]∪[1,31]∪[1048544,1048575]. encode_c_lui([Reg(rd), Imm(imm)]) = Err
**Impact:** An immediate that cannot be represented in C.LUI's signed 6-bit nzimm is silently wrapped. `c.lui x3, 32` encodes as C.LUI of nzimm=-32 (halfword 0x7181), which loads 0xfffe0000 into x3 instead of being rejected. llvm-mc requires immediates in [1, 31] ∪ [0xfffe0, 0xfffff]; the in-tree compressor (compress.rs:41) requires signed 6-bit -32..31 excluding 0. A caller that meant LUI 32 (0x00020000) gets a completely different value.
**Function:** encode_c_lui
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:6
**Detected by:** Negative/Error Contract — out-of-range immediate (3)
**Minimal input:** encode_c_lui([Reg("x3"), Imm(32)])
**Expected:** Err (32 is outside signed 6-bit nzimm and outside llvm-mc's [1,31]∪[0xfffe0,0xfffff])
**Actual:** Ok(Half(0x7181)) — the encoding of `c.lui x3, -32` / `c.lui x3, 1048544`
**Severity:** high
**Root cause:** compressed.rs:12-13 take bit 5 and bits 4:0 of nzimm with no range check, so 32 (0b100000) is packed as the 6-bit pattern of -32.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:12`
```rust
    let bit17 = ((nzimm >> 5) & 1) as u16;
    let bits16_12 = (nzimm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | ((bits16_12 & 0x1F) << 2) | ((rd as u16) << 7) | (bit17 << 12) | (0b011 << 13)))
```
**Suggested fix:** Reject nzimm values that do not fit in signed 6 bits and are not the 20-bit LUI-style form of those values, before packing.
```rust
if !(-32..=31).contains(&nzimm) && !(0xfffe0..=0xfffff).contains(&imm) {
    return Err("c.lui: nzimm out of range".into());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_lui_neg_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_lui_pbt::encode_c_lui_neg_imm_oob' (2838213) panicked at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:295:1:
Test failed: oob imm 32 must Err (llvm-mc range [1,31]∪[0xfffe0,0xfffff]; ISA simm6); got Ok(Half(29057)) at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:370.
minimal failing input: rd = "x3", imm = 32
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
