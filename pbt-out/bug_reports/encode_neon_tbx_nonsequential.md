# Bug: encode_neon_tbx ignores non-sequential table register names
**Law:** ARM TBX table registers must be consecutive wrapping v0–v31; llvm-mc rejects gaps, so encode_neon_tbx({v0.16b, v2.16b}) = Err
**Impact:** `tbx v0.8b, {v0.16b, v2.16b}, v0.8b` is assembled as a 2-register table starting at v0, so the gap is silently accepted and the encoding names the wrong second register (v1, not v2).
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, {v0.16b, v2.16b}, v0.8b])
**Expected:** Err (llvm-mc: "registers must be sequential")
**Actual:** Ok(Word(0x0e003000)) — len=1, Rn=0
**Severity:** medium
**Root cause:** neon.rs:812-817 only reads regs[0] and regs.len(); later names and sequentiality are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:812`
```rust
            let first_reg = match &regs[0] {
                Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid reg")?,
                Operand::Reg(name) => parse_reg_num(name).ok_or("invalid reg")?,
                _ => return Err("tbx: expected register in list".to_string()),
            };
            (first_reg, regs.len() as u32)
```
**Suggested fix:** Require each subsequent list member to be (first+i) mod 32.
```rust
            for (i, r) in regs.iter().enumerate() {
                let (num, _) = match r {
                    Operand::RegArrangement { reg, arrangement } if arrangement == "16b" => {
                        (parse_reg_num(reg).ok_or("invalid reg")?, arrangement)
                    }
                    _ => return Err("tbx: expected Vn.16b in list".to_string()),
                };
                if num != (first_reg + i as u32) % 32 {
                    return Err("tbx: registers must be sequential".to_string());
                }
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_nonsequential -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_nonsequential' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:625:5:
tbx v0.8b, {v0.16b, v2.16b}, v0.8b must Err (registers must be sequential)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
