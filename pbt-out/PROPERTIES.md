# Properties: encode_fmov

## encode_fmov_diff_valid
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FMOV decoder). Sibling encode_fp_arith / encode_fneg / encode_fp_1src rejected (same-job gate: different mnemonics). Doc evidence: README.md:11 gas-compatible assembler; README.md:223 lists fmov; encoder/mod.rs:376 dispatch; ARM ARM FMOV (register) and FMOV (general).
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt / float_ops.rs:27-36
- Formal: ∀ rd,rn ∈ {0..31}, form ∈ {SS, DD, SW, DX, WS, XD}, spelling ∈ {canonical, uppercase, zr/w31/x31/lr where valid}. encode_fmov([Reg(dest), Reg(src)]) = Word(w) ∧ w = llvm-mc("fmov dest, src")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, form, dest_kind, src_kind]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fmov([Reg(dest), Reg(src)])
    rhs: llvm_mc("fmov dest, src")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  form: { gen: int, min: 0, max: 5, type: u32 }
evidence: src/backend/arm/assembler/README.md:11
```

## encode_fmov_arm_fields
- Tier: 3
- Rationale: Algebraic invariant from ARM ARM encoding tables (weaker than differential, independently pins field layout). FP-to-FP: 00011110 ftype 1 000000 10000 Rn Rd. GP-to-FP: sf 00 11110 ftype 1 00 111 000000 Rn Rd. FP-to-GP: opcode 110.
- Seed: encode_fcvt_rounding_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, is_d ∈ Bool. encode_fmov([Reg(Sd/Dd), Reg(Sn/Dn)]) = Word(w) ⇒ w[31:24]=00011110 ∧ w[23:22]=ftype ∧ w[21:16]=100000 ∧ w[15:10]=010000 ∧ w[9:5]=rn ∧ w[4:0]=rd. Matching-width GP↔FP words equal the ARM FMOV (general) bit template.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmov
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fmov([Reg(fp(is_d,rd)), Reg(fp(is_d,rn))])
    rhs: (0b00011110 << 24) | (ftype << 22) | (0b100000 << 16) | (0b10000 << 10) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: src/backend/arm/assembler/encoder/fp_scalar.rs:32
```

## encode_fmov_metamorphic_fields
- Tier: 3
- Rationale: Algebraic metamorphic: Rd/Rn occupy independent 5-bit fields; S vs D flips only ftype bit 22 on FP-to-FP; GP→FP vs FP→GP flips only opcode LSB bit 16. Weaker than differential.
- Seed: encode_fcvt_rounding_metamorphic_fields
- Formal: ∀ rd,rn ∈ {0..30}. let w = encode_fmov(s{rd}, s{rn}). encode_fmov(s{rd+1}, s{rn}) = w+1 ∧ encode_fmov(s{rd}, s{rn+1}) = w+(1<<5) ∧ encode_fmov(d{rd}, d{rn}) xor w = 1<<22 ∧ encode_fmov(s{rd}, w{rn}) xor encode_fmov(w{rd}, s{rn}) = 1<<16
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmov
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..30, rn: 0..30 }
  relation:
    op: eq
    lhs: encode_fmov([Reg(s{rd+1}), Reg(s{rn})])
    rhs: encode_fmov([Reg(s{rd}), Reg(s{rn})]) + 1
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: src/backend/arm/assembler/encoder/fp_scalar.rs:32-33
```

## encode_fmov_neg_arity
- Tier: 2
- Rationale: Negative/error contract. llvm-mc/gas: "too few operands". SUT documents `fmov requires 2 operands` at fp_scalar.rs:8. README.md:11 gas-compatible.
- Seed: encode_fcvt_rounding_neg_arity
- Formal: ∀ ops with |ops| ∈ {0,1}. encode_fmov(ops) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..1 }
  relation:
    op: holds
    expr: encode_fmov(ops).is_err()
expected_error: String
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
evidence: src/backend/arm/assembler/encoder/fp_scalar.rs:8
```

## encode_fmov_neg_extra_operand
- Tier: 2
- Rationale: Negative/error contract. llvm-mc "invalid operand"; gas "unexpected characters following instruction". FMOV (register/general) is 2-operand. README.md:11.
- Seed: encode_fcvt_rounding_neg_extra_operand
- Formal: ∀ rd,rn ∈ {0..31}, extra ∈ Operand. encode_fmov([Reg(s{rd}), Reg(s{rn}), extra]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=Reg("s0")
- Bug report: pbt-out/bug_reports/encode_fmov_extra_operand.md

```property
function: encoder.fp_scalar.encode_fmov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: holds
    expr: encode_fmov([Reg(s{rd}), Reg(s{rn}), extra]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:11
```

## encode_fmov_neg_incompatible
- Tier: 2
- Rationale: Negative/error contract. llvm-mc/gas reject mixed S/D, size-mismatched GP/FP (D/W, S/X, X/S, W/D), Q/V/B scalar, and two GP registers. ARM FMOV (register) requires matching ftype; FMOV (general) requires matching widths.
- Seed: encode_fcvt_rounding_neg_wrong_types
- Formal: ∀ (dest, src) incompatible. encode_fmov([Reg(dest), Reg(src)]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: dest="q0", src="s0"
- Bug report: pbt-out/bug_reports/encode_fmov_wrong_types.md

```property
function: encoder.fp_scalar.encode_fmov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src]
  domain: { dest: incompatible_reg, src: incompatible_reg }
  relation:
    op: holds
    expr: encode_fmov([Reg(dest), Reg(src)]).is_err()
expected_error: String
generators:
  dest: { gen: string }
  src: { gen: string }
evidence: src/backend/arm/assembler/README.md:11
```

## encode_fmov_neg_sp
- Tier: 2
- Rationale: Negative/error contract. ARM FMOV (general) uses ZR not SP at register 31. llvm-mc/gas reject SP/WSP. parse_reg_num maps sp/wsp to 31 (ZR encoding) which would silently emit FMOV with WZR/XZR.
- Seed: encode_fcvt_rounding_neg_sp_dest
- Formal: ∀ sp ∈ {sp, wsp}, other a valid FMOV partner. encode_fmov with SP/WSP in either slot = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: is_64=false, n=0, which=0, partner_fp=true (wsp, s0)
- Bug report: pbt-out/bug_reports/encode_fmov_sp.md

```property
function: encoder.fp_scalar.encode_fmov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, n, which]
  domain: { n: 0..31 }
  relation:
    op: holds
    expr: encode_fmov(ops_with_sp).is_err()
expected_error: String
generators:
  is_64: { gen: bool }
  n: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/arm/assembler/README.md:11
```

## encode_fmov_diff_half
- Tier: 4
- Rationale: Differential vs llvm-mc -mattr=+fullfp16. ARM FMOV (register) ftype=11 and FMOV (general) Hd/Wn, Wd/Hn. gas -march=armv8.2-a+fp16 agrees (`fmov h0, h1` = 0x1ee04020, `fmov h0, w1` = 0x1ee70020).
- Seed: encode_fcvt_rounding_diff_half
- Formal: ∀ rd,rn ∈ {0..31}, form ∈ {HH, HW, WH}. encode_fmov([Reg(dest), Reg(src)]) = Word(llvm-mc_fp16("fmov dest, src"))
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, form=0 (fmov h0, h0) — SUT 0x1e204000 vs llvm-mc 0x1ee04000
- Bug report: pbt-out/bug_reports/encode_fmov_half_ftype.md

```property
function: encoder.fp_scalar.encode_fmov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, form]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fmov([Reg(dest), Reg(src)])
    rhs: llvm_mc_fp16("fmov dest, src")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  form: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md:11
```

## encode_fmov_neg_nonreg
- Tier: 2
- Rationale: Negative/error contract (sweep). Symbol/Label/Mem/Cond/Shift/Expr are not FMOV operands. Imm at src is FMOV (immediate) — excluded here (documented TODO at fp_scalar.rs:14-15). llvm-mc/gas reject these kinds.
- Seed: encode_fcvt_rounding_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Symbol, Label, Mem, Cond, Shift, Expr}. encode_fmov with bad at slot which = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..1, kind: 0..5 }
  relation:
    op: holds
    expr: encode_fmov(ops_with_nonreg).is_err()
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
evidence: src/backend/arm/assembler/README.md:11
```

## encode_fmov_neg_invalid_name
- Tier: 2
- Rationale: Negative/error contract (sweep). parse_reg_num returns None for foo/s32/d32/empty/r0. llvm-mc rejects invalid names.
- Seed: encode_fcvt_rounding_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, s32, d32, h32, x32, r0, s, d, empty}. encode_fmov with Reg(name) at slot which = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmov
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1 }
  relation:
    op: holds
    expr: encode_fmov(ops_with_bad_name).is_err()
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: string }
evidence: src/backend/arm/assembler/encoder/mod.rs:131
```

## encode_fmov_diff_vd1
- Tier: 4
- Rationale: Differential vs llvm-mc (sweep). ARM FMOV (general) top-half: FMOV Xd, Vn.D[1] / FMOV Vd.D[1], Xn. Encoding sf=1 ftype=10 rmode=01 opcode 110/111. gas agrees (`fmov x0, v1.d[1]` = 0x9eae0020). Operand::RegLane is caller-reachable from the parser.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, dir ∈ {to_vec, from_vec}. encode_fmov([Reg(Xd)|RegLane(Vd.D[1])]) = Word(llvm-mc("fmov Xd, Vn.D[1]")) (and the reverse)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, to_vec=false (fmov x0, v0.d[1]) — Err("fmov needs register operands") vs llvm-mc 0x9eae0000
- Bug report: pbt-out/bug_reports/encode_fmov_vd1.md

```property
function: encoder.fp_scalar.encode_fmov
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, to_vec]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fmov(ops_vd1)
    rhs: llvm_mc("fmov Xd, Vn.D[1]")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  to_vec: { gen: bool }
evidence: src/backend/arm/assembler/README.md:11
```
