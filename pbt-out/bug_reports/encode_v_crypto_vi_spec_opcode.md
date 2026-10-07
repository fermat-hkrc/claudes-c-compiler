# Bug: encode_v_crypto_vi uses OP-P (1110111) instead of OP-V (1010111)
**Law:** ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, (mnem,funct6) ∈ {(vsm3c.vi,101011),(vsm4k.vi,100001)}. (encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) as Word) & 0x7F = 0b1010111
**Impact:** Assembled vsm3c.vi / vsm4k.vi words have major opcode 1110111 (OP-P / custom-3). A spec-compliant RISC-V Vector Crypto decoder and a Zvksh/Zvksed core look for OP-V 1010111, so objects this assembler emits will not execute as SM3/SM4 crypto instructions.
**Function:** encode_v_crypto_vi
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:191
**Detected by:** Reference — RISC-V Cryptography Extensions Volume II opcode
**Minimal input:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(0)], 0b101011)
**Expected:** Word with opcode 0b1010111 (vsm3c.vi v0, v0, 0 = 0xae002057)
**Actual:** Word(0xae002077) with opcode 0b1110111
**Severity:** high
**Root cause:** vector.rs:195 ORs OP_V_CRYPTO, which encoder/mod.rs:445 defines as 0b1110111. RISC-V Cryptography Extensions Volume II encodes Zvksh/Zvksed in OP-V (1010111).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:195`
```rust
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
```
**Suggested fix:** Pack OP-V (1010111), the Volume II major opcode, instead of OP_V_CRYPTO.
```rust
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_spec_opcode -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vi_pbt::encode_v_crypto_vi_spec_opcode' (2891634) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:229:1:
Test failed: assertion failed: `(left == right)`
  left: `119`,
 right: `87`: opcode must be OP-V 1010111 per RISC-V Crypto Volume II; got 0b1110111 at src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs:242.
minimal failing input: vd = 0, vs2 = 0, uimm = 0, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
