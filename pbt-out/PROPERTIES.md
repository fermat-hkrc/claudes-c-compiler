# Properties: encode_csri

## encode_csri_diff_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler of the same I-type SYSTEM CSR-immediate encoding the SUT claims (encoder/mod.rs:3, README.md:311-312). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree SYSTEM/CSR decoder). encode_i / encode_csr / encode_csrw rejected as primary differential (same-job gate: private packer / register-form sibling / pseudo with rd=x0).
- Doc contract: (none) — encode_csri has no function-level rustdoc. encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290. encoder/mod.rs:706 `"csrrwi" => encode_csri(operands, 0b101)` — asserted fingerprint a994f39a. README.md:311-312 "csrr/csrw/csrs/csrc and their immediate variants (csrwi, csrsi, csrci)." — asserted fingerprint 2f3e3590.
- Seed: encode_csr_pbt.rs:encode_csr_diff_llvm_mc
- Formal: ∀ mn ∈ {csrrwi,csrrsi,csrrci}, rd ∈ GPR, csr ∈ KNOWN_CSR, zimm ∈ 0..=31. encode_csri([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn)) = llvm-mc("mn rd, csr, zimm")
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csri
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, csr, zimm]
  domain: { mn: {csrrwi,csrrsi,csrrci}, rd: gpr, csr: known_csr, zimm: 0..=31 }
  relation:
    op: eq
    lhs: encode_csri([Reg(rd), Csr(csr), Imm(zimm)], funct3(mn))
    rhs: llvm_mc("mn rd, csr, zimm")
generators:
  mn: { gen: oneof, items: ["csrrwi", "csrrsi", "csrrci"] }
  rd: { gen: string }
  csr: { gen: string }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:3 encoder/mod.rs:706-708 README.md:311-312
```

## encode_csri_i_type_fields
- Tier: 3
- Rationale: Algebraic invariant from the documented I-type layout (encoder/mod.rs:313, README.md:353) and RISC-V SYSTEM CSR-immediate encoding: opcode=OP_SYSTEM, funct3 as given, rd in bits[11:7], zimm in rs1 bits[19:15], csr in imm[11:0]. Stronger differential is the sibling property; this unpacks the word independently of llvm-mc.
- Doc contract: encoder/mod.rs:313 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0.
- Seed: encode_csr_pbt.rs:encode_csr_i_type_fields
- Formal: ∀ rd ∈ 0..=31, csr ∈ 0..=4095, zimm ∈ 0..=31, f3 ∈ {0b101,0b110,0b111}. let w = encode_csri([Reg(x{rd}), Imm(csr), Imm(zimm)], f3) in unpack_i(w) = (OP_SYSTEM, f3, rd, zimm, csr)
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csri
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, csr, zimm, f3]
  domain: { rd: 0..=31, csr: 0..=4095, zimm: 0..=31, f3: {0b101,0b110,0b111} }
  relation:
    op: eq
    lhs: unpack_i(encode_csri([Reg(x{rd}), Imm(csr), Imm(zimm)], f3))
    rhs: (OP_SYSTEM, f3, rd, zimm, csr)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  csr: { gen: int, min: 0, max: 4095, type: u32 }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, items: [5, 6, 7] }
evidence: encoder/mod.rs:313 README.md:353
```

## encode_csri_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names, xN, and fp=s0/x8 encode the same rd (parser.rs:22-23 Register: x0-x31, zero, ra, ...; get_reg accepts Imm 0..=31 as GCC bare register numbers — caller helper, not encode_csri's own contract). Stronger differential covers the xN/ABI surface via llvm-mc; this checks alias equality without the reference.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1. encoder/mod.rs:386 "GCC sometimes emits bare register numbers (0-31) in inline asm" — other fingerprint f1b1a1fb.
- Seed: encode_csr_pbt.rs:encode_csr_abi_xn_alias
- Formal: ∀ n ∈ 0..=31, csr ∈ KNOWN_CSR, zimm ∈ 0..=31, f3 ∈ {0b101,0b110,0b111}. encode_csri([Reg(x{n}), Csr(csr), Imm(zimm)], f3) = encode_csri([Reg(ABI[n]), Csr(csr), Imm(zimm)], f3) ∧ (n=8 ⇒ also equals encode_csri([Reg("fp"), ...], f3)) ∧ encode_csri([Imm(n), Csr(csr), Imm(zimm)], f3) = encode_csri([Reg(x{n}), ...], f3)
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csri
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, csr, zimm, f3]
  domain: { n: 0..=31, csr: known_csr, zimm: 0..=31, f3: {0b101,0b110,0b111} }
  relation:
    op: eq
    lhs: encode_csri([Reg(x{n}), Csr(csr), Imm(zimm)], f3)
    rhs: encode_csri([Reg(ABI[n]), Csr(csr), Imm(zimm)], f3)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  csr: { gen: string }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, items: [5, 6, 7] }
evidence: parser.rs:22 encoder/mod.rs:386
```

## encode_csri_name_vs_imm
- Tier: 3
- Rationale: Algebraic metamorphic: CSR as Csr(name), Imm(number), Symbol(name), Csr("0xNNN"), Csr(decimal), and Reg(name) ("sometimes CSR names look like regs", system.rs:69) must encode the same word. get_csr_num accepts all of those kinds.
- Doc contract: parser.rs:40 "CSR register name or number" — asserted fingerprint 37a2a045. system.rs:69 "sometimes CSR names look like regs" — other fingerprint 76a7ce01.
- Seed: encode_csr_pbt.rs:encode_csr_name_vs_imm
- Formal: ∀ rd ∈ GPR, (name,num) ∈ KNOWN_CSR, zimm ∈ 0..=31, f3 ∈ {0b101,0b110,0b111}. encode_csri([Reg(rd), Csr(name), Imm(zimm)], f3) = encode_csri([Reg(rd), Imm(num), Imm(zimm)], f3) = encode_csri([Reg(rd), Symbol(name), Imm(zimm)], f3) = encode_csri([Reg(rd), Csr("0x{num:x}"), Imm(zimm)], f3) = encode_csri([Reg(rd), Csr(decimal(num)), Imm(zimm)], f3) = encode_csri([Reg(rd), Reg(name), Imm(zimm)], f3)
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csri
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, name, num, zimm, f3]
  domain: { rd: gpr, (name,num): known_csr, zimm: 0..=31, f3: {0b101,0b110,0b111} }
  relation:
    op: eq
    lhs: encode_csri([Reg(rd), Csr(name), Imm(zimm)], f3)
    rhs: encode_csri([Reg(rd), Imm(num), Imm(zimm)], f3)
generators:
  rd: { gen: string }
  name: { gen: string }
  num: { gen: int, min: 0, max: 4095, type: u32 }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, items: [5, 6, 7] }
evidence: parser.rs:40 system.rs:69
```

## encode_csri_neg_extra
- Tier: 3
- Rationale: Negative/error contract from llvm-mc ("invalid operand for instruction" on a fourth operand) and the public encode_instruction surface which passes operands through. RISC-V csrrwi/csrrsi/csrrci take exactly three operands. Extra operands must Err, not silently encode.
- Doc contract: (none) — encode_csri has no rustdoc declaring extra operands valid. Contract evidence: inferred (llvm-mc rejects extra operands; encode_instruction passes operands through at encoder/mod.rs:706-708).
- Seed: encode_csr_pbt.rs:encode_csr_neg_extra
- Formal: ∀ mn ∈ {csrrwi,csrrsi,csrrci}, rd ∈ GPR, csr ∈ KNOWN_CSR, zimm ∈ 0..=31, extra ∈ Operand. encode_csri([Reg(rd), Csr(csr), Imm(zimm), extra], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: failing
- Counterexample: encode_csri([Reg("x0"), Csr("fflags"), Imm(0), Imm(0)], funct3=0b101) -> Ok(Word(1069171))
- Bug report: pbt-out/bug_reports/encode_csri_extra_operand.md

```property
function: encoder.encode_csri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, csr, zimm, extra]
  domain: { mn: csrrwi_csrrsi_csrrci, rd: gpr, csr: known_csr, zimm: 0..=31, extra: operand }
  relation:
    op: throws
    expr: encode_csri([Reg(rd), Csr(csr), Imm(zimm), extra], funct3(mn))
expected_error: String
generators:
  mn: { gen: oneof, items: ["csrrwi", "csrrsi", "csrrci"] }
  rd: { gen: string }
  csr: { gen: string }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: string }
evidence: encoder/mod.rs:706-708 llvm-mc extra-operand error
```

## encode_csri_neg_zimm_oob
- Tier: 3
- Rationale: Negative/error contract from llvm-mc ("immediate must be an integer in the range [0, 31]") and RISC-V uimm5 zimm field. encode_csri currently masks `zimm & 0x1F` (system.rs:59-60) instead of rejecting. Bound 0 and 31 are in the valid differential;  -1 and 32 must Err.
- Doc contract: (none). parser.rs:26 "Immediate value: 42, -1, 0x1000" — other fingerprint 37c3d55b (does not declare zimm unbounded). Contract evidence: inferred (RISC-V uimm5; llvm-mc range [0, 31]).
- Seed: encode_csr_pbt.rs:encode_csr_neg_zimm_oob
- Formal: ∀ rd ∈ GPR, csr ∈ KNOWN_CSR, zimm ∈ ℤ \ [0,31], f3 ∈ {0b101,0b110,0b111}. encode_csri([Reg(rd), Csr(csr), Imm(zimm)], f3) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: failing
- Counterexample: encode_csri([Reg("x0"), Csr("fflags"), Imm(-1)], funct3=0b101) -> Ok(Word(2084979))
- Bug report: pbt-out/bug_reports/encode_csri_zimm_oob.md

```property
function: encoder.encode_csri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, csr, zimm, f3]
  domain: { rd: gpr, csr: known_csr, zimm: i64_outside_0_to_31, f3: csri_funct3 }
  relation:
    op: throws
    expr: encode_csri([Reg(rd), Csr(csr), Imm(zimm)], f3)
expected_error: String
generators:
  rd: { gen: string }
  csr: { gen: string }
  zimm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  f3: { gen: oneof, items: [5, 6, 7] }
evidence: RISC-V uimm5 llvm-mc range [0, 31]
```

## encode_csri_neg_csr_oob
- Tier: 3
- Rationale: Negative/error contract from llvm-mc ("immediate must be an integer in the range [0, 4095]") and RISC-V csr[11:0]. encode_csri passes `csr as i32` into encode_i which masks `& 0xFFF`. Bound 0 and 4095 are in the I-type invariant; -1 and 4096 must Err.
- Doc contract: (none). parser.rs:40 "CSR register name or number" — asserted fingerprint 37a2a045 (does not declare csr unbounded). Contract evidence: inferred (RISC-V csr[11:0]; llvm-mc range [0, 4095]).
- Seed: encode_csr_pbt.rs:encode_csr_neg_csr_oob
- Formal: ∀ rd ∈ GPR, csr ∈ ℤ \ [0,4095], zimm ∈ 0..=31, f3 ∈ {0b101,0b110,0b111}. encode_csri([Reg(rd), Imm(csr), Imm(zimm)], f3) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: failing
- Counterexample: encode_csri([Reg("x0"), Imm(-1), Imm(0)], funct3=0b101) -> Ok(Word(4293939315))
- Bug report: pbt-out/bug_reports/encode_csri_csr_oob.md

```property
function: encoder.encode_csri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, csr, zimm, f3]
  domain: { rd: gpr, csr: i64_outside_0_to_4095, zimm: 0..=31, f3: csri_funct3 }
  relation:
    op: throws
    expr: encode_csri([Reg(rd), Imm(csr), Imm(zimm)], f3)
expected_error: String
generators:
  rd: { gen: string }
  csr: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, items: [5, 6, 7] }
evidence: RISC-V csr[11:0] llvm-mc range [0, 4095]
```

## encode_csri_neg_arity_fp_unknown
- Tier: 3
- Rationale: Negative/error contract from llvm-mc ("too few operands" / "invalid operand") and get_reg/get_csr_num/get_imm Err paths. Empty, missing csr/zimm, FP rd, invalid GPR name, unknown CSR name, and non-Imm zimm (e.g. a register) must Err.
- Doc contract: (none). get_reg / get_imm / get_csr_num return Err on missing/wrong-kind operands. llvm-mc rejects the same cases.
- Seed: encode_csr_pbt.rs:encode_csr_neg_arity_fp_unknown
- Formal: ∀ f3 ∈ {0b101,0b110,0b111}, rd ∈ GPR, csr ∈ KNOWN_CSR, zimm ∈ 0..=31, fp ∈ FPR, bad ∈ invalid_gpr, unknown ∈ unknown_csr. encode_csri([], f3) = Err ∧ encode_csri([Reg(rd)], f3) = Err ∧ encode_csri([Reg(rd), Csr(csr)], f3) = Err ∧ encode_csri([Reg(fp), Csr(csr), Imm(zimm)], f3) = Err ∧ encode_csri([Reg(bad), Csr(csr), Imm(zimm)], f3) = Err ∧ encode_csri([Reg(rd), Csr(unknown), Imm(zimm)], f3) = Err ∧ encode_csri([Reg(rd), Csr(csr), Reg(rd)], f3) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_csri_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_csri
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f3, rd, csr, zimm, fp, bad, unknown]
  domain: { f3: csri_funct3, rd: gpr, csr: known_csr, zimm: 0..=31, fp: fpr, bad: invalid_gpr, unknown: unknown_csr }
  relation:
    op: throws
    expr: encode_csri([], f3)
expected_error: String
generators:
  f3: { gen: oneof, items: [5, 6, 7] }
  rd: { gen: string }
  csr: { gen: string }
  zimm: { gen: int, min: 0, max: 31, type: u32 }
  fp: { gen: string }
  bad: { gen: string }
  unknown: { gen: string }
evidence: llvm-mc too-few/invalid-operand get_reg/get_imm/get_csr_num Err
```
