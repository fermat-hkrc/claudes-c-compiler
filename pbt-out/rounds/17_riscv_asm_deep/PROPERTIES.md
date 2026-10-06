# PROPERTIES.md — round 17 (RISC-V assembler deep)

## p_imm12_range_witness
- Status: failing — shrunk `addi x0, x0, -2049`-class; addi 2048 encodes as -2048 (bytes verified) → #796.
## p_shamt_range_witness
- Status: failing — shrunk `slli x0, x0, 64`-class → #797.
## p_malformed_witness
- Status: failing — shrunk `add x0, x0, 2048`-class (imm in reg slot; beq raw offset; fence.i operands) → #798.
- Test files: src/backend/riscv/mod.rs (pbt_witness)
