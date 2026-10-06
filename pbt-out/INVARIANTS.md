# Confirmed invariants (encode_float_load)

- Valid 2-operand `flw`/`fld` with FP dest, GPR base, and offset in [-2048, 2047] matches llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins flw fa0, 0(x1) = 0x0000a507, fld fa0, 8(sp) = 0x00813507, flw fa0, 2047(x1) = 0x7ff0a507, flw fa0, -2048(x1) = 0x8000a507, fld ft0, -8(s0) = 0xff843007, flw ft11, -1(t6) = 0xffffaf87.
- I-type LOAD-FP layout holds: opcode=0b0000111, rd/funct3/rs1/imm as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd as fN; GPR ABI names and fp=s0=x8 encode the same rs1 as xN (1000 cases).
- %pcrel_lo/%lo/%tprel_lo MemSymbol form yields WordWithReloc whose word equals offset-0 encoding, addend 0, and reloc_type PcrelLo12I/Lo12I/TprelLo12I (1000 cases).
- Empty, missing memory operand, GPR dest, FP base, and non-memory 2nd operand return Err (1000 cases).
- Extra operands, immediates outside [-2048, 2047], and %hi/%pcrel_hi/%tprel_hi currently disagree with llvm-mc (see bugs).

## Environment (encode_float_load)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects flw/fld as missing F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs, cargo test --lib encode_float_load, proptest cases=1000.
- Dispatch: encoder/mod.rs:715 flw => encode_float_load(..., 0b010); mod.rs:743 fld => encode_float_load(..., 0b011) with operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries). Sweep was a manual audit of valid Mem / extra / imm OOB / hi-type / arity / GPR dest / FP base. Closed: tier round spent; remaining documented gaps are the three filed bugs.

## Quirks (encode_float_load)

- llvm-mc reloc-form encodings contain unresolved fixup bits (`0bAAAA0000`); reloc-form oracle compares SUT word against offset-0 Mem encoding, not llvm-mc bytes.
- encode_i masks imm to 12 bits; encode_float_load does not range-check before that pack (B1).
- Bare `flw fa0, foo` is rejected by llvm-mc ("too few operands") and by the SUT (`float load: expected memory operand`); no auipc+load expansion (unlike integer encode_load).
- proptest 1.11 requires `#[test]` inside `proptest!`.
