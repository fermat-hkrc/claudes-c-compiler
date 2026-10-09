# Bug: encode_system_table omits segment-override prefix on memory operands

**Law:** For SGDT/SIDT/LGDT/LIDT with a memory operand that names a segment (`%es:`, `%cs:`, `%ss:`, `%ds:`, `%fs:`, `%gs:`), the encoder must emit the corresponding override prefix byte (0x26/0x2E/0x36/0x3E/0x64/0x65) before the `0F 01` opcode, matching Intel SDM and llvm-mc i686.
**Impact:** Any AT&T assembly that uses a segment override on a descriptor-table load/store (common in boot/OS code, e.g. `lgdt %fs:(%eax)`) assembles to wrong machine code that ignores the segment and reads/writes through the default DS path — silent wrong encoding, not a hard error.
**Function:** encode_system_table
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:184
**Detected by:** Differential — llvm-mc i686 (segment forms); algebraic witnesses via KAT/regression
**Minimal input:** `lgdt %fs:(%eax)` (also `sgdt %es:(%eax)`, `lidt %gs:8(%ebx)`, and SIB+segment)
**Expected:** `[0x64, 0x0f, 0x01, 0x10]` for `lgdt %fs:(%eax)`
**Actual:** `[0x0f, 0x01, 0x10]` (FS override 0x64 missing)
**Severity:** high
**Root cause:** `system.rs:184-186` pushes `0F 01` and calls `encode_modrm_mem` without first calling `emit_segment_prefix(mem)`, unlike GP-integer memory paths in `gp_integer.rs` that do call it. Same defect class as `encode_invlpg` / `encode_prefetch` / `encode_verw` / `encode_lsl` memory arms.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:184`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(reg_ext, mem)
            }
```
**Suggested fix:** Emit the segment override before the opcode, reusing the existing helper:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(reg_ext, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_system_table_regression_missing_fs_prefix -- --test-threads=1 --exact
```
**Raw output:**
```text
assertion `left == right` failed: lgdt %fs:(%eax) must be [64, 0f, 01, 10], got [0f, 01, 10]
  left: [15, 1, 16]
 right: [100, 15, 1, 16]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_system_table_pbt.rs (`test_encode_system_table_regression_missing_fs_prefix`, `test_encode_system_table_regression_missing_gs_prefix_disp`, `test_encode_system_table_regression_missing_es_prefix`)
