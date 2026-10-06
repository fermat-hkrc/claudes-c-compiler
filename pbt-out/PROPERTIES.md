# Properties: encode_float_store

## encode_float_store_diff_imm_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (LLVM 15.0.6), the independent RISC-V assembler. SUT-boundary is internal-helper of the in-tree assembler; encode_instruction (mod.rs:718/746) passes operands through for fsw/fsd. Mapping: [Reg(rs2), Mem{base, offset}] <-> `mn rs2, offset(rs1)` with -triple=riscv64 -mattr=+f,+d (RV64GC includes F/D). Stronger rejected: state machine (pure function, no lifecycle). Algebraic round-trip rejected — no in-tree STORE-FP decoder. encode_s / encode_store / C.FSW / encode_float_load rejected as primary differential (same-job gate: private packer / integer store / compressed / loads).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:3 "//! Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint b0c6d4db
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_diff_imm_llvm_mc
- Formal: ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, imm ∈ [-2048, 2047]. encode_float_store([Reg(rs2), Mem{rs1, imm}], funct3(mn)) = Word(w) ∧ w = llvm-mc(mn rs2, imm(rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_store
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, imm]
  domain: { mn: {fsw, fsd}, rs2: FPRegs, rs1: GPRs, imm: i12 }
  relation:
    op: eq
    lhs: encode_float_store([Reg(rs2), Mem{rs1, imm}], funct3(mn))
    rhs: llvm_mc(mn + " " + rs2 + ", " + imm + "(" + rs1 + ")")
generators:
  mn: { gen: oneof, options: ["fsw", "fsd"] }
  rs2: { gen: string, type: String }
  rs1: { gen: string, type: String }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:718 fsw; mod.rs:746 fsd; RISC-V Unprivileged ISA STORE-FP
```

## encode_float_store_s_type_fields
- Tier: 4
- Rationale: RISC-V S-type layout is documented at encoder/mod.rs:325 independently of encode_s. Unpack via ISA field positions (not a copy of encode_s). Weaker than differential (does not check agreement with llvm-mc) but pins opcode=OP_STORE_FP and field placement. Stronger rejected as above.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:325 "/// S-type: imm[11:5] | rs2[24:20] | rs1[19:15] | funct3[14:12] | imm[4:0] | opcode[6:0]" — asserted fingerprint a933be07
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_s_type_fields
- Formal: ∀ rs2 ∈ 0..31, rs1 ∈ 0..31, f3 ∈ {0b010, 0b011}, imm ∈ [-2048, 2047]. let w = encode_float_store([Reg(fN(rs2)), Mem{xN(rs1), imm}], f3) in Word. unpack_s(w) = (OP_STORE_FP, f3, rs1, rs2, imm)
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_store
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs2, rs1, f3, imm]
  domain: { rs2: u5, rs1: u5, f3: {2, 3}, imm: i12 }
  relation:
    op: eq
    lhs: unpack_s(word(encode_float_store([Reg(fN(rs2)), Mem{xN(rs1), imm}], f3)))
    rhs: (OP_STORE_FP, f3, rs1, rs2, imm)
generators:
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, options: [2, 3] }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:325
```

## encode_float_store_abi_fn_alias
- Tier: 4
- Rationale: Metamorphic alias invariance: FP ABI names (ft0/fa0/fs0/...) encode the same rs2 as fN, and GPR ABI names (including fp=s0=x8) encode the same rs1 as xN. Documented by parser.rs register comment. Not a differential (same SUT, two namings). Stronger rejected as above.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "    /// Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint a60fc01d
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31, f3 ∈ {0b010, 0b011}, imm ∈ [-2048, 2047]. encode_float_store([Reg(fN(n)), Mem{xN(m), imm}], f3) = encode_float_store([Reg(FABI(n)), Mem{GABI(m), imm}], f3) ∧ (m=8 ⇒ Mem{fp, imm} agrees)
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_store
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, f3, imm]
  domain: { n: u5, m: u5, f3: {2, 3}, imm: i12 }
  relation:
    op: eq
    lhs: encode_float_store([Reg(fN(n)), Mem{xN(m), imm}], f3)
    rhs: encode_float_store([Reg(FABI(n)), Mem{GABI(m), imm}], f3)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, options: [2, 3] }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/parser.rs:22
```

## encode_float_store_reloc_lo
- Tier: 4
- Rationale: STORE-FP is S-type; llvm-mc emits fixup_riscv_lo12_s / pcrel_lo12_s / tprel_lo12_s for %lo/%pcrel_lo/%tprel_lo on fsw/fsd. encoder/mod.rs:99/105 document R_RISCV_PCREL_LO12_S and R_RISCV_LO12_S for S-type stores. Reloc-form word must equal the offset-0 Mem encoding (imm patched later). Stronger rejected as above. encode_s is the private packer, not an independent differential.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:99 "    /// R_RISCV_PCREL_LO12_S - for SW/SD (low 12 bits of PC-relative, S-type)" — asserted fingerprint e704b07e
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_reloc_lo
- Formal: ∀ rs2 ∈ FPRegs, rs1 ∈ GPRs, f3 ∈ {0b010, 0b011}, s ∈ Idents. encode_float_store([Reg(rs2), MemSymbol{rs1, %pcrel_lo(s)}], f3) = WordWithReloc{word = encode_float_store([Reg(rs2), Mem{rs1, 0}], f3), reloc_type = PcrelLo12S, symbol = s, addend = 0} ∧ likewise %lo → Lo12S ∧ %tprel_lo → TprelLo12S
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: failing
- Counterexample: encode_float_store([Reg("f0"), MemSymbol{base:"x0", symbol:"%pcrel_lo(foo)"}], 0b010) reloc_type = PcrelLo12I (want PcrelLo12S); %lo(foo) → Lo12I (want Lo12S)
- Bug report: bug_reports/encode_float_store_lo_reloc_i_type.md

```property
function: encoder.encode_float_store
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs2, rs1, f3, s]
  domain: { rs2: FPRegs, rs1: GPRs, f3: {2, 3}, s: ident }
  relation:
    op: eq
    lhs: reloc_of(encode_float_store([Reg(rs2), MemSymbol{rs1, lo(s)}], f3))
    rhs: (word0, Lo12S, s, 0)
generators:
  rs2: { gen: string, type: String }
  rs1: { gen: string, type: String }
  f3: { gen: oneof, options: [2, 3] }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:99; llvm-mc fixup_riscv_lo12_s on fsw
```

## encode_float_store_neg_imm_oob
- Tier: 3
- Rationale: llvm-mc rejects STORE-FP immediates outside [-2048, 2047] with "operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier or an integer in the range [-2048, 2047]". The documented domain is the signed 12-bit S-type immediate; values outside that domain must be rejected. encode_s currently masks rather than range-checks; that is the contract, not a generator exclusion.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:325 "/// S-type: imm[11:5] | rs2[24:20] | rs1[19:15] | funct3[14:12] | imm[4:0] | opcode[6:0]" — asserted fingerprint a933be07
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_neg_imm_oob
- Formal: ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, imm ∉ [-2048, 2047]. llvm-mc rejects mn rs2, imm(rs1) ⇒ encode_float_store([Reg(rs2), Mem{rs1, imm}], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: failing
- Counterexample: encode_float_store([Reg("f0"), Mem{base:"x0", offset:2048}], 0b010) = Ok(Word(2147491879))
- Bug report: bug_reports/encode_float_store_imm_oob.md

```property
function: encoder.encode_float_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, imm]
  domain: { mn: {fsw, fsd}, rs2: FPRegs, rs1: GPRs, imm: i64_outside_i12 }
  relation:
    op: throws
    expr: encode_float_store([Reg(rs2), Mem{rs1, imm}], funct3(mn))
generators:
  mn: { gen: oneof, options: ["fsw", "fsd"] }
  rs2: { gen: string, type: String }
  rs1: { gen: string, type: String }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc "integer in the range [-2048, 2047]"; encoder/mod.rs:325 S-type 12-bit imm
```

## encode_float_store_neg_extra
- Tier: 3
- Rationale: llvm-mc rejects a third operand on fsw/fsd ("invalid operand for instruction"). encode_instruction passes the operand list through; extra operands must be Err, not silently ignored.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:57 "float store: expected memory operand" — asserted fingerprint cbc37f1f
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_neg_extra
- Formal: ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, imm ∈ [-2048, 2047], extra ∈ Operand. encode_float_store([Reg(rs2), Mem{rs1, imm}, extra], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: failing
- Counterexample: encode_float_store([Reg("f0"), Mem{base:"x0", offset:0}, Imm(0)], 0b010) = Ok(Word(8231))
- Bug report: bug_reports/encode_float_store_extra_operand.md

```property
function: encoder.encode_float_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, imm, extra]
  domain: { mn: {fsw, fsd}, rs2: FPRegs, rs1: GPRs, imm: i12, extra: Operand }
  relation:
    op: throws
    expr: encode_float_store([Reg(rs2), Mem{rs1, imm}, extra], funct3(mn))
generators:
  mn: { gen: oneof, options: ["fsw", "fsd"] }
  rs2: { gen: string, type: String }
  rs1: { gen: string, type: String }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on fsw fa0, 0(x1), 0
```

## encode_float_store_neg_arity_gpr
- Tier: 3
- Rationale: Empty list and missing memory operand must Err (float.rs:57). GPR as rs2 is not an FP register (get_freg). FP register as base is not a GPR (reg_num). Non-memory second operand is not Mem/MemSymbol. llvm-mc rejects GPR dest and FP base ("invalid operand for instruction").
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:57 "float store: expected memory operand" — asserted fingerprint cbc37f1f
- Seed: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:encode_float_load_neg_arity_gpr
- Formal: ∀ f3 ∈ {0b010, 0b011}, rs2 ∈ FPRegs, gpr ∈ GPRs, fp ∈ FPRegs, imm ∈ [-2048, 2047], bad ∉ {Mem, MemSymbol}. encode_float_store([], f3) is Err ∧ encode_float_store([Reg(rs2)], f3) is Err ∧ encode_float_store([Reg(gpr), Mem{gpr, imm}], f3) is Err ∧ encode_float_store([Reg(rs2), Mem{fp, imm}], f3) is Err ∧ encode_float_store([Reg(rs2), bad], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f3, rs2, gpr, fp, imm, bad]
  domain: { f3: {2, 3}, rs2: FPRegs, gpr: GPRs, fp: FPRegs, imm: i12, bad: non-Mem }
  relation:
    op: throws
    expr: encode_float_store([], f3)
generators:
  f3: { gen: oneof, options: [2, 3] }
  rs2: { gen: string, type: String }
  gpr: { gen: string, type: String }
  fp: { gen: string, type: String }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  bad: { gen: string, type: Operand }
expected_error: String
evidence: float.rs:57; llvm-mc rejects fsw x1, 0(x1) and fsw fa0, 0(fa0)
```

## encode_float_store_neg_hi_modifier
- Tier: 3
- Rationale: llvm-mc accepts only %lo/%pcrel_lo/%tprel_lo on STORE-FP memory operands and rejects %hi/%pcrel_hi/%tprel_hi ("operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier or an integer in the range [-2048, 2047]"). The SUT must Err, not emit a WordWithReloc.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:99 "    /// R_RISCV_PCREL_LO12_S - for SW/SD (low 12 bits of PC-relative, S-type)" — asserted fingerprint e704b07e
- Seed: src/backend/riscv/assembler/encoder/encode_store_pbt.rs:encode_store_neg_hi_modifier
- Formal: ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, s ∈ Idents, hi ∈ {%hi, %pcrel_hi, %tprel_hi}. llvm-mc rejects mn rs2, hi(s)(rs1) ⇒ encode_float_store([Reg(rs2), MemSymbol{rs1, hi(s)}], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs
- Status: failing
- Counterexample: encode_float_store([Reg("f0"), MemSymbol{base:"x0", symbol:"%hi(foo)"}], 0b010) = Ok(WordWithReloc { word: 8231, reloc_type: Lo12S, symbol: "foo", addend: 0 })
- Bug report: bug_reports/encode_float_store_hi_modifier.md

```property
function: encoder.encode_float_store
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rs2, rs1, s, hi]
  domain: { mn: {fsw, fsd}, rs2: FPRegs, rs1: GPRs, s: ident, hi: {hi, pcrel_hi, tprel_hi} }
  relation:
    op: throws
    expr: encode_float_store([Reg(rs2), MemSymbol{rs1, hi(s)}], funct3(mn))
generators:
  mn: { gen: oneof, options: ["fsw", "fsd"] }
  rs2: { gen: string, type: String }
  rs1: { gen: string, type: String }
  s: { gen: string, type: String }
  hi: { gen: oneof, options: ["hi", "pcrel_hi", "tprel_hi"] }
expected_error: String
evidence: llvm-mc rejects fsw fa0, %hi(foo)(x1)
```
