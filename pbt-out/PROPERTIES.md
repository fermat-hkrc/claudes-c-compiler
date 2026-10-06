# Properties: encode_swp

## encode_swp_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SWP decoder). Sibling encode_cas / encode_ldop / encode_stop rejected (same-job gate: different LSE class / operand grammar). README.md:12 gas-compat plus encoder/mod.rs:3 32-bit words. SUT-boundary: internal-helper, operands passed through from encode_instruction.
- Doc contract: load_store.rs:846 "Encode SWP/SWPA/SWPAL/SWPL and byte/halfword variants (Swap)." — asserted fingerprint 88297187
- Seed: load_store.rs:8609 encode_cas_diff_llvm_mc
- Formal: ∀ v ∈ {swp,swpa,swpal,swpl,swpb,swpab,swpalb,swplb,swph,swpah,swpalh,swplh}, ∀ rs,rt,rn ∈ [0,31], ∀ is_64 ∈ Bool. let wide = byte(v)∨half(v) ? false : is_64. encode_swp(v, [Reg(gpr(rs,wide)), Reg(gpr(rt,wide)), Mem{base(rn),0}]) = llvm-mc("-triple=aarch64 -mattr=+lse", "v gpr(rs,wide), gpr(rt,wide), [base(rn)]")
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_swp
oracle: differential
predicate:
  quantifier: forall
  vars: [v, rs, rt, rn, is_64]
  domain: { v: swp_variant, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool }
  relation:
    op: eq
    lhs: encode_swp(v, valid_ops(v, rs, rt, rn, is_64))
    rhs: llvm_mc(asm(v, rs, rt, rn, is_64))
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: README.md:12 gas-compat; load_store.rs:846-848; encoder/mod.rs:1045-1047
```

## encode_swp_arm_fields
- Tier: 4
- Rationale: ARM ARM SWP layout is an independent structural invariant (size 111000 A R 1 Rs 1 000 00 Rn Rt). Stronger differential is the sibling property; this unpacks fields from the ARM formula, not a copy of the SUT packer. Round-trip rejected (no decoder).
- Doc contract: load_store.rs:847 "SWP Xs, Xt, [Xn]: size 111000 AR 1 Rs 1 000 00 Rn Rt" — asserted fingerprint 87cf5ebf
- Seed: load_store.rs:8630 encode_cas_arm_fields
- Formal: ∀ v,rs,rt,rn,is_64 in the valid domain. let w = encode_swp(...). (w[31:30]=expected_size(v,wide)) ∧ (w[29:24]=0b111000) ∧ (w[23]=A(v)) ∧ (w[22]=R(v)) ∧ (w[21]=1) ∧ (w[20:16]=rs) ∧ (w[15]=1) ∧ (w[14:10]=0) ∧ (w[9:5]=rn) ∧ (w[4:0]=rt)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_swp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [v, rs, rt, rn, is_64]
  domain: { v: swp_variant, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool }
  relation:
    op: holds
    expr: arm_swp_fields_match(encode_swp(v, valid_ops(v, rs, rt, rn, is_64)), v, rs, rt, rn, is_64)
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: load_store.rs:847 ARM pack formula
```

## encode_swp_metamorphic_regs_ar
- Tier: 4
- Rationale: Rs/Rt/Rn occupy disjoint bit-fields; A (bit 23) and R (bit 22) are independent of the register fields; ASCII case of the mnemonic is behavior-preserving (mnemonic.to_lowercase). Stronger differential covers the valid domain; this isolates field independence.
- Doc contract: load_store.rs:847 "SWP Xs, Xt, [Xn]: size 111000 AR 1 Rs 1 000 00 Rn Rt" — asserted fingerprint 87cf5ebf
- Seed: load_store.rs:8656 encode_cas_metamorphic_regs_ao
- Formal: ∀ rs,rt,rn ∈ [0,30], ∀ is_64. let b = encode_swp("swp", ops(rs,rt,rn)). encode_swp("swp", ops(rs,rt+1,rn)) differs only in bits[4:0]; encode_swp("swp", ops(rs,rt,rn+1)) differs only in bits[9:5]; encode_swp("swp", ops(rs+1,rt,rn)) differs only in bits[20:16]; encode_swp("swpa", ops) ⊕ b = 1<<23; encode_swp("swpl", ops) ⊕ b = 1<<22; encode_swp("swpal", ops) ⊕ b = (1<<23)|(1<<22); encode_swp("SWP", ops) = b
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_swp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, rt, rn, is_64]
  domain: { rs: u32_0_30, rt: u32_0_30, rn: u32_0_30, is_64: bool }
  relation:
    op: holds
    expr: field_isolation_and_AR_and_case(rs, rt, rn, is_64)
generators:
  rs: { gen: int, min: 0, max: 30, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
evidence: load_store.rs:847 field layout; load_store.rs:860 mnemonic.to_lowercase
```

## encode_swp_neg_extra_operand
- Tier: 5
- Rationale: GNU gas / llvm-mc reject a fourth SWP operand. README.md:12 gas-compat. The body only checks operands.len() < 3, so extra operands stay in the generator. Stronger oracles do not cover this invalid domain.
- Doc contract: load_store.rs:846 "Encode SWP/SWPA/SWPAL/SWPL and byte/halfword variants (Swap)." — asserted fingerprint 88297187
- Seed: load_store.rs:8695 encode_cas_neg_extra_operand
- Formal: ∀ v,rs,rt,rn,is_64 in the valid domain, ∀ extra ∈ Operand. encode_swp(v, valid_ops(v,rs,rt,rn,is_64) ++ [extra]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: failing
- Counterexample: v=0, rs=0, rt=0, rn=0, is_64=false, extra=Reg("x2")
- Bug report: bug_reports/encode_swp_extra_operand.md

```property
function: encoder.load_store.encode_swp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [v, rs, rt, rn, is_64, extra]
  domain: { v: swp_variant, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool, extra: Operand }
  relation:
    op: holds
    expr: encode_swp(v, valid_ops(v, rs, rt, rn, is_64).push(extra)).is_err()
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: README.md:12 gas-compat; llvm-mc invalid operand for instruction on a 4th SWP operand
```

## encode_swp_neg_sp_zr_base
- Tier: 5
- Rationale: ARM SWP Rs/Rt are ZR not SP; Rn is Xn|SP not ZR. llvm-mc/gas reject SP/WSP as Rs/Rt and W/WSP/XZR/x31 as base. parse_reg_num maps SP and XZR both to 31. Not a documented domain restriction of encode_swp itself.
- Doc contract: load_store.rs:847 "SWP Xs, Xt, [Xn]: size 111000 AR 1 Rs 1 000 00 Rn Rt" — asserted fingerprint 87cf5ebf
- Seed: load_store.rs:8716 encode_cas_neg_sp_zr_base
- Formal: ∀ v in variants, ∀ n ∈ [0,30]. encode_swp(v, ops with SP/WSP as Rs or Rt, or W/WSP/XZR/x31/WZR as base) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: failing
- Counterexample: v=0, n=0, kind=0, is_64=false
- Bug report: bug_reports/encode_swp_sp_as_rs.md

```property
function: encoder.load_store.encode_swp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [v, n, kind, is_64]
  domain: { v: swp_variant, n: u32_0_30, kind: 0..8, is_64: bool }
  relation:
    op: holds
    expr: encode_swp(v, bad_sp_zr_ops(kind, n, is_64)).is_err()
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 8, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: ARM ARM SWP Rs/Rt are ZR not SP, Rn is Xn|SP; llvm-mc rejects SP/XZR/W-base
```

## encode_swp_neg_mixed_fp_xbyte
- Tier: 5
- Rationale: llvm-mc/gas require matching W or X for word/doubleword SWP, W-only for swpb/swph, and integer GPRs not FP/SIMD. get_reg accepts FP prefixes via parse_reg_num and ignores Rt width.
- Doc contract: load_store.rs:846 "Encode SWP/SWPA/SWPAL/SWPL and byte/halfword variants (Swap)." — asserted fingerprint 88297187
- Seed: load_store.rs:8784 encode_cas_neg_mixed_fp_xbyte
- Formal: ∀ n ∈ [0,30]. encode_swp on mixed W/X, FP/SIMD Rs/Rt, or swpb/swph/swpab/swpalh with X registers = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: failing
- Counterexample: n=0, kind=0, fp='b'
- Bug report: bug_reports/encode_swp_mixed_width.md

```property
function: encoder.load_store.encode_swp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, kind, fp]
  domain: { n: u32_0_30, kind: 0..8, fp: {b,h,s,d,q,v} }
  relation:
    op: holds
    expr: encode_swp(mnem(kind), mixed_fp_ops(kind, n, fp)).is_err()
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 8, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand on mixed W/X, FP, and swpb with X
```

## encode_swp_neg_arity_and_shape
- Tier: 5
- Rationale: Body requires 3 operands and a Mem third operand. llvm-mc/gas also reject Imm/Symbol/pre/post/reg-offset/Cond as the address. Documented by the rustdoc form SWP Xs, Xt, [Xn] and the explicit swp requires memory operand [Xn] error.
- Doc contract: load_store.rs:847 "SWP Xs, Xt, [Xn]: size 111000 AR 1 Rs 1 000 00 Rn Rt" — asserted fingerprint 87cf5ebf
- Seed: load_store.rs:8873 encode_cas_neg_arity_and_shape
- Formal: ∀ shape ∈ {empty, 1-reg, 2-reg, Imm, Symbol, MemPreIndex, MemPostIndex, MemRegOffset, Cond}. encode_swp("swp", ops(shape)) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_swp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape, n, off]
  domain: { shape: 0..8, n: u32_0_30, off: i64_-8_8 }
  relation:
    op: holds
    expr: encode_swp("swp", arity_shape_ops(shape, n, off)).is_err()
generators:
  shape: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
  off: { gen: int, min: -8, max: 8, type: i64 }
expected_error: String
evidence: load_store.rs:850-857 arity/Mem check; load_store.rs:847 [Xn] form
```

## encode_swp_neg_nonzero_offset
- Tier: 5
- Rationale: ARM optional offset on SWP is only #0. llvm-mc/gas reject [Xn, #imm] for imm≠0. Body matches Mem { base, .. } and ignores offset. Not a documented domain restriction of encode_swp itself.
- Doc contract: load_store.rs:847 "SWP Xs, Xt, [Xn]: size 111000 AR 1 Rs 1 000 00 Rn Rt" — asserted fingerprint 87cf5ebf
- Seed: load_store.rs:8930 encode_cas_neg_nonzero_offset
- Formal: ∀ v,rs,rt,rn,is_64 in the valid domain, ∀ off ∈ ℤ\{0}. encode_swp(v, [Reg(Rs), Reg(Rt), Mem{base(rn), off}]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: failing
- Counterexample: v=0, rs=0, rt=0, rn=0, is_64=false, off=-1
- Bug report: bug_reports/encode_swp_nonzero_offset.md

```property
function: encoder.load_store.encode_swp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [v, rs, rt, rn, is_64, off]
  domain: { v: swp_variant, rs: u32_0_31, rt: u32_0_31, rn: u32_0_30, is_64: bool, off: i64_nonzero }
  relation:
    op: holds
    expr: encode_swp(v, [Reg(gpr(rs,wide)), Reg(gpr(rt,wide)), Mem{base(rn), off}]).is_err()
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  off: { gen: int, min: -4096, max: 4096, type: i64 }
expected_error: String
evidence: ARM optional offset only #0; llvm-mc invalid operand on [x2, #8]
```

## encode_swp_neg_invalid_name
- Tier: 5
- Rationale: Sweep of parse_reg_num None path (foo/x32/empty/r0). Documented by get_reg "invalid register" and "swp: invalid base".
- Doc contract: load_store.rs:846 "Encode SWP/SWPA/SWPAL/SWPL and byte/halfword variants (Swap)." — asserted fingerprint 88297187
- Seed: load_store.rs:8955 encode_cas_neg_invalid_name
- Formal: ∀ slot ∈ {Rs,Rt,Rn}, ∀ name ∈ {foo, x32, w32, empty, r0, x-1, 31}. encode_swp("swp", ops with name in slot) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_swp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, name]
  domain: { slot: 0..2, name: invalid_reg_name }
  relation:
    op: holds
    expr: encode_swp("swp", ops_with_name(slot, name)).is_err()
generators:
  slot: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: load_store.rs:853-856 get_reg / parse_reg_num None
```

## encode_swp_diff_alt_spellings
- Tier: 2
- Rationale: Sweep of mnemonic.to_lowercase. llvm-mc accepts SWP/Swp/SWPB. Documented by load_store.rs:860.
- Doc contract: load_store.rs:846 "Encode SWP/SWPA/SWPAL/SWPL and byte/halfword variants (Swap)." — asserted fingerprint 88297187
- Seed: load_store.rs:8988 encode_cas_diff_alt_spellings
- Formal: ∀ v,rs,rt,rn,is_64 in the valid domain, ∀ mode ∈ {upper, title, lower}. encode_swp(cased(v,mode), valid_ops) = llvm-mc(cased asm)
- Test file: src/backend/arm/assembler/encoder/encode_swp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_swp
oracle: differential
predicate:
  quantifier: forall
  vars: [v, rs, rt, rn, is_64, mode]
  domain: { v: swp_variant, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool, mode: 0..2 }
  relation:
    op: eq
    lhs: encode_swp(cased(v, mode), valid_ops(v, rs, rt, rn, is_64))
    rhs: llvm_mc(cased_asm(v, rs, rt, rn, is_64, mode))
generators:
  v: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  mode: { gen: int, min: 0, max: 2, type: u32 }
evidence: load_store.rs:860 mnemonic.to_lowercase; llvm-mc accepts SWP/Swp
```
