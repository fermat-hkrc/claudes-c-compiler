# Bug: encode_csr truncates CSR numbers outside 0..=4095
**Law:** The CSR specifier is a 12-bit field (RISC-V csr[11:0] / llvm-mc "immediate must be an integer in the range [0, 4095]"); a numeric CSR outside 0..=4095 must return Err, not a wrapped 12-bit encoding.
**Impact:** `csrrw x1, 4096, x2` is assembled as CSR 0 (fflags) and `csrrw x0, -1, x0` as CSR 4095, silently targeting the wrong control/status register.
**Function:** encode_csr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:40
**Detected by:** Negative/Error Contract
**Minimal input:** encode_csr([Reg("x0"), Imm(-1), Reg("x0")], funct3=0b001)  // csrrw x0, -1, x0
**Expected:** Err
**Actual:** Ok(Word(4293922931))  // 0xFFF01073 = csrrw x0, 0xfff, x0
**Severity:** high
**Root cause:** get_csr_num (system.rs:66) accepts any Imm as u32 with no 12-bit range check; encode_csr then passes `csr as i32` to encode_i, which masks with 0xFFF, so -1 becomes 0xFFF and 4096 becomes 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/system.rs:66`
```rust
        Some(Operand::Imm(v)) => Ok(*v as u32),
```
**Suggested fix:** Reject CSR numbers outside 0..=4095 in get_csr_num (or in encode_csr before encode_i).
```rust
        Some(Operand::Imm(v)) => {
            if *v < 0 || *v > 4095 {
                Err(format!("CSR number {} out of range 0..=4095", v))
            } else {
                Ok(*v as u32)
            }
        }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_csr_neg_csr_oob -- --test-threads=1
```
**Raw output:**
```text
Test failed: csr -1 outside 0..=4095 must Err (llvm-mc csr[11:0]); got Ok(Word(4293922931)) at src/backend/riscv/assembler/encoder/encode_csr_pbt.rs:519.
minimal failing input: (_mn, f3) = (
    "csrrw",
    1,
), rd = "x0", rs1 = "x0", csr_num = -1
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
