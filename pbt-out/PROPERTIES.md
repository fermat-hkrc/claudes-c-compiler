# Properties: encode_bsr_bsf_16

## encode_bsr_bsf_16_diff_r16
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. State machine rejected (pure encoder). Round-trip rejected (no in-tree i686 decoder).
- Doc contract: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw" — asserted fingerprint a7c3e91f
- Seed: (none) — pattern from encode_pop16_pbt.rs
- Formal: ∀ m ∈ {bsfw,bsrw}, ∀ s,d ∈ R16. encode(m, %s, %d) = llvm-mc(m %s, %d)
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf_16
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, src, dst]
  domain: { mnemonic: bsfw_or_bsrw, src: R16, dst: R16 }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"{mnemonic} %{src}, %{dst}\"))"
generators:
  mnemonic: { gen: oneof, options: [bsfw, bsrw] }
  src: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
  dst: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
evidence: system.rs:352; Intel SDM BSF/BSR r16,r/m16; llvm-mc -triple=i686
```

## encode_bsr_bsf_16_diff_mem
- Tier: 5
- Rationale: Differential vs llvm-mc for memory source forms (base/disp/SIB/abs, no segment).
- Doc contract: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw" — asserted fingerprint a7c3e91f
- Seed: (none)
- Formal: ∀ m ∈ {bsfw,bsrw}, ∀ mem ∈ MemForms(no-seg), ∀ d ∈ R16. encode(m, mem, %d) = llvm-mc(m mem, %d)
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf_16
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, mem, dst]
  domain: { mnemonic: bsfw_or_bsrw, mem: base_disp_sib_abs, dst: R16 }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Mem(mem), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"{mnemonic} {att_mem}, %{dst}\"))"
generators:
  mnemonic: { gen: oneof, options: [bsfw, bsrw] }
  dst: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
  mem_kind: { gen: int, min: 0, max: 5 }
evidence: system.rs:371-375; llvm-mc -triple=i686
```

## encode_bsr_bsf_16_diff_mem_segment
- Tier: 5
- Rationale: Differential vs llvm-mc for memory with segment override. Segment prefix must precede 0x66.
- Doc contract: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw" — asserted fingerprint a7c3e91f
- Seed: (none)
- Formal: ∀ m ∈ {bsfw,bsrw}, ∀ seg ∈ SREG, ∀ base ∈ R32, ∀ d ∈ R16. encode(m, %seg:(%base), %d) = llvm-mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: failing
- Counterexample: bsfw %es:(%eax), %ax
- Bug report: bug_reports/encode_bsr_bsf_16_missing_segment_prefix.md

```property
function: encode_bsr_bsf_16
oracle: differential
predicate:
  quantifier: forall
  vars: [mnemonic, seg, base, dst]
  domain: { mnemonic: bsfw_or_bsrw, seg: SREG, base: R32, dst: R16 }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Mem(seg:base), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"{mnemonic} %{seg}:(%{base}), %{dst}\"))"
generators:
  mnemonic: { gen: oneof, options: [bsfw, bsrw] }
  seg: { gen: oneof, options: [es, cs, ss, ds, fs, gs] }
  base: { gen: oneof, options: [eax, ecx, edx, ebx, esp, ebp, esi, edi] }
  dst: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
evidence: system.rs:371-375; llvm-mc emits seg then 66 then 0F BC/BD
```

## encode_bsr_bsf_16_invariant_r16
- Tier: 4
- Rationale: Algebraic invariant from Intel encoding: bsfw/bsrw r16,r16 = [0x66, 0x0F, 0xBC|0xBD, modrm(3,dst,src)].
- Doc contract: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw" — asserted fingerprint a7c3e91f
- Seed: (none)
- Formal: ∀ m ∈ {bsfw,bsrw}, ∀ s,d ∈ R16. bytes = [0x66, 0x0F, opc(m), 0xC0 | (d<<3) | s]
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf_16
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mnemonic, src, dst]
  domain: { mnemonic: bsfw_or_bsrw, src: R16, dst: R16 }
  relation:
    op: eq
    lhs: "sut_encode(mnemonic, [Reg(src), Reg(dst)])"
    rhs: "[0x66, 0x0F, opc(mnemonic), modrm(3, dst_num, src_num)]"
generators:
  mnemonic: { gen: oneof, options: [bsfw, bsrw] }
  src: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
  dst: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
evidence: system.rs:358-368; Intel SDM Vol.2 BSF/BSR
```

## encode_bsr_bsf_16_metamorphic_vs_32
- Tier: 4
- Rationale: Metamorphic — 16-bit form is operand-size override of 32-bit sibling encode_bsr_bsf.
- Doc contract: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw" — asserted fingerprint a7c3e91f
- Seed: (none)
- Formal: ∀ pair (r16,r32) ∈ R16_R32², ∀ (m16,m32) ∈ {(bsfw,bsfl),(bsrw,bsrl)}. encode(m16,r16s,r16d) = [0x66] ‖ encode(m32,r32s,r32d)
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf_16
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [src_pair, dst_pair, m_pair]
  domain: { pairs: R16_R32, m_pair: bsfw_bsfl_or_bsrw_bsrl }
  relation:
    op: eq
    lhs: "sut_encode(m16, [Reg(s16), Reg(d16)])"
    rhs: "[0x66] ++ sut_encode(m32, [Reg(s32), Reg(d32)])"
generators:
  src_pair: { gen: oneof, options: [[ax, eax], [cx, ecx], [dx, edx], [bx, ebx], [sp, esp], [bp, ebp], [si, esi], [di, edi]] }
  dst_pair: { gen: oneof, options: [[ax, eax], [cx, ecx], [dx, edx], [bx, ebx], [sp, esp], [bp, ebp], [si, esi], [di, edi]] }
  m_pair: { gen: oneof, options: [[bsfw, bsfl], [bsrw, bsrl]] }
evidence: system.rs:362 vs gp_integer.rs:984; Intel operand-size override
```

## encode_bsr_bsf_16_neg_arity
- Tier: 3
- Rationale: Negative/error — ops.len() != 2 must Err.
- Doc contract: system.rs:354-356 "if ops.len() != 2 { return Err(...) }" — asserted fingerprint c4f1a002
- Seed: (none)
- Formal: ∀ m ∈ {bsfw,bsrw}, ∀ n ≠ 2. encode(m, ops_n) = Err
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf_16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, n]
  domain: { mnemonic: bsfw_or_bsrw, n: arity_not_2 }
  relation:
    op: holds
    expr: sut_encode(mnemonic, ops_of_len_n).is_err()
generators:
  mnemonic: { gen: oneof, options: [bsfw, bsrw] }
  n: { gen: int, min: 0, max: 3 }
expected_error: String
evidence: system.rs:354-356
```

## encode_bsr_bsf_16_neg_wrong_width
- Tier: 4
- Rationale: Negative/error — r32/r8 source or dest invalid for bsfw/bsrw (llvm-mc rejects). SUT uses reg_num which aliases widths.
- Doc contract: system.rs:352 "Encode 16-bit BSF/BSR: bsfw/bsrw" — asserted fingerprint a7c3e91f (16-bit contract)
- Seed: (none)
- Formal: ∀ m ∈ {bsfw,bsrw}, ∀ bad ∈ R32∪R8, ∀ good ∈ R16. encode(m, bad, good) = Err ∧ encode(m, good, bad) = Err
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: failing
- Counterexample: bsfw %ax, %eax
- Bug report: bug_reports/encode_bsr_bsf_16_wrong_width_accepted.md

```property
function: encode_bsr_bsf_16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnemonic, bad, good, side]
  domain: { mnemonic: bsfw_or_bsrw, bad: R32_or_R8, good: R16, side: src_or_dst }
  relation:
    op: holds
    expr: llvm_mc_rejects(asm) && sut_encode(mnemonic, ops).is_err()
generators:
  mnemonic: { gen: oneof, options: [bsfw, bsrw] }
  bad: { gen: oneof, options: [eax, ecx, edx, ebx, esp, ebp, esi, edi, al, cl, dl, bl, ah, ch, dh, bh] }
  good: { gen: oneof, options: [ax, cx, dx, bx, sp, bp, si, di] }
  side: { gen: oneof, options: [src, dst] }
expected_error: String
evidence: llvm-mc rejects bsfw %eax,%bx / bsfw %al,%bx / bsrw %ax,%ebx; Intel r16,r/m16
```

## encode_bsr_bsf_16_neg_unsupported_ops
- Tier: 3
- Rationale: Retired — catch-all arm at system.rs:376 is a documented domain restriction (`unsupported {} operands`); the property only restated that Err path and added no independent contract beyond arity/width negatives already present. Kept as regression coverage in the test file but not as an active ledger claim.
- Doc contract: system.rs:376 `_ => Err(format!("unsupported {} operands", mnemonic))` — domain-restriction fingerprint d8e2b110
- Seed: (none)
- Formal: (retired)
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs
- Status: retired
- Counterexample: (none)
- Bug report: (none)
