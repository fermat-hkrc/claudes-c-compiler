# Bug: encode_v_crypto_vs uses OP-P (1110111) instead of OP-V (1010111)
**Law:** ∀ vd, vs2 ∈ {0..31}. (encode_v_crypto_vs([v{vd}, v{vs2}], 0b101001) as Word) & 0x7F = 0b1010111
**Impact:** Assembled vsm4r.vs words have major opcode 1110111 (OP-P / custom-3). A spec-compliant RISC-V Vector Crypto decoder and a Zvksed core look for OP-V 1010111, so objects this assembler emits will not execute as SM4 round instructions.
**Function:** encode_v_crypto_vs
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:211
**Detected by:** Reference — RISC-V Cryptography Extensions Volume II opcode
**Minimal input:** encode_v_crypto_vs([Reg("v0"), Reg("v0")], 0b101001)
**Expected:** Word with opcode 0b1010111 (vsm4r.vs v0, v0 = 0xa6082057)
**Actual:** Word(0xa6082077) with opcode 0b1110111
**Severity:** high
**Root cause:** vector.rs:214 ORs OP_V_CRYPTO, which encoder/mod.rs:449 defines as 0b1110111. RISC-V Cryptography Extensions Volume II encodes Zvksed vsm4r.vs in OP-V (1010111).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:214`
```rust
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (0b10000u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
```
**Suggested fix:** Pack OP-V (1010111), the Volume II major opcode, instead of OP_V_CRYPTO.
```rust
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (0b10000u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vs_regression_spec_opcode -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_crypto_vs_pbt::encode_v_crypto_vs_spec_opcode' (2893435) panicked at src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs:176:1:
Test failed: assertion failed: `(left == right)` 
  left: `119`, 
 right: `87`: opcode must be OP-V 1010111 per RISC-V Crypto Volume II; got 0b1110111 at src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs:186.
minimal failing input: vd = 0, vs2 = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
