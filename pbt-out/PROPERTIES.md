# Properties: encode_float_load

## encode_float_load_diff_imm_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (LLVM 15.0.6), the independent RISC-V assembler. SUT-boundary is internal-helper of the in-tree assembler; encode_instruction (mod.rs:715/743) passes operands through for flw/fld. Mapping: [Reg(rd), Mem{base, offset}] <-> `mn rd, offset(rs1)` with -triple=riscv64 -mattr=+f,+d (RV64GC includes F/D). Stronger rejected: state machine (pure function, no lifecycle). Algebraic round-trip rejected — no in-tree LOAD-FP decoder. encode_i / encode_load / C.FLW / encode_float_store rejected as primary differential (same-job gate: private packer / integer load / compressed / stores).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:3 "//! Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint b0c6d4db
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_diff_imm_llvm_mc
- Formal: ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, imm ∈ [-2048, 2047]. encode_float_load([Reg(rd), Mem{rs1, imm}], funct3(mn)) = Word(w) ∧ w = llvm-mc(mn rd, imm(rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_load
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, imm]
  domain: { mn: {flw, fld}, rd: FPRegs, rs1: GPRs, imm: i12 }
  relation:
    op: eq
    lhs: encode_float_load([Reg(rd), Mem{rs1, imm}], funct3(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + imm + "(" + rs1 + ")")
generators:
  mn: { gen: oneof, options: ["flw", "fld"] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:715 flw; mod.rs:743 fld; RISC-V Unprivileged ISA LOAD-FP
```

## encode_float_load_i_type_fields
- Tier: 4
- Rationale: RISC-V I-type layout is documented at encoder/mod.rs:317 independently of encode_i. Unpack via ISA field positions (not a copy of encode_i). Weaker than differential (does not check agreement with llvm-mc) but pins opcode=OP_LOAD_FP and field placement. Stronger rejected as above.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:317 "/// I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint a1e0fcc3
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_i_type_fields
- Formal: ∀ rd ∈ 0..31, rs1 ∈ 0..31, f3 ∈ {0b010, 0b011}, imm ∈ [-2048, 2047]. let w = encode_float_load([Reg(fN(rd)), Mem{xN(rs1), imm}], f3) in Word. unpack_i(w) = (OP_LOAD_FP, rd, f3, rs1, imm)
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_load
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, f3, imm]
  domain: { rd: u5, rs1: u5, f3: {2, 3}, imm: i12 }
  relation:
    op: eq
    lhs: unpack_i(word(encode_float_load([Reg(fN(rd)), Mem{xN(rs1), imm}], f3)))
    rhs: (OP_LOAD_FP, rd, f3, rs1, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, options: [2, 3] }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:317
```

## encode_float_load_abi_fn_alias
- Tier: 4
- Rationale: Metamorphic alias invariance: FP ABI names (ft0/fa0/fs0/...) encode the same rd as fN, and GPR ABI names (including fp=s0=x8) encode the same rs1 as xN. Documented by parser.rs register comment. Not a differential (same SUT, two namings). Stronger rejected as above.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "    /// Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint a60fc01d
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31, f3 ∈ {0b010, 0b011}, imm ∈ [-2048, 2047]. encode_float_load([Reg(fN(n)), Mem{xN(m), imm}], f3) = encode_float_load([Reg(FABI(n)), Mem{GABI(m), imm}], f3) ∧ (m=8 ⇒ Mem{fp, imm} agrees)
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_load
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, f3, imm]
  domain: { n: u5, m: u5, f3: {2, 3}, imm: i12 }
  relation:
    op: eq
    lhs: encode_float_load([Reg(fN(n)), Mem{xN(m), imm}], f3)
    rhs: encode_float_load([Reg(FABI(n)), Mem{GABI(m), imm}], f3)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  f3: { gen: oneof, options: [2, 3] }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/parser.rs:22
```

## encode_float_load_reloc_lo
- Tier: 4
- Rationale: llvm-mc accepts only %lo/%pcrel_lo/%tprel_lo on FLW/FLD memory operands and emits an I-type word with imm=0 plus the corresponding lo12 fixup. Reloc-form encodings from llvm-mc contain unresolved fixup bits, so the oracle is algebraic: WordWithReloc.word equals the offset-0 Mem encoding, reloc_type is Lo12I / PcrelLo12I / TprelLo12I, addend=0, symbol is the identifier. Stronger llvm-mc byte differential rejected — unresolved fixups are not a stable 32-bit word.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:95 "    /// R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)" — asserted fingerprint abd2c21c
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_reloc_lo
- Formal: ∀ rd ∈ FPRegs, rs1 ∈ GPRs, s ∈ Idents, f3 ∈ {0b010, 0b011}, mod ∈ {%lo, %pcrel_lo, %tprel_lo}. encode_float_load([Reg(rd), MemSymbol{rs1, mod(s)}], f3) = WordWithReloc{word = encode_float_load([Reg(rd), Mem{rs1, 0}], f3), reloc_type = lo12_of(mod), symbol = s, addend = 0}
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_load
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, s, f3]
  domain: { rd: FPRegs, rs1: GPRs, s: ident, f3: {2, 3} }
  relation:
    op: eq
    lhs: reloc_word(encode_float_load([Reg(rd), MemSymbol{rs1, "%lo("+s+")"}], f3))
    rhs: word(encode_float_load([Reg(rd), Mem{rs1, 0}], f3))
generators:
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  s: { gen: string, type: String }
  f3: { gen: oneof, options: [2, 3] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:95
```

## encode_float_load_neg_imm_oob
- Tier: 4
- Rationale: Negative/error contract from llvm-mc: immediates outside [-2048, 2047] are rejected. Documented I-type imm[11:0] at encoder/mod.rs:317. Bounds 2047/2048/-2048/-2049 pinned in the generator. Not a domain restriction on encode_float_load's Operand::Mem offset (i64) — the API accepts the value; llvm-mc and the ISA require rejection.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:317 "/// I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint a1e0fcc3
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_imm_oob
- Formal: ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, imm ∉ [-2048, 2047]. llvm-mc rejects mn rd, imm(rs1) ∧ encode_float_load([Reg(rd), Mem{rs1, imm}], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: failing
- Counterexample: encode_float_load([Reg("f0"), Mem{base:"x0", offset:2048}], 0b010) = Ok(Word(2147491847))  // 0x80002007, 2048 truncated to -2048
- Bug report: bug_reports/encode_float_load_imm_oob.md

```property
function: encoder.encode_float_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, imm]
  domain: { mn: {flw, fld}, rd: FPRegs, rs1: GPRs, imm: i64_oob_i12 }
  relation:
    op: holds
    expr: encode_float_load([Reg(rd), Mem{rs1, imm}], funct3(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, options: ["flw", "fld"] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  imm: { gen: int, min: 2048, max: 9223372036854775807, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:317 I-type imm[11:0]
```

## encode_float_load_neg_extra
- Tier: 4
- Rationale: llvm-mc rejects a third operand (`flw fa0, 0(x1), x2` → "invalid operand for instruction"). encode_instruction passes the full operand slice through. Extra operands are Operand values the public assembler accepts as input and must reject.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:715 "        \"flw\" => encode_float_load(operands, 0b010)," — asserted fingerprint f7a9159f
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_extra
- Formal: ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, imm ∈ [-2048, 2047], extra ∈ Operands. encode_float_load([Reg(rd), Mem{rs1, imm}, extra], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: failing
- Counterexample: encode_float_load([Reg("f0"), Mem{base:"x0", offset:0}, Imm(0)], 0b010) = Ok(Word(8199))
- Bug report: bug_reports/encode_float_load_extra_operand.md

```property
function: encoder.encode_float_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, imm, extra]
  domain: { mn: {flw, fld}, rd: FPRegs, rs1: GPRs, imm: i12, extra: Operand }
  relation:
    op: holds
    expr: encode_float_load([Reg(rd), Mem{rs1, imm}, extra], funct3(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, options: ["flw", "fld"] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  imm: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string, type: Operand }
evidence: src/backend/riscv/assembler/encoder/mod.rs:715 operands passed through
```

## encode_float_load_neg_arity_gpr
- Tier: 4
- Rationale: llvm-mc rejects empty/missing operands ("too few operands"), GPR dest (`flw x1, 0(x2)` "invalid operand"), FP base (`flw fa0, 0(fa1)` "invalid operand"), and a non-memory 2nd operand. get_freg / reg_num / the `_` arm are the SUT paths.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:29 "        _ => Err(\"float load: expected memory operand\".to_string())," — asserted fingerprint bfb0fc21
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_arity_fp
- Formal: ∀ f3 ∈ {0b010, 0b011}, rd ∈ FPRegs, gpr ∈ GPRs, fp ∈ FPRegs, off ∈ [-2048, 2047], bad ∈ {Imm, Csr, FenceArg, RoundingMode}. encode_float_load([], f3)=Err ∧ encode_float_load([Reg(rd)], f3)=Err ∧ encode_float_load([Reg(gpr), Mem{gpr, off}], f3)=Err ∧ encode_float_load([Reg(rd), Mem{fp, off}], f3)=Err ∧ encode_float_load([Reg(rd), bad], f3)=Err
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_float_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f3, rd, gpr, fp, off, bad]
  domain: { f3: {2, 3}, rd: FPRegs, gpr: GPRs, fp: FPRegs, off: i12, bad: non_mem }
  relation:
    op: holds
    expr: encode_float_load([], f3).is_err()
expected_error: String
generators:
  f3: { gen: oneof, options: [2, 3] }
  rd: { gen: string, type: String }
  gpr: { gen: string, type: String }
  fp: { gen: string, type: String }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
  bad: { gen: string, type: Operand }
evidence: src/backend/riscv/assembler/encoder/float.rs:29
```

## encode_float_load_neg_hi_modifier
- Tier: 4
- Rationale: llvm-mc rejects %hi/%pcrel_hi/%tprel_hi on FLW/FLD. The SUT remaps PcrelHi20→PcrelLo12I and Hi20→Lo12I (float.rs:15-18) instead of rejecting, and does not remap TprelHi20. Documented valid modifiers are the lo12 family only (mod.rs:95).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:95 "    /// R_RISCV_PCREL_LO12_I - for ADDI/LW/LD (low 12 bits of PC-relative, I-type)" — asserted fingerprint abd2c21c
- Seed: src/backend/riscv/assembler/encoder/encode_load_pbt.rs:encode_load_neg_hi_modifier
- Formal: ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, s ∈ Idents, hi ∈ {%hi, %pcrel_hi, %tprel_hi}. llvm-mc rejects mn rd, hi(s)(rs1) ∧ encode_float_load([Reg(rd), MemSymbol{rs1, hi(s)}], funct3(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs
- Status: failing
- Counterexample: encode_float_load([Reg("f0"), MemSymbol{base:"x0", symbol:"%hi(foo)"}], 0b010) = Ok(WordWithReloc { word: 8199, reloc_type: Lo12I, symbol: "foo", addend: 0 })
- Bug report: bug_reports/encode_float_load_hi_modifier.md

```property
function: encoder.encode_float_load
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, s, hi]
  domain: { mn: {flw, fld}, rd: FPRegs, rs1: GPRs, s: ident, hi: hi_mods }
  relation:
    op: holds
    expr: encode_float_load([Reg(rd), MemSymbol{rs1, hi+"("+s+")"}], funct3(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, options: ["flw", "fld"] }
  rd: { gen: string, type: String }
  rs1: { gen: string, type: String }
  s: { gen: string, type: String }
  hi: { gen: oneof, options: ["pct_hi", "pct_pcrel_hi", "pct_tprel_hi"] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:95 PCREL_LO12_I for loads
```
