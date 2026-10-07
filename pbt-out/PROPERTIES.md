# Properties: encode_v_arith_vx

## encode_v_arith_vx_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_v_arith_vx is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree OPIVX decoder. encode_v_arith_vv / encode_v_arith_vi are same-file siblings with different jobs (OPIVV funct3=000 / OPIVI funct3=011). Spec ownership: rustdoc plus assembler README claim RVV OPIVX encoding; llvm-mc is a trusted pinned tool implementing that ISA. Wrapper encode_instruction passes operands through.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_diff_llvm_mc (same RVV encoder family)
- Formal: ∀ vd, vs2, rs1 ∈ {0..31}, (mnem, funct6) ∈ {(vadd.vx, 0b000000), (vsub.vx, 0b000010), (vand.vx, 0b001001), (vor.vx, 0b001010), (vxor.vx, 0b001011), (vslideup.vx, 0b001110), (vslidedown.vx, 0b001111)} with not (slide mnemonic and vd = vs2). encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1})], funct6) = llvm-mc(mnem v{vd}, v{vs2}, x{rs1}). Slide overlap is skipped because llvm-mc rejects dest overlapping vs2 (architectural, not encoding).
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vx
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs2, rs1, mnem, funct6]
  domain: { vd: v0..v31, vs2: v0..v31, rs1: x0..x31, (mnem,funct6): opivx_family }
  relation:
    op: eq
    lhs: encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1})], funct6)
    rhs: llvm_mc(mnem + " v" + vd + ", v" + vs2 + ", x" + rs1)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u32 }
evidence: vector.rs:133-140; encoder/mod.rs:976-994; assembler/README.md:14
```

## encode_v_arith_vx_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 OPIVX layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug. funct6 covers the full 6-bit field 0..63, not only dispatched mnemonics.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_format_fields
- Formal: ∀ vd, vs2, rs1 ∈ 0..31, funct6 ∈ 0..63. let w = encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1})], funct6) in Word. (w & 0x7f) = 0b1010111 ∧ ((w>>7)&0x1f)=vd ∧ ((w>>12)&0x7)=0b100 ∧ ((w>>15)&0x1f)=rs1 ∧ ((w>>20)&0x1f)=vs2 ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3f)=funct6
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, rs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, rs1: 0..31, funct6: 0..63 }
  body: unpack(encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1})], funct6)) matches RISC-V V 1.0 OPIVX fields
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:133-140
```

## encode_v_arith_vx_field_isolation
- Tier: 4
- Rationale: Algebraic invariant: vd/vs2/rs1/funct6 occupy disjoint bit fields. Changing one field must not alter the others. Catches vs2/rs1 swap independently of llvm-mc.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_field_isolation
- Formal: ∀ vd_a, vd_b, vs2_a, vs2_b, rs1_a, rs1_b ∈ 0..31, f6_a, f6_b ∈ 0..63. let wa = encode_v_arith_vx([v{vd_a}, v{vs2_a}, x{rs1_a}], f6_a). Changing only vd (resp. vs2, rs1, funct6) flips only bits [11:7] (resp. [24:20], [19:15], [31:26]).
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs2_a, vs2_b, rs1_a, rs1_b, f6_a, f6_b]
  domain: { vd_*: 0..31, vs2_*: 0..31, rs1_*: 0..31, f6_*: 0..63 }
  body: changing one of vd/vs2/rs1/funct6 flips only that field's bits
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs2_a: { gen: int, min: 0, max: 31, type: u32 }
  vs2_b: { gen: int, min: 0, max: 31, type: u32 }
  rs1_a: { gen: int, min: 0, max: 31, type: u32 }
  rs1_b: { gen: int, min: 0, max: 31, type: u32 }
  f6_a: { gen: int, min: 0, max: 63, type: u32 }
  f6_b: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:133-140
```

## encode_v_arith_vx_vs2_rs1_swap
- Tier: 4
- Rationale: Algebraic metamorphic: assembly order is vd, vs2, rs1 so operand 1 occupies bits[24:20] and operand 2 occupies bits[19:15]. Swapping those two numbers must swap only those fields. Stronger differential is p1; this pins operand-to-field mapping without llvm-mc.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_vs2_vs1_swap
- Formal: ∀ vd, vs2, rs1 ∈ 0..31, funct6 ∈ 0..63. let w = encode_v_arith_vx([v{vd}, v{vs2}, x{rs1}], funct6), w' = encode_v_arith_vx([v{vd}, v{rs1}, x{vs2}], funct6). ((w>>20)&0x1f)=vs2 ∧ ((w>>15)&0x1f)=rs1 ∧ ((w'>>20)&0x1f)=rs1 ∧ ((w'>>15)&0x1f)=vs2 ∧ (w & ~src_mask) = (w' & ~src_mask) where src_mask = bits[24:20]|bits[19:15].
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs2, rs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, rs1: 0..31, funct6: 0..63 }
  body: swapping operand-1 vs2 with operand-2 rs1 swaps bits[24:20] with bits[19:15] only
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:133-140
```

## encode_v_arith_vx_abi_alias
- Tier: 4
- Rationale: Algebraic metamorphic: RISC-V ABI names (zero/ra/sp/.../t6) are aliases of x0..x31. Encoding rs1 as ABI name must equal encoding rs1 as xN. Differential vs llvm-mc is p1 over xN; this pins the alias mapping that get_reg documents.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_vsetvl_pbt.rs ABI vs xN alias
- Formal: ∀ vd, vs2, rs1 ∈ 0..31, funct6 ∈ 0..63. encode_v_arith_vx([v{vd}, v{vs2}, Reg(ABI[rs1])], funct6) = encode_v_arith_vx([v{vd}, v{vs2}, Reg(x{rs1})], funct6)
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs2, rs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, rs1: 0..31, funct6: 0..63 }
  relation:
    op: eq
    lhs: encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(ABI[rs1])], funct6)
    rhs: encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1})], funct6)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: encoder/mod.rs:236-284; vector.rs:137
```

## encode_v_arith_vx_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects too few operands and non-matching register classes. get_vreg/get_reg return Err for missing or wrong-class operands. Stronger oracles do not apply to the invalid domain.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_neg_arity_bad_regs
- Formal: ∀ ops with |ops|<3, funct6 ∈ 0..63. encode_v_arith_vx(ops, funct6) is Err. ∀ pos ∈ {0,1}, bad not a v0..v31 Reg. encode_v_arith_vx(ops3 with ops[pos]=bad, funct6) is Err. ∀ bad_rs1 not an integer register or Imm(0..31). encode_v_arith_vx(ops3 with ops[2]=bad_rs1, funct6) is Err.
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad, pos, funct6]
  domain: { ops: arity 0..2 valid-looking, bad: non-matching register class, pos: 0..2, funct6: 0..63 }
  relation:
    op: throws
    lhs: encode_v_arith_vx(short_or_bad, funct6)
    rhs: String
generators:
  funct6: { gen: int, min: 0, max: 63, type: u32 }
  pos: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: vector.rs:135-137; encoder/mod.rs:437-445; encoder/mod.rs:468-474
```

## encode_v_arith_vx_neg_extra
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects a fourth operand on OPIVX (`operand must be v0.t` / invalid operand). The public wrapper passes operands through. No documentation declares extra operands valid. Domain includes extra tokens; a silent ignore is a bug.
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_neg_extra
- Formal: ∀ vd, vs2, rs1 ∈ 0..31, extra ∉ {v0.t mask}, (mnem, funct6) ∈ opivx_family. encode_v_arith_vx([v{vd}, v{vs2}, x{rs1}, extra], funct6) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vx([Reg("v0"), Reg("v0"), Reg("x0"), Imm(0)], funct6=0b000000) = Ok(Word(0x02004057))
- Bug report: pbt-out/bug_reports/encode_v_arith_vx_extra_operand.md

```property
function: encoder.vector.encode_v_arith_vx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, rs1, extra, funct6]
  domain: { vd: 0..31, vs2: 0..31, rs1: 0..31, extra: non-mask extra operand, funct6: opivx_family }
  relation:
    op: throws
    lhs: encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1}), extra], funct6)
    rhs: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u32 }
expected_error: String
evidence: vector.rs:133-140; llvm-mc OPIVX arity
```

## encode_v_arith_vx_mask_v0t
- Tier: 5
- Rationale: Differential vs llvm-mc for the documented RVV masked form `mnem vd, vs2, rs1, v0.t` (vm=0). Dispatcher TODO encoder/mod.rs:950 admits `masked variants (v0.t) are not yet supported` on an input the API accepts (operands passed through). That is a documented limitation, not an input-domain exclusion. Keep the input; file if it fails. vd generated in 1..31 so the destination does not overlap v0 (llvm-mc constraint).
- Doc contract: vector.rs:133 "Encode vector arithmetic VX (vector-scalar): funct6[31:26] | vm[25] | vs2[24:20] | rs1[19:15] | funct3[14:12]=100 | vd[11:7] | OP_V" — asserted fingerprint 8042a5f7
- Seed: encode_v_arith_vv_pbt.rs encode_v_arith_vv_mask_v0t
- Formal: ∀ vd ∈ {1..31}, vs2, rs1 ∈ {0..31}, (mnem, funct6) ∈ opivx_family with not (slide mnemonic and vd = vs2). encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1}), Symbol("v0.t")], funct6) = llvm-mc(mnem v{vd}, v{vs2}, x{rs1}, v0.t)
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vx([Reg("v1"), Reg("v0"), Reg("x0"), Symbol("v0.t")], funct6=0b000000) = Ok(Word(0x020040d7)); llvm-mc vadd.vx v1, v0, x0, v0.t = 0x000040d7
- Bug report: pbt-out/bug_reports/encode_v_arith_vx_mask_v0t.md

```property
function: encoder.vector.encode_v_arith_vx
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs2, rs1, mnem, funct6]
  domain: { vd: v1..v31, vs2: v0..v31, rs1: x0..x31, (mnem,funct6): opivx_family }
  relation:
    op: eq
    lhs: encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1}), Symbol("v0.t")], funct6)
    rhs: llvm_mc(mnem + " v" + vd + ", v" + vs2 + ", x" + rs1 + ", v0.t")
generators:
  vd: { gen: int, min: 1, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u32 }
evidence: vector.rs:133-140; encoder/mod.rs:950; RISC-V V 1.0 vm bit
```
