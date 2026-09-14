# Properties: encode_madd

## encode_madd_diff_gpr
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (gas-compatible AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree MADD decoder. encode_msub fails the same-job sibling gate (o0=1 vs o0=0). Doc evidence: README.md:5-14 gas-compatible text; encoder/mod.rs:245 madd dispatch; ARM ARM Data-processing (3 source) MADD `sf 00 11011 000 Rm 0 Ra Rn Rd`.
- Seed: encode_div_pbt::encode_div_diff_gpr_same_width (data_processing.rs)
- Formal: ∀ rd,rn,rm,ra ∈ [0,31], sf ∈ {0,1}. encode_madd([Rd, Rn, Rm, Ra]) = llvm-mc("madd Rd, Rn, Rm, Ra") as little-endian u32, where register 31 is XZR/WZR and all four registers share width sf.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_madd([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm)), Reg(gpr(is_64,ra))])
    rhs: llvm_mc("madd "+gpr(is_64,rd)+", "+gpr(is_64,rn)+", "+gpr(is_64,rm)+", "+gpr(is_64,ra))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:245; ARM ARM Data-processing (3 source) MADD
```

## encode_madd_diff_ra_zr_is_mul
- Tier: 2
- Rationale: Documented alias `MUL Rd, Rn, Rm is MADD Rd, Rn, Rm, XZR` (data_processing.rs:589). llvm-mc prints madd-with-ZR as mul and encodes identically. Differential vs llvm-mc MUL (and madd ... zr) on encode_madd with Ra=31. encode_mul is not called (single-symbol campaign). Stronger state machine / round-trip rejected as for encode_madd_diff_gpr.
- Seed: data_processing.rs:589 encode_mul comment
- Formal: ∀ rd,rn,rm ∈ [0,31], sf ∈ {0,1}. encode_madd([Rd, Rn, Rm, ZR]) = llvm-mc("mul Rd, Rn, Rm") = llvm-mc("madd Rd, Rn, Rm, ZR"), ZR = XZR if sf else WZR.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_madd([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm)), Reg(zr(is_64))])
    rhs: llvm_mc("mul "+gpr(is_64,rd)+", "+gpr(is_64,rn)+", "+gpr(is_64,rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: data_processing.rs:589; llvm-mc madd x0,x1,x2,xzr aliases mul x0,x1,x2 = 0x9b027c20
```

## encode_madd_metamorphic_sf_bit
- Tier: 4
- Rationale: ARM ARM places sf at bit 31; 64-bit and 32-bit MADD of equal register numbers differ only by that bit. Differential already covers absolute encoding; this metamorphic check isolates the sf contract. Stronger oracles (state machine, round-trip) rejected as above.
- Seed: encode_div_pbt metamorphic shape
- Formal: ∀ rd,rn,rm,ra ∈ [0,31]. encode_madd(X-ops) XOR encode_madd(W-ops) = 1<<31 at equal register numbers.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  relation:
    op: eq
    lhs: encode_madd([Reg(x(rd)),Reg(x(rn)),Reg(x(rm)),Reg(x(ra))]) XOR encode_madd([Reg(w(rd)),Reg(w(rn)),Reg(w(rm)),Reg(w(ra))])
    rhs: 1<<31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) sf at bit 31; llvm-mc madd x0,x1,x2,x3 = 0x9b020c20 vs madd w0,w1,w2,w3 = 0x1b020c20
```

## encode_madd_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM Data-processing (3 source) MADD field layout is an exact structural predicate on every success-path word. Weaker than differential (does not pin absolute opcode against an independent assembler) but catches field packing bugs. o0 (bit 15) must be 0 (MSUB is 1).
- Seed: encode_div_pbt::encode_div_invariant_arm_fields
- Formal: ∀ rd,rn,rm,ra ∈ [0,31], sf ∈ {0,1}. let w = encode_madd([Rd,Rn,Rm,Ra]). w[4:0]=rd ∧ w[9:5]=rn ∧ w[14:10]=ra ∧ w[15]=0 ∧ w[20:16]=rm ∧ w[30:21]=0011011000 ∧ w[31]=sf.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, is_64: bool }
  relation:
    op: holds
    expr: fields(encode_madd([Rd,Rn,Rm,Ra])) match ARM MADD layout
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Data-processing (3 source) MADD sf 00 11011 000 Rm 0 Ra Rn Rd; data_processing.rs:603-604
```

## encode_madd_diff_lr
- Tier: 2
- Rationale: parse_reg_num and is_64bit_reg treat `lr` as X30. llvm-mc accepts `madd lr, x0, x1, x2` as X30. Coverage-sweep property for the documented alias not reached by the x0–x30/xzr generator. Differential vs llvm-mc.
- Seed: llvm-mc madd lr, x0, x1, x2 encoding; encoder/mod.rs:135-136 "lr" => 30
- Formal: ∀ which ∈ {0,1,2,3}, a,b,c ∈ [0,30]. encode_madd with `lr` at position which (other slots X registers) = llvm-mc of the same text.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: differential
predicate:
  quantifier: forall
  vars: [which, a, b, c]
  domain: { which: 0..3, a: 0..30, b: 0..30, c: 0..30 }
  relation:
    op: eq
    lhs: encode_madd(ops with lr at which)
    rhs: llvm_mc(asm with lr at which)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  c: { gen: int, min: 0, max: 30, type: u32 }
evidence: encoder/mod.rs:135-136 lr => 30; llvm-mc madd lr, x0, x1, x2 = madd x30, x0, x1, x2
```

## encode_madd_neg_too_few
- Tier: 4e
- Rationale: llvm-mc rejects `madd x0, x1, x2` (too few operands). ARM MADD is a 4-operand instruction. get_reg at missing index must Err. Negative/error contract from gas-compatible assembler, not inferred from SUT body.
- Seed: encode_div_pbt::encode_div_neg_too_few_operands
- Formal: ∀ ops with |ops| < 4. encode_madd(ops) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_64, r0, r1, r2]
  domain: { n: 0..3, is_64: bool, r0: 0..31, r1: 0..31, r2: 0..31 }
  relation:
    op: throws
    expr: encode_madd(ops[..n])
    error: String
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  r0: { gen: int, min: 0, max: 31, type: u32 }
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction" on madd x0, x1, x2; ARM ARM MADD four registers
```

## encode_madd_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects a 5th operand (`madd x0, x1, x2, x3, x4` and `..., lsl #0`). Gas-compatible assembler must reject extra operands. README.md:5-14.
- Seed: encode_div_pbt::encode_div_neg_extra_operand
- Formal: ∀ rd,rn,rm,ra ∈ [0,31], sf ∈ {0,1}, extra ∈ Operand. encode_madd([Rd,Rn,Rm,Ra, extra]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, is_64=false, extra=Reg("x0")  (madd w0, w0, w0, w0, x0 encodes instead of Err)
- Bug report: pbt-out/bug_reports/encode_madd_extra_operand.md

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_64, extra]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, ra: 0..30, is_64: bool, extra: Operand }
  relation:
    op: throws
    expr: encode_madd([Rd,Rn,Rm,Ra,extra])
    error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, choices: ["Reg", "Imm", "Shift"] }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on madd x0, x1, x2, x3, x4; README.md:5-14
```

## encode_madd_neg_mixed_width
- Tier: 4e
- Rationale: llvm-mc rejects mixed X/W (`madd w0, x1, x2, x3` etc.). ARM MADD requires a single sf for all four registers. Gas-compatible assembler must Err.
- Seed: encode_div_pbt::encode_div_neg_mixed_width
- Formal: ∀ rd,rn,rm,ra ∈ [0,30], widths ∈ {0,1}^4 not all equal. encode_madd([Rd@w0, Rn@w1, Rm@w2, Ra@w3]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, rd64=false, rn64=false, rm64=false, ra64=true  (madd w0, w0, w0, x0 encodes; sf taken only from Rd)
- Bug report: pbt-out/bug_reports/encode_madd_mixed_width.md

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, rd64, rn64, rm64, ra64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, ra: 0..30, widths: not-all-equal bools }
  relation:
    op: throws
    expr: encode_madd(mixed-width regs)
    error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
  ra64: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on madd w0, x1, x2, x3; ARM ARM single sf
```

## encode_madd_neg_sp
- Tier: 4e
- Rationale: ARM ARM register 31 in MADD is XZR/WZR, never SP/WSP. llvm-mc rejects SP/WSP in every slot. parse_reg_num maps sp/wsp to 31, which would silently encode ZR if accepted — that is the contract under test, not an oracle guessed from the body.
- Seed: encode_div_pbt::encode_div_neg_sp
- Formal: ∀ which ∈ {0,1,2,3}, sf ∈ {0,1}, other regs in [0,30] same width. encode_madd with SP/WSP at position which = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0, c=0  (madd wsp, w0, w0, w0 encodes as madd wzr, w0, w0, w0)
- Bug report: pbt-out/bug_reports/encode_madd_sp.md

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b, c]
  domain: { which: 0..3, is_64: bool, a: 0..30, b: 0..30, c: 0..30 }
  relation:
    op: throws
    expr: encode_madd(ops with SP/WSP at which)
    error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  c: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" on madd sp, x0, x1, x2 and madd x0, sp, x1, x2; ARM ARM Rd/Rn/Rm/Ra=31 is ZR not SP
```

## encode_madd_neg_fp
- Tier: 4e
- Rationale: llvm-mc rejects FP/SIMD names (`madd d0, d1, d2, d3` and mixed `madd x0, x1, x2, d3`). Integer MADD is GPR-only. is_fp_reg exists in the encoder but encode_madd does not use it. Gas-compatible assembler must Err. Strengthening round covering remaining documented rejection.
- Seed: encode_div_pbt::encode_div_neg_fp
- Formal: ∀ which ∈ {0,1,2,3}, prefix ∈ {d,s,q,v,h,b}, n ∈ [0,31]. encode_madd with prefix+n at position which = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0  (madd d0, x1, x2, x3 encodes as madd w0, x1, x2, x3; parse_reg_num accepts d/s/q/v/h/b)
- Bug report: pbt-out/bug_reports/encode_madd_fp_reg.md

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..3, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_madd(ops with FP name at which)
    error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  prefix: { gen: oneof, choices: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" on madd d0, d1, d2, d3; README.md:5-14
```

## encode_madd_neg_invalid_reg
- Tier: 4e
- Rationale: llvm-mc rejects out-of-range and non-register names (x32, foo, empty). parse_reg_num returns None for these; get_reg must Err. Strengthening round.
- Seed: encode_div_pbt::encode_div_neg_invalid_reg_name
- Formal: ∀ which ∈ {0,1,2,3}, name ∈ {x32,w32,x99,w99,"",foo,r0,x,x-1}. encode_madd with name at which = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: {x32,w32,x99,w99,"",foo,r0,x,x-1} }
  relation:
    op: throws
    expr: encode_madd(ops with bad name at which)
    error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  bad: { gen: oneof, choices: ["x32", "w32", "x99", "w99", "", "foo", "r0", "x", "x-1"] }
expected_error: String
evidence: llvm-mc "invalid operand" on madd x32, x0, x1, x2; parse_reg_num None for these names
```

## encode_madd_neg_non_register
- Tier: 4e
- Rationale: llvm-mc and ARM MADD require four registers. Imm/Symbol/Mem/Shift/Cond/Label at any of the four slots is invalid. get_reg returns Err for non-Reg. Strengthening round.
- Seed: encode_div_pbt::encode_div_neg_non_register
- Formal: ∀ which ∈ {0,1,2,3}, bad ∈ {Imm, Symbol, Mem, Shift, Cond, Label}. encode_madd with bad at which = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_madd
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: non-Reg Operand }
  relation:
    op: throws
    expr: encode_madd(ops with non-Reg at which)
    error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  bad: { gen: oneof, choices: ["Imm", "Symbol", "Mem", "Shift", "Cond", "Label"] }
expected_error: String
evidence: ARM ARM MADD four GPR operands; get_reg expected-register error; llvm-mc rejects non-register tokens
```
