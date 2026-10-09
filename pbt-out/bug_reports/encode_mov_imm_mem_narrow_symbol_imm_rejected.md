# Bug: encode_mov_imm_mem rejects symbol immediates for movb/movw
**Law:** ∀ width ∈ {1,2}, base, disp. encode(mov{b,w} $sym, mem) equals llvm-mc encoding (FK_Data_1 / FK_Data_2 fixup placeholders)
**Impact:** Valid AT&T `movb $sym, (%eax)` / `movw $sym, mem` is rejected instead of emitting C6/C7 with a 1- or 2-byte relocatable immediate; callers cannot assemble symbol immediates into byte/word memory stores
**Function:** encode_mov_imm_mem
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:238
**Detected by:** Differential — llvm-mc i686 (narrow symbol immediate forms)
**Minimal input:** `movb $sym, (%eax)` (width=1, base=eax, disp=0); also `movw $sym, (%eax)`
**Expected:** llvm-mc bytes `[0xc6, 0x00, 0x00]` with FK_Data_1 fixup (movb); `[0x66, 0xc7, 0x00, 0x00, 0x00]` with FK_Data_2 (movw)
**Actual:** `Err("symbol immediate only supported for 32-bit mov to memory")`
**Severity:** medium (documented by the author)
**Root cause:** gp_integer.rs:259-265 only emits R_386_32 + 4 zero bytes when size==4; otherwise returns Err. The error string admits the gap on inputs the public movb/movw path accepts (mod.rs:167-169). llvm-mc accepts both widths.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:259`
```rust
            ImmediateValue::Symbol(sym) | ImmediateValue::SymbolPlusOffset(sym, _) => {
                let addend = if let ImmediateValue::SymbolPlusOffset(_, a) = imm { *a } else { 0 };
                if size == 4 {
                    self.add_relocation(sym, R_386_32, addend);
                    self.bytes.extend_from_slice(&[0, 0, 0, 0]);
                } else {
                    return Err("symbol immediate only supported for 32-bit mov to memory".to_string());
                }
            }
```
**Suggested fix:** Support size 1 and 2 with appropriate relocation width and placeholder bytes (R_386_8 / R_386_16 or the tree's equivalent), e.g.:
```rust
            ImmediateValue::Symbol(sym) | ImmediateValue::SymbolPlusOffset(sym, _) => {
                let addend = if let ImmediateValue::SymbolPlusOffset(_, a) = imm { *a } else { 0 };
                match size {
                    1 => {
                        self.add_relocation(sym, R_386_8, addend); // or project reloc const
                        self.bytes.push(0);
                    }
                    2 => {
                        self.add_relocation(sym, R_386_16, addend);
                        self.bytes.extend_from_slice(&[0, 0]);
                    }
                    4 => {
                        self.add_relocation(sym, R_386_32, addend);
                        self.bytes.extend_from_slice(&[0, 0, 0, 0]);
                    }
                    _ => unreachable!(),
                }
            }
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_imm_mem_diff_symbol_narrow -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected valid narrow symbol imm `movb $sym, (%eax)`: symbol immediate only supported for 32-bit mov to memory; llvm-mc encodes [c6, 00, 00]. AT&T movb/movw $sym,mem is valid (FK_Data_1/2); body only allows size==4..
minimal failing input: width = 1, base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs (`encode_mov_imm_mem_diff_symbol_narrow` property; KAT path via same witness)
