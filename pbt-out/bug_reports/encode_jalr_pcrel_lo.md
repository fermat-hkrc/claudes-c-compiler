# Bug: encode_jalr rejects %pcrel_lo/%lo mem-symbol operands
**Law:** ∀ rd, rs1 ∈ GPR, ∀ s ∈ ident. encode_jalr([Reg(rd), MemSymbol{base: rs1, symbol: "%pcrel_lo(s)"}]) = WordWithReloc{word: encode_jalr([Reg(rd), Mem{rs1,0}]), type: PcrelLo12I, symbol: s, addend: 0} and likewise %lo → Lo12I
**Impact:** Documented `jalr ra, %pcrel_lo(sym)(ra)` (README call expansion; llvm-mc accepts) is rejected as invalid operands, so hand-written or non-pseudo JALR with a lo12 reloc cannot be assembled.
**Function:** encode_jalr
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:92
**Detected by:** Differential — llvm-mc / assembler reloc contract
**Minimal input:** encode_jalr([Reg("ra"), MemSymbol { base: "ra", symbol: "%pcrel_lo(foo)", modifier: "" }])
**Expected:** Ok(WordWithReloc { word: jalr ra, 0(ra), reloc_type: PcrelLo12I, symbol: "foo", addend: 0 })
**Actual:** Err("jalr: invalid operands")
**Severity:** medium
**Root cause:** base.rs:112 the 2-operand match handles only Reg and Mem; MemSymbol falls through to the invalid-operands error. encode_load/encode_alu_imm already map %pcrel_lo/%lo to PcrelLo12I/Lo12I for other I-type mnemonics.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:112`
```rust
                _ => Err("jalr: invalid operands".to_string()),
```
**Suggested fix:** Handle MemSymbol like encode_load's I-type reloc arm.
```rust
                Operand::MemSymbol { base, symbol, .. } => {
                    let rs1 = reg_num(base).ok_or("invalid base register")?;
                    let (reloc_type, sym) = parse_reloc_modifier(symbol);
                    let reloc_type = match reloc_type {
                        RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                        RelocType::Hi20 => RelocType::Lo12I,
                        RelocType::TprelHi20 => RelocType::TprelLo12I,
                        other => other,
                    };
                    Ok(EncodeResult::WordWithReloc {
                        word: encode_i(OP_JALR, rd, 0, rs1, 0),
                        reloc: Relocation {
                            reloc_type,
                            symbol: sym,
                            addend: 0,
                        },
                    })
                }
                _ => Err("jalr: invalid operands".to_string()),
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jalr_regression_pcrel_lo -- --test-threads=1
```
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jalr_pbt::test_encode_jalr_regression_pcrel_lo' (2734248) panicked at src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs:369:18:
expected WordWithReloc for jalr ra, %pcrel_lo(foo)(ra), got Err("jalr: invalid operands")
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
