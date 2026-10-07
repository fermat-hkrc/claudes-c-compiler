# Properties: encode_vid_v

## encode_vid_v_diff_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (LLVM 15.0.6), an independently meaningful assembler of the same RISC-V V 1.0 vid.v encoding. State machine rejected: encode_vid_v is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree vid.v decoder. Sibling encode_vmv_v_v / encode_v_arith_vv rejected by same-job gate (OPIVV funct3=000 / 3-operand OPIVV with vs2/vs1 vs OPMVV unary with vs1=10001). Public wrapper encoder/mod.rs:1015 passes operands through, so the helper contract is the assembler contract.
- Doc contract: vector.rs:180 "vid.v vd: OPMVV, funct6=010100, vm=1, vs2=00000, rs1=10001" — asserted fingerprint d70b41ef
- Seed: encode_vmv_v_v_pbt.rs:208 encode_vmv_v_v_diff_llvm_mc (generalized from 2-operand vmv.v.v to 1-operand vid.v)
- Formal: ∀ vd ∈ {0..31}. encode_vid_v([Reg("v{vd}")]) = llvm-mc("vid.v v{vd}") as little-endian Word
- Test file: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vid_v
oracle: differential
predicate:
  quantifier: forall
  vars: [vd]
  domain: { vd: "v0..v31" }
  relation:
    op: eq
    lhs: "encode_vid_v([Reg(v{vd})])"
    rhs: "llvm_mc(vid.v v{vd})"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:180; encoder/mod.rs:1015; assembler/README.md:14; llvm-mc RISC-V V 1.0
```

## encode_vid_v_format_fields
- Tier: 3
- Rationale: Algebraic invariant unpacking the RISC-V V 1.0 OPMVV vid.v layout named by the rustdoc (opcode, funct3=010, vm=1, vs2=0, vs1=10001, funct6=010100, vd). Stronger differential already present as a sibling property; this pins field placement independently of llvm-mc.
- Doc contract: vector.rs:180 "vid.v vd: OPMVV, funct6=010100, vm=1, vs2=00000, rs1=10001" — asserted fingerprint d70b41ef
- Seed: encode_vmv_v_v_pbt.rs:218 encode_vmv_v_v_format_fields
- Formal: ∀ vd ∈ {0..31}. let w = encode_vid_v([Reg("v{vd}")]). w[6:0]=1010111 ∧ w[11:7]=vd ∧ w[14:12]=010 ∧ w[19:15]=10001 ∧ w[24:20]=0 ∧ w[25]=1 ∧ w[31:26]=010100
- Test file: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vid_v
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd]
  domain: { vd: "v0..v31" }
  relation:
    op: holds
    expr: "unpack(encode_vid_v([Reg(v{vd})])) matches OPMVV vid.v layout"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:180
```

## encode_vid_v_field_isolation
- Tier: 3
- Rationale: Metamorphic isolation — changing vd must only affect bits [11:7]; all other bits are constant across the vd domain. Required metamorphic/differential property for standard tier.
- Doc contract: vector.rs:180 "vid.v vd: OPMVV, funct6=010100, vm=1, vs2=00000, rs1=10001" — asserted fingerprint d70b41ef
- Seed: encode_vmv_v_v_pbt.rs:232 encode_vmv_v_v_field_isolation
- Formal: ∀ vd_a, vd_b ∈ {0..31}. let wa = encode_vid_v([v{vd_a}]); wb = encode_vid_v([v{vd_b}]). (wa & ~(0x1F<<7)) = (wb & ~(0x1F<<7)) ∧ (wa>>7)&0x1F = vd_a ∧ (wb>>7)&0x1F = vd_b
- Test file: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vid_v
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b]
  domain: { vd_a: "v0..v31", vd_b: "v0..v31" }
  relation:
    op: holds
    expr: "non-vd bits independent of vd"
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
evidence: vector.rs:180
```

## encode_vid_v_neg_arity_bad_regs
- Tier: 2
- Rationale: Negative/error contract. llvm-mc rejects too-few operands (`vid.v` with no vd) and non-vector vd (GPR/FP/v32/non-Reg). get_vreg returns Err for missing or non-vreg operand 0. No documented acceptance of those inputs.
- Doc contract: vector.rs:180 "vid.v vd: OPMVV, funct6=010100, vm=1, vs2=00000, rs1=10001" — asserted fingerprint d70b41ef
- Seed: encode_vmv_v_v_pbt.rs:273 encode_vmv_v_v_neg_arity_bad_regs
- Formal: ∀ ops. ops = [] ∨ (ops = [bad] ∧ bad ∉ v0..v31 as Operand::Reg) ⇒ encode_vid_v(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_vid_v
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "empty or one non-vreg operand" }
  relation:
    op: throws
    expr: "encode_vid_v(ops)"
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
expected_error: String
evidence: vector.rs:180; llvm-mc too few operands / invalid operand
```

## encode_vid_v_neg_extra
- Tier: 2
- Rationale: Negative/error contract. llvm-mc rejects any second operand other than v0.t (`operand must be v0.t` / `expected '.t' suffix` / `invalid operand`). The public wrapper passes extra operands through. No SUT comment declares extra tokens valid or out of domain. v0.t is excluded from this generator and covered by encode_vid_v_mask_v0t_diff_llvm_mc (it is a valid masked form, not extra garbage).
- Doc contract: vector.rs:180 "vid.v vd: OPMVV, funct6=010100, vm=1, vs2=00000, rs1=10001" — asserted fingerprint d70b41ef
- Seed: encode_vmv_v_v_pbt.rs:295 encode_vmv_v_v_neg_extra
- Formal: ∀ vd ∈ {0..31}, extra ∈ Operand \ {Symbol("v0.t")}. encode_vid_v([Reg("v{vd}"), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
- Status: failing
- Counterexample: encode_vid_v([Reg("v0"), Imm(0)]) → Ok(Word(0x5208a057))
- Bug report: pbt-out/bug_reports/encode_vid_v_extra_operand.md

```property
function: encoder.encode_vid_v
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, extra]
  domain: { vd: "v0..v31", extra: "Operand except Symbol(v0.t)" }
  relation:
    op: throws
    expr: "encode_vid_v([Reg(v{vd}), extra])"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc RISC-V V 1.0 vid.v operand list; encoder/mod.rs:1015
```

## encode_vid_v_mask_v0t_diff_llvm_mc
- Tier: 4
- Rationale: Differential vs llvm-mc for the documented RISC-V V 1.0 masked form `vid.v vd, v0.t` (vm=0). Unlike vmv.v.v, vid.v is maskable. Full vd domain {0..31}: llvm-mc encodes vm=0 for vd ∈ {1..31} and rejects vd=v0 (destination overlaps mask). Agreement is: SUT Word equals llvm-mc Word when llvm-mc accepts, and SUT is Err when llvm-mc rejects. Dispatcher TODO encoder/mod.rs:962 admits masked variants are not yet supported — a known limitation on an input the public API accepts, not a domain exclusion.
- Doc contract: encoder/mod.rs:962 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1 (unmasked)." — limitation fingerprint 99cac70e
- Seed: encode_vmv_v_v_pbt.rs:311 encode_vmv_v_v_neg_mask_v0t (inverted: for vid.v the mask form is valid, so the oracle is differential agreement not rejection)
- Formal: ∀ vd ∈ {0..31}. let mc = llvm-mc("vid.v v{vd}, v0.t"); let sut = encode_vid_v([Reg("v{vd}"), Symbol("v0.t")]). (mc = Ok(w) ⇒ sut = Ok(w)) ∧ (mc = Err ⇒ sut = Err)
- Test file: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs
- Status: failing
- Counterexample: encode_vid_v([Reg("v0"), Symbol("v0.t")]) → Ok(Word(0x5208a057))
- Bug report: pbt-out/bug_reports/encode_vid_v_mask_v0t.md

```property
function: encoder.encode_vid_v
oracle: differential
predicate:
  quantifier: forall
  vars: [vd]
  domain: { vd: "v0..v31" }
  relation:
    op: holds
    expr: "llvm_mc_ok iff sut_ok and then sut_word == mc_word"
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
evidence: RISC-V V 1.0 vid.v vm; encoder/mod.rs:962; llvm-mc -mattr=+v
```
