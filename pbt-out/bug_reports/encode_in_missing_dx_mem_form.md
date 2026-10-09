# Bug: encode_in rejects AT&T (%dx) memory port form

**Law:** GNU AT&T assemblers accept `inb (%dx), %al` / `inl (%dx), %eax` as an alias of the DX-port form and emit EC/ED; the encoder must accept Memory(%dx) + data-register the same way the x86-64 sibling does.
**Impact:** Valid AT&T input that uses the parenthesized port form fails with `unsupported inb/inl operands`, breaking assembly of code that uses `(%dx)` (common in kernel/boot sources).
**Function:** encode_in
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:93
**Detected by:** Differential vs llvm-mc (encode_in_diff_dx_memory_form); KAT encode_in_kat_llvm_mc_inl_dx_mem
**Minimal input:** `inl (%dx), %eax` (ops = [Memory(base=dx), Register("eax")])
**Expected:** `Ok([0xed])` (llvm-mc encoding)
**Actual:** `Err("unsupported inl operands")`
**Severity:** medium
**Root cause:** The match arms cover only `(Register, Register)` and `(Immediate, Register)`; there is no `(Memory, Register)` arm. The x86-64 sibling at system.rs:83-87 already handles this form.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:93`
```rust
match (&ops[0], &ops[1]) {
    (Operand::Register(_src), Operand::Register(_dst)) => {
        if size == 2 { self.bytes.push(0x66); }
        self.bytes.push(if size == 1 { 0xEC } else { 0xED });
        Ok(())
    }
    (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(_dst)) => {
        ...
    }
    _ => Err(format!("unsupported {} operands", mnemonic)),
}
```
**Suggested fix:** Add a Memory,Register arm mirroring the x86 sibling:
```rust
(Operand::Memory(_), Operand::Register(_)) => {
    // inl (%dx), %eax  =>  same encoding as register form
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xEC } else { 0xED });
    Ok(())
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_in_pbt::test_encode_in_regression_dx_mem_form -- --test-threads=1
cargo test --lib encode_in_pbt::encode_in_diff_dx_memory_form -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid AT&T form `inb (%dx), %al`: unsupported inb operands; llvm-mc=[ec].
minimal failing input: mnemonic = "inb"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_in_pbt.rs (test_encode_in_regression_dx_mem_form)
