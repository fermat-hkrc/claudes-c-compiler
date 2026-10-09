# Bug: encode_test accepts non-GP register names via reg_num aliasing
**Law:** TEST r/m forms are GP-only (Intel SDM); xmm/mm/st/ymm names that llvm-mc rejects must return Err.
**Impact:** `testb %al, %xmm0` silently encodes as `testb %al, %al` (reg_num maps xmm0→0), emitting plausible but wrong GP encodings for invalid operands.
**Function:** encode_test
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:654
**Detected by:** Negative/error contract — llvm-mc rejection differential
**Minimal input:** `testb %al, %xmm0`
**Expected:** `Err(...)`
**Actual:** `Ok([0x84, 0xc0])` (same as testb %al, %al)
**Severity:** medium
**Root cause:** RR arm uses `reg_num`, which maps xmm/mm/st/ymm names onto 0–7 without rejecting non-GP classes.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:654`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x84 } else { 0x85 });
                self.bytes.push(self.modrm(3, src_num, dst_num));
                Ok(())
            }
```
**Suggested fix:** Reject non-GP names (require `reg_size` Some and no xmm/mm/st/ymm/cr/dr prefix) before `reg_num`:
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if !is_gp_reg(&src.name) || !is_gp_reg(&dst.name) {
                    return Err("test requires GP registers".into());
                }
                if reg_size(&src.name) != Some(size) || reg_size(&dst.name) != Some(size) {
                    return Err("size-mismatched test operands".into());
                }
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                // ... existing encode ...
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_test_regression_non_gp_xmm -- --test-threads=1
```
**Raw output:**
```text
regression: non-GP TEST must Err, got Ok([132, 192])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_test_pbt.rs (encode_test_regression_non_gp_xmm)
