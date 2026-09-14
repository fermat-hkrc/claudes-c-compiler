# Properties: encode_div

## encode_div_diff_gpr_same_width
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc. State machine rejected — encode_div is a pure function with no lifecycle. Algebraic round-trip rejected — no in-tree UDIV/SDIV decoder. Same-job sibling gate: encode_adc/encode_mul implement different opcodes, not UDIV/SDIV. README claims gas-compatible AArch64 text; llvm-mc is an independent assembler of that contract.
- Seed: encode_adc_pbt::encode_adc_diff_gpr_same_width at data_processing.rs (3-GPR twin)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, unsigned ∈ Bool. let names = gpr(is_64, ·) using xzr/wzr for 31. encode_div([Reg(rd),Reg(rn),Reg(rm)], unsigned) = Word(w) ∧ w = llvm-mc("udiv"|"sdiv" rd, rn, rm)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_div
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, unsigned]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31" }
  relation:
    op: eq
    lhs: encode_div([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))], unsigned)
    rhs: llvm_mc_word(mnemonic(unsigned), gpr(is_64,rd), gpr(is_64,rn), gpr(is_64,rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  unsigned: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:272-273 udiv/sdiv dispatch; ARM ARM UDIV/SDIV
```

## encode_div_metamorphic_o1_bit
- Tier: 4
- Rationale: ARM ARM Data-processing (2 source) documents UDIV opcode 000010 vs SDIV opcode 000011, i.e. they differ only by o1 at bit 10. Stronger differential already covers each mnemonic independently; this metamorphic checks the documented sibling transform without copying the SUT body.
- Seed: encode_adc_pbt::encode_adc_metamorphic_s_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool. encode_div(ops, true) XOR encode_div(ops, false) = 1<<10
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_div
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31" }
  relation:
    op: eq
    lhs: encode_div(ops, true) XOR encode_div(ops, false)
    rhs: 1u32 << 10
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: data_processing.rs:623-624 comment "00001 o1"; ARM ARM UDIV opcode 000010 / SDIV opcode 000011
```

## encode_div_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout for Data-processing (2 source) UDIV/SDIV is an exact structural predicate on the output word. Weaker than differential (does not pin the full 32-bit value against an independent assembler) but independently evidenced.
- Seed: encode_adc_pbt::encode_adc_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, unsigned ∈ Bool. let w = encode_div([Reg(gpr(is_64,rd)),…], unsigned). w[4:0]=rd ∧ w[9:5]=rn ∧ w[20:16]=rm ∧ w[31]=sf(is_64) ∧ w[30]=0 ∧ w[29]=0 ∧ w[28:21]=0b11010110 ∧ w[15:11]=0b00001 ∧ w[10]=¬unsigned
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_div
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, unsigned]
  domain: { rd: "0..=31" }
  body: fields of encode_div match ARM ARM UDIV/SDIV layout
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  unsigned: { gen: bool }
evidence: data_processing.rs:624 "Data-processing (2 source): sf 0 S=0 11010110 Rm 00001 o1 Rn Rd"; ARM ARM UDIV/SDIV
```

## encode_div_neg_too_few_operands
- Tier: 4
- Rationale: llvm-mc and ARM ARM require three register operands. get_reg on a missing index returns Err. Documented rejection: llvm-mc "too few operands for instruction".
- Seed: encode_adc_pbt::encode_adc_neg_too_few_operands
- Formal: ∀ n ∈ {0,1,2}, unsigned ∈ Bool, ops a length-n prefix of three GPRs. encode_div(ops, unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, unsigned]
  domain: { n: "0..=2" }
  relation:
    op: throws
    lhs: encode_div(ops[..n], unsigned)
    rhs: Err
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  unsigned: { gen: bool }
evidence: llvm-mc rejects "udiv x0, x1"; get_reg encoder/mod.rs:956-966
```

## encode_div_neg_non_register
- Tier: 4
- Rationale: ARM ARM UDIV/SDIV take only Wt/Xt. get_reg requires Operand::Reg. llvm-mc rejects immediates. Documented error contract for a non-register in any of the three slots.
- Seed: encode_adc_pbt::encode_adc_neg_non_register
- Formal: ∀ which ∈ {0,1,2}, unsigned ∈ Bool, bad ∈ {Imm, Symbol, Mem, Shift, Cond}. encode_div(ops with slot which = bad, unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, unsigned, bad]
  domain: { which: "0..=2" }
  relation:
    op: throws
    lhs: encode_div(ops_with_non_reg, unsigned)
    rhs: Err
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  unsigned: { gen: bool }
evidence: llvm-mc rejects "udiv x0, x1, #1"; get_reg requires Operand::Reg
```

## encode_div_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a fourth operand ("invalid operand for instruction"). ARM ARM UDIV/SDIV have no shifted-register form. The assembler public contract is gas-compatible, so extra operands must be rejected. Bounds: arity exactly 3; 4th operand at bound 3 and 3+1.
- Seed: encode_adc_pbt::encode_adc_neg_extra_shift
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ Bool, unsigned ∈ Bool, extra ∈ {Reg, Shift, Imm}. encode_div([Rd,Rn,Rm,extra], unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_64=false, unsigned=false, extra=Reg("x0")  (sdiv w0, w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_div_extra_operand.md

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, unsigned, extra]
  domain: { rd: "0..=30" }
  relation:
    op: throws
    lhs: encode_div([Rd,Rn,Rm,extra], unsigned)
    rhs: Err
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  extra: { gen: oneof, variants: [Reg, Shift, Imm] }
evidence: llvm-mc rejects "udiv x0, x1, x2, x3"; ARM ARM UDIV/SDIV 3-operand only
```

## encode_div_neg_mixed_width
- Tier: 4
- Rationale: ARM ARM requires same-width GPRs (all W or all X). llvm-mc rejects mixed x/w. sf is documented as the operand-width bit; mixed width is invalid input the public assembler must reject.
- Seed: encode_adc_pbt::encode_adc_neg_mixed_width
- Formal: ∀ rd,rn,rm ∈ {0..30}, rd64,rn64,rm64 ∈ Bool not all equal, unsigned ∈ Bool. encode_div([Reg(gpr(rd64,rd)),…], unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true, unsigned=false  (sdiv w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_div_mixed_width.md

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64, unsigned]
  domain: { "not all widths equal" }
  relation:
    op: throws
    lhs: encode_div(mixed_width_ops, unsigned)
    rhs: Err
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  unsigned: { gen: bool }
evidence: llvm-mc rejects "udiv x0, w1, x2"; ARM ARM UDIV <Wd>,<Wn>,<Wm> / <Xd>,<Xn>,<Xm>
```

## encode_div_neg_sp
- Tier: 4
- Rationale: ARM ARM UDIV/SDIV encode register 31 as XZR/WZR, never SP/WSP. llvm-mc rejects `udiv sp, ...`. parse_reg_num maps both sp and xzr to 31, so the assembler must still reject the SP name.
- Seed: encode_adc_pbt::encode_adc_neg_sp
- Formal: ∀ which ∈ {0,1,2}, is_64 ∈ Bool, unsigned ∈ Bool. encode_div(ops with SP/WSP at slot which, unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, unsigned=false, a=0, b=0  (sdiv wsp, w0, w0)
- Bug report: pbt-out/bug_reports/encode_div_sp.md

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, unsigned]
  domain: { which: "0..=2" }
  relation:
    op: throws
    lhs: encode_div(ops_with_sp, unsigned)
    rhs: Err
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  unsigned: { gen: bool }
evidence: llvm-mc rejects "udiv sp, x0, x1"; ARM ARM register 31 is XZR/WZR for UDIV/SDIV
```

## encode_div_neg_invalid_reg_name
- Tier: 4
- Rationale: Sweep of get_reg/parse_reg_num None arm. llvm-mc rejects x32/w32/foo. get_reg returns Err for parse_reg_num None. Documented invalid-register contract.
- Seed: encode_adc_pbt::encode_adc_neg_invalid_reg_name
- Formal: ∀ which ∈ {0,1,2}, unsigned ∈ Bool, bad ∈ {x32,w32,x99,w99,"",foo,r0,x,x-1}. encode_div(ops with Reg(bad) at slot which, unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, unsigned, bad]
  domain: { which: "0..=2", bad: "{x32,w32,x99,w99,empty,foo,r0,x,x-1}" }
  relation:
    op: throws
    lhs: encode_div(ops_with_bad_name, unsigned)
    rhs: Err
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  unsigned: { gen: bool }
evidence: llvm-mc rejects "sdiv x32, x0, x1"; parse_reg_num encoder/mod.rs:131-148 returns None for num>31 and unknown prefixes
```

## encode_div_neg_fp
- Tier: 4
- Rationale: Sweep of FP/SIMD names accepted by parse_reg_num (d/s/q/v/h/b prefixes). ARM ARM UDIV/SDIV take Wt/Xt only. llvm-mc rejects `udiv d0, ...`. Public assembler contract requires rejection.
- Seed: encode_adc_pbt::encode_adc_neg_fp_reg
- Formal: ∀ which ∈ {0,1,2}, unsigned ∈ Bool, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_div(ops with Reg(prefix+n) at slot which, unsigned) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, unsigned=false, prefix="d", n=0  (sdiv d0, x1, x2)
- Bug report: pbt-out/bug_reports/encode_div_fp_reg.md

```property
function: encoder.data_processing.encode_div
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, unsigned, prefix, n]
  domain: { which: "0..=2", prefix: "{d,s,q,v,h,b}", n: "0..=31" }
  relation:
    op: throws
    lhs: encode_div(ops_with_fp, unsigned)
    rhs: Err
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  unsigned: { gen: bool }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects "udiv d0, x1, x2"; ARM ARM UDIV/SDIV Wt/Xt only; parse_reg_num accepts d/s/q/v/h/b
```
