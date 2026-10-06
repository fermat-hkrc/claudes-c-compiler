# Bug: encode_alu_imm_w rejects I-type %lo/%pcrel_lo/%tprel_lo
**Law:** addiw rd, rs1, %lo(s) (and %pcrel_lo / %tprel_lo) must encode as WordWithReloc with word = addiw rd, rs1, 0 and reloc type Lo12I / PcrelLo12I / TprelLo12I, matching llvm-mc (fixup_riscv_lo12_i) and the documented I-type lo12 relocs.
**Impact:** Valid I-type relocation forms on addiw fail to assemble (`expected immediate`), so RV64 code that uses addiw with %lo/%pcrel_lo/%tprel_lo cannot be encoded. Sibling encode_alu_imm implements this for OP-IMM.
**Function:** encode_alu_imm_w
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:267
**Detected by:** Differential — llvm-mc I-type lo12 reloc
**Minimal input:** encode_alu_imm_w([Reg("x0"), Reg("x0"), Symbol("%lo(foo)")], funct3=0)  // addiw x0, x0, %lo(foo)
**Expected:** Ok(WordWithReloc { word: 0x0000001b, reloc: Lo12I, symbol: "foo", addend: 0 })
**Actual:** Err("expected immediate at operand 2, got Some(Symbol(\"%lo(foo)\"))")
**Severity:** medium
**Root cause:** base.rs:270 always calls get_imm on operand 2; unlike encode_alu_imm there is no Symbol branch to emit WordWithReloc.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:270`
```rust
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
```
**Suggested fix:** Accept Symbol %lo/%pcrel_lo/%tprel_lo the same way encode_alu_imm does, packing imm=0 with the matching I-type lo12 reloc.
```rust
    match &operands.get(2) {
        Some(Operand::Imm(imm)) => {
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, *imm as i32)))
        }
        Some(Operand::Symbol(s)) => {
            let (reloc_type, sym) = parse_reloc_modifier(s);
            Ok(EncodeResult::WordWithReloc {
                word: encode_i(OP_OP_IMM_32, rd, funct3, rs1, 0),
                reloc: Relocation { reloc_type, symbol: sym, addend: 0 },
            })
        }
        _ => Err("alu_imm_w: expected immediate".to_string()),
    }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w_reloc_lo -- --test-threads=1
```
**Raw output:**
```text
Test failed: SUT rejected %lo(foo): expected immediate at operand 2, got Some(Symbol("%lo(foo)")).
minimal failing input: rd = "x0", rs1 = "x0", s = "foo", m = "%lo"
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
