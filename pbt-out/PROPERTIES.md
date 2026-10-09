# Properties: encode_mov_rr (i686)

## encode_mov_rr_diff_same_width_gp
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine N/A (pure encoder). Round-trip N/A (no i686 MOV decoder). Same-width GP pairs under movb/movw/movl must match llvm-mc bytes exactly (Intel 88/89 /r + optional 0x66).
- Doc contract: gp_integer.rs:163 "// Handle segment register moves" — other fingerprint 28e3197a (segment arms only; GP path has no doc). Contract for GP RR inferred from Intel SDM MOV + AT&T suffix size + llvm-mc.
- Seed: encode_mov_infer_size_pbt.rs:175
- Formal: ∀ src,dst ∈ GP_w, w ∈ {1,2,4}. encode(suffix(w), src, dst) = llvm-mc(suffix(w) %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_rr
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst, width]
  domain: { src: gp_reg(width), dst: gp_reg(width), width: {1,2,4} }
  relation:
    op: eq
    lhs: "sut_encode(suffix(width), [Reg(src), Reg(dst)])"
    rhs: "llvm_mc(suffix(width) + ' %' + src + ', %' + dst)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  src: { gen: string, type: "&str" }
  dst: { gen: string, type: "&str" }
evidence: gp_integer.rs:162-192; Intel SDM MOV r/m,r; llvm-mc -triple=i686
```

## encode_mov_rr_invariant_opcode_modrm
- Tier: 3
- Rationale: Algebraic invariant from Intel encoding: bytes are [0x66?] ++ [0x88|0x89] ++ modrm(mod=3, reg=src_num, rm=dst_num). Rejects stronger differential as complementary check.
- Doc contract: gp_integer.rs:183-191 body — asserted fingerprint 28e3197a (size==2 → 0x66; size==1 → 0x88 else 0x89; modrm(3,src,dst))
- Seed: encode_mov_cr_pbt.rs:255
- Formal: ∀ src,dst ∈ GP_w, w ∈ {1,2,4}. let b = encode(...). (w=2 ⇒ b[0]=0x66) ∧ opcode∈{0x88,0x89} ∧ (b.last & 0xC0)=0xC0 ∧ reg_field=src_num ∧ rm_field=dst_num
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_rr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [src, dst, width]
  domain: { src: gp_reg(width), dst: gp_reg(width), width: {1,2,4} }
  body: "bytes match 0x66? + (0x88 if w=1 else 0x89) + modrm(3, src_num, dst_num)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: gp_integer.rs:183-191
```

## encode_mov_rr_metamorphic_identity
- Tier: 3
- Rationale: Metamorphic — MOV same register to itself is well-formed and equals llvm-mc; also src≠dst vs identity share opcode/prefix shape.
- Doc contract: (none on GP identity) — other fingerprint 28e3197a
- Seed: (none)
- Formal: ∀ r ∈ GP_w, w ∈ {1,2,4}. encode(suffix(w), r, r) = llvm-mc(suffix(w) %r, %r)
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_rr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r, width]
  domain: { r: gp_reg(width), width: {1,2,4} }
  relation:
    op: eq
    lhs: "sut_encode(suffix(width), [Reg(r), Reg(r)])"
    rhs: "llvm_mc(...)"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
evidence: Intel SDM MOV; llvm-mc
```

## encode_mov_rr_neg_mismatched_width
- Tier: 4
- Rationale: Negative/error — mnemonic size must match both register widths. llvm-mc rejects mismatched pairs; SUT must Err (not silently alias via reg_num). Same defect class as encode_mov_cr width acceptance.
- Doc contract: (none — no exclusion of mismatched width) fingerprint 28e3197a. Contract inferred: suffix size = reg_size(src) = reg_size(dst).
- Seed: encode_mov_cr_pbt.rs regression rejects_ax
- Formal: ∀ src ∈ GP_a, dst ∈ GP_b, mnemonic size s. (reg_size(src)≠s ∨ reg_size(dst)≠s) ∧ llvm-mc rejects ⇒ encode returns Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: failing
- Counterexample: movl %ax, %ebx → Ok([0x89, 0xc3])
- Bug report: pbt-out/bug_reports/encode_mov_rr_mismatched_width.md

```property
function: encode_mov_rr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [src, dst, mnemonic]
  domain: { mismatched width pairs where llvm-mc rejects }
  body: "sut_encode returns Err"
generators:
  src: { gen: string }
  dst: { gen: string }
expected_error: String
evidence: Intel SDM MOV same-size operands; llvm-mc rejects; reg_num aliases widths
```

## encode_mov_rr_neg_non_gp
- Tier: 4
- Rationale: Non-GP names that still have reg_num (xmm/mm/st) must not be accepted as GP MOV RR under movb/movw/movl. llvm-mc rejects; SUT must Err.
- Doc contract: (none) fingerprint 28e3197a
- Seed: encode_mov_cr_pbt non-r32 rejection
- Formal: ∀ r ∈ {xmm*, mm*, st*}, gp ∈ GP_w, m ∈ {movb,movw,movl}. llvm-mc rejects(m %r,%gp) ⇒ encode(m,r,gp)=Err ∧ encode(m,gp,r)=Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: failing
- Counterexample: movl %xmm0, %eax → Ok([0x89, 0xc0])
- Bug report: pbt-out/bug_reports/encode_mov_rr_non_gp.md

```property
function: encode_mov_rr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [non_gp, gp, mnemonic]
  domain: { non_gp: xmm|mm|st, gp: gp_reg(width), mnemonic: movb|movw|movl }
  body: "llvm_mc_rejects(asm) => sut_encode(mnemonic, [Reg(non_gp), Reg(gp)]).is_err()"
generators:
  non_gp: { gen: oneof, values: ["xmm0", "mm0", "st(0)"], type: "&str" }
  gp: { gen: oneof, values: ["eax", "ax", "al"], type: "&str" }
  mnemonic: { gen: oneof, values: ["movb", "movw", "movl"], type: "&str" }
expected_error: String
evidence: registers.rs:4-15 reg_num aliases xmm/mm/st; Intel MOV GP-only for 88/89
```

## encode_mov_rr_diff_ah_high_byte
- Tier: 4
- Rationale: Differential edge — high-byte registers ah/ch/dh/bh use reg numbers 4-7 under 0x88; must match llvm-mc (distinct from sp/bp/si/di aliasing in reg_num for other sizes).
- Doc contract: (none) fingerprint 28e3197a
- Seed: (none)
- Formal: ∀ src,dst ∈ R8. encode(movb, src, dst) = llvm-mc(movb %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_rr
oracle: differential
predicate:
  quantifier: forall
  vars: [src, dst]
  domain: { src: R8, dst: R8 }
  relation:
    op: eq
    lhs: "sut_encode(\"movb\", [Reg(src), Reg(dst)])"
    rhs: "llvm_mc(\"movb %\" + src + \", %\" + dst)"
generators:
  src: { gen: oneof, values: ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"], type: "&str" }
  dst: { gen: oneof, values: ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"], type: "&str" }
evidence: registers.rs ah=4; Intel 88 /r
```

## encode_mov_rr_metamorphic_commute_modrm
- Tier: 3
- Rationale: Metamorphic — swapping src/dst swaps ModRM.reg and ModRM.rm while keeping opcode/prefix; both directions match llvm-mc.
- Doc contract: gp_integer.rs:191 modrm(3, src_num, dst_num) — asserted fingerprint 28e3197a
- Seed: (none)
- Formal: ∀ a≠b ∈ GP_w. let f=encode(a,b), g=encode(b,a). opcode(f)=opcode(g) ∧ prefix(f)=prefix(g) ∧ modrm_reg(f)=modrm_rm(g) ∧ modrm_rm(f)=modrm_reg(g)
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mov_rr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [a, b, width]
  domain: { a: gp_reg(width), b: gp_reg(width), a != b, width: {1,2,4} }
  body: "modrm_reg(encode(a,b)) == modrm_rm(encode(b,a)) && opcode same"
generators:
  width: { gen: oneof, values: [1, 2, 4], type: u8 }
  a: { gen: string, type: "&str" }
  b: { gen: string, type: "&str" }
evidence: gp_integer.rs:191
```

## encode_mov_rr_neg_both_wrong_width
- Tier: 4
- Rationale: Strengthen round — both operands wrong width for the suffix (e.g. movl %al,%al). Same root-cause path as mismatched_width; kept failing with its own witness/report.
- Doc contract: (none) fingerprint 28e3197a
- Seed: (none)
- Formal: ∀ src,dst with reg_size ≠ mnemonic size. llvm-mc rejects ⇒ encode returns Err
- Test file: src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs
- Status: failing
- Counterexample: movl %al, %al → Ok([0x89, 0xc0])
- Bug report: pbt-out/bug_reports/encode_mov_rr_both_wrong_width.md

```property
function: encode_mov_rr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [src, dst, mnemonic]
  body: "both wrong width => Err"
generators:
  pair: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: same as encode_mov_rr_neg_mismatched_width
```
