# Bug: encode_push encodes r16 as bare r32 short form (missing 0x66)
**Law:** In 32-bit mode, PUSH r16 requires the operand-size override prefix 0x66 before 0x50+rw (Intel SDM; llvm-mc `push %ax` → `[0x66, 0x50]`). Emitting bare `0x50` is PUSH r32 of the corresponding register.
**Impact:** `push %ax` (dispatched to encode_push via mnemonic `push`) silently produces `pushl %eax` machine code — wrong stack delta (4 bytes vs 2) and clobbers the high half of EAX's stack slot semantics.
**Function:** encode_push
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351
**Detected by:** Differential — llvm-mc i686 (5)
**Minimal input:** `push %ax` (mnemonic `push`, Reg("ax"))
**Expected:** `[0x66, 0x50]`
**Actual:** `[0x50]`
**Severity:** high
**Root cause:** Register arm uses `reg_num` which aliases ax→0 same as eax, then emits `0x50+n` with no `reg_size` check and no 0x66. Mnemonic `push` (unsuffixed) routes to encode_push (mod.rs:191); `pushw` routes to encode_push16 which only handles immediates.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Emit 0x66 when the register is 16-bit GP.
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                if reg_size(&reg.name) == 2 && !is_segment_reg(&reg.name) {
                    self.bytes.push(0x66);
                }
                self.bytes.push(0x50 + num);
                Ok(())
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push_diff_r16_via_push -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[80]`,
 right: `[102, 80]`: r16 push diff `push %ax`: sut=[50] mc=[66, 50]
minimal failing input: r16 = "ax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_push_pbt.rs (encode_push_diff_r16_via_push; add dedicated KAT if desired)
