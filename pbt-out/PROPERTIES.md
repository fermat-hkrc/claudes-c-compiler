# Properties: encode_ldop

## encode_ldop_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree LDADD decoder). Sibling encode_cas / encode_swp / encode_stop rejected (same-job gate: different LSE class / operand grammar). README.md:12 gas-compat plus encoder/mod.rs:3 32-bit words. SUT-boundary: internal-helper, operands passed through from encode_instruction.
- Doc contract: load_store.rs:879 "Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics)." — asserted fingerprint 028cc568
- Seed: encode_swp_pbt.rs:encode_swp_diff_llvm_mc
- Formal: ∀ op ∈ {ldadd,ldclr,ldeor,ldset}, ∀ suf ∈ {ε,a,al,l,b,ab,alb,lb,h,ah,alh,lh}, ∀ rs,rt,rn ∈ [0,31], ∀ is_64 ∈ Bool. let v = op+suf. let wide = byte(suf)∨half(suf) ? false : is_64. encode_ldop(v, [Reg(gpr(rs,wide)), Reg(gpr(rt,wide)), Mem{base(rn),0}]) = llvm-mc("-triple=aarch64 -mattr=+lse", "v gpr(rs,wide), gpr(rt,wide), [base(rn)]")
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: differential
predicate:
  quantifier: forall
  vars: [op, suf, rs, rt, rn, is_64]
  domain: { op: ldop_base, suf: ldop_suffix, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool }
  relation:
    op: eq
    lhs: encode_ldop(mnemonic(op,suf), valid_ops(op,suf,rs,rt,rn,is_64))
    rhs: llvm_mc(asm(op,suf,rs,rt,rn,is_64))
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: load_store.rs:879 Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
```

## encode_ldop_arm_fields
- Tier: 4
- Rationale: ARM ARM LDADD layout is an independent structural invariant (size 111000 A R 1 Rs 0 opc 00 Rn Rt). Stronger differential is the sibling property; this unpacks fields from the ARM formula, not a copy of the SUT packer. Round-trip rejected (no decoder).
- Doc contract: load_store.rs:880 "LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt" — asserted fingerprint 5c1b199d
- Seed: encode_swp_pbt.rs:encode_swp_arm_fields
- Formal: ∀ op,suf,rs,rt,rn,is_64 in the valid domain. let w = encode_ldop(...). (w[31:30]=expected_size(suf,wide)) ∧ (w[29:24]=0b111000) ∧ (w[23]=A(suf)) ∧ (w[22]=R(suf)) ∧ (w[21]=1) ∧ (w[20:16]=rs) ∧ (w[15]=0) ∧ (w[14:12]=opc(op)) ∧ (w[11:10]=0) ∧ (w[9:5]=rn) ∧ (w[4:0]=rt)
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, suf, rs, rt, rn, is_64]
  domain: { op: ldop_base, suf: ldop_suffix, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool }
  relation:
    op: holds
    expr: arm_ldop_fields_match(encode_ldop(mnemonic(op,suf), valid_ops(op,suf,rs,rt,rn,is_64)), op, suf, rs, rt, rn, is_64)
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: "load_store.rs:880 LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt"
```

## encode_ldop_metamorphic_regs_ar_opc
- Tier: 4
- Rationale: Rs/Rt/Rn occupy disjoint bit-fields; A (bit 23) and R (bit 22) are independent of the register fields; opc (bits[14:12]) distinguishes LDADD/LDCLR/LDEOR/LDSET; ASCII case of the mnemonic is behavior-preserving (mnemonic.to_lowercase). Stronger differential covers the valid domain; this isolates field independence.
- Doc contract: load_store.rs:880 "LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt" — asserted fingerprint 5c1b199d
- Seed: encode_swp_pbt.rs:encode_swp_metamorphic_regs_ar
- Formal: ∀ rs,rt,rn ∈ [0,30], ∀ is_64. let b = encode_ldop("ldadd", ops(rs,rt,rn)). encode_ldop("ldadd", ops(rs,rt+1,rn)) differs only in bits[4:0]; encode_ldop("ldadd", ops(rs,rt,rn+1)) differs only in bits[9:5]; encode_ldop("ldadd", ops(rs+1,rt,rn)) differs only in bits[20:16]; encode_ldop("ldadda", ops) ⊕ b = 1<<23; encode_ldop("ldaddl", ops) ⊕ b = 1<<22; encode_ldop("ldaddal", ops) ⊕ b = (1<<23)|(1<<22); encode_ldop("ldclr", ops) ⊕ b = 1<<12; encode_ldop("ldeor", ops) ⊕ b = 2<<12; encode_ldop("ldset", ops) ⊕ b = 3<<12; encode_ldop("LDADD", ops) = b
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, rt, rn, is_64]
  domain: { rs: u32_0_30, rt: u32_0_30, rn: u32_0_30, is_64: bool }
  relation:
    op: holds
    expr: field_isolation_and_opc_ar_case(encode_ldop, rs, rt, rn, is_64)
generators:
  rs: { gen: int, min: 0, max: 30, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
evidence: "load_store.rs:880 LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt"
```

## encode_ldop_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc/gas reject a fourth operand on LDADD. README.md:12 gas-compat. Body `operands.len() < 3` does not declare extra operands invalid; they stay in the generator. Negative/error contract from the independent assembler, not from the SUT body.
- Doc contract: load_store.rs:879 "Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics)." — asserted fingerprint 028cc568
- Seed: encode_swp_pbt.rs:encode_swp_neg_extra_operand
- Formal: ∀ op,suf,rs,rt,rn,is_64 in the valid domain, ∀ extra ∈ Operand. encode_ldop(v, valid_ops ++ [extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: failing
- Counterexample: encode_ldop("ldadd", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:0}, Reg("x2")])
- Bug report: pbt-out/bug_reports/encode_ldop_extra_operand.md

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, suf, rs, rt, rn, is_64, extra]
  domain: { op: ldop_base, suf: ldop_suffix, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool, extra: Operand }
  relation:
    op: throws
    expr: encode_ldop(mnemonic(op,suf), valid_ops(op,suf,rs,rt,rn,is_64) ++ [extra])
    error: Err
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, options: [Reg, Imm, Symbol, Mem] }
expected_error: Err
evidence: load_store.rs:879 Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
```

## encode_ldop_neg_sp_zr_base
- Tier: 4
- Rationale: ARM LDADD Rs/Rt are ZR not SP; Rn is Xn|SP not ZR. llvm-mc/gas reject SP/WSP as Rs/Rt and W/WSP/XZR/x31 as base. parse_reg_num maps both SP and XZR to 31; that is not a domain restriction on encode_ldop.
- Doc contract: load_store.rs:880 "LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt" — asserted fingerprint 5c1b199d
- Seed: encode_swp_pbt.rs:encode_swp_neg_sp_zr_base
- Formal: ∀ op,suf in the documented variants, ∀ n ∈ [0,30]. encode_ldop(v, [Reg(sp|wsp), …]) is Err ∧ encode_ldop(v, […, Mem{xzr|x31|wN|wsp|wzr, 0}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: failing
- Counterexample: encode_ldop("ldadd", [Reg("sp"), Reg("w1"), Mem{base:"x2", offset:0}])
- Bug report: pbt-out/bug_reports/encode_ldop_sp_as_rs.md

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, suf, n, kind, is_64]
  domain: { op: ldop_base, suf: ldop_suffix, n: u32_0_30, kind: sp_zr_kind, is_64: bool }
  relation:
    op: throws
    expr: encode_ldop(mnemonic(op,suf), sp_zr_ops(kind,n,suf,is_64))
    error: Err
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 11, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 8, type: u32 }
  is_64: { gen: bool }
expected_error: Err
evidence: "load_store.rs:880 LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt"
```

## encode_ldop_neg_mixed_fp_xbyte
- Tier: 4
- Rationale: llvm-mc/gas reject mixed W/X, FP/SIMD Rs/Rt, and byte/half variants with X registers. get_reg/parse_reg_num accept d/s/q/v/h/b prefixes; encode_ldop does not check matching widths. Those inputs stay in the generator.
- Doc contract: load_store.rs:879 "Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics)." — asserted fingerprint 028cc568
- Seed: encode_swp_pbt.rs:encode_swp_neg_mixed_fp_xbyte
- Formal: ∀ n ∈ [0,30], ∀ fp ∈ {b,h,s,d,q,v}. encode_ldop("ldadd", [Reg(xN), Reg(wN), Mem{xN',0}]) is Err ∧ encode_ldop("ldadd", [Reg(fpN), …]) is Err ∧ encode_ldop("ldaddb"|"ldaddh"|"ldaddab"|"ldsetlh", [Reg(xN), Reg(xN'), Mem{xN'',0}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: failing
- Counterexample: encode_ldop("ldadd", [Reg("x0"), Reg("w0"), Mem{base:"x1", offset:0}])
- Bug report: pbt-out/bug_reports/encode_ldop_mixed_width.md

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, kind, fp]
  domain: { n: u32_0_30, kind: mixed_fp_kind, fp: fp_prefix }
  relation:
    op: throws
    expr: encode_ldop(mixed_fp_mnem(kind), mixed_fp_ops(kind,n,fp))
    error: Err
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 8, type: u32 }
  fp: { gen: oneof, options: [b, h, s, d, q, v] }
expected_error: Err
evidence: load_store.rs:879 Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
```

## encode_ldop_neg_arity_and_shape
- Tier: 4
- Rationale: llvm-mc reports "too few operands" for arity < 3; gas requires a memory operand [Xn|SP], not Imm/Symbol/pre/post/reg-offset/Cond. Body returns Err on len() < 3 and non-Mem third operand; the contract is the assembler grammar, not the producing if.
- Doc contract: load_store.rs:880 "LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt" — asserted fingerprint 5c1b199d
- Seed: encode_swp_pbt.rs:encode_swp_neg_arity_and_shape
- Formal: ∀ shape ∈ {empty, 1-reg, 2-reg, Imm, Symbol, MemPreIndex, MemPostIndex, MemRegOffset, Cond}, ∀ n ∈ [0,30]. encode_ldop("ldadd", ops(shape,n)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape, n, off]
  domain: { shape: arity_shape, n: u32_0_30, off: i64_m8_8 }
  relation:
    op: throws
    expr: encode_ldop("ldadd", arity_ops(shape,n,off))
    error: Err
generators:
  shape: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
  off: { gen: int, min: -8, max: 8, type: i64 }
expected_error: Err
evidence: "load_store.rs:880 LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt"
```

## encode_ldop_neg_nonzero_offset
- Tier: 4
- Rationale: ARM optional immediate offset on LDADD is only #0; llvm-mc/gas reject [Xn, #imm] when imm ≠ 0. Body `Mem { base, .. }` ignores offset; that is not a domain restriction. Offset ≠ 0 stays in the generator.
- Doc contract: load_store.rs:880 "LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt" — asserted fingerprint 5c1b199d
- Seed: encode_swp_pbt.rs:encode_swp_neg_nonzero_offset
- Formal: ∀ op,suf,rs,rt in the valid domain, ∀ rn ∈ [0,30], ∀ off ∈ ℤ\{0}. encode_ldop(v, [Reg(gpr(rs)), Reg(gpr(rt)), Mem{base(rn), off}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: failing
- Counterexample: encode_ldop("ldadd", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:-1}])
- Bug report: pbt-out/bug_reports/encode_ldop_nonzero_offset.md

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, suf, rs, rt, rn, is_64, off]
  domain: { op: ldop_base, suf: ldop_suffix, rs: u32_0_31, rt: u32_0_31, rn: u32_0_30, is_64: bool, off: nonzero_i64 }
  relation:
    op: throws
    expr: encode_ldop(mnemonic(op,suf), [Reg(gpr(rs,wide)), Reg(gpr(rt,wide)), Mem{base(rn), off}])
    error: Err
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  off: { gen: int, min: -4096, max: 4096, type: i64 }
expected_error: Err
evidence: "load_store.rs:880 LDADD Rs, Rt, [Xn]: size 111000 A R 1 Rs 0 opc 00 Rn Rt"
```

## encode_ldop_neg_invalid_name
- Tier: 4
- Rationale: Sweep of parse_reg_num None on Rs/Rt/base (foo, x32, empty, r0). Documented by get_reg returning Err on unparsable names. Strengthening/coverage-gaps round.
- Doc contract: load_store.rs:879 "Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics)." — asserted fingerprint 028cc568
- Seed: encode_swp_pbt.rs:encode_swp_neg_invalid_name
- Formal: ∀ slot ∈ {Rs,Rt,Rn}, ∀ name ∈ {foo,x32,w32,"",r0,x-1,31}. encode_ldop("ldadd", ops_with(slot,name)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, name]
  domain: { slot: {0,1,2}, name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_ldop("ldadd", ops_with(slot, name))
    error: Err
generators:
  slot: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: oneof, options: [foo, x32, w32, empty, r0, x-1, 31] }
expected_error: Err
evidence: load_store.rs:879 Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
```

## encode_ldop_diff_alt_spellings
- Tier: 2
- Rationale: mnemonic.to_lowercase is behavior-preserving; llvm-mc accepts LDADD/Ldadd. Sweep of the to_lowercase arm.
- Doc contract: load_store.rs:879 "Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics)." — asserted fingerprint 028cc568
- Seed: encode_swp_pbt.rs:encode_swp_diff_alt_spellings
- Formal: ∀ op,suf,rs,rt,rn,is_64 in the valid domain, ∀ mode ∈ {upper, title, lower}. encode_ldop(casefold(v,mode), valid_ops) = llvm-mc(casefold(v,mode) …)
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: differential
predicate:
  quantifier: forall
  vars: [op, suf, rs, rt, rn, is_64, mode]
  domain: { op: ldop_base, suf: ldop_suffix, rs: u32_0_31, rt: u32_0_31, rn: u32_0_31, is_64: bool, mode: case_mode }
  relation:
    op: eq
    lhs: encode_ldop(casefold(mnemonic(op,suf), mode), valid_ops(op,suf,rs,rt,rn,is_64))
    rhs: llvm_mc(casefold(asm(op,suf,rs,rt,rn,is_64), mode))
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 11, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  mode: { gen: int, min: 0, max: 2, type: u32 }
evidence: load_store.rs:879 Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
```

## encode_ldop_neg_unknown_op
- Tier: 4
- Rationale: Sweep of the unknown-mnemonic else branch. encode_ldop returns Err for names that do not start with ldadd/ldclr/ldeor/ldset.
- Doc contract: load_store.rs:879 "Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics)." — asserted fingerprint 028cc568
- Seed: (none)
- Formal: ∀ name ∈ {ldfoo,swp,cas,ld,add,"",stadd}. encode_ldop(name, [Reg(x0), Reg(x1), Mem{x2,0}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: unknown_ldop_mnemonic }
  relation:
    op: throws
    expr: encode_ldop(name, valid_ops(0,0,1,2,true))
    error: Err
generators:
  name: { gen: oneof, options: [ldfoo, swp, cas, ld, add, empty, stadd] }
expected_error: Err
evidence: load_store.rs:879 Encode LDADD/LDCLR/LDEOR/LDSET and their acquire/release/byte/halfword variants (LSE atomics).
```
