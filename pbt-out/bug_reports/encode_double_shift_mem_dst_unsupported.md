# Bug: encode_double_shift rejects valid memory-destination SHLD/SHRD

**Law:** SHLD/SHRD encode Imm8|CL, r32, r/m32. A memory destination is a documented Intel form and is accepted by llvm-mc; the public mnemonics `shldl`/`shrdl`/`shld`/`shrd` (assembler README) must encode it.
**Impact:** Valid AT&T such as `shldl $1, %eax, (%ebx)` or `shldl %cl, %eax, %fs:(%ebx)` fails assembly with `unsupported double shift operands`. Callers cannot assemble memory double-shifts; segment overrides on those forms are unreachable.
**Function:** encode_double_shift
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:940
**Detected by:** Differential — llvm-mc i686 (`encode_double_shift_diff_mem_dst`)
**Minimal input:** `shldl $1, %eax, (%ebx)` (also `shldl $0, %eax, (%eax)`)
**Expected:** `[0x0f, 0xa4, 0x03, 0x01]` (llvm-mc)
**Actual:** `Err("unsupported double shift operands")`
**Severity:** high
**Root cause:** `gp_integer.rs:925-941` only matches Imm/CL + Register + Register; the catch-all `_` arm rejects Memory destinations. No call to `emit_segment_prefix` / `encode_modrm_mem`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:940`
```rust
            _ => Err("unsupported double shift operands".to_string()),
```
**Suggested fix:** Add Imm+Reg+Mem and CL+Reg+Mem arms (and optionally size==2 → 0x66), emitting segment prefix then `0F opc[/+1]` then `encode_modrm_mem(src_num, mem)` and Imm8 when needed.
```rust
            (Operand::Immediate(ImmediateValue::Integer(count)), Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                // validate src is GP32 and count fits Imm8
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, opcode]);
                self.encode_modrm_mem(src_num, mem)?;
                self.bytes.push(*count as u8); // after Imm8-range check
                Ok(())
            }
            (Operand::Register(cl), Operand::Register(src), Operand::Memory(mem)) if cl.name == "cl" => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, opcode + 1]);
                self.encode_modrm_mem(src_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_double_shift_regression_mem_dst_rejected -- --test-threads=1
```
**Raw output:**
```text
valid mem-dst shldl must encode: "unsupported double shift operands"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs (`encode_double_shift_regression_mem_dst_rejected`, `encode_double_shift_kat_llvm_mc_mem_dst`, `encode_double_shift_kat_llvm_mc_mem_seg`)
