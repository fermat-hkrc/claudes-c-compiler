# Bug: encode_bt accepts r16/r8 GP names via reg_num aliasing for *l forms
**Law:** For mnemonics `btl`/`btsl`/`btrl`/`btcl` (and bare `bt`… defaulting to 32-bit), both register operands must be r32; r16/r8 forms that llvm-mc rejects must return `Err`, not silently alias to the same 3-bit encoding as the r32 name.
**Impact:** `btl %ax, %ebx` is accepted and emits the same bytes as `btl %eax, %ebx`. Callers that accidentally pass a 16- or 8-bit name get wrong-width machine code with no diagnostic — a silent assembler acceptance bug.
**Function:** encode_bt
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1026
**Detected by:** Negative/error contract — llvm-mc rejects mismatched width; SUT accepts
**Minimal input:** `btl %ax, %ebx` → SUT `Ok([0f,a3,c3])` (same as `%eax,%ebx`); llvm-mc: `invalid operand`
**Expected:** `Err(...)`
**Actual:** `Ok([0f, a3, c3])` via `reg_num("ax") == reg_num("eax") == 0`
**Severity:** medium
**Root cause:** `gp_integer.rs:1026-1031` (and the other register arms) call `reg_num` with no `reg_size` gate. `registers.rs::reg_num` deliberately aliases ax/al→0, so wrong-width names encode as r32.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1026`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.bytes.push(self.modrm(3, src_num, dst_num));
                Ok(())
            }
```
**Suggested fix:** Require `reg_size == 4` on both GP operands for the *l / bare path (and reject non-GP). For a future `btw` path, require size 2 and emit `0x66`.
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err(format!("{mnemonic} requires 32-bit registers"));
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                // ...
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_regression_wrong_width_ax_ebx -- --test-threads=1
cargo test --lib encode_bt_neg_wrong_width -- --test-threads=1
```
**Raw output:**
```text
SUT accepted btl %ax, %ebx → Ok([15, 163, 195]); r16 not valid for btl
SUT accepted wrong-width `btl %ax, %eax` → [0f, a3, c0]; llvm-mc rejects; Intel BT *l is r32 only (reg_num aliasing bug).
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bt_pbt.rs (`encode_bt_regression_wrong_width_ax_ebx`)
