# Properties: encode_alu_imm (requested encode_op_imm)

Requested `--func encode_op_imm` is absent from `src/backend/riscv/assembler/encoder/base.rs`. The in-scope OP-IMM encoder is `encode_alu_imm`.

## encode_alu_imm_diff_imm_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree OP-IMM decoder). encode_i / encode_shift_imm / encode_alu_imm_w / C.ADDI rejected as same-job siblings (private packer / shamt / OP-IMM-32 / compressed).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_load_pbt.rs: encode_load_diff_imm_llvm_mc
- Formal: ∀ mn ∈ {addi,slti,sltiu,xori,ori,andi}, rd,rs1 ∈ GPR, imm ∈ [-2048,2047]. encode_alu_imm([Reg(rd),Reg(rs1),Imm(imm)], funct3(mn)) = Word(llvm-mc(mn rd, rs1, imm))
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, imm]
  domain: { mn: {addi,slti,sltiu,xori,ori,andi}, rd: gpr, rs1: gpr, imm: i12 }
  relation:
    op: eq
    lhs: encode_alu_imm([Reg(rd), Reg(rs1), Imm(imm)], funct3(mn))
    rhs: llvm_mc_word(mn + " " + rd + ", " + rs1 + ", " + imm)
generators:
  mn: { gen: oneof, items: [addi, slti, sltiu, xori, ori, andi] }
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: encoder/mod.rs:497-502 addi/slti/sltiu/xori/ori/andi => encode_alu_imm; README.md:300 I-type OP-IMM mnemonics
```

## encode_alu_imm_i_type_fields
- Tier: 4
- Rationale: Algebraic invariant from RISC-V I-type layout. Stronger differential is the sibling property above; this unpacks opcode/funct3/rd/rs1/imm independently of llvm-mc.
- Doc contract: encoder/mod.rs:293 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0
- Seed: encode_load_pbt.rs: encode_load_i_type_fields
- Formal: ∀ rd,rs1 ∈ [0,31], imm ∈ [-2048,2047], f3 ∈ {0,2,3,4,6,7}. let w = encode_alu_imm([Reg(x{rd}),Reg(x{rs1}),Imm(imm)], f3) in Word. unpack_i(w) = (OP_OP_IMM=0b0010011, f3, rd, rs1, imm)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, f3]
  domain: { rd: 0..31, rs1: 0..31, imm: i12, f3: {0,2,3,4,6,7} }
  relation:
    op: eq
    lhs: unpack_i(encode_alu_imm([Reg(x{rd}), Reg(x{rs1}), Imm(imm)], f3))
    rhs: (0b0010011, f3, rd, rs1, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: encoder/mod.rs:293 I-type layout; encoder/mod.rs:342 OP_OP_IMM; README.md:353
```

## encode_alu_imm_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic: ABI names, xN, and fp=s0/x8 encode the same rd/rs1. Stronger differential covers numeric agreement with llvm-mc; this checks alias identity independently.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 9c9bff0f
- Seed: encode_load_pbt.rs: encode_load_abi_xn_alias
- Formal: ∀ n,m ∈ [0,31], imm ∈ [-2048,2047], f3 ∈ OP-IMM funct3. encode_alu_imm([Reg(x{n}),Reg(x{m}),Imm(imm)], f3) = encode_alu_imm([Reg(ABI[n]),Reg(ABI[m]),Imm(imm)], f3) ∧ (n=8 ⇒ fp alias) ∧ (m=8 ⇒ fp alias)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, imm, f3]
  domain: { n: 0..31, m: 0..31, imm: i12, f3: op_imm_funct3 }
  relation:
    op: eq
    lhs: encode_alu_imm([Reg(x{n}), Reg(x{m}), Imm(imm)], f3)
    rhs: encode_alu_imm([Reg(ABI[n]), Reg(ABI[m]), Imm(imm)], f3)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: encoder/mod.rs:159-170 reg_num ABI aliases; fp is s0/x8
```

## encode_alu_imm_reloc_lo
- Tier: 4
- Rationale: Documented I-type lo relocs for ADDI. Word equals mn rd, rs1, 0; RelocType Lo12I/PcrelLo12I/TprelLo12I; symbol extracted; addend 0. Stronger llvm-mc encoding comparison is not available for unresolved fixups (llvm-mc prints 0b bits).
- Doc contract: encoder/mod.rs:69 "R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)" — asserted fingerprint e2b525e8
- Seed: encode_load_pbt.rs: encode_load_reloc_lo
- Formal: ∀ mn, rd, rs1, s. encode_alu_imm([Reg(rd),Reg(rs1),Symbol("%pcrel_lo(s)"|"%lo(s)"|"%tprel_lo(s)")], f3) = WordWithReloc{word: encode_alu_imm([Reg(rd),Reg(rs1),Imm(0)], f3), reloc_type: PcrelLo12I|Lo12I|TprelLo12I, symbol: s, addend: 0}
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, s, f3]
  domain: { rd: gpr, rs1: gpr, s: ident, f3: op_imm_funct3 }
  relation:
    op: eq
    lhs: encode_alu_imm([Reg(rd), Reg(rs1), Symbol("%lo(s)")], f3)
    rhs: WordWithReloc(encode_alu_imm([Reg(rd), Reg(rs1), Imm(0)], f3), Lo12I, s, 0)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: encoder/mod.rs:69 PcrelLo12I for ADDI; encoder/mod.rs:75 Lo12I for ADDI; base.rs:230-244 Symbol reloc remap
```

## encode_alu_imm_neg_imm_oob
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects I-type immediates outside [-2048, 2047]. Documented 12-bit signed field (README.md:353). encode_i masks with 0xFFF so the SUT currently wraps; the contract is rejection.
- Doc contract: encoder/mod.rs:293 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0
- Seed: encode_store_pbt.rs: encode_store_neg_imm_oob
- Formal: ∀ mn, rd, rs1, imm ∉ [-2048,2047]. llvm-mc rejects mn rd, rs1, imm ∧ encode_alu_imm([Reg(rd),Reg(rs1),Imm(imm)], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm([Reg("x0"), Reg("x0"), Imm(2048)], funct3=0) -> Ok(Word(0x80000013))
- Bug report: pbt-out/bug_reports/encode_alu_imm_imm_oob.md

```property
function: encoder.encode_alu_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, f3]
  domain: { rd: gpr, rs1: gpr, imm: oob_i12, f3: op_imm_funct3 }
  relation:
    op: throws
    expr: encode_alu_imm([Reg(rd), Reg(rs1), Imm(imm)], f3)
expected_error: Err
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: README.md:353 12-bit I-type immediate; llvm-mc range [-2048, 2047]
```

## encode_alu_imm_neg_extra
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects extra operands on OP-IMM. SUT has no operands.len()==3 check so extras are ignored; the contract is rejection.
- Doc contract: encoder/mod.rs:496 "Immediate arithmetic (I-type)" — asserted fingerprint 805fbdb0
- Seed: encode_store_pbt.rs: encode_store_neg_extra
- Formal: ∀ mn, rd, rs1, imm ∈ [-2048,2047], extra. encode_alu_imm([Reg(rd),Reg(rs1),Imm(imm), extra], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], funct3=0) -> Ok(Word(0x13))
- Bug report: pbt-out/bug_reports/encode_alu_imm_extra_operand.md

```property
function: encoder.encode_alu_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, extra, f3]
  domain: { rd: gpr, rs1: gpr, imm: i12, extra: Operand, f3: op_imm_funct3 }
  relation:
    op: throws
    expr: encode_alu_imm([Reg(rd), Reg(rs1), Imm(imm), extra], f3)
expected_error: Err
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: llvm-mc rejects extra operands; README.md:300 three-operand I-type OP-IMM
```

## encode_alu_imm_neg_arity_fp
- Tier: 3
- Rationale: Negative/error contract: empty list, missing 3rd operand, FP rd/rs1, and non-Imm/non-Symbol 3rd operand must Err (get_reg / "alu_imm: expected immediate").
- Doc contract: base.rs:247 "alu_imm: expected immediate" — asserted fingerprint 143431cd
- Seed: encode_store_pbt.rs: encode_store_neg_arity_fp
- Formal: ∀ f3, rd, rs1, imm, fp, bad ∈ {Imm-as-reg-slot already covered, FP, Mem, Csr, Fence, RoundingMode, Label, SymbolOffset, MemSymbol}. encode_alu_imm([], f3)=Err ∧ encode_alu_imm([Reg(rd)], f3)=Err ∧ encode_alu_imm([Reg(rd),Reg(rs1)], f3)=Err ∧ encode_alu_imm([Reg(fp),Reg(rs1),Imm(imm)], f3)=Err ∧ encode_alu_imm([Reg(rd),Reg(fp),Imm(imm)], f3)=Err ∧ encode_alu_imm([Reg(rd),Reg(rs1), bad], f3)=Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, imm, fp, bad, f3]
  domain: { rd: gpr, rs1: gpr, imm: i12, fp: fpr, bad: non_imm_non_symbol, f3: op_imm_funct3 }
  relation:
    op: throws
    expr: encode_alu_imm([], f3)
expected_error: Err
generators:
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  fp: { gen: string }
  bad: { gen: string }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: base.rs:247 alu_imm expected immediate; encoder/mod.rs:360-365 get_reg integer-only
```

## encode_alu_imm_neg_hi_modifier
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects %hi/%pcrel_hi/%tprel_hi on OP-IMM (those modifiers belong on LUI/AUIPC). SUT remaps Hi20/PcrelHi20/TprelHi20 to Lo12I twins; the contract is rejection.
- Doc contract: encoder/mod.rs:69 "R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)" — asserted fingerprint e2b525e8
- Seed: encode_store_pbt.rs: encode_store_neg_hi_modifier
- Formal: ∀ mn, rd, rs1, s, hi ∈ {%hi,%pcrel_hi,%tprel_hi}. llvm-mc rejects mn rd, rs1, hi(s) ∧ encode_alu_imm([Reg(rd),Reg(rs1),Symbol("hi(s)")], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm([Reg("x0"), Reg("x0"), Symbol("%hi(foo)")], funct3=0) -> Ok(WordWithReloc { word: 0x13, reloc_type: Lo12I, symbol: "foo", addend: 0 })
- Bug report: pbt-out/bug_reports/encode_alu_imm_hi_modifier.md

```property
function: encoder.encode_alu_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, s, hi, f3]
  domain: { rd: gpr, rs1: gpr, s: ident, hi: hi_modifier, f3: op_imm_funct3 }
  relation:
    op: throws
    expr: encode_alu_imm([Reg(rd), Reg(rs1), Symbol(hi_form(s))], f3)
expected_error: Err
generators:
  rd: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
  hi: { gen: oneof, items: [hi, pcrel_hi, tprel_hi] }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: encoder/mod.rs:69 PcrelLo12I and mod.rs:75 Lo12I for ADDI; llvm-mc rejects hi modifiers on addi
```

## encode_alu_imm_neg_other_modifier
- Tier: 3
- Rationale: Sweep of the documented `other => other` reloc arm (base.rs:236) plus plain-symbol parse_reloc_modifier fallback. llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on OP-IMM; GOT/TLS/tprel_add/plain symbol must Err.
- Doc contract: encoder/mod.rs:69 "R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)" — asserted fingerprint e2b525e8
- Seed: encode_store_pbt.rs: encode_store_neg_other_modifier
- Formal: ∀ mn, rd, rs1, form ∈ {%got_pcrel_hi(s), %tls_ie_pcrel_hi(s), %tls_gd_pcrel_hi(s), %tprel_add(s), s}. llvm-mc rejects mn rd, rs1, form ∧ encode_alu_imm([Reg(rd),Reg(rs1),Symbol(form)], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- Status: failing
- Counterexample: encode_alu_imm([Reg("x0"), Reg("x0"), Symbol("%got_pcrel_hi(foo)")], funct3=0) -> Ok(WordWithReloc { word: 0x13, reloc_type: GotHi20, symbol: "foo", addend: 0 })
- Bug report: pbt-out/bug_reports/encode_alu_imm_other_modifier.md

```property
function: encoder.encode_alu_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, form, f3]
  domain: { rd: gpr, rs1: gpr, form: other_modifier, f3: op_imm_funct3 }
  relation:
    op: throws
    expr: encode_alu_imm([Reg(rd), Reg(rs1), Symbol(form)], f3)
expected_error: Err
generators:
  rd: { gen: string }
  rs1: { gen: string }
  form: { gen: string }
  f3: { gen: oneof, items: [0, 2, 3, 4, 6, 7] }
evidence: encoder/mod.rs:69 PcrelLo12I for ADDI; llvm-mc only allows lo modifiers on addi; base.rs:236 other arm
```
