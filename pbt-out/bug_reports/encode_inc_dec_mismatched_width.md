# Bug: encode_inc_dec accepts width-mismatched GP registers

**Law:** Mnemonic size must match register width: `incl`/`decl` take r32, `incw`/`decw` take r16, `incb`/`decb` take r8. A mismatched register must be rejected (llvm-mc rejects `incl %ax`).
**Impact:** `incl %ax` encodes as `incl %eax` (`[0x40]`). The assembler accepts invalid AT&T and emits a different-width operation than written.
**Function:** encode_inc_dec
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:811
**Detected by:** Negative/error contract (cross-checked with llvm-mc rejection)
**Minimal input:** `incl %ax` → SUT `Ok([0x40])`, llvm-mc error
**Expected:** `Err(...)`
**Actual:** `Ok([0x40])` (compact INC of EAX)
**Severity:** medium
**Root cause:** Register arm never compares `reg_size(&reg.name)` to the `size` argument from the mnemonic; `reg_num("ax")` and `reg_num("eax")` both yield 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:811`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                if size == 4 {
                    // Use compact single-byte encoding: 0x40+reg (inc) or 0x48+reg (dec)
                    let base = if op_ext == 0 { 0x40 } else { 0x48 };
                    self.bytes.push(base + num);
```
**Suggested fix:** Gate on `reg_size(&reg.name) == size` before encoding.
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != size {
                    return Err(format!(
                        "register %{} width does not match inc/dec size {size}",
                        reg.name
                    ));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec_regression_mismatched_width -- --test-threads=1
```
**Raw output:**
```text
encode_inc_dec must reject width-mismatched incl %ax, got Ok([64])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs (encode_inc_dec_regression_mismatched_width)
