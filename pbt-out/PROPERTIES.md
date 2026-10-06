# Properties: encode_vload

## encode_vload_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_vload is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree vector-load decoder. encode_vstore is a same-file sibling with a different job (STORE-FP opcode). Spec ownership: rustdoc plus assembler README claim RVV unit-stride load encoding; llvm-mc is a trusted pinned tool implementing that ISA.
- Doc contract: vector.rs:76 "Encode vector unit-stride load: vle{8,16,32,64}.v vd, (rs1)" — asserted fingerprint 5e2215e5
- Seed: (none) — no project-owned unit test of encode_vload; KAT gate uses llvm-mc encodings
- Formal: ∀ vd ∈ {v0..v31}, rs1 ∈ GPR, (mnem, width, lumop) ∈ {(vle8.v, 0b000, 0), (vle16.v, 0b101, 0), (vle32.v, 0b110, 0), (vle64.v, 0b111, 0), (vlm.v, 0b000, 0x0B)}. encode_vload([Reg(vd), Mem{base:rs1, offset:0}], width, lumop) = llvm-mc(mnem vd, (rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, rs1, mnem, width, lumop]
  domain: { vd: v0..v31, rs1: GPR, (mnem,width,lumop): vle_family }
  relation:
    op: eq
    lhs: encode_vload([Reg(vd), Mem{rs1,0}], width, lumop)
    rhs: llvm_mc(mnem + " " + vd + ", (" + rs1 + ")")
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: vector.rs:76-79; encoder/mod.rs:954-966; assembler/README.md:14
```

## encode_vload_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 unit-stride load layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug.
- Doc contract: vector.rs:77 "Format: nf[31:29] | mew[28] | mop[27:26]=00 | vm[25] | lumop[24:20] | rs1[19:15] | width[14:12] | vd[11:7] | 0000111" — asserted fingerprint 2a0061c8
- Seed: (none)
- Formal: ∀ vd, rs1 ∈ 0..31, width ∈ 0..7, lumop ∈ 0..31. let w = encode_vload([Reg(v{vd}), Mem{x{rs1},0}], width, lumop) in Word. (w & 0x7f) = 0b0000111 ∧ ((w>>7)&0x1f)=vd ∧ ((w>>12)&0x7)=width ∧ ((w>>15)&0x1f)=rs1 ∧ ((w>>20)&0x1f)=lumop ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3)=0 ∧ ((w>>28)&1)=0 ∧ (w>>29)=0
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, rs1, width, lumop]
  domain: { vd: 0..31, rs1: 0..31, width: 0..7, lumop: 0..31 }
  body: unpack(encode_vload([Reg(v{vd}), Mem{x{rs1},0}], width, lumop)) matches RISC-V V 1.0 unit-stride load fields
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  lumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:77-79
```

## encode_vload_mem_reg_alias
- Tier: 4
- Rationale: Algebraic metamorphic. rustdoc says the second operand is (rs1) and the body treats Mem{offset:0} and Reg as the same rs1. Independent of llvm-mc (which requires parentheses).
- Doc contract: vector.rs:83 "The second operand should be a memory operand (rs1) like (a1)" — asserted fingerprint b5d3b1c7
- Seed: (none)
- Formal: ∀ vd ∈ 0..31, rs1-name ∈ GPR names, width ∈ 0..7, lumop ∈ 0..31. encode_vload([Reg(v{vd}), Mem{rs1,0}], width, lumop) = encode_vload([Reg(v{vd}), Reg(rs1)], width, lumop)
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, rs1, width, lumop]
  domain: { vd: 0..31, rs1: GPR, width: 0..7, lumop: 0..31 }
  relation:
    op: eq
    lhs: encode_vload([Reg(v{vd}), Mem{rs1,0}], width, lumop)
    rhs: encode_vload([Reg(v{vd}), Reg(rs1)], width, lumop)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  lumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:83-91
```

## encode_vload_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic. RISC-V ABI names (zero/ra/sp/…/fp) and xN encode the same GPR number; rustdoc (rs1) and reg_num treat them as aliases. fp = s0 = x8.
- Doc contract: vector.rs:76 "vle{8,16,32,64}.v vd, (rs1)" — asserted fingerprint 5e2215e5
- Seed: (none)
- Formal: ∀ vd ∈ 0..31, n ∈ 0..31, width ∈ 0..7, lumop ∈ 0..31. encode_vload([Reg(v{vd}), Mem{x{n},0}], width, lumop) = encode_vload([Reg(v{vd}), Mem{ABI[n],0}], width, lumop) ∧ (n=8 ⇒ same with base fp)
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, n, width, lumop]
  domain: { vd: 0..31, n: 0..31, width: 0..7, lumop: 0..31 }
  relation:
    op: eq
    lhs: encode_vload([Reg(v{vd}), Mem{x{n},0}], width, lumop)
    rhs: encode_vload([Reg(v{vd}), Mem{ABI[n],0}], width, lumop)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  lumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:230-275; vector.rs:84-91
```

## encode_vload_field_isolation
- Tier: 4
- Rationale: Algebraic metamorphic. Changing one of vd/rs1/width/lumop must only change that field's bits; nf/mew/mop/vm/opcode stay fixed. Catches field bleed from unmasked shifts.
- Doc contract: vector.rs:77 "Format: nf[31:29] | mew[28] | mop[27:26]=00 | vm[25] | lumop[24:20] | rs1[19:15] | width[14:12] | vd[11:7] | 0000111" — asserted fingerprint 2a0061c8
- Seed: (none)
- Formal: ∀ vd_a, vd_b, rs1_a, rs1_b ∈ 0..31, width_a, width_b ∈ 0..7, lumop_a, lumop_b ∈ 0..31. words that differ in one parameter agree on all bits outside that field
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, rs1_a, rs1_b, width_a, width_b, lumop_a, lumop_b]
  domain: { vd: 0..31, rs1: 0..31, width: 0..7, lumop: 0..31 }
  body: changing one parameter only mutates that field mask
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs1_a: { gen: int, min: 0, max: 31, type: u32 }
  rs1_b: { gen: int, min: 0, max: 31, type: u32 }
  width_a: { gen: int, min: 0, max: 7, type: u32 }
  width_b: { gen: int, min: 0, max: 7, type: u32 }
  lumop_a: { gen: int, min: 0, max: 31, type: u32 }
  lumop_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:77-79
```

## encode_vload_neg_arity_bad_regs
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects too-few operands, GPR/FP as vd, and vector/FP as rs1. rustdoc syntax is `vd, (rs1)` with vd a vector register (get_vreg) and rs1 a GPR (reg_num).
- Doc contract: vector.rs:76 "Encode vector unit-stride load: vle{8,16,32,64}.v vd, (rs1)" — asserted fingerprint 5e2215e5
- Seed: (none)
- Formal: ∀ short operand lists of length < 2, ∀ bad_vd ∉ {v0..v31}, ∀ bad_rs1 ∉ GPR. encode_vload(short, width, lumop) is Err ∧ encode_vload([Reg(bad_vd), Mem{a0,0}], …) is Err ∧ encode_vload([Reg(v0), Mem{bad_rs1,0}], …) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad_vd, bad_rs1, width, lumop]
  domain: { ops: short_arity, bad_vd: not_vreg, bad_rs1: not_gpr }
  relation:
    op: throws
    expr: encode_vload(ops, width, lumop)
    error: String
generators:
  width: { gen: int, min: 0, max: 7, type: u32 }
  lumop: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: vector.rs:82-92; llvm-mc invalid operand / too few operands
```

## encode_vload_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a third operand that is not the mask `v0.t` (error: expected '.t' suffix / invalid operand). rustdoc syntax is two operands `vd, (rs1)`. encode_instruction passes operands through unchanged, so extra operands are caller-reachable. The function currently ignores operands past index 1.
- Doc contract: vector.rs:76 "Encode vector unit-stride load: vle{8,16,32,64}.v vd, (rs1)" — asserted fingerprint 5e2215e5
- Seed: encode_vsetvl_pbt.rs extra-operand property (same assembler extra-operand contract)
- Formal: ∀ vd ∈ {v0..v31}, rs1 ∈ GPR, extra ∉ {mask v0.t}. encode_vload([Reg(vd), Mem{rs1,0}, extra], width, lumop) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: failing
- Counterexample: encode_vload([Reg("v0"), Mem{base:"x0", offset:0}, Imm(0)], width=0b000, lumop=0)
- Bug report: pbt-out/bug_reports/encode_vload_extra_operand.md

```property
function: encoder.vector.encode_vload
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, rs1, extra, width, lumop]
  domain: { vd: vreg, rs1: gpr, extra: non_mask_operand }
  relation:
    op: throws
    expr: encode_vload([Reg(vd), Mem{rs1,0}, extra], width, lumop)
    error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: [Imm, Reg, Symbol, Label, Mem, Csr, FenceArg, RoundingMode] }
expected_error: String
evidence: vector.rs:76; llvm-mc extra operand rejection; encoder/mod.rs:954 operands passed through
```

## encode_vload_neg_nonzero_offset
- Tier: 4
- Rationale: Negative/error contract. rustdoc and body require Mem offset 0 (`(rs1)`); llvm-mc rejects `8(a0)` and `0(a0)` textual forms and MemSymbol. Non-zero offset, MemSymbol, and non-Mem/non-Reg operand 1 must Err.
- Doc contract: vector.rs:83 "The second operand should be a memory operand (rs1) like (a1)" — asserted fingerprint b5d3b1c7
- Seed: (none)
- Formal: ∀ vd ∈ {v0..v31}, rs1 ∈ GPR, off ∈ ℤ\{0}. encode_vload([Reg(vd), Mem{rs1,off}], width, lumop) is Err ∧ encode_vload([Reg(vd), MemSymbol{…}], …) is Err ∧ encode_vload([Reg(vd), non-Mem-non-Reg], …) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vload
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, rs1, off, bad1, width, lumop]
  domain: { vd: vreg, rs1: gpr, off: nonzero_i64, bad1: non_mem_non_reg }
  relation:
    op: throws
    expr: encode_vload([Reg(vd), Mem{rs1,off}], width, lumop)
    error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -4096, max: 4096, type: i64 }
expected_error: String
evidence: vector.rs:83-92; llvm-mc invalid operand for 8(a0)
```
