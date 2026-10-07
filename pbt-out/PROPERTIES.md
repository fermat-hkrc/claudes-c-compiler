# Properties: encode_v_arith_vv

## encode_v_arith_vv_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_v_arith_vv is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree OPIVV decoder. encode_v_arith_vx / encode_v_arith_vi are same-file siblings with different jobs (OPIVX funct3=100 / OPIVI funct3=011, scalar/immediate vs1). Spec ownership: rustdoc plus assembler README claim RVV OPIVV encoding; llvm-mc is a trusted pinned tool implementing that ISA. Wrapper encode_instruction passes operands through.
- Doc contract: vector.rs:122 "Encode vector arithmetic VV (vector-vector): funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V" — asserted fingerprint 38e970d0
- Seed: encode_vstore_pbt.rs encode_vstore_diff_llvm_mc (same RVV encoder family)
- Formal: ∀ vd, vs2, vs1 ∈ {0..31}, (mnem, funct6) ∈ {(vadd.vv, 0b000000), (vsub.vv, 0b000010), (vand.vv, 0b001001), (vor.vv, 0b001010), (vxor.vv, 0b001011)}. encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1})], funct6) = llvm-mc(mnem v{vd}, v{vs2}, v{vs1})
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vv
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, mnem, funct6]
  domain: { vd: v0..v31, vs2: v0..v31, vs1: v0..v31, (mnem,funct6): opivv_family }
  relation:
    op: eq
    lhs: encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1})], funct6)
    rhs: llvm_mc(mnem + " v" + vd + ", v" + vs2 + ", v" + vs1)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: vector.rs:122-129; encoder/mod.rs:974-987; assembler/README.md:14
```

## encode_v_arith_vv_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 OPIVV layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug. funct6 covers the full 6-bit field 0..63, not only dispatched mnemonics.
- Doc contract: vector.rs:122 "Encode vector arithmetic VV (vector-vector): funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V" — asserted fingerprint 38e970d0
- Seed: encode_vstore_pbt.rs encode_vstore_format_fields
- Formal: ∀ vd, vs2, vs1 ∈ 0..31, funct6 ∈ 0..63. let w = encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1})], funct6) in Word. (w & 0x7f) = 0b1010111 ∧ ((w>>7)&0x1f)=vd ∧ ((w>>12)&0x7)=0 ∧ ((w>>15)&0x1f)=vs1 ∧ ((w>>20)&0x1f)=vs2 ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3f)=funct6
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31, funct6: 0..63 }
  body: unpack(encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1})], funct6)) matches RISC-V V 1.0 OPIVV fields
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:122-129
```

## encode_v_arith_vv_field_isolation
- Tier: 4
- Rationale: Algebraic invariant: vd/vs1/vs2/funct6 occupy disjoint bit fields. Changing one field must not alter the others. Catches vs1/vs2 swap independently of llvm-mc.
- Doc contract: vector.rs:122 "Encode vector arithmetic VV (vector-vector): funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V" — asserted fingerprint 38e970d0
- Seed: encode_vstore_pbt.rs encode_vstore_field_isolation
- Formal: ∀ vd_a, vd_b, vs2_a, vs2_b, vs1_a, vs1_b ∈ 0..31, f6_a, f6_b ∈ 0..63. let wa = encode_v_arith_vv([v{vd_a}, v{vs2_a}, v{vs1_a}], f6_a). Changing only vd (resp. vs2, vs1, funct6) flips only bits [11:7] (resp. [24:20], [19:15], [31:26]).
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs2_a, vs2_b, vs1_a, vs1_b, f6_a, f6_b]
  domain: { vd_*: 0..31, vs2_*: 0..31, vs1_*: 0..31, f6_*: 0..63 }
  body: changing one of vd/vs2/vs1/funct6 flips only that field's bits
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs2_a: { gen: int, min: 0, max: 31, type: u32 }
  vs2_b: { gen: int, min: 0, max: 31, type: u32 }
  vs1_a: { gen: int, min: 0, max: 31, type: u32 }
  vs1_b: { gen: int, min: 0, max: 31, type: u32 }
  f6_a: { gen: int, min: 0, max: 63, type: u32 }
  f6_b: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:122
```

## encode_v_arith_vv_vs2_vs1_swap
- Tier: 4
- Rationale: Algebraic metamorphic from rustdoc operand order vd, vs2, vs1 (RISC-V V assembly, not ALU rd,rs1,rs2). Swapping the second and third operands must swap bits [24:20] and [19:15] and leave every other bit unchanged. Stronger than crash-only; independent of llvm-mc so a mapping bug in the differential cannot hide a vs1/vs2 swap.
- Doc contract: vector.rs:122 "Encode vector arithmetic VV (vector-vector): funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V" — asserted fingerprint 38e970d0
- Seed: (none)
- Formal: ∀ vd, vs2, vs1 ∈ 0..31, funct6 ∈ 0..63. let w = encode_v_arith_vv([v{vd}, v{vs2}, v{vs1}], funct6), w' = encode_v_arith_vv([v{vd}, v{vs1}, v{vs2}], funct6). ((w>>20)&0x1f)=vs2 ∧ ((w>>15)&0x1f)=vs1 ∧ ((w'>>20)&0x1f)=vs1 ∧ ((w'>>15)&0x1f)=vs2 ∧ (w & ~(0x1f<<20) & ~(0x1f<<15)) = (w' & ~(0x1f<<20) & ~(0x1f<<15))
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31, funct6: 0..63 }
  body: swap(operands[1], operands[2]) swaps bits[24:20] with bits[19:15] and preserves all other bits
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:122-127
```

## encode_v_arith_vv_neg_arity_bad_regs
- Tier: 4
- Rationale: Negative/error contract from llvm-mc: too few operands, non-vector registers (GPR/FP/v32), and non-Reg operand kinds at any of the three positions are rejected (`too few operands for instruction` / `invalid operand for instruction`). get_vreg fails on those names. Documented domain is three vector registers.
- Doc contract: vector.rs:122 "Encode vector arithmetic VV (vector-vector): funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V" — asserted fingerprint 38e970d0
- Seed: encode_vstore_pbt.rs encode_vstore_neg_arity_bad_regs
- Formal: ∀ ops with |ops|<3, ∀ bad ∉ {v0..v31} as Operand::Reg or non-Reg, ∀ pos ∈ {0,1,2}, ∀ funct6 ∈ 0..63. encode_v_arith_vv(ops, funct6) is Err ∧ encode_v_arith_vv(three-operand list with bad at pos, funct6) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad, pos, funct6]
  domain: { ops: arity 0..2, bad: non-vreg, pos: 0..2, funct6: 0..63 }
  relation:
    op: holds
    expr: encode_v_arith_vv(short_or_bad, funct6).is_err()
expected_error: String
generators:
  pos: { gen: int, min: 0, max: 2, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: llvm-mc rejects too few / GPR-as-vd / FP-as-vs / v32; get_vreg
```

## encode_v_arith_vv_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a fourth operand unless it is v0.t (`operand must be v0.t` / `expected '.t' suffix`). Dispatcher TODO encoder/mod.rs:947 does not support masked v0.t, so any extra operand after a complete vd, vs2, vs1 triple must Err. Wrapper encode_instruction passes operands through. This function's rustdoc documents the 3-register OPIVV form, not a 4-operand form.
- Doc contract: vector.rs:122 "Encode vector arithmetic VV (vector-vector): funct6[31:26] | vm[25] | vs2[24:20] | vs1[19:15] | funct3[14:12]=000 | vd[11:7] | OP_V" — asserted fingerprint 38e970d0
- Seed: encode_vstore_pbt.rs encode_vstore_neg_extra
- Formal: ∀ vd, vs2, vs1 ∈ 0..31, extra ∈ Operand \ {v0.t mask token}, (mnem, funct6) ∈ opivv_family. encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1}), extra], funct6) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vv([Reg("v0"), Reg("v0"), Reg("v0"), Imm(0)], funct6=0b000000) → Ok(Word(0x02000057))
- Bug report: pbt-out/bug_reports/encode_v_arith_vv_extra_operand.md

```property
function: encoder.vector.encode_v_arith_vv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, extra, funct6]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31, extra: Operand, funct6: opivv_family }
  relation:
    op: holds
    expr: encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1}), extra], funct6).is_err()
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: llvm-mc operand must be v0.t; encoder/mod.rs:947 TODO masked; encoder/mod.rs:974-987 operands passed through
```

## encode_v_arith_vv_mask_v0t
- Tier: 5
- Rationale: Differential against llvm-mc for the documented RISC-V V masked form `mnem vd, vs2, vs1, v0.t` (vm=0) when vd ≠ 0 (llvm-mc rejects vd=v0 overlap with the mask). Dispatcher TODO encoder/mod.rs:947 admits masked variants are not yet supported — known limitation on an input the public wrapper accepts (operands passed through). Stronger state machine / round-trip rejected as for p1.
- Doc contract: encoder/mod.rs:947 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1 (unmasked)." — limitation (caller encode_instruction) fingerprint 99cac70e
- Seed: (none)
- Formal: ∀ vd ∈ {1..31}, vs2, vs1 ∈ {0..31}, (mnem, funct6) ∈ opivv_family. encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1}), Symbol("v0.t")], funct6) = llvm-mc(mnem v{vd}, v{vs2}, v{vs1}, v0.t)
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vv([Reg("v1"), Reg("v0"), Reg("v0"), Symbol("v0.t")], funct6=0b000000) → Ok(Word(0x020000d7)); llvm-mc(vadd.vv v1, v0, v0, v0.t) = 0x000000d7
- Bug report: pbt-out/bug_reports/encode_v_arith_vv_mask_v0t.md

```property
function: encoder.vector.encode_v_arith_vv
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, mnem, funct6]
  domain: { vd: 1..31, vs2: 0..31, vs1: 0..31, (mnem,funct6): opivv_family }
  relation:
    op: eq
    lhs: encode_v_arith_vv([Reg(v{vd}), Reg(v{vs2}), Reg(v{vs1}), Symbol("v0.t")], funct6)
    rhs: llvm_mc(mnem + " v" + vd + ", v" + vs2 + ", v" + vs1 + ", v0.t")
generators:
  vd: { gen: int, min: 1, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: encoder/mod.rs:947 TODO masked; llvm-mc vadd.vv v1, v2, v3, v0.t encoding vm=0; encoder/mod.rs:974 operands passed through
```
