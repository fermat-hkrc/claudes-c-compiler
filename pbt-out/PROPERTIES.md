# Properties: encode_lr

## encode_lr_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler (A-extension). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree LR decoder). encode_r / encode_lr_suffixed / encode_sc / encode_amo rejected as same-job siblings (private packer / aqrl-suffixed mnemonic / store-conditional / AMO with rs2). Domain is llvm-mc-valid unsuffixed LR: lr.{w,d} with GPR rd and mem (rs1) / 0(rs1).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_amo_pbt.rs:305 encode_amo_diff_llvm_mc
- Formal: ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ GPR. encode_lr([Reg(rd), Mem{rs1, 0}], funct3(mn)) = Word(w) ∧ w = llvm-mc("mn rd, (rs1)")
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lr
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: lr_mnemonic, rd: gpr, rs1: gpr }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Mem{rs1, 0}], funct3(mn))
    rhs: llvm_mc_word(mn + " " + rd + ", (" + rs1 + ")")
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  rd: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
evidence: encoder/mod.rs:655-656 lr.w/d => encode_lr; llvm-mc -triple=riscv64 -mattr=+a -show-encoding
```

## encode_lr_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from RISC-V ISA / README.md:352 / encoder/mod.rs:300 R-type layout and atomics.rs:8-9 (funct5=00010, rs2=0, aq=rl=0). Weaker than differential (does not pin the encoding against an independent assembler) but catches rd/rs1/rs2/funct3/opcode/aq/rl packing bugs even if llvm-mc is unavailable. Independent unpack, not a copy of encode_r.
- Doc contract: atomics.rs:8 "LR: funct7 = 00010 | aq | rl, rs2 = 0" — asserted fingerprint c95b5fa2
- Seed: encode_amo_pbt.rs:320 encode_amo_r_type_fields
- Formal: ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ 0..31. let w = encode_lr([Reg(x(rd)), Mem{x(rs1), 0}], funct3(mn)) in opcode(w)=0b0101111 ∧ rd(w)=rd ∧ rs1(w)=rs1 ∧ rs2(w)=0 ∧ funct3(w)=funct3(mn) ∧ funct5(w)=0b00010 ∧ aq(w)=0 ∧ rl(w)=0
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: lr_mnemonic, rd: u32(0..31), rs1: u32(0..31) }
  relation:
    op: holds
    expr: unpack_lr_ok(sut_word([Reg(x(rd)), Mem{x(rs1), 0}], funct3(mn)), mn, rd, rs1)
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:300 R-type layout; atomics.rs:8-9 rs2=0 aq=rl=0; RISC-V ISA LR opcode 0101111 funct5=00010
```

## encode_lr_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names (zero/ra/sp/...), xN, and fp (x8) name the same GPR, so encode_lr must emit the same word. Independent of llvm-mc so a packer regression still fails.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_amo_pbt.rs:341 encode_amo_abi_xn_alias
- Formal: ∀ mn ∈ {lr.w, lr.d}, n, k ∈ 0..31. encode_lr([Reg(xN(n)), Mem{xN(k), 0}], f3) = encode_lr([Reg(ABI(n)), Mem{ABI(k), 0}], f3) ∧ (n=8 ⇒ fp-as-rd equal) ∧ (k=8 ⇒ fp-as-base equal)
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, k]
  domain: { mn: lr_mnemonic, n: u32(0..31), k: u32(0..31) }
  relation:
    op: eq
    lhs: sut_word([Reg(xN(n)), Mem{xN(k), 0}], funct3(mn))
    rhs: sut_word([Reg(ABI(n)), Mem{ABI(k), 0}], funct3(mn))
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
evidence: parser.rs:22 ABI and xN name the same GPR
```

## encode_lr_neg_extra
- Tier: 5
- Rationale: Negative/error contract. llvm-mc rejects a third operand (`invalid operand for instruction`). encode_instruction passes the operand slice through to encode_lr unchanged (mod.rs:655-656), so extra operands are caller-reachable. Documented assembler contract (README.md:6-7 textual assembly) requires rejection.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_amo_pbt.rs:372 encode_amo_neg_extra
- Formal: ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ GPR, extra ∈ Operand. encode_lr([Reg(rd), Mem{rs1, 0}, extra], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: failing
- Counterexample: encode_lr([Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0b010) -> Ok(Word(0x1000202f))
- Bug report: bug_reports/encode_lr_extra_operand.md

```property
function: encoder.encode_lr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: lr_mnemonic, rd: gpr, rs1: gpr, extra: operand }
  relation:
    op: throws
    expr: encode_lr([Reg(rd), Mem{rs1, 0}, extra], funct3(mn))
    error: String
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  rd: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
  extra: { gen: operand }
expected_error: String
evidence: llvm-mc rejects extra operand on lr.w/d; encoder/mod.rs:655-656 operands passed through
```

## encode_lr_neg_nonzero_offset
- Tier: 5
- Rationale: Negative/error contract. llvm-mc rejects nonzero mem offset (`optional integer offset must be 0`). encode_lr binds `_offset` and discards it. The public wrapper does not sanitize the offset. Documented bound 0 is sampled at ±1 and extremes.
- Doc contract: atomics.rs:8 "LR: funct7 = 00010 | aq | rl, rs2 = 0" — asserted fingerprint c95b5fa2
- Seed: encode_amo_pbt.rs:391 encode_amo_neg_nonzero_offset
- Formal: ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ GPR, off ∈ ℤ\{0}. encode_lr([Reg(rd), Mem{rs1, off}], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: failing
- Counterexample: encode_lr([Reg("x0"), Mem { base: "x0", offset: 1 }], funct3=0b010) -> Ok(Word(0x1000202f))
- Bug report: bug_reports/encode_lr_nonzero_offset.md

```property
function: encoder.encode_lr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, off]
  domain: { mn: lr_mnemonic, rd: gpr, rs1: gpr, off: i64_nonzero }
  relation:
    op: throws
    expr: encode_lr([Reg(rd), Mem{rs1, off}], funct3(mn))
    error: String
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  rd: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
  off: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc "optional integer offset must be 0"; encoder/mod.rs:655-656 operands passed through
```

## encode_lr_neg_arity_fp
- Tier: 5
- Rationale: Negative/error contract. Too few operands fail get_reg/get_mem. FP names are not GPRs (parser.rs:22 integer vs float register classes; llvm-mc `invalid operand for instruction` on fa0).
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_amo_pbt.rs:410 encode_amo_neg_arity_fp
- Formal: ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ GPR, fp ∈ FPR. encode_lr([], f3) is Err ∧ encode_lr([Reg(rd)], f3) is Err ∧ encode_lr([Reg(fp), Mem{rs1, 0}], f3) is Err ∧ encode_lr([Reg(rd), Mem{fp, 0}], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, fp]
  domain: { mn: lr_mnemonic, rd: gpr, rs1: gpr, fp: fpr }
  relation:
    op: holds
    expr: encode_lr([], f3).is_err() && encode_lr([Reg(rd)], f3).is_err() && encode_lr([Reg(fp), Mem{rs1, 0}], f3).is_err() && encode_lr([Reg(rd), Mem{fp, 0}], f3).is_err()
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  rd: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
  fp: { gen: string, type: fpr_name }
expected_error: String
evidence: get_reg/get_mem error on missing/FP; llvm-mc rejects FP on lr.w/d
```

## encode_lr_neg_non_mem
- Tier: 5
- Rationale: Negative/error contract. Slot 1 must be a Mem operand (assembly `(rs1)`). A non-Mem (Imm, Reg, Symbol, Label, Csr, FenceArg, RoundingMode, MemSymbol) must Err, matching llvm-mc's requirement of a memory operand.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_amo_pbt.rs:447 encode_amo_neg_non_mem
- Formal: ∀ mn ∈ {lr.w, lr.d}, rd ∈ GPR, bad ∉ Mem. encode_lr([Reg(rd), bad], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_lr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, bad]
  domain: { mn: lr_mnemonic, rd: gpr, bad: non_mem_operand }
  relation:
    op: throws
    expr: encode_lr([Reg(rd), bad], funct3(mn))
    error: String
generators:
  mn: { gen: oneof, options: [lr.w, lr.d] }
  rd: { gen: string, type: gpr_name }
  bad: { gen: operand }
expected_error: String
evidence: get_mem requires Operand::Mem; llvm-mc requires (rs1)
```
