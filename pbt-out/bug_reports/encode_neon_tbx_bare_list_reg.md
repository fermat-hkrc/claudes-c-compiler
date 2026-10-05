# Bug: encode_neon_tbx accepts a bare Reg as a table-list member
**Law:** Each TBX table member must be Vn.16B; llvm-mc rejects `{v0}`, so encode_neon_tbx with Operand::Reg in the list = Err
**Impact:** `tbx v0.8b, {v0}, v0.8b` is assembled as `{v0.16b}`, so a missing arrangement specifier is silently filled in.
**Function:** encode_neon_tbx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:803
**Detected by:** Negative/Error Contract
**Minimal input:** encode_neon_tbx([v0.8b, RegList([Reg("v0")]), v0.8b])
**Expected:** Err
**Actual:** Ok(Word(0x0e001000))
**Severity:** medium
**Root cause:** neon.rs:814 accepts Operand::Reg in the table list via parse_reg_num.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:814`
```rust
                Operand::Reg(name) => parse_reg_num(name).ok_or("invalid reg")?,
```
**Suggested fix:** Accept only RegArrangement with .16b in the list.
```rust
                Operand::Reg(_) => return Err("tbx: table member must be Vn.16b".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_tbx_regression_bare_list_reg -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_tbx_pbt::test_encode_neon_tbx_regression_bare_list_reg' panicked at src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs:667:5:
tbx v0.8b, {v0}, v0.8b must Err (table member is not Vn.16B)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/encode_neon_tbx_pbt.rs
