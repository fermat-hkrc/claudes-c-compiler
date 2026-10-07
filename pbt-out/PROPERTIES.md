# Properties: encode_vmv_v_i

## encode_vmv_v_i_diff_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (LLVM 15.0.6), an independently meaningful assembler of the same RISC-V V 1.0 vmv.v.i encoding. State machine rejected: encode_vmv_v_i is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree vmv.v.i decoder. Sibling encode_vmv_v_v / encode_vmv_v_x / encode_v_arith_vi rejected by same-job gate (OPIVV / OPIVX / 3-operand OPIVI with vs2). Public wrapper encoder/mod.rs:1008 passes operands through, so the helper contract is the assembler contract.
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_vmv_v_x_pbt.rs:247 encode_vmv_v_x_diff_llvm_mc (generalized from sibling 2-operand vmv.v.x to vmv.v.i with simm5)
- Formal: ∀ vd ∈ {0..31}, simm ∈ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(simm)]) = llvm-mc("vmv.v.i v{vd}, {simm}") as little-endian Word
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vmv_v_i
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, simm]
  domain: { vd: "v0..v31", simm: "signed_simm5" }
  relation:
    op: eq
    lhs: "encode_vmv_v_i([Reg(v{vd}), Imm(simm)])"
    rhs: "llvm_mc(vmv.v.i v{vd}, {simm})"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
evidence: vector.rs:171; encoder/mod.rs:1008; assembler/README.md:14; llvm-mc RISC-V V 1.0
```

## encode_vmv_v_i_format_fields
- Tier: 3
- Rationale: Algebraic invariant unpacking the RISC-V V 1.0 OPIVI layout named by the rustdoc (opcode, funct3=011, vm=1, vs2=0, funct6=010111, vd, simm5). Stronger differential already present as a sibling property; this pins field placement independently of llvm-mc.
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_vmv_v_x_pbt.rs:256 encode_vmv_v_x_format_fields
- Formal: ∀ vd ∈ {0..31}, simm ∈ {-16..15}. let w = encode_vmv_v_i([Reg("v{vd}"), Imm(simm)]). w[6:0]=1010111 ∧ w[11:7]=vd ∧ w[14:12]=011 ∧ w[19:15]=(simm as u32)&0x1F ∧ w[24:20]=0 ∧ w[25]=1 ∧ w[31:26]=010111
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vmv_v_i
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, simm]
  domain: { vd: "v0..v31", simm: "signed_simm5" }
  relation:
    op: holds
    expr: "unpack(encode_vmv_v_i([Reg(v{vd}), Imm(simm)])) matches OPIVI layout"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
evidence: vector.rs:171
```

## encode_vmv_v_i_field_isolation
- Tier: 3
- Rationale: Metamorphic isolation — changing vd (resp. simm5) must only affect bits [11:7] (resp. [19:15]). Required metamorphic/differential property for standard tier.
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_vmv_v_x_pbt.rs:268 encode_vmv_v_x_field_isolation
- Formal: ∀ vd_a, vd_b ∈ {0..31}, simm_a, simm_b ∈ {-16..15}. let wa = encode_vmv_v_i([v{vd_a}, Imm(simm_a)]); wb = encode_vmv_v_i([v{vd_b}, Imm(simm_a)]); wc = encode_vmv_v_i([v{vd_a}, Imm(simm_b)]). (wa & ~(0x1F<<7)) = (wb & ~(0x1F<<7)) ∧ (wa & ~(0x1F<<15)) = (wc & ~(0x1F<<15))
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vmv_v_i
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, simm_a, simm_b]
  domain: { vd_a: "v0..v31", vd_b: "v0..v31", simm_a: "signed_simm5", simm_b: "signed_simm5" }
  relation:
    op: holds
    expr: "vd and simm5 bits are independent"
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  simm_a: { gen: int, min: -16, max: 15, type: i64 }
  simm_b: { gen: int, min: -16, max: 15, type: i64 }
evidence: vector.rs:171-176
```

## encode_vmv_v_i_simm5_twos_complement
- Tier: 3
- Rationale: Algebraic invariant that simm5 is two's-complement packed into bits [19:15]. Documented bounds [-16, 15] are sampled exactly (generator min/max pin the edges). Weaker than differential; pins the encoding of negative immediates independently.
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_v_arith_vi_pbt.rs:438 encode_v_arith_vi_simm5_twos_complement
- Formal: ∀ vd ∈ {0..31}, simm ∈ {-16..15}. (encode_vmv_v_i([Reg("v{vd}"), Imm(simm)]) >> 15) & 0x1F = (simm as u32) & 0x1F
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vmv_v_i
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, simm]
  domain: { vd: "v0..v31", simm: "signed_simm5" }
  relation:
    op: eq
    lhs: "(encode_vmv_v_i([Reg(v{vd}), Imm(simm)]) >> 15) & 0x1F"
    rhs: "(simm as u32) & 0x1F"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
evidence: vector.rs:171-174
```

## encode_vmv_v_i_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: too few operands and non-vector vd / non-Imm simm5 must Err. get_vreg/get_imm return Err on missing or wrong-kind operands. Documented error is Result::Err(String).
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_vmv_v_x_pbt.rs:326 encode_vmv_v_x_neg_arity_bad_regs
- Formal: ∀ ops with |ops|<2 ∨ ops[0] ∉ v0..v31 ∨ ops[1] is not Imm. encode_vmv_v_i(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vmv_v_i
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "arity_lt_2_or_bad_vd_or_non_imm" }
  relation:
    op: holds
    expr: "encode_vmv_v_i(ops).is_err()"
generators:
  ops: { gen: list, elem: { gen: int, min: 0, max: 31, type: u32 }, maxLen: 1 }
expected_error: String
evidence: encoder/mod.rs:476-487 get_vreg/get_imm; llvm-mc invalid operand
```

## encode_vmv_v_i_neg_extra
- Tier: 3
- Rationale: Negative/error contract from llvm-mc: extra operand after a complete `vmv.v.i vd, simm5` is invalid. Public wrapper passes extra operands through. SUT currently ignores operands beyond index 1.
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_vmv_v_x_pbt.rs:360 encode_vmv_v_x_neg_extra
- Formal: ∀ vd ∈ {0..31}, simm ∈ {-16..15}, extra ∈ Operand. encode_vmv_v_i([Reg("v{vd}"), Imm(simm), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_i([Reg("v0"), Imm(-16), Imm(0)]) → Ok(Word(0x5e083057))
- Bug report: pbt-out/bug_reports/encode_vmv_v_i_extra_operand.md

```property
function: encoder.encode_vmv_v_i
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, simm, extra]
  domain: { vd: "v0..v31", simm: "signed_simm5", extra: "Operand" }
  relation:
    op: holds
    expr: "encode_vmv_v_i([Reg(v{vd}), Imm(simm), extra]).is_err()"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
  extra: { gen: int, min: 0, max: 1, type: i64 }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on extra token; encoder/mod.rs:1008 operands passed through
```

## encode_vmv_v_i_neg_mask_v0t
- Tier: 3
- Rationale: Negative/error contract: RISC-V V 1.0 vmv.v.i is unmasked-only (vm=1); llvm-mc rejects trailing v0.t. Dispatcher TODO encoder/mod.rs:956 admits masked variants are not supported; for vmv.v.i the masked form is not a valid ISA encoding, so accepting v0.t is a bug (not a deferred feature).
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_vmv_v_x_pbt.rs:375 encode_vmv_v_x_neg_mask_v0t
- Formal: ∀ vd ∈ {0..31}, simm ∈ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(simm), Symbol("v0.t")]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_i([Reg("v0"), Imm(-16), Symbol("v0.t")]) → Ok(Word(0x5e083057))
- Bug report: pbt-out/bug_reports/encode_vmv_v_i_mask_v0t.md

```property
function: encoder.encode_vmv_v_i
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, simm]
  domain: { vd: "v0..v31", simm: "signed_simm5" }
  relation:
    op: holds
    expr: "encode_vmv_v_i([Reg(v{vd}), Imm(simm), Symbol(v0.t)]).is_err()"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -16, max: 15, type: i64 }
expected_error: String
evidence: vector.rs:171 vm=1; llvm-mc rejects v0.t on vmv.v.i; RISC-V V 1.0 unmasked-only
```

## encode_vmv_v_i_neg_imm_oob
- Tier: 3
- Rationale: Negative/error contract: llvm-mc requires simm5 ∈ [-16, 15] ("immediate must be an integer in the range [-16, 15]"). The rustdoc names the field simm5. Documented bounds must be exercised at bound±1 (16 and -17). SUT currently truncates with `& 0x1F`.
- Doc contract: vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" — asserted fingerprint 00b9eacd
- Seed: encode_v_arith_vi_pbt.rs:530 encode_v_arith_vi_neg_imm_oob
- Formal: ∀ vd ∈ {0..31}, imm ∈ ℤ \ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(imm)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs
- Status: failing
- Counterexample: encode_vmv_v_i([Reg("v0"), Imm(16)]) → Ok(Word(0x5e083057)) (encodes as -16)
- Bug report: pbt-out/bug_reports/encode_vmv_v_i_imm_oob.md

```property
function: encoder.encode_vmv_v_i
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, imm]
  domain: { vd: "v0..v31", imm: "i64_outside_simm5" }
  relation:
    op: holds
    expr: "encode_vmv_v_i([Reg(v{vd}), Imm(imm)]).is_err()"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 16, max: 1024, type: i64 }
expected_error: String
evidence: llvm-mc "immediate must be an integer in the range [-16, 15]"; vector.rs:171 simm5
```
