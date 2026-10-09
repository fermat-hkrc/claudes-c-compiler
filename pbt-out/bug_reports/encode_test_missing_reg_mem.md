# Bug: encode_test rejects valid Reg→Mem TEST forms
**Law:** AT&T `test{b,w,l} %reg, mem` (Intel TEST r/m, r) must encode to 84/85 + ModR/M memory, matching llvm-mc and the x86-64 sibling.
**Impact:** Any assembler path emitting `test %eax, (%ebx)`-style forms fails with "unsupported test operands", so condition tests against memory cannot be assembled on i686.
**Function:** encode_test
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:645
**Detected by:** Differential — llvm-mc i686 (Reg→Mem bare)
**Minimal input:** `testb %al, (%eax)` (also `testl %eax, (%ebx)`)
**Expected:** `[0x84, 0x00]` for testb %al, (%eax); `[0x85, 0x03]` for testl %eax, (%ebx)
**Actual:** `Err("unsupported test operands")`
**Severity:** high
**Root cause:** `gp_integer.rs:708` default arm — match covers RR, Imm→Reg, Imm→Mem only; no `(Register, Memory)` arm (x86-64 sibling has it at gp_integer.rs:608-616).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:708`
```rust
            _ => Err("unsupported test operands".to_string()),
```
**Suggested fix:** Add a Reg→Mem arm mirroring the x86-64 sibling (with emit_segment_prefix + optional 0x66 + 84/85 + encode_modrm_mem):
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x84 } else { 0x85 });
                self.encode_modrm_mem(src_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_test_regression_reg_mem_bare -- --test-threads=1
```
**Raw output:**
```text
regression: Reg→Mem TEST must encode, got Err(unsupported test operands)
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_test_pbt.rs (encode_test_regression_reg_mem_bare)
