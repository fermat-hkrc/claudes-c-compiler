# Properties: encode_vstore

## encode_vstore_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_vstore is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree vector-store decoder. encode_vload is a same-file sibling with a different job (LOAD-FP opcode). Spec ownership: rustdoc plus assembler README claim RVV unit-stride store encoding; llvm-mc is a trusted pinned tool implementing that ISA.
- Doc contract: vector.rs:101 "Encode vector unit-stride store: vse{8,16,32,64}.v vs3, (rs1)" — asserted fingerprint 98a5e98c
- Seed: encode_vload_pbt.rs:329 encode_vload_diff_llvm_mc (same unit-stride vector memory family, store opcode)
- Formal: ∀ vs3 ∈ {v0..v31}, rs1 ∈ GPR, (mnem, width, sumop) ∈ {(vse8.v, 0b000, 0), (vse16.v, 0b101, 0), (vse32.v, 0b110, 0), (vse64.v, 0b111, 0), (vsm.v, 0b000, 0x0B)}. encode_vstore([Reg(vs3), Mem{base:rs1, offset:0}], width, sumop) = llvm-mc(mnem vs3, (rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: differential
predicate:
  quantifier: forall
  vars: [vs3, rs1, mnem, width, sumop]
  domain: { vs3: v0..v31, rs1: GPR, (mnem,width,sumop): vse_family }
  relation:
    op: eq
    lhs: encode_vstore([Reg(vs3), Mem{rs1,0}], width, sumop)
    rhs: llvm_mc(mnem + " " + vs3 + ", (" + rs1 + ")")
generators:
  vs3: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: vector.rs:101-102; encoder/mod.rs:962-969; assembler/README.md:14
```

## encode_vstore_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 unit-stride store layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug.
- Doc contract: vector.rs:102 "Format: nf[31:29] | mew[28] | mop[27:26]=00 | vm[25] | sumop[24:20] | rs1[19:15] | width[14:12] | vs3[11:7] | 0100111" — asserted fingerprint 16d031b4
- Seed: encode_vload_pbt.rs:344 encode_vload_format_fields
- Formal: ∀ vs3, rs1 ∈ 0..31, width ∈ 0..7, sumop ∈ 0..31. let w = encode_vstore([Reg(v{vs3}), Mem{x{rs1},0}], width, sumop) in Word. (w & 0x7f) = 0b0100111 ∧ ((w>>7)&0x1f)=vs3 ∧ ((w>>12)&0x7)=width ∧ ((w>>15)&0x1f)=rs1 ∧ ((w>>20)&0x1f)=sumop ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3)=0 ∧ ((w>>28)&1)=0 ∧ (w>>29)=0
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vs3, rs1, width, sumop]
  domain: { vs3: 0..31, rs1: 0..31, width: 0..7, sumop: 0..31 }
  body: unpack(encode_vstore([Reg(v{vs3}), Mem{x{rs1},0}], width, sumop)) matches RISC-V V 1.0 unit-stride store fields
generators:
  vs3: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  sumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:102
```

## encode_vstore_mem_reg_alias
- Tier: 4
- Rationale: Algebraic metamorphic. rustdoc says the second operand is (rs1) and the body treats Mem{offset:0} and Reg as the same rs1. Independent of llvm-mc (which requires parentheses).
- Doc contract: vector.rs:101 "Encode vector unit-stride store: vse{8,16,32,64}.v vs3, (rs1)" — asserted fingerprint 98a5e98c
- Seed: encode_vload_pbt.rs:364 encode_vload_mem_reg_alias
- Formal: ∀ vs3 ∈ 0..31, rs1-name ∈ GPR names, width ∈ 0..7, sumop ∈ 0..31. encode_vstore([Reg(v{vs3}), Mem{rs1,0}], width, sumop) = encode_vstore([Reg(v{vs3}), Reg(rs1)], width, sumop)
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vs3, rs1, width, sumop]
  domain: { vs3: 0..31, rs1: GPR, width: 0..7, sumop: 0..31 }
  relation:
    op: eq
    lhs: encode_vstore([Reg(v{vs3}), Mem{rs1,0}], width, sumop)
    rhs: encode_vstore([Reg(v{vs3}), Reg(rs1)], width, sumop)
generators:
  vs3: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  sumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:101-111
```

## encode_vstore_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic. ABI names (zero/ra/sp/a0/…/fp) and xN encode the same rs1 field via reg_num. Independent of llvm-mc.
- Doc contract: vector.rs:101 "Encode vector unit-stride store: vse{8,16,32,64}.v vs3, (rs1)" — asserted fingerprint 98a5e98c
- Seed: encode_vload_pbt.rs:376 encode_vload_abi_xn_alias
- Formal: ∀ vs3 ∈ 0..31, n ∈ 0..31, width ∈ 0..7, sumop ∈ 0..31. encode_vstore([Reg(v{vs3}), Mem{x{n},0}], width, sumop) = encode_vstore([Reg(v{vs3}), Mem{ABI(n),0}], width, sumop) ∧ (n=8 ⇒ also equals Mem{fp,0})
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vs3, n, width, sumop]
  domain: { vs3: 0..31, n: 0..31, width: 0..7, sumop: 0..31 }
  relation:
    op: eq
    lhs: encode_vstore([Reg(v{vs3}), Mem{x{n},0}], width, sumop)
    rhs: encode_vstore([Reg(v{vs3}), Mem{ABI(n),0}], width, sumop)
generators:
  vs3: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  sumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:232-276; vector.rs:107-111
```

## encode_vstore_field_isolation
- Tier: 4
- Rationale: Algebraic invariant: vs3/rs1/width/sumop occupy disjoint bit fields. Changing one field must not alter the others.
- Doc contract: vector.rs:102 "Format: nf[31:29] | mew[28] | mop[27:26]=00 | vm[25] | sumop[24:20] | rs1[19:15] | width[14:12] | vs3[11:7] | 0100111" — asserted fingerprint 16d031b4
- Seed: encode_vload_pbt.rs:394 encode_vload_field_isolation
- Formal: ∀ vs3_a, vs3_b, rs1_a, rs1_b ∈ 0..31, width_a, width_b ∈ 0..7, sumop_a, sumop_b ∈ 0..31. let wa = encode_vstore(..., vs3_a, rs1_a, width_a, sumop_a). Changing only vs3 (resp. rs1, width, sumop) flips only bits [11:7] (resp. [19:15], [14:12], [24:20]).
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vs3_a, vs3_b, rs1_a, rs1_b, width_a, width_b, sumop_a, sumop_b]
  domain: { vs3_*: 0..31, rs1_*: 0..31, width_*: 0..7, sumop_*: 0..31 }
  body: changing one of vs3/rs1/width/sumop flips only that field's bits
generators:
  vs3_a: { gen: int, min: 0, max: 31, type: u32 }
  vs3_b: { gen: int, min: 0, max: 31, type: u32 }
  rs1_a: { gen: int, min: 0, max: 31, type: u32 }
  rs1_b: { gen: int, min: 0, max: 31, type: u32 }
  width_a: { gen: int, min: 0, max: 7, type: u32 }
  width_b: { gen: int, min: 0, max: 7, type: u32 }
  sumop_a: { gen: int, min: 0, max: 31, type: u32 }
  sumop_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:102
```

## encode_vstore_neg_arity_bad_regs
- Tier: 4
- Rationale: Negative/error contract from llvm-mc: too few operands, non-vector vs3, and non-GPR rs1 are rejected. Documented by llvm-mc errors `too few operands for instruction` / `invalid operand for instruction` and by get_vreg / reg_num failing on those names.
- Doc contract: vector.rs:101 "Encode vector unit-stride store: vse{8,16,32,64}.v vs3, (rs1)" — asserted fingerprint 98a5e98c
- Seed: encode_vload_pbt.rs:441 encode_vload_neg_arity_bad_regs
- Formal: ∀ ops with |ops|<2, ∀ bad_vs3 ∉ {v0..v31}, ∀ bad_base ∉ GPR, ∀ width ∈ 0..7, sumop ∈ 0..31. encode_vstore(ops, width, sumop) is Err ∧ encode_vstore([bad_vs3, Mem{a0,0}], width, sumop) is Err ∧ encode_vstore([Reg(v0), Mem{bad_base,0}], width, sumop) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad_vs3, bad_base, width, sumop]
  domain: { ops: arity 0..1, bad_vs3: non-vreg, bad_base: non-GPR, width: 0..7, sumop: 0..31 }
  relation:
    op: holds
    expr: encode_vstore(short_or_bad, width, sumop).is_err()
expected_error: String
generators:
  width: { gen: int, min: 0, max: 7, type: u32 }
  sumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects too few / GPR-as-vs3 / FP-or-vector-as-rs1; get_vreg / reg_num
```

## encode_vstore_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a third operand unless it is v0.t (`operand must be v0.t`). Dispatcher TODO encoder/mod.rs:947 does not support masked v0.t, so any extra operand after a complete vs3, (rs1) pair must Err. Wrapper encode_instruction passes operands through.
- Doc contract: vector.rs:101 "Encode vector unit-stride store: vse{8,16,32,64}.v vs3, (rs1)" — asserted fingerprint 98a5e98c
- Seed: encode_vload_pbt.rs:470 encode_vload_neg_extra
- Formal: ∀ vs3 ∈ 0..31, rs1 ∈ GPR, extra ∈ Operand, (mnem,width,sumop) ∈ vse_family. encode_vstore([Reg(v{vs3}), Mem{rs1,0}, extra], width, sumop) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: failing
- Counterexample: encode_vstore([Reg("v0"), Mem{base:"x0", offset:0}, Imm(0)], width=0b000, sumop=0) → Ok(Word(0x02000027))
- Bug report: pbt-out/bug_reports/encode_vstore_extra_operand.md

```property
function: encoder.vector.encode_vstore
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vs3, rs1, extra, width, sumop]
  domain: { vs3: 0..31, rs1: GPR, extra: Operand, width: vse_family, sumop: vse_family }
  relation:
    op: holds
    expr: encode_vstore([Reg(v{vs3}), Mem{rs1,0}, extra], width, sumop).is_err()
expected_error: String
generators:
  vs3: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: llvm-mc operand must be v0.t; encoder/mod.rs:947 TODO masked; encoder/mod.rs:962-969 operands passed through
```

## encode_vstore_neg_nonzero_offset
- Tier: 4
- Rationale: Negative/error contract. rustdoc format is unit-stride (rs1) with no offset; the match arm only accepts Mem{offset:0} or Reg. llvm-mc rejects `vse8.v v0, 8(a0)` and non-Mem/non-Reg operand 1.
- Doc contract: vector.rs:101 "Encode vector unit-stride store: vse{8,16,32,64}.v vs3, (rs1)" — asserted fingerprint 98a5e98c
- Seed: encode_vload_pbt.rs:487 encode_vload_neg_nonzero_offset
- Formal: ∀ vs3 ∈ 0..31, rs1 ∈ GPR, off ≠ 0, bad1 ∉ {Mem{offset:0}, Reg}, width ∈ 0..7, sumop ∈ 0..31. encode_vstore([Reg(v{vs3}), Mem{rs1,off}], width, sumop) is Err ∧ encode_vstore([Reg(v{vs3}), bad1], width, sumop) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vstore
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vs3, rs1, off, bad1, width, sumop]
  domain: { vs3: 0..31, rs1: GPR, off: nonzero i64, bad1: non-Mem0-non-Reg, width: 0..7, sumop: 0..31 }
  relation:
    op: holds
    expr: encode_vstore([Reg(v{vs3}), Mem{rs1,off}], width, sumop).is_err() and encode_vstore([Reg(v{vs3}), bad1], width, sumop).is_err()
expected_error: String
generators:
  vs3: { gen: int, min: 0, max: 31, type: u32 }
  width: { gen: int, min: 0, max: 7, type: u32 }
  sumop: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:106-113; llvm-mc invalid operand for instruction on 8(a0)
```
