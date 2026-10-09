# Properties: encode_inc_dec (i686)

## encode_inc_dec_diff_reg32
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc i686 for GP r32 INC/DEC compact form. State machine N/A; no decoder for round-trip.
- Doc contract: gp_integer.rs:801-805 "In 32-bit mode, inc/dec have compact single-byte encodings for 32-bit registers: inc: 0x40+reg, dec: 0x48+reg" — asserted fingerprint a3f1c802
- Seed: (none — no prior unit test for encode_inc_dec)
- Formal: ∀ r ∈ GP32, op ∈ {incl,decl}. encode(op, r) = llvm_mc(op %r)
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_inc_dec
oracle: differential
predicate:
  quantifier: forall
  vars: [r, op]
  domain: { r: GP32, op: {incl, decl} }
  relation:
    op: eq
    lhs: sut_encode(op, r)
    rhs: llvm_mc(op + " %" + r)
generators:
  r: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
  op: { gen: oneof, values: [incl, decl], type: "&str" }
evidence: gp_integer.rs:801-817; Intel SDM INC/DEC r32; llvm-mc i686
```

## encode_inc_dec_diff_reg16
- Tier: 4
- Rationale: Word form must emit 0x66 + compact 0x40/0x48 per doc.
- Doc contract: gp_integer.rs:818-821 "16-bit: operand size prefix + 0x40+reg (inc) or 0x48+reg (dec)" — asserted fingerprint b7e2d914
- Seed: (none)
- Formal: ∀ r ∈ GP16, op ∈ {incw,decw}. encode(op, r) = llvm_mc(op %r)
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_inc_dec
oracle: differential
predicate:
  quantifier: forall
  vars: [r, op]
  domain: { r: GP16, op: {incw, decw} }
  relation:
    op: eq
    lhs: sut_encode(op, r)
    rhs: llvm_mc(op + " %" + r)
generators:
  r: { gen: oneof, values: [ax, cx, dx, bx, sp, bp, si, di], type: "&str" }
  op: { gen: oneof, values: [incw, decw], type: "&str" }
evidence: gp_integer.rs:818-821
```

## encode_inc_dec_diff_reg8
- Tier: 4
- Rationale: Byte form uses FE /0|/1 + modrm, not compact 40-form.
- Doc contract: gp_integer.rs:822-826 "8-bit: use 0xFE /0 (inc) or 0xFE /1 (dec) with modrm" — asserted fingerprint c8d3e025
- Seed: (none)
- Formal: ∀ r ∈ GP8, op ∈ {incb,decb}. encode(op, r) = llvm_mc(op %r)
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_inc_dec
oracle: differential
predicate:
  quantifier: forall
  vars: [r, op]
  domain: { r: GP8, op: {incb, decb} }
  relation:
    op: eq
    lhs: sut_encode(op, r)
    rhs: llvm_mc(op + " %" + r)
generators:
  r: { gen: oneof, values: [al, cl, dl, bl, ah, ch, dh, bh], type: "&str" }
  op: { gen: oneof, values: [incb, decb], type: "&str" }
evidence: gp_integer.rs:822-826
```

## encode_inc_dec_diff_mem
- Tier: 4
- Rationale: Memory INC/DEC across sizes must match llvm-mc (FE/FF /ext + modrm/sib/disp; 0x66 for word).
- Doc contract: gp_integer.rs:803-805 "For memory operands or byte/word sizes, use opcode 0xFE (byte) / 0xFF (word/dword) with modrm extension /0 (inc) or /1 (dec)" — asserted fingerprint d9e4f136
- Seed: (none)
- Formal: ∀ mem bare, size∈{b,w,l}, op∈{inc,dec}. encode(op+suf, mem) = llvm_mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_inc_dec
oracle: differential
predicate:
  quantifier: forall
  vars: [mem, mnem]
  domain: { mem: bare_mem_forms, mnem: {incl,incw,incb,decl,decw,decb} }
  relation:
    op: eq
    lhs: sut_encode(mnem, mem)
    rhs: llvm_mc(mnem + " " + att(mem))
generators:
  mem: { gen: custom, type: MemoryOperand }
  mnem: { gen: oneof, values: [incl, incw, incb, decl, decw, decb], type: "&str" }
evidence: gp_integer.rs:829-833
```

## encode_inc_dec_diff_mem_segment
- Tier: 5
- Rationale: Differential + documented emit_segment_prefix contract for all six overrides. Prior campaigns found this missing on sibling encoders.
- Doc contract: core.rs:31-42 emit_segment_prefix for es/cs/ss/ds/fs/gs — domain-restriction fingerprint e0f5a247
- Seed: encode_pop_pbt.rs / encode_push_pbt.rs segment differentials
- Formal: ∀ seg ∈ SREGS, base ∈ GP32, op ∈ {incl,decl}. encode(op, seg:(base)) = llvm_mc(...) ∧ starts_with(seg_prefix(seg))
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: failing
- Counterexample: incl %es:(%eax) → sut=[ff,00] mc=[26,ff,00]
- Bug report: pbt-out/bug_reports/encode_inc_dec_missing_segment_prefix.md

```property
function: encode_inc_dec
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, op]
  domain: { seg: SREGS, base: GP32, op: {incl, decl} }
  relation:
    op: eq
    lhs: sut_encode(op, seg:(base))
    rhs: llvm_mc(op + " %" + seg + ":(%" + base + ")")
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: "&str" }
  base: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
  op: { gen: oneof, values: [incl, decl], type: "&str" }
evidence: core.rs:31-42; Intel SDM 2.1.1 segment override prefixes
```

## encode_inc_dec_meta_segment
- Tier: 4
- Rationale: Metamorphic — segmented encoding must be prefix ‖ bare body (required metamorphic/differential at standard tier).
- Doc contract: core.rs:31-42 — asserted fingerprint e0f5a247
- Seed: encode_push_meta_segment_stripped_eq_bare
- Formal: ∀ seg, mem_bare. encode(seg:mem)[0]=seg_prefix(seg) ∧ strip_seg(encode(seg:mem)) body-relates bare
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: failing
- Counterexample: incl %es:(%eax) → got [ff,00], expected start 0x26
- Bug report: pbt-out/bug_reports/encode_inc_dec_missing_segment_prefix.md

```property
function: encode_inc_dec
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [seg, base]
  domain: { seg: SREGS, base: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(incl, seg:(base))[0]"
    rhs: "seg_prefix_byte(seg)"
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: "&str" }
  base: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
evidence: core.rs:31-42
```

## encode_inc_dec_invariant_compact
- Tier: 3
- Rationale: Algebraic invariant from doc — r32 INC = [0x40+n], DEC = [0x48+n].
- Doc contract: gp_integer.rs:801-805 — asserted fingerprint a3f1c802
- Seed: (none)
- Formal: ∀ r ∈ GP32. encode(incl,r) = [0x40+reg_num(r)] ∧ encode(decl,r) = [0x48+reg_num(r)]
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_inc_dec
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(incl, r)"
    rhs: "[0x40 + reg_num(r)]"
generators:
  r: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
evidence: gp_integer.rs:814-817
```

## encode_inc_dec_neg_arity
- Tier: 3
- Rationale: Arity ≠ 1 must Err per doc.
- Doc contract: gp_integer.rs:807-808 "inc/dec requires 1 operand" — asserted fingerprint f1a6b358
- Seed: encode_push_neg_arity
- Formal: ∀ n≠1. encode(ops_n)=Err
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_inc_dec
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: arity_ne_1 }
  relation:
    op: holds
    expr: "sut_encode_arity(n).is_err()"
expected_error: String
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
evidence: gp_integer.rs:807-808
```

## encode_inc_dec_neg_xmm
- Tier: 3
- Rationale: Non-GP xmm must be rejected (llvm-mc rejects; reg_num alias would silently accept).
- Doc contract: (none on function — inferred from Intel SDM INC r/m + llvm-mc) fingerprint 00000000
- Seed: encode_push_neg_xmm
- Formal: ∀ x∈XMM, op∈{incl,decl}. encode(op,x)=Err
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: failing
- Counterexample: incl %xmm0 → Ok([0x40])
- Bug report: pbt-out/bug_reports/encode_inc_dec_accepts_non_gp.md

```property
function: encode_inc_dec
oracle: negative_error
predicate:
  quantifier: forall
  vars: [x]
  domain: { x: XMM }
  relation:
    op: holds
    expr: "sut_encode(\"incl\", x).is_err()"
expected_error: String
generators:
  x: { gen: oneof, values: [xmm0, xmm1, xmm7], type: "&str" }
evidence: registers.rs:4-15 reg_num aliases xmm; llvm-mc rejects
```

## encode_inc_dec_neg_mismatched_width
- Tier: 3
- Rationale: Mnemonic size must match register width; llvm-mc rejects incl %ax.
- Doc contract: (none explicit — inferred from AT&T size suffixes + llvm-mc) fingerprint 00000000
- Seed: (none)
- Formal: ∀ mismatched (mnem,reg). encode=Err
- Test file: src/backend/i686/assembler/encoder/encode_inc_dec_pbt.rs
- Status: failing
- Counterexample: incl %ax → Ok([0x40])
- Bug report: pbt-out/bug_reports/encode_inc_dec_mismatched_width.md

```property
function: encode_inc_dec
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, reg]
  domain: { mismatched_width_pairs }
  relation:
    op: holds
    expr: "sut_encode(mnem, reg).is_err()"
expected_error: String
generators:
  mnem: { gen: oneof, values: [incl, incw, incb], type: "&str" }
  reg: { gen: oneof, values: [ax, al, eax], type: "&str" }
evidence: llvm-mc i686 rejects width-mismatched forms; reg_size in registers.rs:63-70
```
