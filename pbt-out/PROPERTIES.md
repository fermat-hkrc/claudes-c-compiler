# Properties: encode_vmv_v_v

## encode_vmv_v_v_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_vmv_v_v is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree vmv.v.v decoder. encode_vmv_v_x / encode_vmv_v_i / encode_v_arith_vv are same-file siblings with different jobs (OPIVX funct3=100 / OPIVI funct3=011 / 3-operand OPIVV with vs2 in bits[24:20]). Spec ownership: rustdoc plus assembler README claim RVV vmv.v.v encoding; llvm-mc is a trusted pinned tool implementing that ISA. Wrapper encode_instruction passes operands through.
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_diff_llvm_mc (same RVV encoder family)
- Formal: ∀ vd, vs1 ∈ {0..31}. encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1})]) = llvm-mc("vmv.v.v v{vd}, v{vs1}")
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_v
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs1]
  domain: { vd: v0..v31, vs1: v0..v31 }
  relation:
    op: eq
    lhs: encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1})])
    rhs: llvm_mc("vmv.v.v v" + vd + ", v" + vs1)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:153-159; encoder/mod.rs:1002; assembler/README.md:14
```

## encode_vmv_v_v_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 OPIVV vmv.v.v layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug. Documented constants: opcode=1010111, funct3=000, vm=1, vs2=0, funct6=010111.
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_format_fields
- Formal: ∀ vd, vs1 ∈ 0..31. let w = encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1})]) in Word. (w & 0x7f) = 0b1010111 ∧ ((w>>7)&0x1f)=vd ∧ ((w>>12)&0x7)=0 ∧ ((w>>15)&0x1f)=vs1 ∧ ((w>>20)&0x1f)=0 ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3f)=0b010111
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_v
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs1]
  domain: { vd: 0..31, vs1: 0..31 }
  body: unpack(encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1})])) matches RISC-V V 1.0 vmv.v.v fields
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:153-159
```

## encode_vmv_v_v_field_isolation
- Tier: 4
- Rationale: Algebraic metamorphic: vd and vs1 occupy disjoint bit fields. Changing one field must not alter the other, nor opcode/funct3/vm/vs2/funct6. Required by the standard tier. Catches vd/vs1 mix-ups independently of llvm-mc.
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_field_isolation
- Formal: ∀ vd_a, vd_b, vs1_a, vs1_b ∈ 0..31. let wa = encode_vmv_v_v([v{vd_a}, v{vs1_a}]). Changing only vd (resp. vs1) flips only bits [11:7] (resp. [19:15]).
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_v
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs1_a, vs1_b]
  domain: { vd_*: 0..31, vs1_*: 0..31 }
  body: changing one of vd/vs1 flips only that field's bits
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs1_a: { gen: int, min: 0, max: 31, type: u32 }
  vs1_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:153-159
```

## encode_vmv_v_v_vd_vs1_swap
- Tier: 4
- Rationale: Algebraic metamorphic: swapping operand 0 and operand 1 must swap bits [11:7] (vd) with bits [19:15] (vs1) and preserve opcode, funct3, vs2=0, vm=1, funct6. Independent of llvm-mc. Catches operand-order bugs (vs2 vs vs1 confusion).
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_vs2_vs1_swap
- Formal: ∀ vd, vs1 ∈ 0..31. let w = encode_vmv_v_v([v{vd}, v{vs1}]); let wp = encode_vmv_v_v([v{vs1}, v{vd}]). ((w>>7)&0x1f)=vd ∧ ((w>>15)&0x1f)=vs1 ∧ ((wp>>7)&0x1f)=vs1 ∧ ((wp>>15)&0x1f)=vd ∧ (w & !((0x1f<<7)|(0x1f<<15))) = (wp & !((0x1f<<7)|(0x1f<<15)))
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_v
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs1]
  domain: { vd: 0..31, vs1: 0..31 }
  body: swapping operands 0 and 1 swaps bits[11:7] with bits[19:15] and preserves all other bits
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:153-159
```

## encode_vmv_v_v_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: too few operands ("too few operands for instruction") and non-vector registers ("invalid operand") must fail. get_vreg returns Err for missing index and for names vreg_num rejects. Stronger differential does not apply on the error path (llvm-mc produces diagnostics, not a word).
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_neg_arity_bad_regs
- Formal: ∀ ops with |ops|<2 ∨ ops[i] ∉ {Reg(v0)..Reg(v31)} for i∈{0,1}. encode_vmv_v_v(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_v
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad, pos]
  domain: { ops: length 0..1 of vregs, bad: non-vreg Operand, pos: 0..1 }
  relation:
    op: throws
    expr: encode_vmv_v_v(short_or_bad)
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  pos: { gen: int, min: 0, max: 1, type: usize }
evidence: llvm-mc rejects too few operands and non-vector registers; get_vreg encoder/mod.rs:472-479
```

## encode_vmv_v_v_neg_extra
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: a third operand after a complete `vmv.v.v vd, vs1` is "invalid operand for instruction". RISC-V V 1.0 assembly form is two vector registers. The SUT reads only indices 0 and 1 and ignores extras. Wrapper encode_instruction passes operands through, so extra tokens reach this helper.
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_neg_extra
- Formal: ∀ vd, vs1 ∈ 0..31, extra ∈ Operand. encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1}), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_v([Reg("v0"), Reg("v0"), Imm(0)]) → Ok(Word(0x5e000057))
- Bug report: pbt-out/bug_reports/encode_vmv_v_v_extra_operand.md

```property
function: encoder.vector.encode_vmv_v_v
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs1, extra]
  domain: { vd: 0..31, vs1: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1}), extra])
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Imm, Reg, Symbol, Label, Mem, FenceArg, Csr, RoundingMode] }
evidence: llvm-mc rejects extra operand on vmv.v.v; RISC-V V 1.0 two-operand form
```

## encode_vmv_v_v_neg_mask_v0t
- Tier: 3
- Rationale: Negative/error contract from RISC-V V 1.0 and llvm-mc: vmv.v.v is the unmasked move (vm=1, vs2=0). Trailing `v0.t` is rejected by llvm-mc ("invalid operand"). Distinct from vadd.vv where `, v0.t` is a valid masked form. Documented by rustdoc `vm=1, vs2=0` and dispatcher TODO that masked variants are not supported.
- Doc contract: vector.rs:153 "vmv.v.v vd, vs1: OPIVV, funct6=010111, vm=1, vs2=0" — asserted fingerprint 9c7d8e52
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_mask_v0t (inverted: vmv cannot be masked)
- Formal: ∀ vd, vs1 ∈ 0..31. encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1}), Symbol("v0.t")]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_v([Reg("v0"), Reg("v0"), Symbol("v0.t")]) → Ok(Word(0x5e000057))
- Bug report: pbt-out/bug_reports/encode_vmv_v_v_mask_v0t.md

```property
function: encoder.vector.encode_vmv_v_v
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs1]
  domain: { vd: 0..31, vs1: 0..31 }
  relation:
    op: throws
    expr: encode_vmv_v_v([Reg(v{vd}), Reg(v{vs1}), Symbol("v0.t")])
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects vmv.v.v ..., v0.t; RISC-V V 1.0 vmv.v.v requires vm=1; vector.rs:153; encoder/mod.rs:955
```
