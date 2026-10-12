# Properties: encode_bsr_bsf (i686)

## encode_bsr_bsf_diff_rr
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. Round-trip rejected (no decoder). State machine rejected (pure encode).
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_diff_r16 (generalized to 32-bit forms)
- Formal: ∀ m ∈ {bsfl,bsrl,bsf,bsr}, s,d ∈ GP32. encode_bsr_bsf(m,[Reg(s),Reg(d)]) = llvm-mc("m %s, %d")
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf
oracle: differential
predicate:
  quantifier: forall
  vars: [m, s, d]
  domain: { m: {bsfl,bsrl,bsf,bsr}, s: GP32, d: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(s), Reg(d)])"
    rhs: "llvm_mc_bytes(format!(\"{m} %{s}, %{d}\"))"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl", "bsf", "bsr"] }
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: mod.rs:270; Intel SDM Vol.2 BSF/BSR r32,r/m32; llvm-mc -triple=i686
```

## encode_bsr_bsf_diff_mem
- Tier: 5
- Rationale: Memory source is architecturally valid (r/m32). Differential vs llvm-mc over base/disp/SIB/abs forms without segment.
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_diff_mem
- Formal: ∀ m ∈ {bsfl,bsrl}, d ∈ GP32, mem ∈ Mem32_no_seg. encode_bsr_bsf(m,[Mem(mem),Reg(d)]) = llvm-mc("m att(mem), %d")
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf
oracle: differential
predicate:
  quantifier: forall
  vars: [m, d, mem]
  domain: { m: {bsfl,bsrl}, d: GP32, mem: base/disp/SIB/abs no-segment }
  relation:
    op: eq
    lhs: "sut_encode(m, [Mem(mem), Reg(d)])"
    rhs: "llvm_mc_bytes(...)"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  mem: { gen: custom, note: "kind 0..6 base/index/scale/disp" }
evidence: gp_integer.rs:1003-1007 Memory arm; Intel SDM BSF r32, r/m32
```

## encode_bsr_bsf_diff_mem_segment
- Tier: 5
- Rationale: Segment override must precede opcode (llvm-mc emits 26/2E/36/3E/64/65). Sibling encode_bsr_bsf_16 omitted it — same pattern expected here.
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_diff_mem_segment
- Formal: ∀ m ∈ {bsfl,bsrl}, seg ∈ Sreg, base ∈ GP32, d ∈ GP32, disp ∈ {0,4,8}. encode_bsr_bsf(m,[Mem(seg:disp(base)),Reg(d)]) = llvm-mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: failing
- Counterexample: bsfl %es:(%eax), %eax → SUT [0f,bc,00] vs llvm-mc [26,0f,bc,00]
- Bug report: bug_reports/encode_bsr_bsf_missing_segment_prefix.md

```property
function: encode_bsr_bsf
oracle: differential
predicate:
  quantifier: forall
  vars: [m, seg, base, d, disp]
  domain: { m: {bsfl,bsrl}, seg: Sreg, base: GP32, d: GP32, disp: {0,4,8} }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl"] }
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  disp: { gen: oneof, values: [0, 4, 8] }
evidence: core.rs:31 emit_segment_prefix; llvm-mc bsfl %es:(%eax), %ebx → [26,0f,bc,18]
```

## encode_bsr_bsf_invariant_rr
- Tier: 4
- Rationale: Algebraic invariant — fixed byte layout 0F opc ModRM(3,dst,src) independent of llvm-mc process.
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_invariant_r16
- Formal: ∀ m ∈ {bsfl,bsrl,bsf,bsr}, s,d ∈ GP32. encode_bsr_bsf(m,[Reg(s),Reg(d)]) = [0x0F, opc(m), 0xC0|(reg(d)<<3)|reg(s)]
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [m, s, d]
  domain: { m: {bsfl,bsrl,bsf,bsr}, s: GP32, d: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(s), Reg(d)])"
    rhs: "[0x0F, opc(m), modrm(3, reg(d), reg(s))]"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl", "bsf", "bsr"] }
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:990-1001; Intel SDM opcode map
```

## encode_bsr_bsf_metamorphic_bare_vs_l
- Tier: 4
- Rationale: Metamorphic — bare bsf/bsr with r32 must equal suffixed bsfl/bsrl (llvm-mc normalizes bare to *l for r32). Required metamorphic/differential at standard tier.
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: (none)
- Formal: ∀ bare ∈ {bsf,bsr}, s,d ∈ GP32. encode(bare,[s,d]) = encode(bare+"l",[s,d])
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [bare, s, d]
  domain: { bare: {bsf,bsr}, s: GP32, d: GP32 }
  relation:
    op: eq
    lhs: "sut_encode(bare, [Reg(s), Reg(d)])"
    rhs: "sut_encode(bare+\"l\", [Reg(s), Reg(d)])"
generators:
  bare: { gen: oneof, values: ["bsf", "bsr"] }
  s: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: llvm-mc bsf %eax,%ebx ≡ bsfl; mod.rs:270 routes both to encode_bsr_bsf
```

## encode_bsr_bsf_neg_arity
- Tier: 3
- Rationale: Negative/error — ops.len()!=2 must Err (documented by code guard and llvm-mc).
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_neg_arity
- Formal: ∀ m ∈ MNEMS, n ≠ 2. encode_bsr_bsf(m, ops_n) = Err
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, n]
  domain: { m: MNEMS, n: arity_not_2 }
  relation:
    op: holds
    expr: "sut_encode(m, ops_of_len_n).is_err()"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl", "bsf", "bsr"] }
  n: { gen: int, min: 0, max: 3 }
expected_error: String
evidence: gp_integer.rs:985-987
```

## encode_bsr_bsf_neg_wrong_width
- Tier: 4
- Rationale: Negative — r16/r8 operands invalid for bsfl/bsrl (llvm-mc rejects). SUT uses reg_num which aliases widths — expected defect class from sibling campaigns.
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_neg_wrong_width; encode_bit_count_pbt.rs
- Formal: ∀ m ∈ {bsfl,bsrl}, bad ∈ R16∪R8, good ∈ GP32. llvm-mc rejects ∧ SUT must Err
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: failing
- Counterexample: bsfl %eax, %ax → Ok([0f, bc, c0]) (should Err)
- Bug report: bug_reports/encode_bsr_bsf_wrong_width_accepted.md

```property
function: encode_bsr_bsf
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, bad, good, bad_is_src]
  domain: { m: {bsfl,bsrl}, bad: R16_or_R8, good: GP32 }
  relation:
    op: holds
    expr: "sut_encode(m, [Reg(src), Reg(dst)]).is_err()"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl"] }
  bad: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di","al","cl","dl","bl","ah","ch","dh","bh"] }
  good: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
expected_error: String
evidence: llvm-mc rejects bsfl %ax,%ebx; registers.rs:4 reg_num aliases; Intel SDM r32,r/m32
```

## encode_bsr_bsf_neg_unsupported
- Tier: 3
- Rationale: Negative — (mem,mem), (reg,mem), (imm,reg) must Err.
- Doc contract: gp_integer.rs:984 (none) — other fingerprint 552e4230
- Seed: encode_bsr_bsf_16_pbt.rs:encode_bsr_bsf_16_neg_unsupported_ops
- Formal: ∀ m, ops ∈ UnsupportedShapes. encode_bsr_bsf(m, ops) = Err
- Test file: src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bsr_bsf
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, kind]
  domain: { m: MNEMS, kind: unsupported_shapes }
  relation:
    op: holds
    expr: "sut_encode(m, ops).is_err()"
generators:
  m: { gen: oneof, values: ["bsfl", "bsrl", "bsf", "bsr"] }
  kind: { gen: int, min: 0, max: 2 }
expected_error: String
evidence: gp_integer.rs:1008 _ => Err unsupported
```
