# Confirmed invariants (encode_float_store)

- Valid 2-operand `fsw`/`fsd` with FP rs2, GPR base, and offset in [-2048, 2047] matches llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins fsw fa0, 0(x1) = 0x00a0a027, fsd fa0, 8(sp) = 0x00a13427, fsw fa0, 2047(x1) = 0x7ea0afa7, fsw fa0, -2048(x1) = 0x80a0a027, fsd ft0, -8(s0) = 0xfe043c27, fsw ft11, -1(t6) = 0xffffafa7.
- S-type STORE-FP layout holds: opcode=0b0100111, rs2/funct3/rs1/imm as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rs2 as fN; GPR ABI names and fp=s0=x8 encode the same rs1 as xN (1000 cases).
- Empty, missing memory operand, GPR rs2, FP base, and non-memory 2nd operand return Err (1000 cases).
- Extra operands, immediates outside [-2048, 2047], hi-type modifiers, and %lo/%pcrel_lo/%tprel_lo currently disagree with llvm-mc (see bugs): extra ignored, oob wrapped, hi remapped to Lo12S, lo modifiers kept as I-type Lo12I/PcrelLo12I.

## Environment (encode_float_store)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects fsw/fsd as missing F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs, cargo test --lib encode_float_store, proptest cases=1000.
- Dispatch: encoder/mod.rs:718 fsw => encode_float_store(..., 0b010); mod.rs:746 fsd => encode_float_store(..., 0b011) with operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries). Sweep was a manual audit of valid Mem / extra / imm OOB / hi-type / arity / GPR rs2 / FP base / reloc-lo. Closed: tier round spent; remaining documented gaps are the four filed bugs.

## Quirks (encode_float_store)

- llvm-mc reloc-form encodings contain unresolved fixup bits (`0x27'A'`); reloc-form oracle compares SUT word against offset-0 Mem encoding and RelocType, not llvm-mc bytes. llvm-mc reports fixup_riscv_lo12_s / pcrel_lo12_s / tprel_lo12_s on fsw/fsd.
- encode_s takes low 12 bits of the i32; encode_float_store does not range-check before that pack (B1).
- parse_reloc_modifier always returns I-type lo variants (%lo → Lo12I); encode_float_store remaps only Hi20 → Lo12S / PcrelHi20 → PcrelLo12S, so valid %lo stays I-type (B4) while invalid %hi becomes Lo12S (B3).
- Bare `fsw fa0, foo` is rejected by llvm-mc and by the SUT (`float store: expected memory operand`).
- proptest 1.11 requires `#[test]` inside `proptest!`.
