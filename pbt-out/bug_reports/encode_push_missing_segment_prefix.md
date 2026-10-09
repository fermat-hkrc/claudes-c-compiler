# Bug: encode_push omits segment-override prefix on memory operands
**Law:** PUSH m32 with a segment override must emit the corresponding one-byte prefix (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before opcode 0xFF /6.
**Impact:** Any `pushl %fs:(…)` / `%gs:(…)` / `%es:(…)` assembled by this encoder silently drops the segment, so the instruction reads the wrong segment at runtime (TLS via FS/GS is especially dangerous).
**Function:** encode_push
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:374
**Detected by:** Differential — llvm-mc i686 (5); Algebraic — Metamorphic (4c)
**Minimal input:** `pushl %fs:(%eax)` (also `%es:(%eax)`, all six segs)
**Expected:** `[0x64, 0xff, 0x30]`
**Actual:** `[0xff, 0x30]`
**Severity:** high
**Root cause:** `gp_integer.rs:374-377` pushes 0xFF then `encode_modrm_mem(6, mem)` without calling `emit_segment_prefix(mem)`. The x86-64 sibling does call it; `core.rs:31-42` documents all six prefixes.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:374`
```rust
            Operand::Memory(mem) => {
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode, matching the x86-64 sibling.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push_regression_missing_fs_prefix -- --test-threads=1 --exact
```
**Raw output:**
```text
assertion `left == right` failed: regression: encode_push must emit segment override before 0xFF /6
  left: [255, 48]
 right: [100, 255, 48]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push_pbt.rs (encode_push_regression_missing_fs_prefix)
