# Bug: encode_pop omits segment-override prefix on memory form
**Law:** For every memory POP with a segment override, the encoding must begin with the corresponding segment-override prefix byte (es=0x26, cs=0x2E, ss=0x36, ds=0x3E, fs=0x64, gs=0x65) before opcode 0x8F /0, matching llvm-mc and the x86-64 sibling.
**Impact:** Any `popl %fs:(%reg)` / `%es:(...)` / etc. assembled by the i686 backend silently drops the segment override, so the instruction addresses the wrong segment — incorrect code generation for TLS, far data, and explicit segment overrides.
**Function:** encode_pop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:424
**Detected by:** Differential — llvm-mc i686; Algebraic — Metamorphic (segmented = prefix ‖ bare)
**Minimal input:** `popl %es:(%eax)` (also `%fs:(%eax)`)
**Expected:** `[0x26, 0x8f, 0x00]` for ES; `[0x64, 0x8f, 0x00]` for FS
**Actual:** `[0x8f, 0x00]` (prefix omitted)
**Severity:** high
**Root cause:** `gp_integer.rs:424-427` pushes `0x8F` then calls `encode_modrm_mem` without calling `emit_segment_prefix(mem)` first. The x86-64 sibling at `src/backend/x86/assembler/encoder/gp_integer.rs:393` does call it; `core.rs:31` documents the helper for this purpose.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:424`
```rust
            Operand::Memory(mem) => {
                // pop m32: 0x8F /0
                self.bytes.push(0x8F);
                self.encode_modrm_mem(0, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode, matching the x86-64 sibling and other fixed encoders (push, lmsw, …).
```rust
            Operand::Memory(mem) => {
                // pop m32: 0x8F /0
                self.emit_segment_prefix(mem);
                self.bytes.push(0x8F);
                self.encode_modrm_mem(0, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop_regression_missing_fs_prefix -- --test-threads=1
cargo test --lib encode_pop_diff_mem_segment -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[143, 0]`,
 right: `[38, 143, 0]`: segment diff `popl %es:(%eax)`: sut=[8f, 00] mc=[26, 8f, 00]
minimal failing input: seg = "es", base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_pop_pbt.rs (encode_pop_regression_missing_fs_prefix)
