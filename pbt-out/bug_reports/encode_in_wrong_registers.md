# Bug: encode_in accepts non-canonical register pairs

**Law:** IN instructions architecturally fix the port register to DX and the data register to AL/AX/EAX; any other register pair must be rejected by the assembler.
**Impact:** The i686 assembler silently emits EC/ED for invalid forms such as `inb %al, %al` or `inb %bx, %cx`, producing machine code that does not match the source operands. Callers assembling hand-written or generated AT&T asm get wrong I/O instructions without an error.
**Function:** encode_in
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:94
**Detected by:** Negative error + differential vs llvm-mc (encode_in_neg_wrong_registers)
**Minimal input:** `inb %al, %al` (ops = [Register("al"), Register("al")])
**Expected:** `Err(...)` (llvm-mc: invalid operand for instruction)
**Actual:** `Ok([0xec])`
**Severity:** high
**Root cause:** system.rs:94 matches any `(Register, Register)` pair and ignores the register names, always emitting EC/ED.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:94`
```rust
(Operand::Register(_src), Operand::Register(_dst)) => {
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xEC } else { 0xED });
    Ok(())
}
```
**Suggested fix:** Require port=`dx` and data=`al`/`ax`/`eax` matching the mnemonic size; otherwise return Err.
```rust
(Operand::Register(src), Operand::Register(dst))
    if src.name == "dx" && dst.name == data_reg_for(size) => {
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xEC } else { 0xED });
    Ok(())
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_in_pbt::test_encode_in_regression_wrong_reg_al_al -- --test-threads=1
cargo test --lib encode_in_pbt::encode_in_neg_wrong_registers -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT accepted invalid IN `inb %al, %al` → [ec]; llvm-mc rejected: llvm-mc error: <stdin>:1:5: error: invalid operand for instruction
inb %al, %al
    ^~~
.
minimal failing input: mnemonic = "inb", src = "al", dst = "al"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_in_pbt.rs (test_encode_in_regression_wrong_reg_al_al)
