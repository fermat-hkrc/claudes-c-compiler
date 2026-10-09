# Bug: encode_movsx accepts non-GP registers via reg_num aliases
**Law:** ∀ form ∈ {movsbl,movsbw,movswl}, operands must be general-purpose registers (or memory source); xmm/mm/st/ymm names must be rejected — 0F BE/BF is a GP form (Intel SDM)
**Impact:** Assembler silently accepts `movsbl %al, %xmm0` and emits the same bytes as `movsbl %al, %eax` (reg_num aliases xmm0→0), so invalid SIMD/x87 names assemble to GP encodings — silent wrong code / wrong register class
**Function:** encode_movsx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:272
**Detected by:** Negative/error contract — llvm-mc rejects; SUT Ok
**Minimal input:** `movsbl %al, %xmm0` → Ok([0x0f, 0xbe, 0xc0])
**Expected:** Err (non-GP operand)
**Actual:** Ok([0x0f, 0xbe, 0xc0]) — identical to movsbl %al, %eax
**Severity:** medium
**Root cause:** `reg_num` (registers.rs:4-15) maps xmm/mm/st/ymm names onto GP 0–7. encode_movsx calls reg_num without rejecting is_xmm / is_mm / st / ymm, so non-GP names encode as their GP aliases.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:286`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
            }
```
and registers.rs:4-15:
```rust
pub(crate) fn reg_num(name: &str) -> Option<u8> {
    match name {
        "al" | "ax" | "eax" | "xmm0" | "mm0" | "st" | "st(0)" | "ymm0" => Some(0),
        // ...
```
**Suggested fix:** Reject non-GP register names before encoding:
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if is_xmm(&src.name) || is_mm(&src.name) || src.name.starts_with("st") || src.name.starts_with("ymm")
                    || is_xmm(&dst.name) || is_mm(&dst.name) || dst.name.starts_with("st") || dst.name.starts_with("ymm")
                {
                    return Err(format!("movsx requires GP registers, got {}, {}", src.name, dst.name));
                }
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                // ...
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_movsx_neg_non_gp -- --test-threads=1
cargo test --lib encode_movsx_regression_non_gp_xmm_dst -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted non-GP MOVSX `movsbl %al, %xmm0` → [0f, be, c0]; 0F BE/BF form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7..
minimal failing input: fi = 0, ni = 0, on_src = false, gi = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_movsx_pbt.rs (`encode_movsx_regression_non_gp_xmm_dst`)
