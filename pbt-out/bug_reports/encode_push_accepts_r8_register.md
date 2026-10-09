# Bug: encode_push accepts r8 registers via reg_num alias
**Law:** PUSH does not accept 8-bit GP registers; `pushl %al` must be Err (llvm-mc rejects).
**Impact:** Silent mis-assembly: `pushl %al` becomes `0x50` (PUSH EAX).
**Function:** encode_push
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351
**Detected by:** Negative/Error Contract (3)
**Minimal input:** r8="al"
**Expected:** Err(...)
**Actual:** Ok([0x50])
**Severity:** high
**Root cause:** Same class as non-GP accept — `reg_num` maps al→0 same as eax (gp_integer.rs:351-354).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Reject reg_size==1.
```rust
                if reg_size(&reg.name) == 1 {
                    return Err(format!("invalid push register {}", reg.name));
                }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push_neg_r8 -- --test-threads=1
```
**Raw output:**
```text
Test failed: encode_push must reject r8 `al` when llvm-mc does, got Ok([80])
minimal failing input: r8 = "al"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push_pbt.rs
