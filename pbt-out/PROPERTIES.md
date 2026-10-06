# Properties: encode_alu_reg (requested encode_op)

Requested `--func encode_op` is absent from base.rs. The in-scope OP/R-type encoder is encode_alu_reg (base.rs:260).

## encode_alu_reg_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree R-type decoder). encode_r / encode_alu_reg_w / C.ADD rejected as primary differential (same-job gate: private packer / OP-32 / compressed).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_imm_pbt.rs:419 llvm-mc differential on sibling OP-IMM
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ rd,rs1,rs2 ∈ GPR. encode_alu_reg([Reg(rd),Reg(rs1),Reg(rs2)], funct3(mn), funct7(mn)) = Word(llvm-mc("-triple=riscv64 -mattr=+m,+zbb", "mn rd, rs1, rs2"))
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: OP_MNEMONICS, rd: GPR, rs1: GPR, rs2: GPR }
  relation:
    op: eq
    lhs: encode_alu_reg([Reg(rd), Reg(rs1), Reg(rs2)], funct3(mn), funct7(mn))
    rhs: Word(llvm_mc("mn rd, rs1, rs2"))
generators:
  mn: { gen: oneof, items: ["add","sub","sll","slt","sltu","xor","srl","sra","or","and","mul","mulh","mulhsu","mulhu","div","divu","rem","remu","andn","orn","xnor","max","maxu","min","minu","rol","ror"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
evidence: src/backend/riscv/assembler/README.md:297 R-type mnemonic list; encoder/mod.rs:512-631 dispatch; llvm-mc 15.0.6
```

## encode_alu_reg_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout. Stronger differential is property 1; this pins field placement independently of llvm-mc.
- Doc contract: encoder/mod.rs:291 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_alu_imm_pbt.rs:434 I-type field unpack
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ rd,rs1,rs2 ∈ 0..31. let w = encode_alu_reg([Reg(x{rd}),Reg(x{rs1}),Reg(x{rs2})], funct3(mn), funct7(mn)). unpack_r(w) = (OP_OP=0b0110011, funct3(mn), rd, rs1, rs2, funct7(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: OP_MNEMONICS, rd: 0..31, rs1: 0..31, rs2: 0..31 }
  relation:
    op: eq
    lhs: unpack_r(encode_alu_reg([Reg(x{rd}),Reg(x{rs1}),Reg(x{rs2})], funct3(mn), funct7(mn)))
    rhs: (0b0110011, funct3(mn), rd, rs1, rs2, funct7(mn))
generators:
  mn: { gen: oneof, items: ["add","sub","sll","slt","sltu","xor","srl","sra","or","and","mul","mulh","mulhsu","mulhu","div","divu","rem","remu","andn","orn","xnor","max","maxu","min","minu","rol","ror"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:291 R-type layout; README.md:352; encoder/mod.rs:345 OP_OP
```

## encode_alu_reg_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic: ABI names, xN, and fp=s0/x8 must encode identically. Independent of llvm-mc.
- Doc contract: encoder/mod.rs:174 "s0" | "fp" => Some(8) — asserted fingerprint (reg_num ABI table)
- Seed: encode_alu_imm_pbt.rs:451 ABI vs xN alias
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ n,m,k ∈ 0..31. encode_alu_reg([Reg(x{n}),Reg(x{m}),Reg(x{k})], f3, f7) = encode_alu_reg([Reg(ABI[n]),Reg(ABI[m]),Reg(ABI[k])], f3, f7) ∧ (n=8 ⇒ fp alias) ∧ (m=8 ⇒ fp alias) ∧ (k=8 ⇒ fp alias)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m, k]
  domain: { mn: OP_MNEMONICS, n: 0..31, m: 0..31, k: 0..31 }
  relation:
    op: eq
    lhs: encode_alu_reg([Reg(x{n}),Reg(x{m}),Reg(x{k})], funct3(mn), funct7(mn))
    rhs: encode_alu_reg([Reg(ABI[n]),Reg(ABI[m]),Reg(ABI[k])], funct3(mn), funct7(mn))
generators:
  mn: { gen: oneof, items: ["add","sub","sll","slt","sltu","xor","srl","sra","or","and","mul","div","andn","max","rol"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:162-202 reg_num ABI and xN and fp=s0
```

## encode_alu_reg_imm_as_reg
- Tier: 4
- Rationale: Algebraic metamorphic from get_reg's documented GCC bare-register-number contract: Imm n in 0..=31 encodes as x{n}. Not a differential against llvm-mc (llvm-mc rejects numeric 3rd operands on sub and treats them as immediates on add).
- Doc contract: encoder/mod.rs:376 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint f1b1a1fb
- Seed: (none)
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ n,m,k ∈ 0..31. encode_alu_reg([Imm(n),Imm(m),Imm(k)], f3, f7) = encode_alu_reg([Reg(x{n}),Reg(x{m}),Reg(x{k})], f3, f7)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m, k]
  domain: { mn: OP_MNEMONICS, n: 0..31, m: 0..31, k: 0..31 }
  relation:
    op: eq
    lhs: encode_alu_reg([Imm(n), Imm(m), Imm(k)], funct3(mn), funct7(mn))
    rhs: encode_alu_reg([Reg(x{n}), Reg(x{m}), Reg(x{k})], funct3(mn), funct7(mn))
generators:
  mn: { gen: oneof, items: ["add","sub","xor","and","mul","andn"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:376-377 get_reg Imm 0-31 as register number
```

## encode_alu_reg_neg_extra
- Tier: 3
- Rationale: Negative/error contract. RISC-V R-type has exactly three register operands; llvm-mc rejects a fourth. encode_alu_reg must Err rather than silently ignore extras.
- Doc contract: README.md:352 "R-type:  [funct7 | rs2 | rs1 | funct3 |  rd  | opcode]" — asserted fingerprint c1a047b4
- Seed: encode_alu_imm_pbt.rs:533 extra-operand rejection
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ rd,rs1,rs2 ∈ GPR, ∀ extra. encode_alu_reg([Reg(rd),Reg(rs1),Reg(rs2), extra], f3, f7) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: failing
- Counterexample: encode_alu_reg([Reg("x0"), Reg("x0"), Reg("x0"), Imm(0)], funct3=0, funct7=0) -> Ok(Word(51))
- Bug report: bug_reports/encode_alu_reg_extra_operand.md

```property
function: encoder.encode_alu_reg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, extra]
  domain: { mn: OP_MNEMONICS, rd: GPR, rs1: GPR, rs2: GPR, extra: Operand }
  relation:
    op: holds
    expr: encode_alu_reg([Reg(rd), Reg(rs1), Reg(rs2), extra], funct3(mn), funct7(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["add","sub","and","mul","andn","rol"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  extra: { gen: oneof }
evidence: README.md:352 three-register R-type; llvm-mc rejects extra operands
```

## encode_alu_reg_neg_arity_fp
- Tier: 3
- Rationale: Negative/error contract. Missing operands, FP registers, and non-register operand kinds must Err (get_reg requires an integer register).
- Doc contract: encoder/mod.rs:381 "expected register at operand {}, got {:?}" — asserted fingerprint 3d1d0841
- Seed: encode_alu_imm_pbt.rs:551 arity/FP rejection
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ rd,rs1,rs2 ∈ GPR, ∀ fp ∈ FPR, ∀ bad ∈ NONREG. encode_alu_reg([], f3, f7)=Err ∧ encode_alu_reg([Reg(rd)], f3, f7)=Err ∧ encode_alu_reg([Reg(rd),Reg(rs1)], f3, f7)=Err ∧ encode_alu_reg([Reg(fp),Reg(rs1),Reg(rs2)], f3, f7)=Err ∧ encode_alu_reg([Reg(rd),Reg(fp),Reg(rs2)], f3, f7)=Err ∧ encode_alu_reg([Reg(rd),Reg(rs1),Reg(fp)], f3, f7)=Err ∧ encode_alu_reg([Reg(rd),Reg(rs1),bad], f3, f7)=Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, fp, bad]
  domain: { mn: OP_MNEMONICS, rd: GPR, rs1: GPR, rs2: GPR, fp: FPR, bad: NONREG }
  relation:
    op: holds
    expr: encode_alu_reg([], funct3(mn), funct7(mn)).is_err() && encode_alu_reg([Reg(rd)], funct3(mn), funct7(mn)).is_err() && encode_alu_reg([Reg(rd), Reg(rs1)], funct3(mn), funct7(mn)).is_err() && encode_alu_reg([Reg(fp), Reg(rs1), Reg(rs2)], funct3(mn), funct7(mn)).is_err() && encode_alu_reg([Reg(rd), Reg(fp), Reg(rs2)], funct3(mn), funct7(mn)).is_err() && encode_alu_reg([Reg(rd), Reg(rs1), Reg(fp)], funct3(mn), funct7(mn)).is_err() && encode_alu_reg([Reg(rd), Reg(rs1), bad], funct3(mn), funct7(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["add","sub","xor","mul"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  fp: { gen: string }
  bad: { gen: oneof }
evidence: encoder/mod.rs:370-381 get_reg; llvm-mc rejects FP and missing operands
```

## encode_alu_reg_neg_oob_imm
- Tier: 3
- Rationale: Negative/error contract. get_reg accepts Imm only in 0..=31; values outside that range (and negative except the documented window) must Err.
- Doc contract: encoder/mod.rs:376 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint f1b1a1fb
- Seed: (none)
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ rd,rs1 ∈ GPR, ∀ imm ∉ 0..31. encode_alu_reg([Reg(rd),Reg(rs1),Imm(imm)], f3, f7) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, imm]
  domain: { mn: OP_MNEMONICS, rd: GPR, rs1: GPR, imm: i64 excluding 0..31 }
  relation:
    op: holds
    expr: encode_alu_reg([Reg(rd), Reg(rs1), Imm(imm)], funct3(mn), funct7(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["add","sub","and","mul"] }
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, type: i64 }
evidence: encoder/mod.rs:376-381 get_reg Imm window 0..=31
```

## encode_alu_reg_neg_invalid_name
- Tier: 3
- Rationale: Negative/error contract sweep of get_reg's Reg arm when reg_num returns None (x32/foo/v0/xzr/w0). Documented error: "invalid integer register". Same-path as FP names; generator pins non-ABI spellings the FP generator does not emit.
- Doc contract: encoder/mod.rs:373 "invalid integer register: {}" — asserted fingerprint 271f3581
- Seed: encode_alu_reg_neg_arity_fp (FP names); this sweep targets non-FP invalid spellings
- Formal: ∀ mn ∈ OP_MNEMONICS, ∀ rd,rs1,rs2 ∈ GPR, ∀ bad ∈ {x32,x33,x99,foo,v0,v31,xzr,w0}, ∀ i ∈ {0,1,2}. encode_alu_reg(ops with ops[i]=Reg(bad), f3, f7) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, bad, which]
  domain: { mn: OP_MNEMONICS, rd: GPR, rs1: GPR, rs2: GPR, bad: INVALID_GPR, which: 0..2 }
  relation:
    op: holds
    expr: encode_alu_reg(ops_with_bad_at(which, bad), funct3(mn), funct7(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["add","sub","and","mul"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  bad: { gen: oneof, items: ["x32","x33","x99","foo","v0","v31","xzr","w0"] }
  which: { gen: int, min: 0, max: 2, type: u32 }
evidence: encoder/mod.rs:373 invalid integer register
```
