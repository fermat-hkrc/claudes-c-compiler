# Bug: encode_bit_count rejects valid memory source (r32, r/m32)

**Law:** ∀ m ∈ {lzcntl,tzcntl,popcntl}, ∀ base,d ∈ GP32. encode(m, Mem(base), Reg(d)) = llvm-mc encoding of `m (%base), %d` (Intel SDM form r32, r/m32)
**Impact:** Assembler cannot encode `lzcntl (%eax), %ebx` and siblings; any AT&T source using a memory operand for LZCNT/TZCNT/POPCNT fails with "unsupported … operands" even though the form is architectural and accepted by llvm-mc / gas.
**Function:** encode_bit_count
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:971
**Detected by:** Differential — llvm-mc i686 (encode_bit_count_diff_mem_llvm_mc)
**Minimal input:** `lzcntl (%eax), %eax`
**Expected:** Ok([0xf3, 0x0f, 0xbd, 0x00]) (llvm-mc)
**Actual:** Err("unsupported lzcntl operands")
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:971-980` — the match only arms `(Register, Register)`; memory source falls through to the default Err. Sibling `encode_bsr_bsf` and the x86-64 `encode_bit_count` both encode `(Memory, Register)`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:971`
```rust
        match (&ops[0], &ops[1]) {
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(prefix);
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            _ => Err(format!("unsupported {} operands", mnemonic)),
        }
```
**Suggested fix:** Add a `(Memory, Register)` arm mirroring `encode_bsr_bsf` / x86 `encode_bit_count`:
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(prefix);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bit_count_regression_mem_src_eax_eax -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid mem-source `lzcntl (%eax), %eax`: unsupported lzcntl operands; Intel SDM / llvm-mc encode r32, r/m32 (got mc=[f3, 0f, bd, 00]).
minimal failing input: m = "lzcntl", base = "eax", d = "eax"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs (test_encode_bit_count_regression_mem_src_eax_eax)
