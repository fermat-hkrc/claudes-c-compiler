# Bug: encode_pop16 rejects memory operands (POP m16)
**Law:** ∀ mem ∈ valid i686 memory forms. encode_pop16([mem]) must equal llvm-mc `popw mem` = `0x66` + `0x8F /0` + ModR/M (+SIB/disp), including optional segment override prefixes.
**Impact:** Valid AT&T `popw (%eax)`, `popw 4(%esi)`, `popw %es:(%eax)`, absolute forms fail assembly with `"unsupported popw operand"`. Sibling `encode_pop` already implements memory for `popl` (`0x8F /0`); `popw` is missing the same arm plus 0x66.
**Function:** encode_pop16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:325
**Detected by:** Differential (llvm-mc i686) — memory and segmented-memory generators
**Minimal input:** `popw (%eax)` → SUT `Err("unsupported popw operand")`, llvm-mc `[0x66, 0x8f, 0x00]`
**Expected:** Ok with bytes `[0x66, 0x8f, 0x00]` (segmented: e.g. `popw %es:(%eax)` → `[0x26, 0x66, 0x8f, 0x00]`)
**Actual:** `Err("unsupported popw operand")`
**Severity:** high
**Root cause:** system.rs:348 catches all non-Register operands with a blanket error; no `Operand::Memory` arm. Compare gp_integer.rs:421-425 (`encode_pop` memory path).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:348`
```rust
            _ => Err("unsupported popw operand".to_string()),
```
**Suggested fix:** Handle memory like `encode_pop`, with 0x66 and segment prefix:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.push(0x8F);
                self.encode_modrm_mem(0, mem)
            }
            _ => Err("unsupported popw operand".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop16_diff_mem -- --test-threads=1
cargo test --lib test_encode_pop16_regression_mem_unsupported -- --test-threads=1
```
**Raw output:**
```text
SUT rejected `popw (%eax)`: unsupported popw operand
minimal failing input: kind = 0, base = "eax", index = "eax", scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_pop16_pbt.rs::test_encode_pop16_regression_mem_unsupported
