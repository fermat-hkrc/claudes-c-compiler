# Properties: encode_bswap (i686)

## encode_bswap_diff_r32_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. State machine N/A (pure encoder). Round-trip N/A (no in-tree i686 BSWAP decoder). Intel SDM Vol.2 BSWAP r32 = 0F C8+rd; dispatch mod.rs:262 `bswapl|bswap`.
- Doc contract: (none) — function has no doc comment at gp_integer.rs:945
- Seed: (none) — no prior unit test for encode_bswap
- Formal: ∀ r ∈ {eax,ecx,edx,ebx,esp,ebp,esi,edi}, m ∈ {bswapl,bswap}. encode(m, [%r]) = llvm-mc(m %r)
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bswap
oracle: differential
predicate:
  quantifier: forall
  vars: [r, m]
  domain: { r: gp32, m: {bswapl, bswap} }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(r)])"
    rhs: "llvm_mc_bytes(format!(\"{m} %{r}\"))"
generators:
  r: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
  m: { gen: oneof, values: [bswapl, bswap], type: "&str" }
evidence: "mod.rs:262 bswapl|bswap => encode_bswap; Intel SDM BSWAP 0F C8+rd; llvm-mc -triple=i686"
```

## encode_bswap_invariant_opcode
- Tier: 4
- Rationale: Algebraic invariant from Intel SDM — valid r32 encoding is exactly [0x0F, 0xC8+n]. Differential covers agreement; invariant pins the structural form.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ r ∈ GP32. encode(bswapl, [%r]) = [0x0F, 0xC8 + reg_num(r)]
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bswap
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"bswapl\", [Reg(r)])"
    rhs: "[0x0Fu8, 0xC8 + reg_num(r)]"
generators:
  r: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
evidence: "Intel SDM Vol.2 BSWAP — Opcode 0F C8+rd; gp_integer.rs:952"
```

## encode_bswap_meta_bswap_eq_bswapl
- Tier: 4
- Rationale: Metamorphic — dispatch aliases bswap and bswapl to the same helper; encodings must be identical for the same r32.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ r ∈ GP32. encode(bswap, [%r]) = encode(bswapl, [%r])
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bswap
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"bswap\", [Reg(r)])"
    rhs: "sut_encode(\"bswapl\", [Reg(r)])"
generators:
  r: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: "&str" }
evidence: "mod.rs:262 \"bswapl\" | \"bswap\" => self.encode_bswap(ops)"
```

## encode_bswap_neg_arity
- Tier: 3
- Rationale: Negative/error — arity must be exactly 1 (gp_integer.rs:946-948).
- Doc contract: gp_integer.rs:947 "bswap requires 1 operand" — asserted fingerprint 7a3c91e2
- Seed: (none)
- Formal: ∀ ops. |ops| ≠ 1 ⇒ encode_bswap(ops) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bswap
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: arity_neq_1 }
  relation:
    op: holds
    expr: "sut_encode(\"bswapl\", ops_len(n)).is_err()"
generators:
  n: { gen: int, min: 0, max: 4, type: "usize" }
expected_error: "bswap requires 1 operand"
evidence: "gp_integer.rs:946-948"
```

## encode_bswap_neg_non_register
- Tier: 3
- Rationale: Negative/error — only Register operand accepted (gp_integer.rs:955).
- Doc contract: gp_integer.rs:955 "bswap requires register operand" — asserted fingerprint 9f2b44aa
- Seed: (none)
- Formal: ∀ op ∈ {Mem, Imm, Label}. encode(bswapl, [op]) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bswap
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: non_register }
  relation:
    op: holds
    expr: "sut_encode(\"bswapl\", [op]).is_err()"
generators:
  op: { gen: oneof, values: [mem, imm, label], type: "Operand" }
expected_error: "bswap requires register operand"
evidence: "gp_integer.rs:955; Intel SDM BSWAP r32 only"
```

## encode_bswap_neg_wrong_width
- Tier: 3
- Rationale: Negative/error — Intel BSWAP is undefined/invalid for 16-bit and 8-bit operands; llvm-mc rejects. SUT uses bare reg_num which aliases r16/r8 → same encoding as r32 (same defect class as encode_mov_cr / encode_lmsw).
- Doc contract: (none) — no width gate in function body
- Seed: encode_mov_cr_pbt.rs (r16/r8 rejection pattern)
- Formal: ∀ r ∈ R16∪R8, m ∈ {bswapl,bswap}. encode(m, [%r]) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: failing
- Counterexample: bswapl %ax → Ok([0x0f, 0xc8]) (also %al → [0x0f, 0xc8])
- Bug report: pbt-out/bug_reports/encode_bswap_wrong_width.md

```property
function: encode_bswap
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r, m]
  domain: { r: r16_or_r8, m: {bswapl, bswap} }
  relation:
    op: holds
    expr: "sut_encode(m, [Reg(r)]).is_err()"
generators:
  r: { gen: oneof, values: [ax, cx, dx, bx, sp, bp, si, di, al, cl, dl, bl, ah, ch, dh, bh], type: "&str" }
  m: { gen: oneof, values: [bswapl, bswap], type: "&str" }
expected_error: "width / bad register"
evidence: "Intel SDM Vol.2 BSWAP — not defined for 16-bit; llvm-mc rejects bswapl %ax / %al"
```

## encode_bswap_neg_non_gp
- Tier: 3
- Rationale: Negative/error — non-GP names that reg_num aliases (xmm/mm/st/ymm) must not encode as BSWAP r32.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ r ∈ {xmm*,mm*,st*,ymm*}. encode(bswapl, [%r]) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: failing
- Counterexample: bswapl %xmm0 → Ok([0x0f, 0xc8])
- Bug report: pbt-out/bug_reports/encode_bswap_non_gp.md

```property
function: encode_bswap
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: non_gp_aliased }
  relation:
    op: holds
    expr: "sut_encode(\"bswapl\", [Reg(r)]).is_err()"
generators:
  r: { gen: oneof, values: [xmm0, xmm1, xmm7, mm0, mm3, st, st(0), st(1), ymm0, ymm7], type: "&str" }
expected_error: "bad register / non-GP"
evidence: "Intel SDM BSWAP r32; llvm-mc rejects bswapl %xmm0; registers.rs:4-15 aliases xmm/mm/st"
```

## encode_bswap_neg_unknown_reg
- Tier: 3
- Rationale: Strengthen round — sreg/cr are unknown to reg_num and already Err("bad register"); lock that path so a future reg_num expansion does not silently accept them.
- Doc contract: (none)
- Seed: (none)
- Formal: ∀ r ∈ {es,cs,ss,ds,fs,gs,cr0,cr2,cr3,cr4}. encode(bswapl, [%r]) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_bswap_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bswap
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: sreg_or_cr }
  relation:
    op: holds
    expr: "sut_encode(\"bswapl\", [Reg(r)]).is_err()"
generators:
  r: { gen: oneof, values: [es, cs, ss, ds, fs, gs, cr0, cr2, cr3, cr4], type: "&str" }
expected_error: "bad register"
evidence: "registers.rs:4-15 omits sreg/cr from reg_num; gp_integer.rs:951 ok_or(\"bad register\")"
```
