# Bug: encode_out accepts any register pair as OUT DX form
**Law:** OUT register form is only valid with data in AL/AX/EAX (matching outb/outw/outl) and port DX; any other pair must be rejected
**Impact:** Invalid AT&T like `outb %bl, %dx` or `outb %al, %al` silently encodes as `0xEE`, so the assembler emits a real OUT AL,DX that ignores the written operands — wrong code generation with no error
**Function:** encode_out
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:59
**Detected by:** Negative/Error Contract + Differential (llvm-mc)
**Minimal input:** `outb` with operands `%al, %al` (also `%bl, %dx`)
**Expected:** `Err` (invalid operands), matching llvm-mc / gas / Intel SDM
**Actual:** `Ok([0xEE])`
**Severity:** high
**Root cause:** `system.rs:59-63` matches any `(Register, Register)` pair and ignores register names (`_src`, `_dst`), always emitting EE/EF
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:59`
```rust
            (Operand::Register(_src), Operand::Register(_dst)) => {
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0xEE } else { 0xEF });
                Ok(())
            }
```
**Suggested fix:** Require data register equal to AL/AX/EAX for the mnemonic size and port register equal to DX; otherwise return Err
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let expect = match size { 1 => "al", 2 => "ax", 4 => "eax", _ => unreachable!() };
                if src.name != expect || dst.name != "dx" {
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
cargo test --lib test_encode_out_regression_wrong_reg_bl_dx -- --test-threads=1
cargo test --lib encode_out_neg_wrong_registers -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid OUT `outb %al, %al` → [ee]; llvm-mc rejected: llvm-mc error: <stdin>:1:11: error: invalid operand for instruction
outb %al, %al
          ^~~
.
minimal failing input: mnemonic = "outb", src = "al", dst = "al"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_out_pbt.rs::test_encode_out_regression_wrong_reg_bl_dx
