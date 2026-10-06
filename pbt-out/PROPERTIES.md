# Properties: encode_amo

## encode_amo_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler (A-extension). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree AMO decoder). encode_r / encode_amo_suffixed / encode_sc rejected as same-job siblings (private packer / aqrl-suffixed mnemonic / store-conditional). Domain is llvm-mc-valid unsuffixed AMO: amo{swap,add,xor,and,or,min,max,minu,maxu}.{w,d} with GPR rd, rs2 and mem (rs1) / 0(rs1).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_reg_pbt.rs:368 encode_alu_reg_diff_llvm_mc
- Formal: ∀ mn ∈ AMO_MN, rd, rs2, rs1 ∈ GPR. encode_amo([Reg(rd), Reg(rs2), Mem{rs1, 0}], funct3(mn), funct5(mn)) = Word(w) ∧ w = llvm-mc("mn rd, rs2, (rs1)")
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_amo
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1]
  domain: { mn: amo_mnemonic, rd: gpr, rs2: gpr, rs1: gpr }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs2), Mem{rs1, 0}], funct3(mn), funct5(mn))
    rhs: llvm_mc_word(mn + " " + rd + ", " + rs2 + ", (" + rs1 + ")")
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  rd: { gen: string, type: gpr_name }
  rs2: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
evidence: encoder/mod.rs:657-674 amo*.w/d => encode_amo; llvm-mc -triple=riscv64 -mattr=+a -show-encoding
```

## encode_amo_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from RISC-V ISA / README.md:352 / encoder/mod.rs:300 R-type layout and atomics.rs:25 aq=rl=0. Weaker than differential (does not pin funct5 bit assignment against an independent assembler) but catches rd/rs1/rs2/funct3/opcode/aq/rl packing bugs even if llvm-mc is unavailable. Independent unpack, not a copy of encode_r.
- Doc contract: encoder/mod.rs:300 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_alu_reg_pbt.rs:399 encode_alu_reg_r_type_fields
- Formal: ∀ mn ∈ AMO_MN, rd, rs2, rs1 ∈ 0..31. let w = encode_amo([Reg(x(rd)), Reg(x(rs2)), Mem{x(rs1), 0}], funct3(mn), funct5(mn)) in opcode(w)=0b0101111 ∧ rd(w)=rd ∧ rs1(w)=rs1 ∧ rs2(w)=rs2 ∧ funct3(w)=funct3(mn) ∧ funct5(w)=funct5(mn) ∧ aq(w)=0 ∧ rl(w)=0
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_amo
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1]
  domain: { mn: amo_mnemonic, rd: u32(0..31), rs2: u32(0..31), rs1: u32(0..31) }
  relation:
    op: holds
    expr: unpack_amo_ok(sut_word([Reg(x(rd)), Reg(x(rs2)), Mem{x(rs1), 0}], funct3(mn), funct5(mn)), mn, rd, rs2, rs1)
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:300 R-type layout; atomics.rs:25 aq=0, rl=0; RISC-V ISA AMO opcode 0101111
```

## encode_amo_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names (zero/ra/sp/...), xN, and fp (x8) name the same GPR, so encode_amo must emit the same word. Independent of llvm-mc so a packer regression still fails.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_alu_reg_pbt.rs:426 encode_alu_reg_abi_xn_alias
- Formal: ∀ mn ∈ AMO_MN, n, m, k ∈ 0..31. encode_amo([Reg(xN(n)), Reg(xN(m)), Mem{xN(k), 0}], f3, f5) = encode_amo([Reg(ABI(n)), Reg(ABI(m)), Mem{ABI(k), 0}], f3, f5) ∧ (n=8 ⇒ fp-as-rd equal) ∧ (m=8 ⇒ fp-as-rs2 equal) ∧ (k=8 ⇒ fp-as-base equal)
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_amo
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m, k]
  domain: { mn: amo_mnemonic, n: u32(0..31), m: u32(0..31), k: u32(0..31) }
  relation:
    op: eq
    lhs: sut_word([Reg(xN(n)), Reg(xN(m)), Mem{xN(k), 0}], funct3(mn), funct5(mn))
    rhs: sut_word([Reg(ABI(n)), Reg(ABI(m)), Mem{ABI(k), 0}], funct3(mn), funct5(mn))
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
evidence: parser.rs:23 GPR name set; encoder/mod.rs:373 get_reg via reg_num
```

## encode_amo_neg_extra
- Tier: 5
- Rationale: Negative/error contract from llvm-mc (rejects a fourth operand as "invalid operand for instruction"). encode_instruction passes operands through; extra operands must not be silently ignored. Documented error: Err.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_reg_pbt.rs:460 encode_alu_reg_neg_extra
- Formal: ∀ mn ∈ AMO_MN, rd, rs2, rs1 ∈ GPR, extra ∈ Operand. encode_amo([Reg(rd), Reg(rs2), Mem{rs1, 0}, extra], funct3(mn), funct5(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: failing
- Counterexample: encode_amo([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], 0b010, 0b00001) = Ok(Word(0x0800202f))
- Bug report: pbt-out/bug_reports/encode_amo_extra_operand.md

```property
function: encoder.encode_amo
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1, extra]
  domain: { mn: amo_mnemonic, rd: gpr, rs2: gpr, rs1: gpr, extra: Operand }
  relation:
    op: throws
    expr: encode_amo([Reg(rd), Reg(rs2), Mem{rs1, 0}, extra], funct3(mn), funct5(mn))
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  rd: { gen: string, type: gpr_name }
  rs2: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
  extra: { gen: oneof, options: [Imm(0), Imm(-1), Reg(x0), Symbol(foo), Label(L0), Csr(mstatus), FenceArg(iorw)] }
expected_error: String
evidence: llvm-mc rejects amoswap.w a0, a1, (a2), a3 as invalid operand
```

## encode_amo_neg_nonzero_offset
- Tier: 5
- Rationale: Negative/error contract from llvm-mc ("optional integer offset must be 0") and RISC-V ISA AMO (no immediate; address is rs1 only). encode_amo currently binds `_offset` and drops it. Nonzero offset is accepted by the parser as Operand::Mem and must be rejected, not encoded as offset 0. Documented bound 0 is sampled at ±1 and far from 0.
- Doc contract: (none) on encode_amo for offset — `_offset` at atomics.rs:24 is the producing statement, not a domain restriction
- Seed: encode_store_pbt.rs Mem offset handling; llvm-mc error on `amoswap.w a0, a1, 8(a2)`
- Formal: ∀ mn ∈ AMO_MN, rd, rs2, rs1 ∈ GPR, off ∈ ℤ\{0}. encode_amo([Reg(rd), Reg(rs2), Mem{rs1, off}], funct3(mn), funct5(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: failing
- Counterexample: encode_amo([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 1 }], 0b010, 0b00001) = Ok(Word(0x0800202f))
- Bug report: pbt-out/bug_reports/encode_amo_nonzero_offset.md

```property
function: encoder.encode_amo
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1, off]
  domain: { mn: amo_mnemonic, rd: gpr, rs2: gpr, rs1: gpr, off: nonzero_i64 }
  relation:
    op: throws
    expr: encode_amo([Reg(rd), Reg(rs2), Mem{rs1, off}], funct3(mn), funct5(mn))
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  rd: { gen: string, type: gpr_name }
  rs2: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
  off: { gen: int, min: -2147483648, max: 2147483647, type: i64, filter: "off != 0" }
expected_error: String
evidence: llvm-mc "optional integer offset must be 0"; RISC-V ISA AMO address is rs1 with no imm field
```

## encode_amo_neg_arity_fp
- Tier: 5
- Rationale: Negative/error contract: llvm-mc rejects too few operands and FP registers (`ft0` is "invalid operand"). get_reg/get_mem must Err on empty/missing slots and on FP names that are not GPRs.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_alu_reg_pbt.rs:478 encode_alu_reg_neg_arity_fp
- Formal: ∀ mn ∈ AMO_MN, rd, rs2, rs1 ∈ GPR, fp ∈ FP_NAMES. encode_amo([], f3, f5)=Err ∧ encode_amo([Reg(rd)], f3, f5)=Err ∧ encode_amo([Reg(rd), Reg(rs2)], f3, f5)=Err ∧ encode_amo([Reg(fp), Reg(rs2), Mem{rs1, 0}], f3, f5)=Err ∧ encode_amo([Reg(rd), Reg(fp), Mem{rs1, 0}], f3, f5)=Err ∧ encode_amo([Reg(rd), Reg(rs2), Mem{fp, 0}], f3, f5)=Err
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_amo
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1, fp]
  domain: { mn: amo_mnemonic, rd: gpr, rs2: gpr, rs1: gpr, fp: fp_name }
  relation:
    op: holds
    expr: encode_amo([],f3,f5).is_err() && encode_amo([Reg(rd)],f3,f5).is_err() && encode_amo([Reg(rd),Reg(rs2)],f3,f5).is_err() && encode_amo([Reg(fp),Reg(rs2),Mem{rs1,0}],f3,f5).is_err() && encode_amo([Reg(rd),Reg(fp),Mem{rs1,0}],f3,f5).is_err() && encode_amo([Reg(rd),Reg(rs2),Mem{fp,0}],f3,f5).is_err()
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  rd: { gen: string, type: gpr_name }
  rs2: { gen: string, type: gpr_name }
  rs1: { gen: string, type: gpr_name }
  fp: { gen: oneof, options: [ft0, fs0, fa0, fa7, ft11, f0, f31] }
expected_error: String
evidence: llvm-mc rejects too few operands and `amoswap.w ft0, a1, (a2)` / `(ft0)`
```

## encode_amo_neg_non_mem
- Tier: 5
- Rationale: Negative/error contract: slot 2 must be a memory operand `(rs1)`. llvm-mc requires `(` / optional integer offset. Non-Mem kinds (Reg, Imm, Symbol, Label, Csr, FenceArg, MemSymbol, RoundingMode) must Err via get_mem.
- Doc contract: parser.rs:31 "Memory operand: offset(base) e.g., 8(sp) or -16(s0)" — asserted fingerprint b251460e
- Seed: encode_store_pbt.rs memory-operand requirement
- Formal: ∀ mn ∈ AMO_MN, rd, rs2 ∈ GPR, bad ∉ Mem. encode_amo([Reg(rd), Reg(rs2), bad], funct3(mn), funct5(mn)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_amo
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, bad]
  domain: { mn: amo_mnemonic, rd: gpr, rs2: gpr, bad: non_mem_operand }
  relation:
    op: throws
    expr: encode_amo([Reg(rd), Reg(rs2), bad], funct3(mn), funct5(mn))
generators:
  mn: { gen: oneof, options: [amoswap.w, amoadd.w, amoxor.w, amoand.w, amoor.w, amomin.w, amomax.w, amominu.w, amomaxu.w, amoswap.d, amoadd.d, amoxor.d, amoand.d, amoor.d, amomin.d, amomax.d, amominu.d, amomaxu.d] }
  rd: { gen: string, type: gpr_name }
  rs2: { gen: string, type: gpr_name }
  bad: { gen: oneof, options: [Imm(0), Reg(x1), Symbol(foo), Label(L0), Csr(mstatus), FenceArg(iorw), RoundingMode(rne)] }
expected_error: String
evidence: encoder/mod.rs:433 get_mem expected memory operand; llvm-mc requires (rs1)
```
