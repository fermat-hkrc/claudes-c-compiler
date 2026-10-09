# Bug: encode_push16 rejects memory operands (PUSH m16)
**Law:** ∀ mem ∈ valid i686 memory forms. encode_push16([Mem(mem)]) must equal llvm-mc `pushw mem` = optional segment override + `0x66` + `0xFF /6` + ModR/M (+SIB/disp).
**Impact:** `pushw (%eax)`, `pushw 4(%ebx)`, and segment-overridden forms like `pushw %es:(%eax)` fail to assemble; any 16-bit memory push is unusable.
**Function:** encode_push16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:382
**Detected by:** Differential — llvm-mc i686 (encode_push16_diff_mem, encode_push16_diff_mem_segment)
**Minimal input:** `pushw (%ebx)` → Memory base=ebx; also `pushw %es:(%eax)`
**Expected:** `pushw (%ebx)` → `[0x66, 0xff, 0x33]`; `pushw %es:(%eax)` → `[0x26, 0x66, 0xff, 0x30]`
**Actual:** `Err("unsupported pushw operand")` for both
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400` — Memory falls through to catch-all Err. Sibling `encode_push` implements `0xFF /6` for m32 but omits `emit_segment_prefix` (prior bug). pushw needs the same ModR/M path with leading `0x66` and segment override before the opcode.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Add a Memory arm that emits segment prefix, 0x66, then FF /6:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push16_diff_mem -- --test-threads=1
cargo test --lib test_encode_push16_regression_mem_unsupported -- --test-threads=1
cargo test --lib test_encode_push16_regression_mem_segment_unsupported -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid mem form `pushw (%eax)`: unsupported pushw operand; Intel PUSH r/m16 = 0x66 + FF /6 + ModR/M (sibling encode_push uses FF /6).
regression: encode_push16 must accept memory, got Err(unsupported pushw operand)
regression: encode_push16 must accept segmented mem, got Err(unsupported pushw operand)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push16_pbt.rs::test_encode_push16_regression_mem_unsupported
