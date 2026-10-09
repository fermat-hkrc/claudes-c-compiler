# Bug: encode_mov_cr accepts movw with control registers
**Law:** MOV to/from a control register on IA-32 is an r32 operation; the `movw` mnemonic with a CR operand must be rejected.
**Impact:** `movw %cr0, %ax` is assembled as `0F 20 C0` (identical to `movl %cr0, %eax`). A 16-bit-sized mov involving CR silently becomes a 32-bit CR move. llvm-mc `-triple=i686` rejects `movw %cr0, %ax`.
**Function:** encode_mov_cr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:250
**Detected by:** Negative/error contract (strengthening round) vs llvm-mc
**Minimal input:** `movw %cr0, %ax`
**Expected:** `Err`
**Actual:** `Ok([0x0f, 0x20, 0xc0])`
**Severity:** medium
**Root cause:** `encode_mov` routes any size mnemonic into `encode_mov_cr` when a CR is present; `encode_mov_cr` never checks operand size and accepts r16 names via `reg_num` (same defect class as non-r32 GP acceptance).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:255`
```rust
            (Operand::Register(cr), Operand::Register(gp)) if is_control_reg(&cr.name) => {
                let cr_num = control_reg_num(&cr.name).ok_or("bad control register")?;
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x20]);
                self.bytes.push(self.modrm(3, cr_num, gp_num));
                Ok(())
            }
```
**Suggested fix:** Require r32 GP width inside `encode_mov_cr` (covers movw/movb as well).
```rust
if reg_size(&gp.name) != 4 {
    return Err("mov cr requires 32-bit register".to_string());
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_cr_neg_movw_width -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted `movw %cr0, %ax` → [0f, 20, c0]; MOV CR is r32-only (Intel SDM; llvm-mc rejects movw with CR).
minimal failing input: write = false, cr = "cr0", r16 = "ax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs (`encode_mov_cr_neg_movw_width`)
