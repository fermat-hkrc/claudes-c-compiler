# Bug: encode_out rejects AT&T (%dx) port form accepted by llvm-mc and x86-64 sibling
**Law:** `outb/outw/outl %data, (%dx)` is a valid AT&T spelling of the DX-port OUT form and must encode as EE/EF (+66 for word), same as `%dx`
**Impact:** Valid assembly using the parenthesized DX port form fails to assemble on i686 while the x86-64 encoder and llvm-mc accept it — kernel/boot code using `outl %eax, (%dx)` cannot round-trip
**Function:** encode_out
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:70
**Detected by:** Differential (llvm-mc) + sibling x86 encode_out
**Minimal input:** `outl` with operands `%eax, (%dx)` (Memory base=dx)
**Expected:** `Ok([0xEF])` (llvm-mc encoding)
**Actual:** `Err("unsupported outl operands")`
**Severity:** medium
**Root cause:** i686 `encode_out` only matches Register+Register and Register+Immediate; the catch-all at line 70 rejects Memory. The x86-64 sibling at `src/backend/x86/assembler/encoder/system.rs:35-39` already handles `(Register, Memory)` as the DX form
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:58`
```rust
        match (&ops[0], &ops[1]) {
            (Operand::Register(_src), Operand::Register(_dst)) => {
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xEE } else { 0xEF });
                Ok(())
            }
            (Operand::Register(_src), Operand::Immediate(ImmediateValue::Integer(val))) => {
                ...
            }
            _ => Err(format!("unsupported {} operands", mnemonic)),
        }
```
**Suggested fix:** Add a Register+Memory arm (optionally requiring base DX, no index) mirroring x86-64
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let expect = match size { 1 => "al", 2 => "ax", 4 => "eax", _ => unreachable!() };
                if src.name != expect {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                // (%dx) only
                if mem.base.as_ref().map(|r| r.name.as_str()) != Some("dx")
                    || mem.index.is_some()
                {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xEE } else { 0xEF });
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_out_regression_dx_mem_form -- --test-threads=1
cargo test --lib encode_out_diff_dx_memory_form -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid AT&T form `outb %al, (%dx)`: unsupported outb operands; llvm-mc=[ee].
minimal failing input: mnemonic = "outb"
SUT erred on valid AT&T (%dx) form: unsupported outl operands
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_out_pbt.rs::test_encode_out_regression_dx_mem_form
