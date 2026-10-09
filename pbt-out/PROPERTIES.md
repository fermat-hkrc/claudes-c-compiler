# Properties: encode_double_shift (i686)

## encode_double_shift_diff_imm_rr
- Tier: 4
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine N/A. Algebraic round-trip N/A (no decoder). Shared contract: Intel SDM SHLD/SHRD Imm8,r32,r32 and assembler README listing shld/shrd.
- Doc contract: (none on function) — dispatch `mod.rs:250-251` `"shldl"|"shld" => 0xA4`, `"shrdl"|"shrd" => 0xAC` — asserted fingerprint a4acshld
- Seed: (none — no prior unit test for double shift)
- Formal: ∀ mnem ∈ {shldl,shld,shrdl,shrd}, src,dst ∈ GP32, c ∈ 0..255. encode(mnem, Imm(c), Reg(src), Reg(dst)) = llvm-mc(mnem $c, %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_double_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, src, dst, count]
  domain: { mnem: {shldl,shld,shrdl,shrd}, src: GP32, dst: GP32, count: 0..255 }
  relation:
    op: eq
    lhs: sut_encode(mnem, [Imm(count), Reg(src), Reg(dst)])
    rhs: llvm_mc(mnem + " $" + count + ", %" + src + ", %" + dst)
generators:
  mnem: { gen: oneof, values: ["shldl", "shld", "shrdl", "shrd"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  count: { gen: int, min: 0, max: 255, type: u8 }
evidence: mod.rs:250-251; Intel SDM Vol.2 SHLD/SHRD; llvm-mc -triple=i686
```

## encode_double_shift_diff_cl_rr
- Tier: 4
- Rationale: Differential CL-count form (opc+1). Same evidence chain as Imm form.
- Doc contract: (none on function) — body `gp_integer.rs:934-939` CL arm uses `opcode + 1` — asserted fingerprint clarm0f
- Seed: (none)
- Formal: ∀ mnem ∈ {shldl,shld,shrdl,shrd}, src,dst ∈ GP32. encode(mnem, Reg(cl), Reg(src), Reg(dst)) = llvm-mc(mnem %cl, %src, %dst)
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_double_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, src, dst]
  domain: { mnem: {shldl,shld,shrdl,shrd}, src: GP32, dst: GP32 }
  relation:
    op: eq
    lhs: sut_encode(mnem, [Reg(cl), Reg(src), Reg(dst)])
    rhs: llvm_mc(mnem + " %cl, %" + src + ", %" + dst)
generators:
  mnem: { gen: oneof, values: ["shldl", "shld", "shrdl", "shrd"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
evidence: gp_integer.rs:934-939; Intel SDM SHLD/SHRD CL form
```

## encode_double_shift_diff_mem_dst
- Tier: 4
- Rationale: Intel SDM and llvm-mc encode memory destinations for SHLD/SHRD. Public mnemonics shldl/shrdl claim support (README). Differential must hold for Imm/CL × Reg × Mem.
- Doc contract: assembler/README.md:171 "Shifts: … shld/shrd" — asserted fingerprint shldshrd
- Seed: (none)
- Formal: ∀ mnem ∈ {shldl,shrdl}, src ∈ GP32, mem ∈ valid_i686_mem, form ∈ {Imm(c), CL}. encode(mnem, form, Reg(src), Mem(mem)) = llvm-mc(…)
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: failing
- Counterexample: shldl $0, %eax, (%eax) → Err("unsupported double shift operands"); llvm-mc Ok([0x0f,0xa4,0x00,0x00])
- Bug report: bug_reports/encode_double_shift_mem_dst_unsupported.md

```property
function: encode_double_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [mnem, src, mem, form]
  domain: { mnem: {shldl,shrdl}, src: GP32, mem: i686_mem, form: Imm0_255|CL }
  relation:
    op: eq
    lhs: sut_encode(mnem, [form, Reg(src), Mem(mem)])
    rhs: llvm_mc(corresponding AT&T)
generators:
  mnem: { gen: oneof, values: ["shldl", "shrdl"] }
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  count: { gen: int, min: 0, max: 255, type: u8 }
evidence: Intel SDM Vol.2 SHLD/SHRD r/m32 forms; assembler/README.md:171
```

## encode_double_shift_invariant_opcodes
- Tier: 3
- Rationale: Algebraic invariant from Intel opcodes: Imm → 0F A4/AC + ModRM(mod=3,reg=src,rm=dst) + ib; CL → 0F A5/AD + ModRM.
- Doc contract: (none) — body pushes `[0x0F, opcode]` / `[0x0F, opcode+1]` — asserted fingerprint 0fa4ac
- Seed: (none)
- Formal: ∀ src,dst ∈ GP32, c ∈ 0..255. encode(shldl,Imm(c),src,dst) = [0x0F,0xA4, modrm(3,src,dst), c] ∧ encode(shrdl,…) = [0x0F,0xAC,…] ∧ CL forms use A5/AD without ib
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_double_shift
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [src, dst, count]
  domain: { src: GP32, dst: GP32, count: 0..255 }
  body: "sut_encode(shldl, Imm(count), src, dst) == [0x0F, 0xA4, modrm(3,src,dst), count] && sut_encode(shrdl, Imm(count), src, dst) == [0x0F, 0xAC, modrm(3,src,dst), count] && sut_encode(shldl, CL, src, dst) == [0x0F, 0xA5, modrm(3,src,dst)] && sut_encode(shrdl, CL, src, dst) == [0x0F, 0xAD, modrm(3,src,dst)]"
generators:
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  count: { gen: int, min: 0, max: 255, type: u8 }
evidence: Intel SDM SHLD 0F A4/A5; SHRD 0F AC/AD
```

## encode_double_shift_meta_alias_mnemonic
- Tier: 3
- Rationale: Metamorphic — dispatcher aliases shld≡shldl and shrd≡shrdl with same opcode; encodings must be identical on identical operands.
- Doc contract: mod.rs:250-251 `"shldl" | "shld"` share 0xA4; `"shrdl" | "shrd"` share 0xAC — asserted fingerprint alias0xa4
- Seed: (none)
- Formal: ∀ src,dst ∈ GP32, c ∈ 0..255. encode(shld,…) = encode(shldl,…) ∧ encode(shrd,…) = encode(shrdl,…)
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_double_shift
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [src, dst, count]
  domain: { src: GP32, dst: GP32, count: 0..255 }
  relation:
    op: eq
    lhs: sut_encode("shld", [Imm(count), Reg(src), Reg(dst)])
    rhs: sut_encode("shldl", [Imm(count), Reg(src), Reg(dst)])
generators:
  src: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  dst: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"] }
  count: { gen: int, min: 0, max: 255, type: u8 }
evidence: mod.rs:250-251
```

## encode_double_shift_neg_arity
- Tier: 2
- Rationale: Negative contract — body requires ops.len()==3.
- Doc contract: gp_integer.rs:921-923 `"double shift requires 3 operands"` — domain-restriction fingerprint need3ops
- Seed: (none)
- Formal: ∀ n ≠ 3. encode(shldl, ops_n) = Err
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_double_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: arity_not_3 }
  relation:
    op: throws
    expr: sut_encode("shldl", ops_of_len(n))
generators:
  n: { gen: int, min: 0, max: 5, type: usize }
expected_error: String
evidence: gp_integer.rs:921-923
```

## encode_double_shift_neg_non_gp
- Tier: 2
- Rationale: Non-GP (xmm) must be rejected; llvm-mc rejects; reg_num alias must not silently accept.
- Doc contract: (none declaring xmm valid) — inferred from Intel SDM register class and llvm-mc — asserted fingerprint nogpxmm
- Seed: encode_inc_dec_pbt.rs neg_xmm
- Formal: ∀ mnem ∈ {shldl,shrdl}, x ∈ XMM, role ∈ {src,dst}. encode(mnem, Imm(1)|CL, … x …) = Err
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: failing
- Counterexample: shldl $1, %eax, %xmm0 → Ok([0x0f,0xa4,0xc0,0x01]) (same as %eax)
- Bug report: bug_reports/encode_double_shift_accepts_non_gp.md

```property
function: encode_double_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, x]
  domain: { mnem: shldl_or_shrdl, x: XMM }
  relation:
    op: throws
    expr: sut_encode(mnem, [Imm(1), Reg(eax), Reg(x)])
generators:
  mnem: { gen: oneof, values: ["shldl", "shrdl"] }
  x: { gen: oneof, values: ["xmm0","xmm1","xmm7"] }
expected_error: String
evidence: Intel SDM SHLD r32 class; llvm-mc rejects
```

## encode_double_shift_neg_mismatched_width
- Tier: 2
- Rationale: Width-mismatched GP (r16/r8) as src/dst of shldl must be rejected.
- Doc contract: (none) — inferred size=4 dispatch and llvm-mc rejection — asserted fingerprint width4
- Seed: encode_inc_dec_pbt.rs neg_mismatched_width
- Formal: ∀ mnem ∈ {shldl,shrdl}, bad ∈ GP16∪GP8. encode(mnem, Imm(1), … bad …) = Err
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: failing
- Counterexample: shldl $1, %eax, %ax → Ok([0x0f,0xa4,0xc0,0x01]) (same as %eax dst)
- Bug report: bug_reports/encode_double_shift_mismatched_width.md

```property
function: encode_double_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mnem, bad]
  domain: { mnem: shldl_or_shrdl, bad: GP16_or_GP8 }
  relation:
    op: throws
    expr: sut_encode(mnem, [Imm(1), Reg(bad), Reg(edx)])
generators:
  mnem: { gen: oneof, values: ["shldl", "shrdl"] }
  bad: { gen: oneof, values: ["ax","al","cx","bl"] }
expected_error: String
evidence: Intel SDM; llvm-mc rejects shldl with r16/r8
```

## encode_double_shift_neg_imm_out_of_u8
- Tier: 2
- Rationale: Imm8 count must be in byte domain. llvm-mc rejects $256; SUT must not silently truncate via `as u8`.
- Doc contract: (none) — inferred Imm8 encoding width; llvm-mc rejects 256 — asserted fingerprint imm8only
- Seed: (none)
- Formal: ∀ c outside Imm8 acceptance of llvm-mc. SUT Err iff llvm Err; never Ok with truncated byte when llvm rejects
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: failing
- Counterexample: shldl $256, %eax, %eax → SUT Ok([0x0f,0xa4,0xc0,0x00]); llvm-mc Err
- Bug report: bug_reports/encode_double_shift_imm8_truncate.md

```property
function: encode_double_shift
oracle: differential
predicate:
  quantifier: forall
  vars: [count]
  domain: { count: edge imm including 256, 512, -129, 0x100 }
  body: "(llvm_ok(count) && sut_ok(count) && sut_bytes == llvm_bytes) || (!llvm_ok(count) && !sut_ok(count))"
generators:
  count: { gen: oneof, values: [256, 512, -129, 0x100, 0x1_0000] }
evidence: Intel Imm8; llvm-mc rejects $256
```

## encode_double_shift_neg_bad_count_reg
- Tier: 2
- Rationale: Documented negative contract — only Imm or %cl may be the count operand (gp_integer.rs:926 Imm arm and :934 `cl.name == "cl"` guard); any other first register is out of domain and the SUT correctly returns Err. This property asserts that rejection holds (passing = contract satisfied).
- Doc contract: gp_integer.rs:934 guard `cl.name == "cl"`; else arm `unsupported double shift operands` — domain-restriction fingerprint onlycl
- Seed: (none)
- Formal: ∀ r ≠ cl, src,dst ∈ GP32. encode(shldl, Reg(r), Reg(src), Reg(dst)) = Err
- Test file: src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_double_shift
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad_count, src, dst]
  domain: { bad_count: GP_except_cl, src: GP32, dst: GP32 }
  relation:
    op: throws
    expr: sut_encode("shldl", [Reg(bad_count), Reg(src), Reg(dst)])
generators:
  bad_count: { gen: oneof, values: ["eax","edx","ax"] }
  src: { gen: oneof, values: ["eax","ecx"] }
  dst: { gen: oneof, values: ["edx","ebx"] }
expected_error: String
evidence: gp_integer.rs:934-941
```
