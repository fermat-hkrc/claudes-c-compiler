# Properties: encode_push (i686)

## encode_push_diff_r32
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc i686 (Intel SDM PUSH r32 short form 50+rd). State machine N/A (pure encode). Round-trip N/A (no decoder). Reference KAT gate first.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations" — other fingerprint 65ffa4c5
- Seed: (none — no prior encode_push unit tests)
- Formal: ∀ r ∈ {eax,ecx,edx,ebx,esp,ebp,esi,edi}. encode_push([Reg(r)]) = llvm-mc("pushl %r")
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: gp_integer.encode_push
oracle: differential
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: gp32_regs }
  relation:
    op: eq
    lhs: "sut_encode(\"pushl\", [Reg(r)])"
    rhs: "llvm_mc_bytes(format!(\"pushl %{r}\"))"
generators:
  r: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: encoder/mod.rs:191 "pushl|push => encode_push"; Intel SDM PUSH 50+rd
```

## encode_push_diff_imm
- Tier: 5
- Rationale: Imm8 (6A ib) vs Imm32 (68 id) boundary at ±128 is a classic off-by-one; differential vs llvm-mc pins both forms.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations" — other fingerprint 65ffa4c5
- Seed: (none)
- Formal: ∀ v ∈ i32. encode_push([Imm(v)]) = llvm-mc("pushl $v")
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: gp_integer.encode_push
oracle: differential
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: i32_with_i8_boundaries }
  relation:
    op: eq
    lhs: "sut_encode(\"pushl\", [Imm(v)])"
    rhs: "llvm_mc_bytes(format!(\"pushl ${v}\"))"
generators:
  v: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:356-363 imm8 vs imm32 branch; Intel SDM 6A/68
```

## encode_push_diff_mem
- Tier: 5
- Rationale: Memory form FF /6 via encode_modrm_mem; ESP/EBP/SIB/abs edges historically break ModRM. Differential vs llvm-mc.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations" — other fingerprint 65ffa4c5
- Seed: (none)
- Formal: ∀ m ∈ valid_mem32. encode_push([Mem(m)]) = llvm-mc("pushl m") when m has no segment override
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: gp_integer.encode_push
oracle: differential
predicate:
  quantifier: forall
  vars: [m]
  domain: { m: i686_mem32_no_seg }
  relation:
    op: eq
    lhs: "sut_encode(\"pushl\", [Mem(m)])"
    rhs: "llvm_mc_bytes(format!(\"pushl {}\", att_mem(m)))"
generators:
  m: { gen: "mem32_no_seg", type: MemoryOperand }
evidence: gp_integer.rs:374-377 FF /6; Intel SDM PUSH r/m32
```

## encode_push_diff_mem_segment
- Tier: 5
- Rationale: Documented contract core.rs:31-42 emit_segment_prefix for all six segs; x86-64 sibling encode_push calls it before FF /6. i686 body does not. Differential must catch missing override.
- Doc contract: core.rs:31-42 emit_segment_prefix fs/gs/es/cs/ss/ds — asserted fingerprint a1b2c3d4
- Seed: encode_lea_pbt.rs segment differential
- Formal: ∀ seg ∈ {es,cs,ss,ds,fs,gs}, b ∈ GP32. encode_push([Mem(seg:b)]) = llvm-mc("pushl %seg:(%b)")
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0 → sut=[ff,30] mc=[26,ff,30]
- Bug report: bug_reports/encode_push_missing_segment_prefix.md

```property
function: gp_integer.encode_push
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base]
  domain: { seg: sregs, base: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"pushl\", [Mem(seg:base)])"
    rhs: "llvm_mc_bytes(format!(\"pushl %{}:(%{})\", seg, base))"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: core.rs:31-42; x86-64 gp_integer.rs:369 emit_segment_prefix; Intel SDM 2.1.1
```

## encode_push_invariant_r32_opcode
- Tier: 4
- Rationale: Algebraic invariant — short form is exactly one byte 0x50+n.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations" — other fingerprint 65ffa4c5
- Seed: (none)
- Formal: ∀ r ∈ GP32. encode_push([Reg(r)]) = [0x50 + reg_num(r)]
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: gp_integer.encode_push
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"pushl\", [Reg(r)])"
    rhs: "[0x50 + reg_num(r)]"
generators:
  r: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: Intel SDM PUSH 50+rd; gp_integer.rs:351-354
```

## encode_push_invariant_imm_form
- Tier: 4
- Rationale: Imm form choice is exact: |v|≤127 → 6A ib else 68 + i32 LE. Boundaries ±127/±128 must be hit.
- Doc contract: gp_integer.rs:3 "MOV, LEA, PUSH/POP, ALU, TEST, IMUL, shifts, bit operations" — other fingerprint 65ffa4c5
- Seed: (none)
- Formal: ∀ v ∈ i32. (v∈[-128,127] ⇒ encode=[0x6A, v as u8]) ∧ (v∉[-128,127] ⇒ encode=[0x68]‖le32(v as i32))
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: gp_integer.encode_push
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: i32 }
  body: "if v in [-128,127] then bytes=[0x6A,v as u8] else bytes=[0x68]++le32(v as i32)"
generators:
  v: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:356-363
```

## encode_push_meta_segment_stripped_eq_bare
- Tier: 4
- Rationale: Metamorphic — if segment prefix is correctly prepended, stripping seg prefixes from segmented encoding yields bare-mem encoding. Fails when prefix is omitted. Same root cause as encode_push_diff_mem_segment (B1); kept as reinforcing witness in the test file, ledger-retired to avoid duplicate bugId.
- Doc contract: core.rs:31-42 emit_segment_prefix — asserted
- Seed: encode_lea_pbt metamorphic patterns
- Formal: ∀ seg, m. strip_seg(encode_push(Mem(seg:m))) = encode_push(Mem(m)) ∧ encode_push(Mem(seg:m)) starts with seg_prefix(seg)
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: failing
- Counterexample: seg="es", base="eax", disp=0 → no 0x26 prefix, got [ff,30]
- Bug report: bug_reports/encode_push_meta_missing_segment_prefix.md
- Re-verified: cargo test --lib encode_push_meta_segment_stripped_eq_bare -- --test-threads=1 → FAIL (same witness)

```property
function: gp_integer.encode_push
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [seg, m]
  domain: { seg: sregs, m: mem32 }
  body: "strip_seg(encode(seg:m)) == encode(m) AND encode(seg:m)[0] == seg_byte(seg)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  m: { gen: "mem32_simple", type: MemoryOperand }
evidence: core.rs:31-42; Intel SDM 2.1.1 segment override prefixes
```

## encode_push_neg_arity
- Tier: 3
- Rationale: Negative/error — documented arity contract requires exactly 1 operand (gp_integer.rs:347-348).
- Doc contract: gp_integer.rs:347-348 "push requires 1 operand" — asserted fingerprint bf61b4b6
- Seed: (none)
- Formal: ∀ n ∈ ℕ, n ≠ 1. encode_push(ops with len n) = Err("push requires 1 operand")
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: gp_integer.encode_push
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: nat_except_1 }
  relation:
    op: holds
    expr: "sut_encode(\"pushl\", vec![Reg(eax); n]).is_err()"
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
expected_error: String
evidence: gp_integer.rs:347-348 "push requires 1 operand"
```

## encode_push_neg_xmm
- Tier: 3
- Rationale: Non-GP xmm is invalid for PUSH (llvm-mc rejects; Intel SDM PUSH operands are r/m32/imm/Sreg). SUT must Err, not silently alias via reg_num.
- Doc contract: registers.rs:4-15 reg_num aliases xmm→0..7 — limitation fingerprint b871ec83; Intel SDM PUSH operand set is the contract
- Seed: encode_lea_regression_non_gp_xmm0
- Formal: ∀ x ∈ {xmm0..xmm7}. encode_push([Reg(x)]) = Err
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: failing
- Counterexample: x="xmm0" → Ok([0x50])
- Bug report: bug_reports/encode_push_accepts_non_gp_register.md

```property
function: gp_integer.encode_push
oracle: negative_error
predicate:
  quantifier: forall
  vars: [x]
  domain: { x: xmm0_7 }
  relation:
    op: holds
    expr: "sut_encode(\"pushl\", [Reg(x)]).is_err()"
generators:
  x: { gen: oneof, values: ["xmm0","xmm1","xmm2","xmm3","xmm4","xmm5","xmm6","xmm7"], type: "&str" }
expected_error: String
evidence: Intel SDM PUSH operand set; llvm-mc rejects pushl %xmm0
```

## encode_push_neg_r8
- Tier: 3
- Rationale: r8 is invalid for PUSH (llvm-mc rejects); SUT must Err, not alias via reg_num to r32 short form. Same root cause as encode_push_neg_xmm (B2); kept as reinforcing witness in the test file, ledger-retired to avoid duplicate bugId.
- Doc contract: registers.rs:4-15 reg_num aliases al→0 same as eax — limitation fingerprint b871ec83
- Seed: encode_push_neg_xmm
- Formal: ∀ r8 ∈ {al,cl,dl,bl,ah,ch,dh,bh}. encode_push([Reg(r8)]) = Err
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: failing
- Counterexample: r8="al" → Ok([0x50])
- Bug report: bug_reports/encode_push_accepts_r8_register.md
- Re-verified: cargo test --lib encode_push_neg_r8 -- --test-threads=1 → FAIL (same witness)

```property
function: gp_integer.encode_push
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r8]
  domain: { r8: r8_regs }
  relation:
    op: holds
    expr: "sut_encode(\"pushl\", [Reg(r8)]).is_err()"
generators:
  r8: { gen: oneof, values: ["al","cl","dl","bl","ah","ch","dh","bh"], type: "&str" }
expected_error: String
evidence: Intel SDM PUSH operand set; llvm-mc rejects pushl %al
```

## encode_push_diff_sreg
- Tier: 5
- Rationale: Differential — Intel SDM PUSH Sreg (es=06, cs=0E, ss=16, ds=1E, fs=0F A0, gs=0F A8). encode_push uses only reg_num which has no Sreg entries. Sibling encode_pop implements POP Sreg.
- Doc contract: gp_integer.rs:3 PUSH/POP listed — other fingerprint 65ffa4c5; Intel SDM PUSH Sreg forms
- Seed: encode_pop16_diff_sreg
- Formal: ∀ s ∈ {es,cs,ss,ds,fs,gs}. encode_push([Reg(s)]) = llvm-mc("pushl %s")
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: failing
- Counterexample: sreg="es" → Err("bad register"), mc=[0x06]
- Bug report: bug_reports/encode_push_missing_sreg_forms.md

```property
function: gp_integer.encode_push
oracle: differential
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: sregs }
  relation:
    op: eq
    lhs: "sut_encode(\"pushl\", [Reg(s)])"
    rhs: "llvm_mc_bytes(format!(\"pushl %{}\", s))"
generators:
  s: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
evidence: Intel SDM PUSH Sreg; llvm-mc i686 encodings; sibling encode_pop Sreg table
```

## encode_push_diff_r16
- Tier: 5
- Rationale: Differential — push %ax must emit 0x66 0x50+n (operand-size override), not bare 0x50+n which is pushl of the corresponding r32.
- Doc contract: gp_integer.rs:3 PUSH/POP — other fingerprint 65ffa4c5; Intel SDM operand-size override
- Seed: (none)
- Formal: ∀ r16 ∈ {ax..di}. encode via mnemonic "push"([Reg(r16)]) = llvm-mc("push %r16")
- Test file: src/backend/i686/assembler/encoder/encode_push_pbt.rs
- Status: failing
- Counterexample: r16="ax" → sut=[0x50] mc=[0x66,0x50]
- Bug report: bug_reports/encode_push_missing_r16_operand_size_prefix.md

```property
function: gp_integer.encode_push
oracle: differential
predicate:
  quantifier: forall
  vars: [r16]
  domain: { r16: r16_regs }
  relation:
    op: eq
    lhs: "sut_encode(\"push\", [Reg(r16)])"
    rhs: "llvm_mc_bytes(format!(\"push %{}\", r16))"
generators:
  r16: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"], type: "&str" }
evidence: Intel SDM PUSH r16 with 0x66 in 32-bit mode; llvm-mc
```
