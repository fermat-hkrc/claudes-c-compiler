# Properties: encode_cmovcc

## encode_cmovcc_diff_rr32_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc i686 (Intel CMOVcc 0F 40+cc /r). State machine N/A (pure encoder). Round-trip N/A (no decoder). Same-width r32 register-register is the baseline happy path.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — domain-restriction fingerprint dcf9474f
- Seed: (none — no prior cmov unit test)
- Formal: ∀ m ∈ CMOV_MNEMS_32, s,d ∈ GP32. encode_cmovcc([Reg(s),Reg(d)], m) = llvm-mc("m %s, %d")
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_cmovcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, src, dst]
  domain: { m: cmov_mnems_32, src: gp32, dst: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"{m} %{src}, %{dst}\"))"
generators:
  m: { gen: oneof, values: ["cmove","cmovne","cmovl","cmovle","cmovg","cmovge","cmovb","cmovbe","cmova","cmovae","cmovs","cmovns","cmovz","cmovnz","cmovp","cmovnp","cmovo","cmovno","cmovc","cmovnc","cmovel","cmovnel","cmovll"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:1097; mod.rs:286-297; Intel SDM CMOVcc; llvm-mc -triple=i686
```

## encode_cmovcc_diff_rr16_w_llvm_mc
- Tier: 5
- Rationale: 16-bit form must emit 0x66 + 0F 40+cc; differential vs llvm-mc on explicit `w` suffix mnemonics with r16 pairs.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — domain-restriction fingerprint dcf9474f
- Seed: (none)
- Formal: ∀ m ∈ CMOV_MNEMS_W, s,d ∈ GP16. encode_cmovcc([Reg(s),Reg(d)], m) = llvm-mc("m %s, %d")
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_cmovcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, src, dst]
  domain: { m: cmov_mnems_w, src: gp16, dst: gp16 }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(format!(\"{m} %{src}, %{dst}\"))"
generators:
  m: { gen: oneof, values: ["cmovew","cmovnew","cmovlw","cmovlew","cmovgw","cmovgew","cmovbw","cmovbew","cmovaw","cmovaew","cmovsw","cmovnsw","cmovzw","cmovnzw","cmovow","cmovnow"] }
  src: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"] }
  dst: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"] }
evidence: gp_integer.rs:1104-1113 (w suffix → is_16bit); Intel CMOVcc r16
```

## encode_cmovcc_diff_mem_base_llvm_mc
- Tier: 5
- Rationale: Memory→register form (AT&T src mem, dst reg) must match llvm-mc ModR/M/SIB/disp for non-segment memory. Also covers SIB, abs-disp32, and strengthen mem_w16 (explicit w suffix mem→r16) sub-generators in the same file.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — domain-restriction fingerprint dcf9474f
- Seed: (none)
- Formal: ∀ m ∈ CMOV_MNEMS_32, base ∈ GP32, d ∈ GP32, disp ∈ i32. encode_cmovcc([Mem(base,disp),Reg(d)], m) = llvm-mc(...); also ∀ m_w, base, d16, disp. encode_cmovcc([Mem,Reg(d16)], m_w) = llvm-mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_cmovcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, base, dst, disp]
  domain: { m: cmov_mnems_32, base: gp32, dst: gp32, disp: i32_edge }
  relation:
    op: eq
    lhs: "sut_encode(m, [Mem(base,disp), Reg(dst)])"
    rhs: "llvm_mc_bytes(...)"
generators:
  m: { gen: oneof, values: ["cmove","cmovne","cmovl","cmovg","cmovo","cmovae"] }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  disp: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:1128-1133; core.rs encode_modrm_mem
```

## encode_cmovcc_diff_segment_llvm_mc
- Tier: 5
- Rationale: Memory with segment override must emit ES/CS/SS/DS/FS/GS prefix before opcode (sibling encoders call emit_segment_prefix; this arm does not). Differential vs llvm-mc.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — other fingerprint dcf9474f (does not authorize omitting segment prefixes)
- Seed: encode_setcc_pbt.rs segment properties (same defect class)
- Formal: ∀ m, seg, base, dst, disp. encode_cmovcc([Mem(seg:base+disp),Reg(dst)], m) = llvm-mc(...)
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: failing
- Counterexample: cmove %es:(%eax), %ebx → SUT [0f,44,18] vs llvm-mc [26,0f,44,18]
- Bug report: pbt-out/bug_reports/encode_cmovcc_missing_segment_prefix.md

```property
function: encode_cmovcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, seg, base, dst, disp]
  domain: { m: cmov_mnems_32, seg: seg_regs, base: gp32, dst: gp32, disp: edge_disp }
  relation:
    op: eq
    lhs: "sut_encode(...)"
    rhs: "llvm_mc_bytes(...)"
generators:
  m: { gen: oneof, values: ["cmove","cmovne","cmovl","cmova"] }
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"] }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  disp: { gen: oneof, values: [0, 8, -4, 127, -128] }
evidence: core.rs:31 emit_segment_prefix; gp_integer.rs:1128-1133 missing call; Intel SDM segment override
```

## encode_cmovcc_diff_unsuffixed_r16_size
- Tier: 5
- Rationale: Unsuffixed mnemonic with r16 operands must match llvm-mc (which emits 0x66). SUT keys is_16bit only off mnemonic suffix → silent wrong encoding. Comment only defaults suffix parsing, does not declare r16 invalid for unsuffixed form.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — asserted/limitation fingerprint dcf9474f — comment describes default size path; accepting r16 without 0x66 still violates AT&T/llvm-mc contract
- Seed: (none)
- Formal: ∀ m ∈ CMOV_UNSUFFIXED, s,d ∈ GP16. encode_cmovcc([Reg(s),Reg(d)], m) = llvm-mc("m %s, %d")
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: failing
- Counterexample: cmove %ax, %ax → SUT [0f,44,c0] vs llvm-mc [66,0f,44,c0]
- Bug report: pbt-out/bug_reports/encode_cmovcc_unsuffixed_r16_no_66.md

```property
function: encode_cmovcc
oracle: differential
predicate:
  quantifier: forall
  vars: [m, src, dst]
  domain: { m: cmov_unsuffixed, src: gp16, dst: gp16 }
  relation:
    op: eq
    lhs: "sut_encode(m, [Reg(src), Reg(dst)])"
    rhs: "llvm_mc_bytes(...)"
generators:
  m: { gen: oneof, values: ["cmove","cmovne","cmovl","cmovle","cmovg","cmovz","cmovo"] }
  src: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"] }
  dst: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"] }
evidence: llvm-mc accepts cmove %ax,%bx as cmovew [66,0f,44,...]; gp_integer.rs:1118-1120 only suffixes set is_16bit
```

## encode_cmovcc_invariant_opcode_rr32
- Tier: 4
- Rationale: Algebraic invariant — RR32 encoding is exactly [0F, 40+cc, modrm(3,dst,src)] with no 0x66.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — domain-restriction fingerprint dcf9474f
- Seed: (none)
- Formal: ∀ m,s,d ∈ GP32. let b=encode(...). b=[0x0F, 0x40+cc(m), modrm(3,num(d),num(s))]
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_cmovcc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [m, src, dst]
  domain: { m: cmov_unsuffixed, src: gp32, dst: gp32 }
  relation:
    op: holds
    expr: "sut_encode(m,[Reg(src),Reg(dst)]) == [0x0F, 0x40+cc(m), modrm(3,num(dst),num(src))]"
generators:
  m: { gen: oneof, values: ["cmove","cmovne","cmovl","cmovg","cmovb","cmovo","cmovs","cmovp"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:1120-1126; Intel CMOVcc opcode 0F 40+cc
```

## encode_cmovcc_meta_alias_pairs
- Tier: 4
- Rationale: Algebraic metamorphic — condition-code aliases (e/z, ne/nz, b/c, ae/nc) must encode identically for the same operands.
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — other fingerprint dcf9474f
- Seed: encode_setcc_pbt.rs alias pairs
- Formal: ∀ (m1,m2) ∈ ALIAS_PAIRS, ops. encode(m1,ops)=encode(m2,ops)
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_cmovcc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [pair, ops]
  domain: { pair: alias_pairs, ops: rr32_or_mem }
  relation:
    op: eq
    lhs: "sut_encode(m1, ops)"
    rhs: "sut_encode(m2, ops)"
generators:
  pair: { gen: oneof, values: [["cmove","cmovz"],["cmovne","cmovnz"],["cmovb","cmovc"],["cmovae","cmovnc"]] }
evidence: registers.rs:118 cc_from_mnemonic aliases
```

## encode_cmovcc_neg_bad_forms
- Tier: 3
- Rationale: Negative/error — arity≠2, r8, non-GP, mismatched widths, reg→mem must Err (Intel CMOVcc is r16/r32 and r/m→reg only; llvm-mc rejects). Arity and imm/label/reg→mem paths pass; r8/non-GP/mismatch fail (B3).
- Doc contract: gp_integer.rs:1103 "Strip size suffix if present, otherwise use as-is (unsuffixed = 32-bit default)" — other fingerprint dcf9474f (does not expand domain to r8/non-GP)
- Seed: encode_setcc_pbt.rs neg_wrong_width / neg_non_gp
- Formal: ∀ invalid ops. encode_cmovcc(ops,m) = Err(_)
- Test file: src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs
- Status: failing
- Counterexample: cmove %al, %al → Ok([0f,44,c0]); also cmove %xmm0/%eax and cmove %ax/%eax
- Bug report: pbt-out/bug_reports/encode_cmovcc_accepts_invalid_regs.md

```property
function: encode_cmovcc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [m, bad_ops]
  domain: { m: cmov_unsuffixed, bad_ops: arity_ne_2 | r8_pair | non_gp | width_mismatch | reg_to_mem }
  relation:
    op: holds
    expr: "sut_encode(m, bad_ops).is_err()"
expected_error: String
generators:
  m: { gen: oneof, values: ["cmove","cmovne","cmovo"] }
  kind: { gen: int, min: 0, max: 4, type: u8 }
evidence: Intel SDM CMOVcc; llvm-mc rejects; gp_integer.rs:1098-1100 arity check only
```
