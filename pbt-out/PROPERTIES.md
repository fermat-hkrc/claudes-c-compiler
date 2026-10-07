# Properties: encode_vmv_v_x

## encode_vmv_v_x_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_vmv_v_x is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree vmv.v.x decoder. encode_vmv_v_v / encode_vmv_v_i / encode_v_arith_vx are same-file siblings with different jobs (OPIVV funct3=000 / OPIVI funct3=011 / 3-operand OPIVX with vs2 in bits[24:20]). Spec ownership: rustdoc plus assembler README claim RVV vmv.v.x encoding; llvm-mc is a trusted pinned tool implementing that ISA. Wrapper encode_instruction passes operands through.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_vmv_v_v_pbt.rs encode_vmv_v_v_diff_llvm_mc; encode_v_arith_vx_pbt.rs encode_v_arith_vx_diff_llvm_mc
- Formal: ∀ vd, rs1 ∈ {0..31}. encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1})]) = llvm-mc("vmv.v.x v{vd}, x{rs1}")
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_x
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, rs1]
  domain: { vd: v0..v31, rs1: x0..x31 }
  relation:
    op: eq
    lhs: encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1})])
    rhs: llvm_mc("vmv.v.x v" + vd + ", x" + rs1)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:162-169; encoder/mod.rs:1005; assembler/README.md:14
```

## encode_vmv_v_x_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 OPIVX vmv.v.x layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug. Documented constants: opcode=1010111, funct3=100, vm=1, vs2=0, funct6=010111.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_vmv_v_v_pbt.rs encode_vmv_v_v_format_fields
- Formal: ∀ vd, rs1 ∈ 0..31. let w = encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1})]) in Word. (w & 0x7f) = 0b1010111 ∧ ((w>>7)&0x1f)=vd ∧ ((w>>12)&0x7)=0b100 ∧ ((w>>15)&0x1f)=rs1 ∧ ((w>>20)&0x1f)=0 ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3f)=0b010111
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_x
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, rs1]
  domain: { vd: 0..31, rs1: 0..31 }
  body: unpack(encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1})])) matches RISC-V V 1.0 vmv.v.x fields
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:162-169
```

## encode_vmv_v_x_field_isolation
- Tier: 4
- Rationale: Algebraic metamorphic: vd and rs1 occupy disjoint bit fields. Changing one field must not alter the other, nor opcode/funct3/vm/vs2/funct6. Required by the standard tier. Catches vd/rs1 mix-ups independently of llvm-mc.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_field_isolation
- Formal: ∀ vd_a, vd_b, rs1_a, rs1_b ∈ 0..31. let wa = encode_vmv_v_x([v{vd_a}, x{rs1_a}]). Changing only vd (resp. rs1) flips only bits [11:7] (resp. [19:15]).
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_x
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, rs1_a, rs1_b]
  domain: { vd_*: 0..31, rs1_*: 0..31 }
  body: changing one of vd/rs1 flips only that field's bits
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs1_a: { gen: int, min: 0, max: 31, type: u32 }
  rs1_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:162-169
```

## encode_vmv_v_x_vd_rs1_swap
- Tier: 4
- Rationale: Algebraic metamorphic: swapping the numeric identities of operand 0 (vd, still a v-reg) and operand 1 (rs1, still a GPR) must swap bits [11:7] with bits [19:15] and preserve opcode, funct3=100, vs2=0, vm=1, funct6. Independent of llvm-mc. Catches operand-order bugs.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_vmv_v_v_pbt.rs encode_vmv_v_v_vd_vs1_swap
- Formal: ∀ vd, rs1 ∈ 0..31. let w = encode_vmv_v_x([v{vd}, x{rs1}]); let wp = encode_vmv_v_x([v{rs1}, x{vd}]). ((w>>7)&0x1f)=vd ∧ ((w>>15)&0x1f)=rs1 ∧ ((wp>>7)&0x1f)=rs1 ∧ ((wp>>15)&0x1f)=vd ∧ (w & !((0x1f<<7)|(0x1f<<15))) = (wp & !((0x1f<<7)|(0x1f<<15)))
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_x
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, rs1]
  domain: { vd: 0..31, rs1: 0..31 }
  body: swapping numeric vd/rs1 swaps bits[11:7] with bits[19:15] and preserves the rest
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:162-169
```

## encode_vmv_v_x_abi_alias
- Tier: 4
- Rationale: Algebraic metamorphic: RISC-V ABI names for integer registers are documented aliases of xN (encoder/mod.rs:242-287). encode_vmv_v_x([v{vd}, ABI[rs1]]) must equal encode_vmv_v_x([v{vd}, x{rs1}]). llvm-mc also accepts ABI names. Domain covers all 32 ABI names including s0/fp alias for x8.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_abi_alias
- Formal: ∀ vd, rs1 ∈ 0..31. encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1})]) = encode_vmv_v_x([Reg(v{vd}), Reg(ABI[rs1])])
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_x
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, rs1]
  domain: { vd: 0..31, rs1: 0..31 }
  relation:
    op: eq
    lhs: encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1})])
    rhs: encode_vmv_v_x([Reg(v{vd}), Reg(ABI[rs1])])
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:242-287; vector.rs:162-169
```

## encode_vmv_v_x_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: too few operands, non-vector vd, and non-GPR rs1 are rejected. SUT get_vreg/get_reg return Err on missing or wrong-class operands. Imm(0..31) as rs1 is excluded from the invalid domain because get_reg documents GCC bare register numbers (encoder/mod.rs:446-447); Imm(-1) and Imm(32) remain invalid.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_neg_arity_bad_regs
- Formal: ∀ ops with |ops|<2. encode_vmv_v_x(ops) is Err. ∀ bad vd ∉ {v0..v31}. encode_vmv_v_x([bad, x0]) is Err. ∀ bad rs1 ∉ GPR names ∪ Imm(0..31). encode_vmv_v_x([v0, bad]) is Err.
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_vmv_v_x
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad_vd, bad_rs1]
  domain: { ops: arity 0..1, bad_vd: non-vreg, bad_rs1: non-GPR }
  relation:
    op: throws
    expr: encode_vmv_v_x(ops_or_bad)
expected_error: String
generators:
  ops: short_ops
  bad_vd: bad_vreg
  bad_rs1: bad_rs1
evidence: llvm-mc rejects too few / invalid operand; get_vreg/get_reg return Err
```

## encode_vmv_v_x_neg_extra
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: extra tokens after a complete `vmv.v.x vd, rs1` are invalid operands. Public wrapper encode_instruction passes operands through (encoder/mod.rs:1005). SUT currently ignores operands past index 1. Domain includes Imm, extra regs, symbols, labels, mem, fence, csr, rounding-mode.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_vmv_v_v_pbt.rs encode_vmv_v_v_neg_extra
- Formal: ∀ vd, rs1 ∈ 0..31. ∀ extra. encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1}), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_x([Reg("v0"), Reg("x0"), Imm(0)]) → Ok(Word(0x5e004057))
- Bug report: pbt-out/bug_reports/encode_vmv_v_x_extra_operand.md

```property
function: encoder.vector.encode_vmv_v_x
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, rs1, extra]
  domain: { vd: 0..31, rs1: 0..31, extra: extra_operand }
  relation:
    op: throws
    expr: encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1}), extra])
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  extra: extra_operand
evidence: llvm-mc "invalid operand for instruction" on extra tokens; encoder/mod.rs:1005 pass-through
```

## encode_vmv_v_x_neg_mask_v0t
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: `vmv.v.x vd, rs1, v0.t` is rejected (vmv.v.x is unmasked-only; RISC-V V 1.0 requires vm=1 and vs2=0). Unlike vadd.vx, there is no masked encoding. Dispatcher TODO (encoder/mod.rs:956) documents missing masked support in general; for this mnemonic the mask is invalid, so the contract is Err, not vm=0.
- Doc contract: vector.rs:162 "vmv.v.x vd, rs1: OPIVX, funct6=010111, vm=1, vs2=0" — asserted fingerprint 4ee490e9
- Seed: encode_vmv_v_v_pbt.rs encode_vmv_v_v_neg_mask_v0t
- Formal: ∀ vd, rs1 ∈ 0..31. encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1}), Symbol("v0.t")]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_x([Reg("v0"), Reg("x0"), Symbol("v0.t")]) → Ok(Word(0x5e004057))
- Bug report: pbt-out/bug_reports/encode_vmv_v_x_mask_v0t.md

```property
function: encoder.vector.encode_vmv_v_x
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, rs1]
  domain: { vd: 0..31, rs1: 0..31 }
  relation:
    op: throws
    expr: encode_vmv_v_x([Reg(v{vd}), Reg(x{rs1}), Symbol("v0.t")])
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects v0.t on vmv.v.x; vector.rs:162 vm=1; RISC-V V 1.0 unmasked-only
```
