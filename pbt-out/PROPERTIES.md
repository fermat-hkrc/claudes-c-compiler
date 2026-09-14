# Properties: encode_msub

## encode_msub_diff_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MSUB decoder). encode_madd / encode_mneg rejected by same-job sibling gate (o0=0 vs o0=1; 3-operand alias vs 4-operand MSUB). Doc evidence: README.md:5-14 gas-compatible assembly; README.md:214 lists msub; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:246 dispatch; ARM ARM Data-processing (3 source) MSUB `sf 00 11011 000 Rm 1 Ra Rn Rd`; llvm-mc `-triple=aarch64`.
- Seed: encode_madd_pbt::encode_madd_diff_gpr
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_64 ∈ {false,true}. encode_msub([Rd, Rn, Rm, Ra]) = llvm-mc("msub Rd, Rn, Rm, Ra") where each name is xN/xzr if is_64 else wN/wzr.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_64]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    ra: 0..31
    is_64: bool
  relation:
    op: eq
    lhs: encode_msub([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn)), Reg(gpr(is_64, rm)), Reg(gpr(is_64, ra))])
    rhs: llvm_mc("msub {Rd}, {Rn}, {Rm}, {Ra}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 README.md:214 encoder/mod.rs:246 ARM ARM Data-processing (3 source) MSUB
```

## encode_msub_diff_ra_zr_is_mneg
- Tier: 2
- Rationale: Documented alias: encode_mneg comment data_processing.rs:677 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR". llvm-mc aliases msub Rd,Rn,Rm,ZR to mneg. Differential vs llvm-mc of both mnemonics. encode_mneg itself is a different job (3-operand public mnemonic) so not a same-job sibling of encode_msub. Doc evidence: data_processing.rs:677; ARM ARM MNEG is MSUB with Ra=XZR/WZR.
- Seed: encode_madd_pbt::encode_madd_diff_ra_zr_is_mul
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ {false,true}. encode_msub([Rd, Rn, Rm, ZR]) = llvm-mc("mneg Rd, Rn, Rm") = llvm-mc("msub Rd, Rn, Rm, ZR").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    is_64: bool
  relation:
    op: eq
    lhs: encode_msub([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn)), Reg(gpr(is_64, rm)), Reg(gpr(is_64, 31))])
    rhs: llvm_mc("mneg {Rd}, {Rn}, {Rm}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: data_processing.rs:677 ARM ARM MNEG is MSUB with Ra=31 encoder/mod.rs:271
```

## encode_msub_metamorphic_sf_bit
- Tier: 4
- Rationale: ARM ARM sf bit is the sole 64 vs 32 distinguisher of otherwise-identical MSUB encodings. Stronger differential covers full-word agreement; this metamorphic isolates sf. Round-trip rejected (no decoder). Doc evidence: ARM ARM `sf 00 11011 000 Rm 1 Ra Rn Rd`.
- Seed: encode_madd_pbt::encode_madd_metamorphic_sf_bit
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}. encode_msub(X-ops) XOR encode_msub(W-ops) = 1<<31 at equal register numbers.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    ra: 0..31
  relation:
    op: eq
    lhs: encode_msub(x_ops) XOR encode_msub(w_ops)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) MSUB sf at bit 31
```

## encode_msub_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on success-path words. Stronger differential covers full-word; this invariant pins o0=1 (the MSUB vs MADD distinguisher) and each register field. Doc evidence: ARM ARM `sf 00 11011 000 Rm 1 Ra Rn Rd`.
- Seed: encode_madd_pbt::encode_madd_invariant_arm_fields
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_64 ∈ {false,true}. let w = encode_msub(...). (w>>31)&1 = sf(is_64) ∧ (w>>21)&0x3FF = 0b0011011000 ∧ (w>>16)&0x1F = rm ∧ (w>>15)&1 = 1 ∧ (w>>10)&0x1F = ra ∧ (w>>5)&0x1F = rn ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_64]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    ra: 0..31
    is_64: bool
  relation:
    op: holds
    expr: arm_msub_fields(encode_msub(ops), rd, rn, rm, ra, is_64)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Data-processing (3 source) MSUB sf 00 11011 000 Rm 1 Ra Rn Rd
```

## encode_msub_diff_lr
- Tier: 2
- Rationale: `lr` is a documented 64-bit alias of X30 (parse_reg_num encoder/mod.rs:135; llvm-mc rewrites lr to x30). Differential vs llvm-mc. Doc evidence: encoder/mod.rs:135 "lr" => 30; is_64bit_reg treats lr as 64-bit.
- Seed: encode_madd_pbt::encode_madd_diff_lr
- Formal: ∀ which ∈ {0..3}, a,b,c ∈ {0..30}. placing "lr" in slot which of msub xA, xB, xC, xD (other slots GPR x0-x30) ⇒ encode_msub = llvm-mc of that asm.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: differential
predicate:
  quantifier: forall
  vars: [which, a, b, c]
  domain:
    which: 0..3
    a: 0..30
    b: 0..30
    c: 0..30
  relation:
    op: eq
    lhs: encode_msub(ops_with_lr_at(which, a, b, c))
    rhs: llvm_mc(msub_asm_with_lr_at(which, a, b, c))
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  c: { gen: int, min: 0, max: 30, type: u32 }
evidence: encoder/mod.rs:135 parse_reg_num lr => 30; llvm-mc lr alias
```

## encode_msub_neg_too_few
- Tier: 5
- Rationale: llvm-mc "too few operands for instruction" for arity < 4. get_reg on missing index returns Err. Documented ARM 4-operand form. Stronger oracles do not apply to the invalid domain. Doc evidence: ARM ARM four-register MSUB; llvm-mc rejects `msub x0, x1, x2`.
- Seed: encode_madd_pbt::encode_madd_neg_too_few
- Formal: ∀ n ∈ {0..3}, ops a length-n register list. encode_msub(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, r0, r1, r2, is_64]
  domain:
    n: 0..3
    r0: 0..31
    r1: 0..31
    r2: 0..31
    is_64: bool
  relation:
    op: throws
    expr: encode_msub(ops[..n])
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
  r0: { gen: int, min: 0, max: 31, type: u32 }
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: ARM ARM MSUB four registers; llvm-mc too few operands
```

## encode_msub_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc "invalid operand for instruction" for a fifth operand. ARM ARM MSUB has exactly four registers. The encoder must reject extra operands rather than silently drop them. Doc evidence: ARM ARM four-register form; llvm-mc rejects `msub x0, x1, x2, x3, x4`.
- Seed: encode_madd_pbt::encode_madd_neg_extra_operand
- Formal: ∀ rd,rn,rm,ra ∈ {0..30}, is_64 ∈ {false,true}, extra ∈ {Reg, Imm, Shift}. encode_msub([Rd,Rn,Rm,Ra,extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, is_64=false, extra=Reg("x0") i.e. msub w0, w0, w0, w0, x0
- Bug report: pbt-out/bug_reports/encode_msub_extra_operand.md

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_64, extra]
  domain:
    rd: 0..30
    rn: 0..30
    rm: 0..30
    ra: 0..30
    is_64: bool
    extra: Reg or Imm or Shift
  relation:
    op: throws
    expr: encode_msub([Rd, Rn, Rm, Ra, extra])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, variants: [{ gen: const, value: "Reg(x0)" }, { gen: const, value: "Imm(0)" }, { gen: const, value: "Shift(lsl,0)" }] }
expected_error: String
evidence: ARM ARM MSUB four registers; llvm-mc invalid operand for fifth
```

## encode_msub_neg_mixed_width
- Tier: 5
- Rationale: ARM MSUB uses a single sf bit for all four registers. llvm-mc rejects mixed X/W (`msub x0, w1, x2, x3`). The encoder must Err when Rd/Rn/Rm/Ra are not all W or all X. Doc evidence: ARM ARM single sf; llvm-mc "invalid operand".
- Seed: encode_madd_pbt::encode_madd_neg_mixed_width
- Formal: ∀ rd,rn,rm,ra ∈ {0..30}, width flags not all equal. encode_msub of mixed X/W names is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, rd64=false, rn64=false, rm64=false, ra64=true i.e. msub w0, w0, w0, x0
- Bug report: pbt-out/bug_reports/encode_msub_mixed_width.md

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, rd64, rn64, rm64, ra64]
  domain:
    rd: 0..30
    rn: 0..30
    rm: 0..30
    ra: 0..30
    rd64: bool
    rn64: bool
    rm64: bool
    ra64: bool
  relation:
    op: throws
    expr: encode_msub(mixed_width_ops)
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
evidence: ARM ARM single sf bit; llvm-mc rejects mixed X/W MSUB
```

## encode_msub_neg_sp
- Tier: 5
- Rationale: ARM MSUB encoding uses register 31 as WZR/XZR, never WSP/SP. llvm-mc rejects `msub wsp, w0, w0, w0` and `msub sp, x0, x1, x2`. Strengthening round after extra-operand / mixed-width failures. Doc evidence: ARM ARM register 31 is ZR; llvm-mc "invalid operand".
- Seed: encode_madd_pbt::encode_madd_neg_sp
- Formal: ∀ which ∈ {0..3}, is_64 ∈ {false,true}, a,b,c ∈ {0..30}. placing sp/wsp in slot which of an otherwise-valid 4-GPR MSUB ⇒ encode_msub is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0, c=0 i.e. msub wsp, w0, w0, w0
- Bug report: pbt-out/bug_reports/encode_msub_sp.md

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b, c]
  domain:
    which: 0..3
    is_64: bool
    a: 0..30
    b: 0..30
    c: 0..30
  relation:
    op: throws
    expr: encode_msub(ops_with_sp_at(which, is_64, a, b, c))
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  c: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM register 31 is ZR not SP; llvm-mc rejects msub wsp / msub sp
```

## encode_msub_neg_fp
- Tier: 5
- Rationale: Integer MSUB is GPR-only. llvm-mc rejects `msub d0, x1, x2, x3`. parse_reg_num accepts d/s/q/v/h/b prefixes. Strengthening round. Doc evidence: ARM ARM GPR-only 3-source; llvm-mc "invalid operand".
- Seed: encode_madd_pbt::encode_madd_neg_fp
- Formal: ∀ which ∈ {0..3}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. placing prefixN in slot which of msub x0,x1,x2,x3 ⇒ encode_msub is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 i.e. msub d0, x1, x2, x3
- Bug report: pbt-out/bug_reports/encode_msub_fp_reg.md

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain:
    which: 0..3
    prefix: {d, s, q, v, h, b}
    n: 0..31
  relation:
    op: throws
    expr: encode_msub(ops_with_fp_at(which, prefix, n))
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  prefix: { gen: oneof, values: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM GPR-only MSUB; llvm-mc rejects FP/SIMD names
```

## encode_msub_neg_invalid_reg
- Tier: 5
- Rationale: parse_reg_num returns None for x32/w32/foo/empty/r0/x/x-1, so get_reg Errs. llvm-mc also rejects these names. Documented invalid-register path. Doc evidence: encoder/mod.rs:131-147 parse_reg_num; llvm-mc.
- Seed: encode_madd_pbt::encode_madd_neg_invalid_reg
- Formal: ∀ which ∈ {0..3}, bad ∈ {x32,w32,x99,w99,"",foo,r0,x,x-1}. encode_msub with bad in slot which is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..3
    bad: {x32, w32, x99, w99, empty, foo, r0, x, x-1}
  relation:
    op: throws
    expr: encode_msub(ops_with_bad_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  bad: { gen: oneof, values: ["x32", "w32", "x99", "w99", "", "foo", "r0", "x", "x-1"] }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num None for invalid names
```

## encode_msub_neg_non_register
- Tier: 5
- Rationale: get_reg requires Operand::Reg; Imm/Symbol/Mem/Shift/Cond/Label at any of the four slots must Err. llvm-mc rejects non-register MSUB operands. Doc evidence: encoder/mod.rs:956-965 get_reg; ARM ARM four-register form.
- Seed: encode_madd_pbt::encode_madd_neg_non_register
- Formal: ∀ which ∈ {0..3}, bad ∈ {Imm, Symbol, Mem, Shift, Cond, Label}. encode_msub with bad in slot which is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_msub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..3
    bad: Imm or Symbol or Mem or Shift or Cond or Label
  relation:
    op: throws
    expr: encode_msub(ops_with_nonreg_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  bad: { gen: oneof, variants: [{ gen: const, value: "Imm" }, { gen: const, value: "Symbol" }, { gen: const, value: "Mem" }, { gen: const, value: "Shift" }, { gen: const, value: "Cond" }, { gen: const, value: "Label" }] }
expected_error: String
evidence: encoder/mod.rs:956-965 get_reg expected register; ARM ARM four-register MSUB
```
