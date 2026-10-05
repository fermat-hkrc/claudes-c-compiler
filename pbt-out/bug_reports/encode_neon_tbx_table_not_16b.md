# Bug: encode_neon_tbx accepts a table arrangement other than .16b
**Law:** ARM TBX table registers are .16B; llvm-mc rejects `{v0.8b}`, so encode_neon_tbx with table T≠16b = Err
**Impact:** `tbx v0.8b, {v0.8b}, v0.8b` is assembled as a .16B table lookup, so the wrong arrangement is silently rewritten.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, {v0.8b}, v0.8b])
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word(0x0e001000))
**Severity:** medium
**Root cause:** neon.rs:813 binds `reg, ..` and never inspects the table arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:813`
```rust
                Operand::RegArrangement { reg, .. } => parse_reg_num(reg).ok_or("invalid reg")?,
```
**Suggested fix:** Require arrangement == "16b" on every list member.
```rust
                Operand::RegArrangement { reg, arrangement } if arrangement == "16b" => {
                    parse_reg_num(reg).ok_or("invalid reg")?
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_table_not_16b -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_table_not_16b' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:639:5:
tbx v0.8b, {v0.8b}, v0.8b must Err (table must be .16b)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
