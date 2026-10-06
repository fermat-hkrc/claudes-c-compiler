# PROPERTIES.md — round 10 (src/backend/arm)

The oracle is llvm-mc (the reference assembler, also the repo's own
comparison standard per #577–#755): the builtin assembler's .text bytes for
a generated instruction sequence must be byte-identical to llvm-mc's.

## p1_arm_asm_matches_llvm_mc
- Re-verified: two generator defects fixed en route (ldur imm range [-256,-1]; load/store base must be an X register) — both were llvm-mc validation errors in MY forms, not SUT encodings. After the fixes: 200 sequences, byte-identical .text vs llvm-mc.
- Tier: 2
- Rationale: the builtin assembler's contract is to encode the documented
  AArch64 instruction set; llvm-mc over the same .s is the reference encoding.
  General (non-NEON) surface: shift-shift ALU (add/sub/and/orr/eor/bic/orn,
  x/w), immediate ALU (add/sub #imm12 [+lsl #12]), cmp/cmn aliases,
  movz/movk/movn (+shifts), ldr/str/ldrb/strb/ldrh/strh with unsigned and
  negative (ldur/stur) offsets, sxtw/uxtw, lsl/asr/ror by immediate,
  shifted-register operands (lsl #k).
- Doc contract: builtin AArch64 assembler per src/backend/arm READMEs (same pipeline role as x86's documented assembler)
- Formal: ∀ S ∈ generated label-free AArch64 instruction sequences (20..40 insts, valid operands): bytes(.text of ccc-assemble(S)) == bytes(.text of llvm-mc(S)).
- Test file: src/backend/arm/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend::arm::assembler (builtin, via driver::Driver -mlittle-endian -c)
oracle: differential
predicate:
  quantifier: forall
  vars: [insts]
  domain: { insts: list(arm_gen_inst, 20..=40) }
  relation:
    op: eq
    lhs: text_bytes(assemble_ccc(insts))
    rhs: text_bytes(assemble_llvm_mc(insts))
generators:
  insts: { gen: list, elem: { gen: tuple, elems: [{ gen: int, min: 0, max: 15, type: u8 }, { gen: int, min: 0, max: 31, type: u8 }, { gen: int, min: 0, max: 31, type: u8 }, { gen: int, min: 0, max: 31, type: u8 }, { gen: int, min: 0, max: 4095, type: u16 }, { gen: int, min: 0, max: 63, type: u8 }] }, minLen: 20, maxLen: 40 }
evidence: manual probe — 12-instruction sample byte-matches llvm-mc; #577–#755 establish llvm-mc as the repo's encoder reference
```
