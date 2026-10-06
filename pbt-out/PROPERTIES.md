# Properties: encode_c_lui

## encode_c_lui_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential against llvm-mc (LLVM 15.0.6), the independent RISC-V assembler this tree already uses as the encoding reference. State machine rejected: encode_c_lui is a pure function with no lifecycle. Algebraic round-trip rejected: no C.LUI decoder in-tree. encode_lui / try_compress_rv64 rejected by the same-job gate (32-bit LUI / post-encode compress pass, not the `c.lui` mnemonic encoder). Domain is llvm-mc's accepted C.LUI immediate set [1, 31] ∪ [0xfffe0, 0xfffff] with rd ∉ {x0, x2}.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: (none) — no existing c.lui unit test; KAT vectors taken from llvm-mc
- Formal: ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∈ [1,31] ∪ [1048544,1048575]. encode_c_lui([Reg(rd), Imm(imm)]) = Half(llvm-mc("c.lui rd, imm", -triple=riscv64 -mattr=+c))
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_except_x0_x2, imm: clui_imm20 }
  relation:
    op: eq
    lhs: sut_half([Reg(rd), Imm(imm)])
    rhs: llvm_mc_half("c.lui {rd}, {imm}")
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, type: i64 }
evidence: encoder/mod.rs:915 "c.lui" => encode_c_lui; README.md:13 C (compressed 16-bit); llvm-mc 15.0.6
```

## encode_c_lui_ci_type_fields
- Tier: 4
- Rationale: RISC-V Unprivileged ISA CI-type layout for C.LUI is an exact structural invariant independent of llvm-mc. Weaker than the differential but pins opcode/funct3/rd/nzimm bit placement so a packing off-by-one cannot hide behind a matching reference if the mapping were wrong. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: (none)
- Formal: ∀ rd ∈ {1,3,…,31}, ∀ nzimm ∈ [-32,-1] ∪ [1,31]. let h = encode_c_lui([Reg(xN), Imm(nzimm as i64)]).as_half(). (h & 0b11) = 0b01 ∧ ((h >> 13) & 0b111) = 0b011 ∧ ((h >> 7) & 0x1F) = rd ∧ ((h >> 12) & 1) = nzimm[5] ∧ ((h >> 2) & 0x1F) = nzimm[4:0]
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, nzimm]
  domain: { rd: gpr_except_x0_x2, nzimm: simm6_nonzero }
  relation:
    op: holds
    lhs: ci_lui_fields(sut_half([Reg(xN), Imm(nzimm)]))
    rhs: (op=01, funct3=011, rd, nzimm6)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  nzimm: { gen: int, min: -32, max: 31, type: i32 }
evidence: compress.rs:41 signed 6-bit range; RISC-V ISA C.LUI CI-type
```

## encode_c_lui_abi_xn_alias
- Tier: 4
- Rationale: ABI names (ra, a0, t6, fp/s0, …) and xN must encode the same halfword. Metamorphic under a behavior-preserving rename. Stronger differential already covers xN vs llvm-mc; this pins the alias table.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: encode_lui_pbt.rs encode_lui_abi_xn_alias
- Formal: ∀ n ∈ {1,3,…,31}, ∀ imm ∈ [1,31] ∪ [1048544,1048575]. encode_c_lui([Reg("x"+n), Imm(imm)]) = encode_c_lui([Reg(ABI[n]), Imm(imm)]) ∧ (n=8 ⇒ encode_c_lui([Reg("fp"), Imm(imm)]) = that halfword)
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, imm]
  domain: { n: gpr_except_x0_x2, imm: clui_imm20 }
  relation:
    op: eq
    lhs: sut_half([Reg(xN), Imm(imm)])
    rhs: sut_half([Reg(ABI[n]), Imm(imm)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, type: i64 }
evidence: encoder/mod.rs:210-256 reg_num ABI/xN/fp
```

## encode_c_lui_neg_rd_x0_x2
- Tier: 3
- Rationale: ISA C.LUI forbids rd ∈ {x0, x2}; x2 is C.ADDI16SP and x0 is HINT. SUT returns Err for both. llvm-mc also rejects x2/sp. (llvm-mc encodes x0 as HINT — not the C.LUI instruction this encoder implements; see Design Caveats.) Documented error strings at compressed.rs:8.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: (none)
- Formal: ∀ rd ∈ {x0, zero, x2, sp}, ∀ imm ∈ [1,31]. encode_c_lui([Reg(rd), Imm(imm)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: {x0,zero,x2,sp}, imm: 1..31 }
  relation:
    op: throws
    lhs: encode_c_lui([Reg(rd), Imm(imm)])
    rhs: Err
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, min: 1, max: 31, type: i64 }
expected_error: String
evidence: compressed.rs:8 rd cannot be x0 or x2; compress.rs:30 rd != {x0, x2}
```

## encode_c_lui_neg_nzimm_zero
- Tier: 3
- Rationale: C.LUI nzimm must not be zero (reserved / HINT). SUT and llvm-mc both reject 0. Documented bound; generator pins 0 exactly.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: (none)
- Formal: ∀ rd ∈ GPR\{x0,x2}. encode_c_lui([Reg(rd), Imm(0)]) = Err ∧ llvm-mc("c.lui rd, 0") rejects
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd]
  domain: { rd: gpr_except_x0_x2 }
  relation:
    op: throws
    lhs: encode_c_lui([Reg(rd), Imm(0)])
    rhs: Err
generators:
  rd: { gen: string, type: String }
expected_error: String
evidence: compressed.rs:11 nzimm must not be zero
```

## encode_c_lui_neg_imm_oob
- Tier: 3
- Rationale: llvm-mc rejects immediates outside [1,31] ∪ [0xfffe0,0xfffff]. compress.rs:41 states signed 6-bit -32..31 excluding 0. Documented bound must be sampled at bound±1 (0, 32, 1048543, 1048576, -1 as assembler syntax llvm-mc rejects). SUT must Err rather than truncate.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: (none)
- Formal: ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∉ [-32,-1]∪[1,31]∪[1048544,1048575]. llvm-mc rejects "c.lui rd, imm" ⇒ encode_c_lui([Reg(rd), Imm(imm)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: failing
- Counterexample: encode_c_lui([Reg("x3"), Imm(32)]) -> Ok(Half(0x7181))
- Bug report: bug_reports/encode_c_lui_imm_oob.md

```property
function: encoder.encode_c_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_except_x0_x2, imm: not_in_clui_imm20 }
  relation:
    op: throws
    lhs: encode_c_lui([Reg(rd), Imm(imm)])
    rhs: Err
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc range [0xfffe0, 0xfffff] or [1, 31]; compress.rs:41
```

## encode_c_lui_neg_extra
- Tier: 3
- Rationale: C.LUI is two-operand. llvm-mc rejects a third operand. encode_c_lui currently reads only indices 0 and 1, so extra operands are a documented-by-reference error path.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: encode_lui_pbt.rs encode_lui_neg_extra
- Formal: ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∈ [1,31], ∀ extra. encode_c_lui([Reg(rd), Imm(imm), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: failing
- Counterexample: encode_c_lui([Reg("x3"), Imm(1), Imm(0)]) -> Ok(Half(0x6185))
- Bug report: bug_reports/encode_c_lui_extra_operand.md

```property
function: encoder.encode_c_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, extra]
  domain: { rd: gpr_except_x0_x2, imm: 1..31, extra: Operand }
  relation:
    op: throws
    lhs: encode_c_lui([Reg(rd), Imm(imm), extra])
    rhs: Err
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, min: 1, max: 31, type: i64 }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: compressed.rs:5 two-operand form; llvm-mc invalid operand for extra
```

## encode_c_lui_neg_arity_fp
- Tier: 3
- Rationale: Missing operands must Err (get_reg/get_imm). FP register names are not integer GPRs (reg_num returns None). llvm-mc rejects both.
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: encode_lui_pbt.rs encode_lui_neg_arity / encode_lui_neg_fp
- Formal: ∀ ops with |ops|<2. encode_c_lui(ops)=Err. ∀ fp ∈ FPR, ∀ imm ∈ [1,31]. encode_c_lui([Reg(fp), Imm(imm)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: arity_lt_2_or_fp_rd }
  relation:
    op: throws
    lhs: encode_c_lui(ops)
    rhs: Err
generators:
  ops: { gen: list, type: Vec<Operand> }
expected_error: String
evidence: encoder/mod.rs:411 get_reg; encoder/mod.rs:451 get_imm; parser.rs:22 integer registers
```

## encode_c_lui_signed_vs_uimm20
- Tier: 4
- Rationale: Sweep gap after coverage_gaps (file-level, encode_c_lui NOT LINKED in C++ binaries). Documented mapping: ISA/compress.rs signed 6-bit negatives equal llvm-mc's 20-bit LUI-style immediates [0xfffe0, 0xfffff]. Metamorphic plus differential. Stronger round-trip rejected (no decoder).
- Doc contract: compressed.rs:5 "c.lui rd, nzimm" — asserted fingerprint f583dec3
- Seed: (none)
- Formal: ∀ rd ∈ GPR\{x0,x2}, ∀ nzimm ∈ [-32,-1]. encode_c_lui([Reg(xN), Imm(nzimm)]) = encode_c_lui([Reg(xN), Imm(nzimm as bits20)]) = llvm-mc("c.lui xN, bits20") where bits20 = nzimm & 0xfffff
- Test file: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_lui
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, nzimm]
  domain: { rd: gpr_except_x0_x2, nzimm: -32..-1 }
  relation:
    op: eq
    lhs: sut_half([Reg(xN), Imm(nzimm)])
    rhs: sut_half([Reg(xN), Imm(nzimm & 0xfffff)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  nzimm: { gen: int, min: -32, max: -1, type: i32 }
evidence: compress.rs:41 signed 6-bit; llvm-mc 20-bit form 0xfffe0..0xfffff
```
