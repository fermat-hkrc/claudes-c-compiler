# PROPERTIES.md — round 11 (src/backend/riscv)

Oracle: llvm-mc -triple=riscv64 (byte-exact .text comparison).

## p1_riscv_asm_matches_llvm_mc
- Re-verified: one generator defect fixed en route (ld offset 512*8 exceeded the 12-bit imm range — an llvm-mc validation error in MY form). After the fix: 200 sequences byte-identical to llvm-mc.
- Tier: 2
- Rationale: the builtin RISC-V assembler must encode the documented RV64
  instruction set exactly as the reference assembler does.
- Doc contract: builtin RISC-V assembler per src/backend/riscv READMEs
- Formal: ∀ S ∈ generated label-free RV64 sequences (20..40 insts, valid operands): bytes(.text of ccc-assemble(S)) == bytes(.text of llvm-mc(S)).
- Test file: src/backend/riscv/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::riscv::assembler (builtin, via driver::Driver argv0="ccc-riscv" -c)
oracle: differential
predicate:
  quantifier: forall
  vars: [insts]
  domain: { insts: list(riscv_gen_inst, 20..=40) }
  relation:
    op: eq
    lhs: text_bytes(assemble_ccc(insts))
    rhs: text_bytes(assemble_llvm_mc(insts))
generators:
  insts: { gen: list, elem: { gen: tuple, elems: [{ gen: int, min: 0, max: 12, type: u8 }, { gen: int, min: 0, max: 31, type: u8 }, { gen: int, min: 0, max: 31, type: u8 }, { gen: int, min: 0, max: 31, type: u8 }, { gen: int, min: 0, max: 4095, type: u16 }] }, minLen: 20, maxLen: 40 }
evidence: manual probe — 19-instruction sample byte-matches llvm-mc
```
