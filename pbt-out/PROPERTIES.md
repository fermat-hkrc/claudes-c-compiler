# Properties: encode_umaddl

## encode_umaddl_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc. README claims the assembler "accepts the same textual assembly that GCC's gas would consume"; encoder docstring claims 32-bit AArch64 words. State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree UMADDL decoder. encode_smaddl fails the same-job gate (U=0 signed). encode_umull is the Ra=XZR alias, not an independent implementation. SUT-boundary: internal-helper of GNU-style assembler. Mapping: encode_umaddl([Xd, Wn, Wm, Xa]) <-> `umaddl Xd, Wn, Wm, Xa`.
- Seed: README.md:214 Data Processing table; encoder/mod.rs:270; llvm-mc KAT `umaddl x0, w1, w2, x3` = 0x9ba20c20
- Formal: ∀ rd,rn,rm,ra ∈ [0,31]. encode_umaddl([Reg(xreg(rd)), Reg(wreg(rn)), Reg(wreg(rm)), Reg(xreg(ra))]) = llvm-mc("umaddl Xd, Wn, Wm, Xa") where xreg(31)=xzr, wreg(31)=wzr, and dest/acc may be spelled lr when the number is 30
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  relation:
    op: eq
    lhs: encode_umaddl([Reg(xreg(rd)), Reg(wreg(rn)), Reg(wreg(rm)), Reg(xreg(ra))])
    rhs: llvm_mc("umaddl {Xd}, {Wn}, {Wm}, {Xa}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:1-14; encoder/mod.rs:1-7; encoder/mod.rs:270; ARM ARM UMADDL
```

## encode_umaddl_alias_umull_xzr
- Tier: 3
- Rationale: Algebraic metamorphic: ARM ARM states UMULL Xd, Wn, Wm is the alias of UMADDL Xd, Wn, Wm, XZR. llvm-mc disassembles Ra=XZR UMADDL as umull. encode_umull shares get_reg so it is not an independent differential; the equality plus llvm-mc agreement is the evidenced contract. Stronger differential on the four-operand form is a sibling property.
- Seed: data_processing.rs:641 docstring "UMULL Xd, Wn, Wm -> UMADDL Xd, Wn, Wm, XZR"; llvm-mc `umaddl x0, w1, w2, xzr` = `umull x0, w1, w2` = 0x9ba27c20
- Formal: ∀ rd,rn,rm ∈ [0,31]. encode_umaddl([Xd, Wn, Wm, XZR]) = encode_umull([Xd, Wn, Wm]) = llvm-mc("umull Xd, Wn, Wm") = llvm-mc("umaddl Xd, Wn, Wm, xzr")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_umaddl([Reg(xreg(rd)), Reg(wreg(rn)), Reg(wreg(rm)), Reg("xzr")])
    rhs: encode_umull([Reg(xreg(rd)), Reg(wreg(rn)), Reg(wreg(rm))])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:641-646; ARM ARM UMULL alias of UMADDL Ra=XZR; llvm-mc umaddl/umull KAT
```

## encode_umaddl_xor_smaddl_u_bit
- Tier: 3
- Rationale: Algebraic metamorphic: ARM ARM Data-processing (3 source) UMADDL and SMADDL share the format and differ only in U (bit 23). Same-job gate fails for treating encode_smaddl as a differential twin (signed vs unsigned), but the U-bit XOR is the documented field difference. Stronger differential is a sibling.
- Seed: data_processing.rs:657 vs 669 comments (001 vs 101 at bits[23:21]); llvm-mc `umaddl x0,w1,w2,x3` XOR `smaddl x0,w1,w2,x3` = 1<<23
- Formal: ∀ rd,rn,rm,ra ∈ [0,31]. encode_umaddl(Xd,Wn,Wm,Xa) XOR encode_smaddl(Xd,Wn,Wm,Xa) = 1<<23
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  relation:
    op: eq
    lhs: encode_umaddl(ops) XOR encode_smaddl(ops)
    rhs: 1 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:657-676; ARM ARM UMADDL U=1 vs SMADDL U=0
```

## encode_umaddl_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Data-processing (3 source) UMADDL layout sf=1 U=1 11011 101 Rm o0=0 Ra Rn Rd. Stronger differential is a sibling property. Round-trip rejected (no decoder).
- Seed: data_processing.rs:669-673 format comment; ARM ARM C6 UMADDL
- Formal: ∀ rd,rn,rm,ra ∈ [0,31]. Let w = encode_umaddl([Xd,Wn,Wm,Xa]). Then w[31]=1 ∧ w[30:21]=00_11011_101 ∧ w[20:16]=rm ∧ w[15]=0 ∧ w[14:10]=ra ∧ w[9:5]=rn ∧ w[4:0]=rd. Equivalently w = 0x9BA00000 | (rm<<16) | (ra<<10) | (rn<<5) | rd
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  body: let w = encode_umaddl([Xd,Wn,Wm,Xa]); w == 0x9BA00000 | (rm<<16) | (ra<<10) | (rn<<5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:669-673; ARM ARM Data-processing (3 source) UMADDL
```

## encode_umaddl_diff_alt_spellings
- Tier: 2
- Rationale: Differential against llvm-mc for alternate GNU-style spellings that get_reg accepts (x31/w31, XZR, LR, uppercase). Strengthening of encode_umaddl_diff_valid_gpr, which under-sampled these names. Same oracle rejection chain as the canonical differential.
- Seed: neighbouring encode_smull_diff_alt_spellings; llvm-mc accepts `umaddl X0, W1, W2, X3`, `umaddl x31, w31, w31, x31`, `umaddl LR, w1, w2, x30`
- Formal: ∀ rd,rn,rm,ra ∈ [0,31], dest/src/acc spellings ∈ {canonical, x31/w31, XZR, LR, uppercase}. encode_umaddl(ops) = llvm-mc(asm) when llvm-mc accepts the spelling
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, dest_spell, src_spell, acc_spell]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, dest_spell: 0..4, src_spell: 0..2, acc_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_umaddl([Reg(dest), Reg(src_n), Reg(src_m), Reg(acc)])
    rhs: llvm_mc("umaddl {dest}, {src_n}, {src_m}, {acc}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 2, type: u32 }
  acc_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: src/backend/arm/assembler/README.md:1-14; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_umaddl_neg_arity
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc reports "too few operands for instruction" for UMADDL with fewer than 4 operands; ARM ARM syntax is four registers. Stronger differential does not apply on the invalid domain (llvm-mc errors).
- Seed: llvm-mc `umaddl x0, w1, w2` → error: too few operands; neighbouring encode_smull_neg_arity
- Formal: ∀ ops with |ops| ∈ [0,3]. encode_umaddl(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..3 }
  relation:
    op: throws
    expr: encode_umaddl(ops_of_len(len))
expected_error: String
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
evidence: llvm-mc aarch64 "too few operands for instruction"; ARM ARM UMADDL four-operand syntax
```

## encode_umaddl_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc rejects a 5th operand ("invalid operand for instruction"). GNU-style UMADDL has exactly four registers. Stronger differential does not apply on the invalid domain.
- Seed: llvm-mc `umaddl x0, w1, w2, x3, x4` → error: invalid operand; neighbouring encode_smull_neg_extra_operand
- Formal: ∀ rd,rn,rm,ra ∈ [0,31], extra ∈ Operand. encode_umaddl([Xd, Wn, Wm, Xa, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, extra=Reg("x0")
- Bug report: pbt-out/bug_reports/encode_umaddl_extra_operand.md

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_umaddl([Xd, Wn, Wm, Xa, extra])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc aarch64 rejects fifth operand; ARM ARM UMADDL four-operand syntax
```

## encode_umaddl_neg_wrong_width
- Tier: 4e
- Rationale: Negative/error contract: ARM ARM syntax is UMADDL Xd, Wn, Wm, Xa (64-bit dest and accumulator, 32-bit multiply sources). llvm-mc errors on W dest, X sources, or W accumulator. Stronger differential does not apply on the invalid domain.
- Seed: llvm-mc `umaddl w0, w1, w2, w3` / `umaddl x0, x1, x2, x3` / `umaddl x0, w1, w2, w3` all error; neighbouring encode_smull_neg_wrong_width
- Formal: ∀ rd,rn,rm,ra ∈ [0,30], width flags not equal to (rd64=true, rn64=false, rm64=false, ra64=true). encode_umaddl([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm), gpr(ra64,ra)]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, rd64=false, rn64=false, rm64=false, ra64=false (umaddl w0, w0, w0, w0)
- Bug report: pbt-out/bug_reports/encode_umaddl_wrong_width.md

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, rd64, rn64, rm64, ra64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, ra: 0..30, widths: not_X_W_W_X }
  relation:
    op: throws
    expr: encode_umaddl([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm), gpr(ra64,ra)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
  ra64: { gen: bool }
evidence: ARM ARM UMADDL Xd, Wn, Wm, Xa; llvm-mc invalid operand on W dest / X sources / W accumulator
```

## encode_umaddl_neg_sp
- Tier: 4e
- Rationale: Negative/error contract: ARM ARM register 31 in this encoding is XZR/WZR, never SP/WSP. llvm-mc rejects sp/wsp in any UMADDL slot. parse_reg_num maps sp/wsp to 31, which would silently encode ZR if accepted. Stronger differential does not apply on the invalid domain.
- Seed: llvm-mc `umaddl sp, w1, w2, x3` / `umaddl x0, wsp, w2, x3` / `umaddl x0, w1, w2, sp` all error; neighbouring encode_smull_neg_sp
- Formal: ∀ which ∈ {0,1,2,3}, sp ∈ {sp,wsp}, other slots valid UMADDL GPRs. encode_umaddl(ops with SP at slot which) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0 (umaddl wsp, w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_umaddl_sp_as_zr.md

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain: { which: 0..3, is_64: bool, a: 0..30, b: 0..30 }
  relation:
    op: throws
    expr: encode_umaddl(ops_with_sp_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM UMADDL Rd/Rn/Rm/Ra use XZR/WZR at 31 not SP; llvm-mc invalid operand for sp/wsp
```

## encode_umaddl_neg_fp
- Tier: 4e
- Rationale: Negative/error contract: UMADDL operands are GPRs. llvm-mc rejects d/s/q/v/h/b names. parse_reg_num accepts those prefixes and would encode them as the matching GPR number. Stronger differential does not apply on the invalid domain.
- Seed: llvm-mc `umaddl d0, w1, w2, x3` → error: invalid operand; neighbouring encode_smull_neg_fp
- Formal: ∀ which ∈ {0,1,2,3}, prefix ∈ {d,s,q,v,h,b}, n ∈ [0,31]. encode_umaddl(ops with prefix+n at slot which) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 (umaddl d0, w1, w2, x3)
- Bug report: pbt-out/bug_reports/encode_umaddl_fp_as_gpr.md

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..3, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_umaddl(ops_with_fp_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc aarch64 rejects FP/SIMD names for UMADDL; ARM ARM GPR-only operands
```

## encode_umaddl_neg_nonreg
- Tier: 4e
- Rationale: Negative/error contract: each of the four slots must be a register. Imm/Shift/Mem/Label/Symbol/Cond/RegArrangement must Err. Stronger differential does not apply on the invalid domain.
- Seed: neighbouring encode_smull_neg_nonreg; get_reg returns Err for non-Reg
- Formal: ∀ which ∈ {0,1,2,3}, bad ∈ non-Reg Operand. encode_umaddl(ops with bad at slot which) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: non-Reg Operand }
  relation:
    op: throws
    expr: encode_umaddl(ops_with_nonreg_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: get_reg at encoder/mod.rs:956-966; ARM ARM four-register syntax
```

## encode_umaddl_neg_invalid_name
- Tier: 4e
- Rationale: Negative/error contract: invalid register names (foo, x32, w32, x, r0, empty, x-1, x99, w) must Err. llvm-mc rejects them. Stronger differential does not apply on the invalid domain.
- Seed: neighbouring encode_smull_neg_invalid_name; parse_reg_num returns None for these
- Formal: ∀ which ∈ {0,1,2,3}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_umaddl(ops with Reg(name) at slot which) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_umaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..3, name: invalid_gpr_name }
  relation:
    op: throws
    expr: encode_umaddl(ops_with_invalid_name_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: parse_reg_num at encoder/mod.rs:131-147; llvm-mc rejects invalid names
```
