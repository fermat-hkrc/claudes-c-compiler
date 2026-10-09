# Bug: encode_inc_dec accepts non-GP registers via reg_num alias

**Law:** INC/DEC register forms are defined only for general-purpose r/m operands. Non-GP names (xmm/mm/st/ymm) that `reg_num` aliases must be rejected, matching llvm-mc.
**Impact:** `incl %xmm0` encodes as `incl %eax` (`[0x40]`). A bad operand is silently rewritten to a different register — wrong machine code with no assembler error.
**Function:** encode_inc_dec
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:811
**Detected by:** Negative/error contract (cross-checked with llvm-mc rejection)
**Minimal input:** `incl %xmm0` → SUT `Ok([0x40])`, llvm-mc error
**Expected:** `Err(...)`
**Actual:** `Ok([0x40])` (same as `incl %eax`)
**Severity:** medium
**Root cause:** Register arm uses only `reg_num(&reg.name)` (`registers.rs:4-15`), which maps `xmm0|mm0|st|ymm0` → 0 with no GP-only gate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:811`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                if size == 4 {
                    // Use compact single-byte encoding: 0x40+reg (inc) or 0x48+reg (dec)
                    let base = if op_ext == 0 { 0x40 } else { 0x48 };
                    self.bytes.push(base + num);
```
**Suggested fix:** Reject non-GP register names before encoding (e.g. allow only the eight GP names per width).
```rust
            Operand::Register(reg) => {
                if is_xmm(&reg.name) || is_mm(&reg.name) || reg.name.starts_with("st")
                    || reg.name.starts_with("ymm")
                {
                    return Err(format!("inc/dec does not accept register %{}", reg.name));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_inc_dec_regression_xmm0_accepted -- --test-threads=1
```
**Raw output:**
```text
encode_inc_dec must reject non-GP xmm0, got Ok([64])
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs (encode_inc_dec_regression_xmm0_accepted)
