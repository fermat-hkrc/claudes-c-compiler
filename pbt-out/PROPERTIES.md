# Properties: encode_in

## encode_in_diff_dx_port
- Tier: 4
- Rationale: Differential vs llvm-mc is strongest available oracle (no in-tree i686 decoder for round-trip; pure encoder so no state machine). Intel SDM fixes IN DX-port as EC/ED; AT&T is `inb %dx, %al`.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:200 (generalized OUT DX-port differential to IN operand order)
- Formal: ∀ m ∈ {inb,inw,inl}. encode_in(m, [%dx, data_reg(m)]) = llvm_mc(m " %dx, %" ++ data_reg(m))
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic]
  domain: { mnemonic: {inb, inw, inl} }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [reg(\"dx\"), reg(data_reg(mnemonic))])"
    rhs: "llvm_mc_bytes(format!(\"{mnemonic} %dx, %{}\", data_reg(mnemonic)))"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
evidence: system.rs:74; Intel SDM IN EC/ED; llvm-mc -triple=i686
```

## encode_in_diff_imm8_port
- Tier: 4
- Rationale: Imm8 port form is the other architecturally valid encoding path (E4/E5 ib). Differential against llvm-mc over the accepted imm domain, and dual-reject when llvm rejects.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:212
- Formal: ∀ m ∈ {inb,inw,inl}, v ∈ imm_domain. agree_or_dual_reject(sut_encode(m,[imm(v),data_reg(m)]), llvm_mc(m " $v, %" ++ data_reg(m)))
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, imm_v]
  domain: { mnemonic: {inb,inw,inl}, imm_v: int }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [imm(imm_v), reg(data_reg(mnemonic))])"
    rhs: "llvm_mc_bytes(format!(\"{mnemonic} ${}, %{}\", imm_v, data_reg(mnemonic)))"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
  imm_v: { gen: int, min: -128, max: 255, type: i64 }
evidence: system.rs:99-103; Intel SDM IN E4/E5 ib
```

## encode_in_invariant_opcodes
- Tier: 3
- Rationale: Algebraic invariant — DX form always emits the fixed Intel opcode table independent of llvm-mc availability.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:247
- Formal: ∀ m ∈ {inb,inw,inl}. encode_in(m,[%dx,data_reg(m)]) = intel_dx_bytes(m)
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic]
  domain: { mnemonic: {inb,inw,inl} }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [reg(\"dx\"), reg(data_reg(mnemonic))])"
    rhs: "intel_dx_bytes(mnemonic)"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
evidence: Intel SDM Vol.2 IN — EC/ED (+66h)
```

## encode_in_invariant_imm_opcodes
- Tier: 3
- Rationale: Imm8 form opcode table E4/E5 (+66h for inw) with port byte preserved.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:261
- Formal: ∀ m ∈ {inb,inw,inl}, p ∈ u8. encode_in(m,[imm(p),data_reg(m)]) = intel_imm_bytes(m,p)
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, port]
  domain: { mnemonic: {inb,inw,inl}, port: u8 }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [imm(port as i64), reg(data_reg(mnemonic))])"
    rhs: "intel_imm_bytes(mnemonic, port)"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
  port: { gen: int, min: 0, max: 255, type: u8 }
evidence: Intel SDM Vol.2 IN — E4 ib / E5 ib
```

## encode_in_meta_size_prefix
- Tier: 3
- Rationale: Metamorphic — inw encoding must equal [0x66] concatenated with inl encoding for the same port shape (Intel operand-size override).
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:279
- Formal: ∀ use_imm ∈ bool, p ∈ u8. encode_in("inw", ops_w(p)) = [0x66] ++ encode_in("inl", ops_l(p))
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [use_imm, port]
  domain: { use_imm: bool, port: u8 }
  relation:
    op: eq
    lhs: "sut_encode(\"inw\", ops_w)"
    rhs: "[0x66] ++ sut_encode(\"inl\", ops_l)"
generators:
  use_imm: { gen: bool }
  port: { gen: int, min: 0, max: 255, type: u8 }
evidence: Intel SDM operand-size override 66h for AX vs EAX IN
```

## encode_in_neg_arity
- Tier: 3
- Rationale: Negative contract — arity must be 0 or 2; other arities Err.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:300
- Formal: ∀ m ∈ {inb,inw,inl}, n ∈ {1,3,4,5}. encode_in(m, ops_n) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, n]
  domain: { mnemonic: {inb,inw,inl}, n: {1,3,4,5} }
  relation:
    op: holds
    expr: "sut_encode(mnemonic, ops_of_len_n).is_err()"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
  n: { gen: int, min: 1, max: 5, type: usize }
expected_error: String
evidence: system.rs:89-91
```

## encode_in_neg_wrong_registers
- Tier: 4
- Rationale: Intel fixes data register to AL/AX/EAX and port to DX; non-canonical pairs must be rejected (llvm-mc independent rejection oracle).
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:326
- Formal: ∀ m, src, dst. ¬(src=dx ∧ dst=data_reg(m)) ∧ llvm_mc rejects(m,src,dst) ⇒ encode_in(m,[src,dst]) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: failing
- Counterexample: inb %al, %al → Ok([0xec]); llvm-mc rejects
- Bug report: bug_reports/encode_in_wrong_registers.md

```property
function: i686.encoder.encode_in
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, src, dst]
  domain: { mnemonic: {inb,inw,inl}, src: gp_regs, dst: gp_regs }
  relation:
    op: holds
    expr: "!(src==\"dx\" && dst==data_reg(mnemonic)) && llvm_rejects => sut_encode(...).is_err()"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
  src: { gen: oneof, values: ["al","ax","eax","bl","bx","ebx","dx","edx"] }
  dst: { gen: oneof, values: ["al","ax","eax","bl","bx","ebx","dx","edx"] }
expected_error: String
evidence: Intel SDM IN fixed AL/AX/EAX + DX; llvm-mc rejects others
```

## encode_in_neg_imm_out_of_range
- Tier: 4
- Rationale: Port immediate outside imm8 must not silently truncate via `*val as u8`. Dual-reject with llvm-mc over out-of-range domain. This property FAILS on the SUT — filed as bug.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:369
- Formal: ∀ m ∈ {inb,inw,inl}, v ∉ imm8_accepted. llvm_mc rejects(m,$v) ⇒ encode_in(m,[imm(v),data_reg(m)]) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: failing
- Counterexample: inb $256, %al → Ok([0xe4, 0x00]); llvm-mc rejects
- Bug report: bug_reports/encode_in_imm_truncation.md

```property
function: i686.encoder.encode_in
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, imm_v]
  domain: { mnemonic: {inb,inw,inl}, imm_v: oor_imm }
  relation:
    op: holds
    expr: "llvm_rejects(m, imm_v) => sut_encode(m, [imm(imm_v), data_reg(m)]).is_err()"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
  imm_v: { gen: int, min: -4096, max: 4096, type: i64 }
expected_error: String
evidence: system.rs:99-103 (*val as u8); Intel imm8 port
```

## encode_in_diff_dx_memory_form
- Tier: 4
- Rationale: AT&T parenthesized port `(%dx)` is accepted by llvm-mc/gas as alias of `%dx`; sibling x86 encode_in handles Memory,Register. Differential must agree. This property FAILS on the SUT — filed as bug.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e; sibling x86/system.rs:75 "Also handle parenthesized form: inl (%dx), %eax"
- Seed: encode_out_pbt.rs:409
- Formal: ∀ m ∈ {inb,inw,inl}. encode_in(m, [mem(%dx), data_reg(m)]) = llvm_mc(m " (%dx), %" ++ data_reg(m))
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: failing
- Counterexample: inb (%dx), %al → Err("unsupported inb operands"); llvm-mc=[0xec]
- Bug report: bug_reports/encode_in_missing_dx_mem_form.md

```property
function: i686.encoder.encode_in
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic]
  domain: { mnemonic: {inb,inw,inl} }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [mem_dx(), reg(data_reg(mnemonic))])"
    rhs: "llvm_mc_bytes(format!(\"{mnemonic} (%dx), %{}\", data_reg(mnemonic)))"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
evidence: x86 sibling system.rs:83-87; llvm-mc accepts (%dx)
```

## encode_in_meta_dx_vs_imm_families
- Tier: 3
- Rationale: Metamorphic strengthening — DX-form last opcode is EC/ED and imm-form opcode is E4/E5; the two families must differ.
- Doc contract: system.rs:74 "Encode IN instruction: inb/inw/inl" — asserted fingerprint 762a8b6e
- Seed: encode_out_pbt.rs:429
- Formal: ∀ m ∈ {inb,inw,inl}, p ∈ u8. last_op(encode_in DX) ∈ {EC,ED} ∧ op(encode_in imm) ∈ {E4,E5} ∧ last_op(DX) ≠ op(imm)
- Test file: src/backend/i686/assembler/encoder/encode_in_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: i686.encoder.encode_in
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mnemonic, port]
  domain: { mnemonic: {inb,inw,inl}, port: u8 }
  relation:
    op: holds
    expr: "dx_op in {0xEC,0xED} && imm_op in {0xE4,0xE5} && dx_op != imm_op"
generators:
  mnemonic: { gen: oneof, values: ["inb", "inw", "inl"] }
  port: { gen: int, min: 0, max: 255, type: u8 }
evidence: Intel SDM IN opcode families EC/ED vs E4/E5
```
