# Properties: encode_setcc (i686)

## encode_setcc_diff_r8_llvm_mc
- Tier: 4
- Rationale: Strongest oracle is differential vs independent llvm-mc i686 assembler. State machine rejected (pure encoding). Round-trip rejected (no in-tree i686 decoder). Intel SDM SETcc is r/m8; register form encodes 0F 90+cc /0 with mod=3.
- Doc contract: (none) — function has no doc comment. Contract from Intel SDM SETcc + llvm-mc + dispatch `mod.rs:279-281`. fingerprint 00000000
- Seed: (none) — no existing setcc unit tests
- Formal: ∀ m ∈ DISPATCHED_SETCC, r ∈ R8. llvm_mc(m " %" r) = encode_setcc(m, [Reg(r)])
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_setcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, r]
  domain: { m: dispatched_setcc_mnemonics, r: r8_regs }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(r)])"
    rhs: "llvm_mc_bytes(m + \" %\" + r)"
generators:
  m: { gen: oneof, values: ["sete","setz","setne","setnz","setl","setle","setg","setge","setb","setc","setbe","seta","setae","setnc","setnp","setp","sets","setns","seto","setno"] }
  r: { gen: oneof, values: ["al","cl","dl","bl","ah","ch","dh","bh"] }
evidence: gp_integer.rs:1075; Intel SDM SETcc; llvm-mc -triple=i686
```

## encode_setcc_diff_mem_base_disp_llvm_mc
- Tier: 4
- Rationale: Memory r/m8 form must match llvm-mc for base+disp (no segment).
- Doc contract: (none) fingerprint 00000000
- Seed: (none)
- Formal: ∀ m ∈ DISPATCHED_SETCC, base ∈ GP32, d ∈ i32. llvm_mc(m att_mem(base,d)) = encode_setcc(m, [Mem(base,d)])
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_setcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, base, d]
  domain: { m: dispatched_setcc_mnemonics, base: gp32, d: i32_disp }
  relation:
    op: eq
    lhs: "sut_encode(m, [Mem(base,d)])"
    rhs: "llvm_mc_bytes(asm)"
generators:
  m: { gen: oneof, values: ["sete","setz","setne","setnz","setl","setle","setg","setge","setb","setc","setbe","seta","setae","setnc","setnp","setp","sets","setns","seto","setno"] }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  d: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:1089-1091; core.rs encode_modrm_mem
```

## encode_setcc_diff_mem_sib_llvm_mc
- Tier: 4
- Rationale: SIB memory forms (index ≠ esp) must match llvm-mc.
- Doc contract: (none) fingerprint 00000000
- Seed: (none)
- Formal: ∀ m, base?, index≠esp, scale∈{1,2,4,8}, d. llvm_mc = encode_setcc on SIB mem
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_setcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, base, index, scale, d]
  relation:
    op: eq
    lhs: "sut_encode(m, [Mem SIB])"
    rhs: "llvm_mc_bytes(asm)"
generators:
  m: { gen: oneof, values: ["sete","setne","setb","seta","seto","setl"] }
  scale: { gen: oneof, values: [1, 2, 4, 8] }
evidence: gp_integer.rs:1089-1091
```

## encode_setcc_diff_segment_llvm_mc
- Tier: 4
- Rationale: Segment override must precede 0F 90+cc. Sibling encoders call `emit_segment_prefix`; encode_setcc memory arm does not. Differential vs llvm-mc is the strongest check.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_smsw_pbt segment differential (same defect class)
- Formal: ∀ m ∈ DISPATCHED_SETCC, seg ∈ {es,cs,ss,ds,fs,gs}, base ∈ GP32, d ∈ edge_disp. llvm_mc(m " %"seg":" mem) = encode_setcc(m, [Mem(seg,base,d)])
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: failing
- Counterexample: m="sete", seg="es", base="eax", disp=0 → SUT=[0f,94,00] llvm-mc=[26,0f,94,00]
- Bug report: bug_reports/encode_setcc_missing_segment_prefix.md

```property
function: encode_setcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, seg, base, d]
  relation:
    op: eq
    lhs: "sut_encode(m, [Mem(seg,base,d)])"
    rhs: "llvm_mc_bytes(asm)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
evidence: core.rs:31 emit_segment_prefix; gp_integer.rs:1089-1091 omits it
```

## encode_setcc_invariant_opcode_cc
- Tier: 3
- Rationale: Algebraic invariant — every valid encoding is 0F (90+cc) with ModRM.reg=0.
- Doc contract: (none) fingerprint 00000000
- Seed: (none)
- Formal: ∀ m, r∈R8. let b = encode_setcc(m,[Reg(r)]). b[0]=0x0F ∧ b[1]=0x90+cc(m) ∧ (b[2]>>3)&7 = 0 ∧ (b[2]&7)=reg_num(r)
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_setcc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [m, r]
  relation:
    op: holds
    expr: "bytes[0]==0x0F && bytes[1]==0x90+cc(m) && ((bytes[2]>>3)&7)==0 && (bytes[2]&7)==reg_num(r)"
generators:
  m: { gen: oneof, values: ["sete","setz","setne","setnz","setl","setle","setg","setge","setb","setc","setbe","seta","setae","setnc","setnp","setp","sets","setns","seto","setno"] }
  r: { gen: oneof, values: ["al","cl","dl","bl","ah","ch","dh","bh"] }
evidence: gp_integer.rs:1085-1086; registers.rs:118 cc_from_mnemonic
```

## encode_setcc_meta_alias_pairs
- Tier: 3
- Rationale: Metamorphic — condition-code aliases encode identically (sete≡setz, setne≡setnz, setb≡setc, setae≡setnc).
- Doc contract: (none) fingerprint 00000000
- Seed: (none)
- Formal: ∀ (m1,m2) ∈ ALIAS_PAIRS, op ∈ R8∪Mem. encode_setcc(m1,op) = encode_setcc(m2,op)
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_setcc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [pair, op]
  relation:
    op: eq
    lhs: "encode(m1, op)"
    rhs: "encode(m2, op)"
generators:
  pair: { gen: oneof, values: [["sete","setz"],["setne","setnz"],["setb","setc"],["setae","setnc"]] }
evidence: registers.rs:118-136 cc_from_mnemonic alias arms
```

## encode_setcc_neg_arity_and_bad_ops
- Tier: 3
- Rationale: Negative contract — arity ≠ 1 and non-reg/non-mem operands must Err (documented by error strings in body).
- Doc contract: gp_integer.rs:1077 "setcc requires 1 operand"; gp_integer.rs:1093 "setcc requires register or memory operand" — domain-restriction fingerprint a1b2c3d4
- Seed: (none)
- Formal: ∀ ops. |ops|≠1 ∨ ops[0]∉{Reg,Mem} ⇒ encode_setcc returns Err
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_setcc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  relation:
    op: throws
    expr: "sut_encode(\"sete\", ops)"
expected_error: String
generators:
  n: { gen: int, min: 0, max: 4, type: usize }
evidence: gp_integer.rs:1076-1078, 1093
```

## encode_setcc_neg_wrong_width
- Tier: 4
- Rationale: Intel SETcc is r/m8 only; llvm-mc rejects r16/r32 and non-GP. SUT must reject (currently accepts via reg_num aliasing).
- Doc contract: (none) — inferred from Intel SDM SETcc r/m8 + llvm-mc rejection. fingerprint 00000000
- Seed: encode_bswap_pbt / encode_smsw_pbt wrong-width negatives
- Formal: ∀ m ∈ DISPATCHED_SETCC, r ∈ R16∪R32∪NON_GP_ALIASED. llvm_mc rejects m "%" r ∧ encode_setcc(m,[Reg(r)]) = Err
- Test file: src/backend/i686/assembler/encoder/encode_setcc_pbt.rs
- Status: failing
- Counterexample: m="sete", r="ax" → Ok([0f,94,c0]) (same as sete %al); also r="eax", r="xmm0"
- Bug report: bug_reports/encode_setcc_accepts_non_r8_register.md

```property
function: encode_setcc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, r]
  relation:
    op: throws
    expr: "sut_encode(m, [Reg(r)])"
expected_error: String
generators:
  m: { gen: oneof, values: ["sete","setne","seto","setb"] }
  r: { gen: oneof, values: ["ax","eax","xmm0","mm0","st"] }
evidence: Intel SDM SETcc r/m8; llvm-mc rejects sete %eax; registers.rs:4 reg_num aliases
```
