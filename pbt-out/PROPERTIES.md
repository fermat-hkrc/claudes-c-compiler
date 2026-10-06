# Properties: encode_csr (requested encode_system)

## encode_csr_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SYSTEM/CSR decoder). encode_i / encode_csri / encode_csrr rejected as primary differential (same-job gate: private packer / explicit-imm sibling / pseudo).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint b8df2b19
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_diff_llvm_mc
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, csr ∈ KnownCsrNames. encode_csr([Reg(rd), Csr(csr), Reg(rs1)], funct3(mn)) = Word(w) ∧ w = llvm-mc("mn rd, csr, rs1")
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, csr, rs1]
  domain: { mn: csr_reg_mnemonic, rd: gpr_name, csr: known_csr_name, rs1: gpr_name }
  relation:
    op: eq
    lhs: encode_csr([Reg(rd), Csr(csr), Reg(rs1)], funct3(mn))
    rhs: llvm_mc("mn rd, csr, rs1")
generators:
  mn: { gen: string }
  rd: { gen: string }
  csr: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/README.md:311-312 System csrr/csrw/csrs/csrc; encoder/mod.rs:691-693 csrrw/csrrs/csrrc dispatch to encode_csr
```

## encode_csr_imm_auto_diff_llvm_mc
- Tier: 2
- Rationale: Documented GNU-as auto-select: a bare Imm as operand 2 encodes the immediate CSR form. Differential vs llvm-mc which also rewrites `csrrc t0, sstatus, 2` to csrrci. Stronger state machine rejected. This is the required metamorphic/differential angle on the Imm branch.
- Doc contract: system.rs:45 "GNU as allows e.g. `csrrc t0, sstatus, 2` and auto-selects the immediate form." — asserted fingerprint b89ccc22
- Seed: (none) — GNU as comment on encode_csr itself
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd ∈ GPRNames, csr ∈ KnownCsrNames, zimm ∈ 0..31. encode_csr([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn)) = Word(w) ∧ w = llvm-mc("mn rd, csr, zimm")
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, csr, zimm]
  domain: { mn: csr_reg_mnemonic, rd: gpr_name, csr: known_csr_name, zimm: u32_0_31 }
  relation:
    op: eq
    lhs: encode_csr([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn))
    rhs: llvm_mc("mn rd, csr, zimm")
generators:
  mn: { gen: string }
  rd: { gen: string }
  csr: { gen: string }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
evidence: system.rs:42-45 GNU as auto-selects immediate CSR encoding; llvm-mc matches
```

## encode_csr_i_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented I-type layout. Stronger differential is the sibling property; this pins opcode OP_SYSTEM and field placement independently of llvm-mc.
- Doc contract: encoder/mod.rs:300 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint c3260c7a
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_r_type_fields
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ 0..31, csr ∈ 0..4095. let w = encode_csr([Reg(x{rd}), Imm(csr), Reg(x{rs1})], funct3(mn)) in Word form. unpack_i(w) = (opcode=0b1110011, funct3(mn), rd, rs1, csr)
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, csr, rs1]
  domain: { mn: csr_reg_mnemonic, rd: u32_0_31, csr: u32_0_4095, rs1: u32_0_31 }
  relation:
    op: eq
    lhs: unpack_i(encode_csr([Reg(x{rd}), Imm(csr), Reg(x{rs1})], funct3(mn)))
    rhs: (0b1110011, funct3(mn), rd, rs1, csr)
generators:
  mn: { gen: string }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  csr: { gen: int, min: 0, max: 4095, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:300 I-type layout; encoder/mod.rs:354 OP_SYSTEM; README.md:353
```

## encode_csr_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names, xN, and fp=s0/x8 name the same GPR. Imm 0..=31 as rd is the get_reg GCC bare-number contract. Imm as operand 2 is the GNU-as zimm path, not xN. Stronger differential is the sibling property.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_abi_xn_alias
- Formal: ∀ n, k ∈ 0..31, mn ∈ {csrrw,csrrs,csrrc}, csr ∈ KnownCsrNames. encode_csr([Reg(x{n}), Csr(csr), Reg(x{k})], f3) = encode_csr([Reg(ABI[n]), Csr(csr), Reg(ABI[k])], f3). When n=8 or k=8, Reg("fp") agrees. Imm(n) as rd agrees with x{n}.
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, k, mn, csr]
  domain: { n: u32_0_31, k: u32_0_31, mn: csr_reg_mnemonic, csr: known_csr_name }
  relation:
    op: eq
    lhs: encode_csr([Reg(x{n}), Csr(csr), Reg(x{k})], funct3(mn))
    rhs: encode_csr([Reg(ABI[n]), Csr(csr), Reg(ABI[k])], funct3(mn))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
  mn: { gen: string }
  csr: { gen: string }
evidence: parser.rs:22 GPR names; encoder/mod.rs:370 get_reg
```

## encode_csr_name_vs_imm
- Tier: 4
- Rationale: Metamorphic: Csr(name), Imm(number), Symbol(name), and hex Csr("0xNNN") are documented get_csr_num alternatives for the same 12-bit CSR. Stronger differential is the sibling property.
- Doc contract: system.rs:64-68 get_csr_num accepts Imm, Csr, Symbol, Reg — helper of encode_csr
- Seed: (none)
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, (name, num) ∈ KnownCsrs. encode_csr([Reg(rd), Csr(name), Reg(rs1)], f3) = encode_csr([Reg(rd), Imm(num), Reg(rs1)], f3) = encode_csr([Reg(rd), Symbol(name), Reg(rs1)], f3) = encode_csr([Reg(rd), Csr("0x{num:x}"), Reg(rs1)], f3)
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, rd, name, num, rs1]
  domain: { mn: csr_reg_mnemonic, rd: gpr_name, name: known_csr_name, num: csr_number_of(name), rs1: gpr_name }
  relation:
    op: eq
    lhs: encode_csr([Reg(rd), Csr(name), Reg(rs1)], funct3(mn))
    rhs: encode_csr([Reg(rd), Imm(num), Reg(rs1)], funct3(mn))
generators:
  mn: { gen: string }
  rd: { gen: string }
  name: { gen: string }
  num: { gen: int, min: 0, max: 4095, type: u32 }
  rs1: { gen: string }
evidence: system.rs:64-68 get_csr_num Imm/Csr/Symbol/Reg
```

## encode_csr_neg_extra
- Tier: 4
- Rationale: Negative/error: llvm-mc rejects a fourth operand (`invalid operand for instruction`). encode_instruction passes operands through; extra must Err. Documented by llvm-mc / assembler contract (README.md:6-7 textual assembly).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint b8df2b19
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_neg_extra
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, csr ∈ KnownCsrNames, extra ∈ Operands. encode_csr([Reg(rd), Csr(csr), Reg(rs1), extra], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: failing
- Counterexample: encode_csr([Reg("x0"), Csr("fflags"), Reg("x0"), Imm(0)], 0b001) => Ok(Word(1052787))
- Bug report: pbt-out/bug_reports/encode_csr_extra_operand.md

```property
function: encoder.encode_csr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, csr, rs1, extra]
  domain: { mn: csr_reg_mnemonic, rd: gpr_name, csr: known_csr_name, rs1: gpr_name, extra: operand }
  relation:
    op: throws
    expr: encode_csr([Reg(rd), Csr(csr), Reg(rs1), extra], funct3(mn))
generators:
  mn: { gen: string }
  rd: { gen: string }
  csr: { gen: string }
  rs1: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc rejects extra operands on csrrw; README.md:6-7 assembler emits textual assembly
```

## encode_csr_neg_zimm_oob
- Tier: 4
- Rationale: Negative/error: llvm-mc requires zimm in [0, 31]. Documented uimm5 bound must be sampled at bound±1. Field masking of out-of-range values is the recurring assembler bug class.
- Doc contract: system.rs:45 "GNU as allows e.g. `csrrc t0, sstatus, 2` and auto-selects the immediate form." — asserted fingerprint b89ccc22. The comment documents auto-select for a valid zimm, not wrapping of out-of-range values.
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_neg_oob_imm
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd ∈ GPRNames, csr ∈ KnownCsrNames, zimm ∉ 0..31. encode_csr([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: failing
- Counterexample: encode_csr([Reg("x0"), Csr("fflags"), Imm(-1)], 0b001) => Ok(Word(2084979))
- Bug report: pbt-out/bug_reports/encode_csr_zimm_oob.md

```property
function: encoder.encode_csr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, csr, zimm]
  domain: { mn: csr_reg_mnemonic, zimm: i64_outside_0_31 }
  relation:
    op: throws
    expr: encode_csr([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn))
generators:
  mn: { gen: string }
  rd: { gen: string }
  csr: { gen: string }
  zimm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc "immediate must be an integer in the range [0, 31]"
```

## encode_csr_neg_csr_oob
- Tier: 4
- Rationale: Negative/error: llvm-mc requires CSR number in [0, 4095]. Documented 12-bit csr[11:0] bound must be sampled at bound±1.
- Doc contract: encoder/mod.rs:300 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint c3260c7a. CSR occupies the 12-bit imm field.
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_neg_oob_imm
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, csr_num ∉ 0..4095. encode_csr([Reg(rd), Imm(csr_num), Reg(rs1)], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: failing
- Counterexample: encode_csr([Reg("x0"), Imm(-1), Reg("x0")], 0b001) => Ok(Word(4293922931))
- Bug report: pbt-out/bug_reports/encode_csr_csr_oob.md

```property
function: encoder.encode_csr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, csr_num, rs1]
  domain: { mn: csr_reg_mnemonic, csr_num: i64_outside_0_4095 }
  relation:
    op: throws
    expr: encode_csr([Reg(rd), Imm(csr_num), Reg(rs1)], funct3(mn))
generators:
  mn: { gen: string }
  rd: { gen: string }
  csr_num: { gen: int, type: i64 }
  rs1: { gen: string }
expected_error: String
evidence: llvm-mc "immediate must be an integer in the range [0, 4095]"
```

## encode_csr_reg_and_decimal_csr
- Tier: 4
- Rationale: Sweep: documented get_csr_num arms Operand::Reg ("sometimes CSR names look like regs") and decimal name.parse::<u32>(). Metamorphic vs Csr(name). Stronger differential is the sibling property.
- Doc contract: system.rs:69 "sometimes CSR names look like regs" — asserted fingerprint 9281a0d6
- Seed: (none)
- Formal: ∀ mn ∈ {csrrw,csrrs,csrrc}, rd, rs1 ∈ GPRNames, (name, num) ∈ KnownCsrs. encode_csr([Reg(rd), Reg(name), Reg(rs1)], f3) = encode_csr([Reg(rd), Csr(name), Reg(rs1)], f3) = encode_csr([Reg(rd), Csr(num.to_string()), Reg(rs1)], f3)
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, rd, name, num, rs1]
  domain: { mn: csr_reg_mnemonic, rd: gpr_name, name: known_csr_name, num: csr_number_of(name), rs1: gpr_name }
  relation:
    op: eq
    lhs: encode_csr([Reg(rd), Reg(name), Reg(rs1)], funct3(mn))
    rhs: encode_csr([Reg(rd), Csr(name), Reg(rs1)], funct3(mn))
generators:
  mn: { gen: string }
  rd: { gen: string }
  name: { gen: string }
  num: { gen: int, min: 0, max: 4095, type: u32 }
  rs1: { gen: string }
evidence: system.rs:69 sometimes CSR names look like regs; system.rs:110-114 parse as number
```

## encode_csr_neg_arity_fp_unknown
- Tier: 4
- Rationale: Negative/error: empty/missing operands, FP registers as rd/rs1, unknown CSR names, and invalid GPR names must Err (get_reg / get_csr_num contracts).
- Doc contract: encoder/mod.rs:370 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint 27d70208. system.rs:110 "unknown CSR: {}"
- Seed: encode_alu_reg_w_pbt.rs:encode_alu_reg_w_neg_arity_fp
- Formal: ∀ f3 ∈ {001,010,011}. encode_csr([], f3) = Err(_). encode_csr([Reg(rd)], f3) = Err(_). encode_csr([Reg(rd), Csr(csr)], f3) = Err(_) (missing rs1). FP dest/rs1 Err. unknown CSR name Err. invalid GPR name Err.
- Test file: src/backend/riscv/assembler/encoder/encode_csr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, csr, rs1, fp, bad, unknown]
  domain: { mn: csr_reg_mnemonic, fp: fp_name, bad: invalid_gpr_name, unknown: unknown_csr_name }
  relation:
    op: throws
    expr: encode_csr(&[], funct3(mn))
generators:
  mn: { gen: string }
  rd: { gen: string }
  csr: { gen: string }
  rs1: { gen: string }
  fp: { gen: string }
  bad: { gen: string }
  unknown: { gen: string }
expected_error: String
evidence: get_reg expected register; get_csr_num unknown CSR; llvm-mc rejects FP/unknown
```
