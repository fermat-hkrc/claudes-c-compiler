# Properties: encode_v_arith_vi

## encode_v_arith_vi_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_v_arith_vi is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree OPIVI decoder. encode_v_arith_vv / encode_v_arith_vx are same-file siblings with different jobs (OPIVV funct3=000 / OPIVX funct3=100). Spec ownership: rustdoc plus assembler README claim RVV OPIVI encoding; llvm-mc is a trusted pinned tool implementing that ISA. Wrapper encode_instruction passes operands through.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_diff_llvm_mc (same RVV encoder family)
- Formal: ∀ vd, vs2 ∈ {0..31}, (mnem, funct6, imm) ∈ signed_family × [-16,15] ∪ slide_family × [0,31] with not (mnem = vslideup.vi ∧ vd = vs2). encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(imm)], funct6) = llvm-mc(mnem v{vd}, v{vs2}, imm). signed_family = {(vadd.vi, 0b000000), (vand.vi, 0b001001), (vor.vi, 0b001010), (vxor.vi, 0b001011)}; slide_family = {(vslideup.vi, 0b001110), (vslidedown.vi, 0b001111)}. vslideup overlap is skipped because llvm-mc rejects dest overlapping vs2 (architectural, not encoding).
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vi
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs2, imm, mnem, funct6]
  domain: { vd: v0..v31, vs2: v0..v31, imm: per-mnemonic valid range, mnem_funct6: opivi_family }
  relation:
    op: eq
    lhs: encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(imm)], funct6)
    rhs: llvm_mc(mnem + " v" + vd + ", v" + vs2 + ", " + imm)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
  uimm: { gen: int, min: 0, max: 31, type: i64 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
evidence: vector.rs:143-150; encoder/mod.rs:980-997; assembler/README.md:14
```

## encode_v_arith_vi_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the rustdoc format line (RISC-V V 1.0 OPIVI layout). Stronger differential is p1; this unpacks fields independently of llvm-mc so a mapping bug cannot hide a layout bug. funct6 covers the full 6-bit field 0..63; simm covers the full signed 5-bit domain [-16,15] so bits 19:15 include 0, 15, 16 (-16), 31 (-1).
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_format_fields
- Formal: ∀ vd, vs2 ∈ 0..31, simm ∈ [-16,15], funct6 ∈ 0..63. let w = encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(simm)], funct6) in Word. (w & 0x7f) = 0b1010111 ∧ ((w>>7)&0x1f)=vd ∧ ((w>>12)&0x7)=0b011 ∧ ((w>>15)&0x1f)=(simm as u32)&0x1F ∧ ((w>>20)&0x1f)=vs2 ∧ ((w>>25)&1)=1 ∧ ((w>>26)&0x3f)=funct6
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, simm, funct6]
  domain: { vd: 0..31, vs2: 0..31, simm: -16..15, funct6: 0..63 }
  body: unpack(encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(simm)], funct6)) matches RISC-V V 1.0 OPIVI fields
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:143-150
```

## encode_v_arith_vi_field_isolation
- Tier: 4
- Rationale: Algebraic invariant: vd/vs2/simm5/funct6 occupy disjoint bit fields. Changing one field must not alter the others. Metamorphic: required by the standard tier. Catches vs2/simm mix-ups independently of llvm-mc.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_field_isolation
- Formal: ∀ vd_a, vd_b, vs2_a, vs2_b ∈ 0..31, simm_a, simm_b ∈ [-16,15], f6_a, f6_b ∈ 0..63. let wa = encode_v_arith_vi([v{vd_a}, v{vs2_a}, Imm(simm_a)], f6_a). Changing only vd (resp. vs2, simm, funct6) flips only bits [11:7] (resp. [24:20], [19:15], [31:26]).
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs2_a, vs2_b, simm_a, simm_b, f6_a, f6_b]
  domain: { vd_*: 0..31, vs2_*: 0..31, simm_*: -16..15, f6_*: 0..63 }
  body: changing one of vd/vs2/simm/funct6 flips only that field's bits
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs2_a: { gen: int, min: 0, max: 31, type: u32 }
  vs2_b: { gen: int, min: 0, max: 31, type: u32 }
  simm_a: { gen: int, min: -16, max: 15, type: i64 }
  simm_b: { gen: int, min: -16, max: 15, type: i64 }
  f6_a: { gen: int, min: 0, max: 63, type: u32 }
  f6_b: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:143-150
```

## encode_v_arith_vi_simm5_twos_complement
- Tier: 4
- Rationale: Documented bound of the simm5 field (RISC-V V 1.0 signed 5-bit immediate, range [-16, 15]). The rustdoc names simm5[19:15]; two's complement packing is the ISA encoding, not a copy of the SUT body. Generator is pinned to the closed interval so -16, -1, 0, 15 are in domain; llvm-mc KAT pins the same bounds.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: llvm-mc vadd.vi v1, v2, -16 / -1 / 0 / 15 encodings
- Formal: ∀ vd, vs2 ∈ 0..31, simm ∈ [-16,15], funct6 ∈ 0..63. ((encode_v_arith_vi([v{vd}, v{vs2}, Imm(simm)], funct6) >> 15) & 0x1F) = (simm as u32) & 0x1F
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, simm, funct6]
  domain: { vd: 0..31, vs2: 0..31, simm: -16..15, funct6: 0..63 }
  relation:
    op: eq
    lhs: (encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(simm)], funct6) >> 15) & 0x1F
    rhs: (simm as u32) & 0x1F
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:143; RISC-V V 1.0 OPIVI simm5; llvm-mc range [-16, 15]
```

## encode_v_arith_vi_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract from llvm-mc (too few operands / invalid operand) and get_vreg/get_imm signatures. Wrapper passes operands through. Stronger differential does not apply on the invalid domain.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_neg_arity_bad_regs
- Formal: ∀ ops with |ops| < 3, funct6 ∈ 0..63. encode_v_arith_vi(ops, funct6) is Err. ∀ bad non-vector at vd or vs2, encode_v_arith_vi is Err. ∀ non-Imm at operand 2, encode_v_arith_vi is Err.
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.vector.encode_v_arith_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad_v, bad_imm, pos, funct6]
  domain: { ops: arity 0..2, bad_v: non-vreg, bad_imm: non-Imm, pos: 0..1, funct6: 0..63 }
  relation:
    op: throws
    expr: encode_v_arith_vi(ops, funct6)
expected_error: String
generators:
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: encoder/mod.rs:470-481 get_vreg/get_imm; llvm-mc too few / invalid operand
```

## encode_v_arith_vi_neg_extra
- Tier: 3
- Rationale: llvm-mc rejects a fourth operand that is not v0.t ("operand must be v0.t"). encode_v_arith_vi never inspects operands past index 2, so extra tokens are silently ignored. Wrapper encode_instruction passes operands through, so this is the public assembler contract.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_neg_extra
- Formal: ∀ vd, vs2 ∈ 0..31, imm in the mnemonic's valid range, extra ∉ {v0.t}, (mnem,funct6) ∈ opivi_family. encode_v_arith_vi([v{vd}, v{vs2}, Imm(imm), extra], funct6) is Err.
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vi([Reg("v0"), Reg("v0"), Imm(-16), Imm(0)], funct6=0b000000) = Ok(Word(0x02083057))
- Bug report: bug_reports/encode_v_arith_vi_extra_operand.md

```property
function: encoder.vector.encode_v_arith_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, imm, extra, funct6]
  domain: { vd: 0..31, vs2: 0..31, extra: Operand minus v0.t, (mnem,funct6): opivi_family }
  relation:
    op: throws
    expr: encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(imm), extra], funct6)
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Imm, Reg, Symbol, Label, Mem, FenceArg, Csr, RoundingMode] }
evidence: llvm-mc "operand must be v0.t"; encoder/mod.rs:980-997 operands passed through
```

## encode_v_arith_vi_mask_v0t
- Tier: 5
- Rationale: Differential against llvm-mc for the documented masked form. RISC-V V 1.0 sets vm=0 when `, v0.t` is present. Dispatcher TODO encoder/mod.rs:952 admits "masked variants (v0.t) are not yet supported" on an input the API accepts (known limitation, not an input-domain restriction). vd generated in 1..31 so dest does not overlap the mask register.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: encode_v_arith_vx_pbt.rs encode_v_arith_vx_mask_v0t
- Formal: ∀ vd ∈ {1..31}, vs2 ∈ {0..31}, (mnem, funct6, imm) in the valid OPIVI domain with not (mnem = vslideup.vi ∧ vd = vs2). encode_v_arith_vi([v{vd}, v{vs2}, Imm(imm), Symbol("v0.t")], funct6) = llvm-mc(mnem v{vd}, v{vs2}, imm, v0.t).
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vi([Reg("v1"), Reg("v0"), Imm(-16), Symbol("v0.t")], funct6=0b000000) = Ok(Word(0x020830d7)); llvm-mc vadd.vi v1, v0, -16, v0.t = 0x000830d7
- Bug report: bug_reports/encode_v_arith_vi_mask_v0t.md

```property
function: encoder.vector.encode_v_arith_vi
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vs2, imm, mnem, funct6]
  domain: { vd: v1..v31, vs2: v0..v31, imm: per-mnemonic valid, (mnem,funct6): opivi_family }
  relation:
    op: eq
    lhs: encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(imm), Symbol("v0.t")], funct6)
    rhs: llvm_mc(mnem + " v" + vd + ", v" + vs2 + ", " + imm + ", v0.t")
generators:
  vd: { gen: int, min: 1, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
evidence: encoder/mod.rs:952 TODO masked variants; llvm-mc vadd.vi v1, v2, 3, v0.t = 0x0021b0d7
```

## encode_v_arith_vi_neg_imm_oob
- Tier: 3
- Rationale: llvm-mc rejects immediates outside the mnemonic's documented range (signed family [-16,15]; slide family [0,31]). The SUT packs `get_imm as u32 & 0x1F` with no range check. Wrapper passes the Imm through. Documented bounds must be sampled at bound±1. Not an input-domain restriction on encode_v_arith_vi itself: the rustdoc names the 5-bit field, llvm-mc is the assembler error contract.
- Doc contract: vector.rs:143 "Encode vector arithmetic VI (vector-immediate): funct6[31:26] | vm[25] | vs2[24:20] | simm5[19:15] | funct3[14:12]=011 | vd[11:7] | OP_V" — asserted fingerprint 1a48b3da
- Seed: llvm-mc "immediate must be an integer in the range [-16, 15]" / "[0, 31]"
- Formal: ∀ vd, vs2 ∈ 0..31, (mnem,funct6) ∈ signed_family, imm ∈ ℤ \ [-16,15]. encode_v_arith_vi([v{vd}, v{vs2}, Imm(imm)], funct6) is Err. ∀ (mnem,funct6) ∈ slide_family, imm ∈ ℤ \ [0,31]. encode_v_arith_vi(...) is Err.
- Test file: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs
- Status: failing
- Counterexample: encode_v_arith_vi([Reg("v0"), Reg("v0"), Imm(-17)], funct6=0b000000) = Ok(Word(0x0207b057))
- Bug report: bug_reports/encode_v_arith_vi_imm_oob.md

```property
function: encoder.vector.encode_v_arith_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, imm, mnem, funct6]
  domain: { vd: 0..31, vs2: 0..31, imm: complement of mnemonic range, (mnem,funct6): opivi_family }
  relation:
    op: throws
    expr: encode_v_arith_vi([Reg(v{vd}), Reg(v{vs2}), Imm(imm)], funct6)
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  signed_oob: { gen: oneof, options: [int(-1000..-17), int(16..1000), const(-17), const(16)] }
  slide_oob: { gen: oneof, options: [int(-1000..-1), int(32..1000), const(-1), const(32)] }
evidence: llvm-mc range errors; vector.rs:147 `as u32 & 0x1F` truncation
```
