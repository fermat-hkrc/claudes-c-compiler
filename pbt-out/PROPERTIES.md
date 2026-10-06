# Properties: encode_alu_imm_w (requested encode_op_imm32)

## encode_alu_imm_w_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree OP-IMM-32 decoder). encode_i / encode_alu_imm / encode_shift_imm_w / C.ADDIW rejected as primary differential (same-job gate: private packer / OP-IMM 64-bit / shamt I-type / compressed).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_diff_imm_llvm_mc
- Formal: ∀ rd, rs1 ∈ GPRNames, imm ∈ [-2048, 2047]. encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm)], 0b000) = Word(w) ∧ w = llvm-mc("addiw rd, rs1, imm")
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm_w
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, imm]
  domain: { rd: gpr_name, rs1: gpr_name, imm: i12 }
  relation:
    op: eq
    lhs: encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm)], 0b000)
    rhs: llvm_mc("addiw rd, rs1, imm")
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/README.md:301 addiw is I-type; encoder/mod.rs:563 addiw => encode_alu_imm_w
```

## encode_alu_imm_w_i_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented I-type layout. Stronger differential is the sibling property; this pins opcode OP_OP_IMM_32 and field placement independently of llvm-mc.
- Doc contract: encoder/mod.rs:298 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_i_type_fields
- Formal: ∀ rd, rs1 ∈ 0..31, imm ∈ [-2048, 2047]. let w = encode_alu_imm_w([Reg(x{rd}), Reg(x{rs1}), Imm(imm)], 0b000) in Word form. unpack_i(w) = (opcode=0b0011011, funct3=0, rd, rs1, imm)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm_w
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, imm]
  domain: { rd: u32_0_31, rs1: u32_0_31, imm: i12 }
  relation:
    op: eq
    lhs: unpack_i(encode_alu_imm_w([Reg(x{rd}), Reg(x{rs1}), Imm(imm)], 0b000))
    rhs: (0b0011011, 0, rd, rs1, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: encoder/mod.rs:298 I-type layout; encoder/mod.rs:348 OP_OP_IMM_32; README.md:353
```

## encode_alu_imm_w_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names, xN, and fp=s0/x8 name the same GPR. Stronger differential is the sibling property; this is a behavior-preserving rename.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_abi_xn_alias
- Formal: ∀ n, m ∈ 0..31, imm ∈ [-2048, 2047]. encode_alu_imm_w([Reg(x{n}), Reg(x{m}), Imm(imm)], 0) = encode_alu_imm_w([Reg(ABI[n]), Reg(ABI[m]), Imm(imm)], 0). When n=8 or m=8, Reg("fp") agrees.
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm_w
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, imm]
  domain: { n: u32_0_31, m: u32_0_31, imm: i12 }
  relation:
    op: eq
    lhs: encode_alu_imm_w([Reg(x{n}), Reg(x{m}), Imm(imm)], 0b000)
    rhs: encode_alu_imm_w([Reg(ABI[n]), Reg(ABI[m]), Imm(imm)], 0b000)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: parser.rs:22 GPR names; encoder/mod.rs:370 get_reg via reg_num
```

## encode_alu_imm_w_imm_as_reg
- Tier: 4
- Rationale: Metamorphic under get_reg's documented GCC bare-register-number contract: Imm n in 0..=31 encodes as x{n}.
- Doc contract: encoder/mod.rs:370 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint f1b1a1fb
- Seed: encode_alu_reg_pbt.rs Imm 0-31 vs xN
- Formal: ∀ n, m ∈ 0..31, imm ∈ [-2048, 2047]. encode_alu_imm_w([Imm(n), Imm(m), Imm(imm)], 0) = encode_alu_imm_w([Reg(x{n}), Reg(x{m}), Imm(imm)], 0)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm_w
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, imm]
  domain: { n: u32_0_31, m: u32_0_31, imm: i12 }
  relation:
    op: eq
    lhs: encode_alu_imm_w([Imm(n), Imm(m), Imm(imm)], 0b000)
    rhs: encode_alu_imm_w([Reg(x{n}), Reg(x{m}), Imm(imm)], 0b000)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: encoder/mod.rs:370 GCC bare register numbers 0-31
```

## encode_alu_imm_w_neg_imm_oob
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects immediates outside [-2048, 2047] for addiw. Documented I-type imm[11:0] is a 12-bit signed field. Domain includes bound±1.
- Doc contract: README.md:353 "I-type:  [    imm[11:0]  | rs1 | funct3 |  rd  | opcode]" — asserted fingerprint a59ffa62
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_neg_imm_oob
- Formal: ∀ rd, rs1 ∈ GPRNames, imm ∉ [-2048, 2047]. llvm-mc rejects "addiw rd, rs1, imm" ⇒ encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm)], 0) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm_w([Reg("x0"), Reg("x0"), Imm(2048)], 0) = Ok(Word(0x8000001b))
- Bug report: bug_reports/encode_alu_imm_w_imm_oob.md

```property
function: encoder.encode_alu_imm_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm]
  domain: { rd: gpr_name, rs1: gpr_name, imm: oob_i12 }
  relation:
    op: throws
    expr: encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm)], 0b000)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, type: i64 }
expected_error: Err
evidence: llvm-mc addiw error integer in the range [-2048, 2047]; README.md:353 imm[11:0]
```

## encode_alu_imm_w_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a fourth operand on addiw. encode_alu_imm_w has no arity check (same class as encode_alu_imm extra-operand bug).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_neg_extra
- Formal: ∀ rd, rs1 ∈ GPRNames, imm ∈ [-2048, 2047], extra ∈ Operand. encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm), extra], 0) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm_w([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], 0) = Ok(Word(0x0000001b))
- Bug report: bug_reports/encode_alu_imm_w_extra_operand.md

```property
function: encoder.encode_alu_imm_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, extra]
  domain: { rd: gpr_name, rs1: gpr_name, imm: i12, extra: Operand }
  relation:
    op: throws
    expr: encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm), extra], 0b000)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string }
expected_error: Err
evidence: llvm-mc invalid operand for instruction on addiw x1, x2, 0, x3
```

## encode_alu_imm_w_neg_arity_fp
- Tier: 4
- Rationale: Negative/error contract. Empty/missing operands and FP registers are not GPRs; get_reg returns Err. Non-Imm 3rd operand fails get_imm.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_neg_arity_fp
- Formal: ∀ rd, rs1 ∈ GPRNames, imm ∈ [-2048, 2047], fp ∈ FPNames, bad ∈ {Csr, FenceArg, RoundingMode, Label, SymbolOffset, Mem, MemSymbol}. encode_alu_imm_w([], 0) = Err ∧ encode_alu_imm_w([Reg(rd)], 0) = Err ∧ encode_alu_imm_w([Reg(rd), Reg(rs1)], 0) = Err ∧ encode_alu_imm_w([Reg(fp), Reg(rs1), Imm(imm)], 0) = Err ∧ encode_alu_imm_w([Reg(rd), Reg(fp), Imm(imm)], 0) = Err ∧ encode_alu_imm_w([Reg(rd), Reg(rs1), bad], 0) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, fp, bad]
  domain: { rd: gpr_name, rs1: gpr_name, imm: i12, fp: fp_name, bad: non_imm_operand }
  relation:
    op: throws
    expr: encode_alu_imm_w_arity_fp_bad(rd, rs1, imm, fp, bad)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  fp: { gen: string }
  bad: { gen: string }
expected_error: Err
evidence: get_reg invalid integer register; get_imm expected immediate
```

## encode_alu_imm_w_reloc_lo
- Tier: 2
- Rationale: Differential/reloc contract. llvm-mc accepts addiw rd, rs1, %lo(s) / %pcrel_lo(s) / %tprel_lo(s) as I-type lo12 relocs. Sibling encode_alu_imm implements this for OP-IMM. RelocType docs name I-type lo12 for ADDI/LW/LD; addiw is the OP-IMM-32 I-type twin. encode_alu_imm_w uses get_imm only (no Symbol branch).
- Doc contract: encoder/mod.rs:81 "R_RISCV_LO12_I - for ADDI/LW/LD (absolute low 12 bits, I-type)" — asserted fingerprint 8c62bff5
- Seed: encode_alu_imm_pbt.rs:encode_alu_imm_reloc_lo
- Formal: ∀ rd, rs1 ∈ GPRNames, s ∈ ident. encode_alu_imm_w([Reg(rd), Reg(rs1), Symbol("%lo(s)")], 0) = WordWithReloc { word = encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(0)], 0), reloc_type=Lo12I, symbol=s, addend=0 }. Likewise %pcrel_lo → PcrelLo12I and %tprel_lo → TprelLo12I.
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm_w([Reg("x0"), Reg("x0"), Symbol("%lo(foo)")], 0) = Err("expected immediate at operand 2")
- Bug report: bug_reports/encode_alu_imm_w_missing_lo_reloc.md

```property
function: encoder.encode_alu_imm_w
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, s]
  domain: { rd: gpr_name, rs1: gpr_name, s: ident }
  relation:
    op: eq
    lhs: encode_alu_imm_w([Reg(rd), Reg(rs1), Symbol("%lo(s)")], 0b000)
    rhs: WordWithReloc(word=addiw_imm0, Lo12I, s, 0)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
evidence: encoder/mod.rs:81 Lo12I I-type; llvm-mc accepts addiw ra, sp, %lo(foo) with fixup_riscv_lo12_i; encode_alu_imm sibling reloc form
```

## encode_alu_imm_w_neg_invalid_name
- Tier: 4
- Rationale: Sweep — get_reg's `invalid integer register` arm (x32/foo/v0/xzr/w0) was not a dedicated generator in the first batch. Negative/error contract: llvm-mc rejects those names as integer registers.
- Doc contract: (none) on encode_alu_imm_w itself — no rustdoc. get_reg (encoder/mod.rs:368) returns Err for unknown integer register names.
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_neg_invalid_name
- Formal: ∀ rd, rs1 ∈ GPRNames, imm ∈ [-2048, 2047], bad ∈ {x32, x33, x99, foo, v0, v31, xzr, w0}, which ∈ {0,1}. encode_alu_imm_w(ops with operand[which]=Reg(bad), 0) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, bad, which]
  domain: { rd: gpr_name, rs1: gpr_name, imm: i12, bad: invalid_gpr, which: 0_or_1 }
  relation:
    op: throws
    expr: encode_alu_imm_w(ops_with_bad_name, 0b000)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  bad: { gen: string }
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: Err
evidence: encoder/mod.rs:368 invalid integer register; llvm-mc rejects x32/foo/v0/xzr/w0
```
