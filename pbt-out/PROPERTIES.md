# Properties: encode_out

## encode_out_diff_dx_port
- Tier: 4
- Rationale: Strongest applicable is differential vs llvm-mc (independent assembler). State machine N/A (pure encode). Round-trip N/A (no i686 decoder). Intel OUT DX forms are fixed encodings EE/EF (+66 for word).
- Doc contract: system.rs:38 "Encode OUT instruction: outb/outw/outl" — asserted fingerprint 852b5a16
- Seed: (none)
- Formal: ∀ m ∈ {outb,outw,outl}. encode(m, Reg(data_reg(m)), Reg(dx)) = llvm_mc("{m} %data, %dx")
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_out
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic]
  domain: { mnemonic: {outb,outw,outl} }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Reg(data_reg(mnemonic)), Reg(dx)])"
    rhs: "llvm_mc(format!(\"{mnemonic} %{data}, %dx\"))"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
evidence: system.rs:38; Intel SDM OUT rDX; llvm-mc i686; encoder/mod.rs:360
```

## encode_out_diff_imm8_port
- Tier: 4
- Rationale: Differential vs llvm-mc for imm8 port form E6/E7 ib over the imm domain llvm-mc accepts.
- Doc contract: system.rs:38 "Encode OUT instruction: outb/outw/outl" — asserted fingerprint 852b5a16
- Seed: (none)
- Formal: ∀ m ∈ {outb,outw,outl}, imm ∈ Imm8Accepted. encode(m, data, $imm) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_out
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, imm]
  domain: { mnemonic: {outb,outw,outl}, imm: imm8_domain }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Reg(data), Imm(imm)])"
    rhs: "llvm_mc(format!(\"{mnemonic} %{data}, ${imm}\"))"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
  imm: { gen: int, min: -128, max: 255, type: i64 }
evidence: system.rs:64-68; Intel E6/E7 ib
```

## encode_out_invariant_opcodes
- Tier: 3
- Rationale: Algebraic invariant — successful DX-port encode must emit exactly the Intel fixed opcode bytes for the mnemonic size.
- Doc contract: system.rs:38 "Encode OUT instruction: outb/outw/outl" — asserted fingerprint 852b5a16
- Seed: (none)
- Formal: ∀ m. encode_ok(m, data_reg(m), dx) ⇒ bytes = intel_dx_opcode(m)
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_out
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic]
  domain: { mnemonic: {outb,outw,outl} }
  relation:
    op: eq
    lhs: "sut_bytes(mnemonic, dx_form)"
    rhs: "intel_opcode(mnemonic)"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
evidence: Intel SDM Vol.2 OUT
```

## encode_out_meta_imm_trunc_independent_of_high_bits_is_wrong
- Tier: 3
- Rationale: Negative — imm outside imm8 that llvm-mc rejects must not silently encode a different port.
- Doc contract: system.rs:38 "Encode OUT instruction: outb/outw/outl" — asserted fingerprint 852b5a16
- Seed: (none)
- Formal: ∀ m, imm. llvm_mc rejects(m,$imm) ⇒ sut.encode returns Err
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: failing
- Counterexample: mnemonic="outb", imm_v=256 → Ok([0xe6, 0x00])
- Bug report: bug_reports/encode_out_imm_truncation.md

```property
function: encode_out
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, imm]
  domain: { imm: outside_u8_non_neg_byte }
  relation:
    op: holds
    expr: "sut_encode(mnemonic, [Reg(data), Imm(imm)]).is_err()"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
  imm: { gen: int, min: 256, max: 65535, type: i64 }
evidence: llvm-mc rejects imm>255; Intel imm8 port
expected_error: Err
```

## encode_out_neg_arity
- Tier: 3
- Rationale: Code requires 0 or 2 operands; arity 1 or ≥3 must Err.
- Doc contract: system.rs:54-55 format!("{} requires 0 or 2 operands", mnemonic) — asserted fingerprint 01ba36db
- Seed: (none)
- Formal: ∀ m, ops. len(ops)∉{0,2} ⇒ encode returns Err containing "requires 0 or 2"
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_out
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, n]
  domain: { n: arity_excluding_0_and_2 }
  relation:
    op: holds
    expr: "sut_encode(mnemonic, ops_of_len(n)).is_err() && err.contains(\"requires 0 or 2\")"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
  n: { gen: int, min: 1, max: 5, type: usize }
evidence: system.rs:54-55
expected_error: Err
```

## encode_out_neg_wrong_registers
- Tier: 4
- Rationale: Intel/AT&T require data in AL/AX/EAX and port DX. Any other register pair is invalid; llvm-mc/gas reject. SUT must not silently emit EE/EF.
- Doc contract: system.rs:38 "Encode OUT instruction: outb/outw/outl" — asserted fingerprint 852b5a16; x86 sibling "AT&T: outb %al, %dx"
- Seed: (none)
- Formal: ∀ m, src, dst. ¬(src=data_reg(m) ∧ dst=dx) ∧ llvm_mc_rejects(m,src,dst) ⇒ encode returns Err
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: failing
- Counterexample: mnemonic="outb", src="al", dst="al" → Ok([0xee])
- Bug report: bug_reports/encode_out_wrong_registers.md

```property
function: encode_out
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, src, dst]
  domain: { pairs: invalid_out_register_pairs }
  relation:
    op: holds
    expr: "sut_encode(mnemonic, [Reg(src), Reg(dst)]).is_err()"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
  src: { gen: string }
  dst: { gen: string }
evidence: Intel SDM OUT; llvm-mc rejects wrong regs; gas operand type mismatch
expected_error: Err
```

## encode_out_diff_dx_memory_form
- Tier: 4
- Rationale: Differential — AT&T `outl %eax, (%dx)` is accepted by llvm-mc and x86-64 sibling encode_out; i686 should agree.
- Doc contract: x86 system.rs:27 "Also handle parenthesized form: outl %eax, (%dx)"; Intel same EE/EF encoding
- Seed: (none)
- Formal: ∀ m. encode(m, Reg(data), Mem(dx)) = llvm_mc("{m} %data, (%dx)")
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: failing
- Counterexample: mnemonic="outb" → Err("unsupported outb operands"); llvm-mc=[0xee]
- Bug report: bug_reports/encode_out_dx_memory_form.md

```property
function: encode_out
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic]
  domain: { mnemonic: {outb,outw,outl} }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Reg(data), Mem(dx)])"
    rhs: "llvm_mc(\"{m} %{data}, (%dx)\")"
generators:
  mnemonic: { gen: oneof, values: ["outb", "outw", "outl"] }
evidence: x86/system.rs:35-39; llvm-mc accepts (%dx)
```

## encode_out_meta_size_prefix
- Tier: 3
- Rationale: Metamorphic — outw bytes equal [0x66] concatenated with outl bytes for the corresponding operand shape.
- Doc contract: system.rs:49-51 size==2 pushes 0x66
- Seed: (none)
- Formal: ∀ form∈{dx,imm8}. bytes(outw,form) = [0x66] ++ bytes(outl,form)
- Test file: src/backend/i686/assembler/encoder/encode_out_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_out
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..255 }
  relation:
    op: eq
    lhs: "sut(outw, form)"
    rhs: "[0x66] ++ sut(outl, form)"
generators:
  imm: { gen: int, min: 0, max: 255, type: i64 }
evidence: system.rs:49-51,65-67; Intel operand-size override
```
