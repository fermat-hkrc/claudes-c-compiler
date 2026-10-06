# Bug: encode_vsetivli silently truncates raw vtypei above 10 bits
**Law:** ∀ rd ∈ GPR names, uimm ∈ 0..=31, v ∈ 1024..=2047. encode_vsetivli([Reg(rd), Imm(uimm), Imm(v)]) = Err(_)
**Impact:** A raw vtypei immediate with bit 10 set (1024..=2047) is accepted. parse_vtypei keeps 11 bits (`& 0x7FF`) then encode_vsetivli keeps 10 (`& 0x3FF`), so 1024 encodes as vtypei=0 (`vsetivli x0, 0, e8, m1, tu, mu`). llvm-mc rejects 1024 (`operand must be e[...],m[...],[ta|tu],[ma|mu]`). Callers that pass an 11-bit vsetvli-style immediate get the wrong vector type with no error.
**Function:** encode_vsetivli
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:59
**Detected by:** Negative/Error Contract — vtypei immediate out of range
**Minimal input:** encode_vsetivli([Reg("x0"), Imm(0), Imm(1024)])
**Expected:** Err (llvm-mc rejects raw vtypei 1024 for vsetivli; field is vtypei[9:0])
**Actual:** Ok(Word(0xc0007057)) — 1024 masked to 0
**Severity:** medium
**Root cause:** parse_vtypei masks Imm with 0x7FF (11 bits, the vsetvli width). encode_vsetivli then packs only 10 bits: `(vtypei & 0x3FF) << 20`, dropping bit 10 without error.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:64`
```rust
    let word = (0b11u32 << 30) | ((vtypei & 0x3FF) << 20) | (uimm << 15) | (0b111 << 12) | (rd << 7) | OP_V;
```
**Suggested fix:** Reject a raw vtypei that does not fit in 10 bits.
```rust
    let vtypei = parse_vtypei(operands, 2)?;
    if vtypei > 0x3FF {
        return Err(format!("vsetivli: vtypei {vtypei} out of range [0, 1023]"));
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vsetivli_regression_vtypei_imm_oob -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vsetivli_pbt::encode_vsetivli_neg_vtypei_imm_oob' panicked at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:298:1:
Test failed: vtypei imm 1024 outside 0..=1023 must Err (llvm-mc rejects 10-bit overflow); got Ok(Word(3221254231)) at src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs:500.
minimal failing input: rd = "x0", uimm = 0, v = 1024
	successes: 0
	local rejects: 0
	global rejects: 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs
