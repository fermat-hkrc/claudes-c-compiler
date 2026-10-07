# Properties: encode_li

## encode_li_diff_llvm_mc_i32
- Tier: 5
- Rationale: Strongest oracle for the 32-bit (including 12-bit) domain is encoding agreement with llvm-mc, an independent assembler that implements the same `li rd, imm` job. encode_li_32bit rustdoc claims GAS `lui+addiw` / 12-bit `addi` compatibility on RV64. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree LI decoder). 64-bit encoding equality rejected as primary here because llvm-mc and the SUT may choose different 64-bit expansions; that domain is covered by the semantic differential.
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none) — no project-owned encode_li unit test; llvm-mc KAT `li a0, 0` = 0x00000513
- Formal: ∀ rd ∈ GPR, ∀ imm ∈ i32. encode_li([Reg(rd), Imm(imm)]) bytes = llvm-mc("li rd, imm") bytes
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_name, imm: i32 }
  relation:
    op: eq
    lhs: encode_li([Reg(rd), Imm(imm)]).words
    rhs: llvm_mc("li rd, imm").words
generators:
  rd: { gen: string }
  imm: { gen: int, min: -2147483648, max: 2147483647, type: i32 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:22-27 GAS lui+addiw; assembler/README.md:318; encoder/mod.rs:854
```

## encode_li_sem_i64
- Tier: 5
- Rationale: For the full i64 domain, encoding sequences may differ from llvm-mc. The shared contract is that executing the expansion leaves `imm` in `rd` (RISC-V Unprivileged ISA LI pseudo; rustdoc "arbitrary 64-bit immediate"). Independent RISC-V interpreter of LUI/ADDI/ADDIW/SLLI (not a copy of the decomposer) plus llvm-mc sequence simulation. State machine rejected. Encoding-equality differential rejected on 64-bit (different expansions). Round-trip rejected (lossy encode, no decoder).
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ rd ∈ GPR\{x0}, ∀ imm ∈ i64. sim(encode_li([Reg(rd), Imm(imm)]))[rd] = imm ∧ sim(llvm-mc("li rd, imm"))[rd] = imm ∧ |encode_li(...)| ≤ 16
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_nonzero, imm: i64 }
  relation:
    op: eq
    lhs: sim(encode_li([Reg(rd), Imm(imm)]))[rd]
    rhs: imm
generators:
  rd: { gen: int, min: 1, max: 31, type: u32 }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:47-51; RISC-V Unprivileged ISA LI; assembler/README.md:318
```

## encode_li_12bit_fields
- Tier: 4
- Rationale: Documented 12-bit path is a single `addi rd, x0, imm` (encode_li_32bit rustdoc; README). Field unpack from RISC-V I-type, not from the encoder body. Stronger encoding differential covers this domain; this pins the ISA layout.
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ imm ∈ [-2048, 2047]. encode_li = Word(w) ∧ opcode(w)=0010011 ∧ rd(w)=rd ∧ funct3(w)=000 ∧ rs1(w)=0 ∧ imm12(w)=imm
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: u32_0_31, imm: i64_simm12 }
  relation:
    op: holds
    expr: is_addi_x0(encode_li([Reg(xN(rd)), Imm(imm)]), rd, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:26-27; assembler/README.md:318
```

## encode_li_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names, xN, and fp/s0 alias the same 5-bit encoding (reg_num). Same encoding for the same (rd_num, imm).
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ n,m ∈ 0..31, ∀ imm ∈ i64. encode_li([Reg(ABI[n]), Imm(imm)]) = encode_li([Reg(xN), Imm(imm)])
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, imm]
  domain: { n: u32_0_31, imm: i64 }
  relation:
    op: eq
    lhs: encode_li([Reg(ABI[n]), Imm(imm)])
    rhs: encode_li([Reg(xN), Imm(imm)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:254-266 reg_num ABI/xN/fp
```

## encode_li_neg_arity
- Tier: 3
- Rationale: README two-operand form `li rd, imm`; llvm-mc "too few operands". Missing rd or imm must Err.
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ ops. |ops| < 2 ⇒ encode_li(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_lists_len_lt_2 }
  relation:
    op: throws
    expr: encode_li(ops)
expected_error: String
generators:
  ops: { gen: list, maxLen: 1 }
evidence: assembler/README.md:318; llvm-mc too few operands
```

## encode_li_neg_invalid
- Tier: 3
- Rationale: get_reg rejects non-GPR names; get_imm rejects non-Imm. Invalid rd or non-immediate second operand must Err (llvm-mc rejects).
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ bad operand in {FP/vec/invalid name, Symbol, Label, Mem, Csr, …}. encode_li with that operand at rd or imm position is Err
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good_rd, good_imm, which]
  domain: { bad: non_gpr_or_non_imm }
  relation:
    op: throws
    expr: encode_li(ops_with_bad)
expected_error: String
generators:
  bad: { gen: string }
evidence: encoder/mod.rs:455-464 get_reg; encoder/mod.rs:495-500 get_imm
```

## encode_li_neg_extra
- Tier: 3
- Rationale: README documents exactly `li rd, imm`. llvm-mc errors on a third operand. encode_li has no arity check (same class as encode_neg extra-operand). Extra operand must Err.
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ rd ∈ GPR, ∀ imm ∈ i64, ∀ extra. encode_li([Reg(rd), Imm(imm), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: failing
- Counterexample: encode_li([Reg("zero"), Imm(0), Reg("zero")]) returns Ok (li zero, 0 encoded) instead of Err. Deterministic witness: encode_li([Reg("a0"), Imm(1), Reg("a1")]).
- Bug report: pbt-out/bug_reports/encode_li_extra_operand.md

```property
function: encoder.encode_li
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, extra]
  domain: { rd: gpr_name, imm: i64, extra: operand }
  relation:
    op: throws
    expr: encode_li([Reg(rd), Imm(imm), extra])
expected_error: String
generators:
  rd: { gen: string }
  imm: { gen: int, type: i64 }
  extra: { gen: string }
evidence: assembler/README.md:318; llvm-mc invalid operand for instruction
```

## encode_li_imm_regnum
- Tier: 4
- Rationale: Metamorphic: get_reg accepts Imm(0..=31) as a bare register number (mod.rs:461-462, GCC inline asm). encode_li([Imm(n), Imm(v)]) = encode_li([Reg(xN), Imm(v)]).
- Doc contract: (none) — encode_li has no function rustdoc
- Seed: (none)
- Formal: ∀ n ∈ 0..31, ∀ imm ∈ i64. encode_li([Imm(n), Imm(imm)]) = encode_li([Reg(xN), Imm(imm)])
- Test file: src/backend/riscv/assembler/encoder/encode_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_li
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, imm]
  domain: { n: 0..31, imm: i64 }
  relation:
    op: eq
    lhs: encode_li([Imm(n), Imm(imm)])
    rhs: encode_li([Reg(xN), Imm(imm)])
generators:
  n: { gen: int, min: 0, max: 31, type: i64 }
  imm: { gen: int, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:461-462
```
