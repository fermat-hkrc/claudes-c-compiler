# Bug: encode_push metamorphic segment strip fails (missing override)
**Law:** Segmented PUSH encoding must start with the segment-override prefix; stripping prefixes must recover the bare-mem encoding.
**Impact:** Same as missing segment prefix on memory PUSH — wrong segment at runtime for `%fs:`/`%gs:`/etc.
**Function:** encode_push
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:374
**Detected by:** Algebraic — Metamorphic (4c)
**Minimal input:** seg="es", base="eax", disp=0
**Expected:** encoding starts with 0x26; strip_seg equals bare push
**Actual:** `[0xff, 0x30]` (no prefix)
**Severity:** high
**Root cause:** Same as B1 — memory arm omits `emit_segment_prefix` (gp_integer.rs:374-377).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:374`
```rust
            Operand::Memory(mem) => {
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Call emit_segment_prefix before the opcode.
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
cargo test --lib encode_push_meta_segment_stripped_eq_bare -- --test-threads=1
```
**Raw output:**
```text
Test failed: segmented push must start with 0x26 for %es:, got [ff, 30]
minimal failing input: seg = "es", base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push_pbt.rs
