# Properties: encode_adc

## encode_adc_diff_gpr_same_width
- Tier: 2
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler). State machine rejected — encode_adc is a pure function with no lifecycle. Algebraic round-trip rejected — no in-tree ADC decoder. Same-job sibling encode_sbc implements SBC (op=1), not ADC. SUT-boundary: internal-helper of the GNU-style assembler; encode_instruction dispatches adc/adcs here with parsed operands. Argument mapping: (Rd,Rn,Rm,set_flags) ↔ `adc`/`adcs` Rd, Rn, Rm. Shared contract: assembler README "accepts the same textual assembly that GCC's gas would consume" plus encoder "Encodes AArch64 instructions into 32-bit machine code words".
- Seed: src/backend/arm/codegen/i128_ops.rs:46 (`adc x1, x1, xzr`) and :68 (`adc x1, x3, x5`)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ bool, set_flags ∈ bool. Let names be xN/wN with 31 → xzr/wzr. encode_adc([Reg(Rd),Reg(Rn),Reg(Rm)], set_flags) = Word(w) ∧ llvm-mc("adc(s) Rd, Rn, Rm") = w.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_adc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_adc([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))], set_flags)
    rhs: llvm_mc(asm_adc(is_64, set_flags, rd, rn, rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 (gas-compatible textual assembly); encoder/mod.rs:1-7 (AArch64 32-bit encoding); encoder/mod.rs:281-282 (adc/adcs dispatch)
```

## encode_adc_metamorphic_s_bit
- Tier: 4c
- Rationale: ARM ARM places the S flag at bit 29 of ADC/ADCS; the two encodings of the same registers must differ only by that bit. Differential is stronger and is a sibling property; this metamorphic check does not depend on llvm-mc and pins the S-bit contract independently. State machine and round-trip rejected as above.
- Seed: encoder/mod.rs:281-282 (`adc` → set_flags=false, `adcs` → set_flags=true)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ bool. encode_adc(ops, true) XOR encode_adc(ops, false) = 1<<29, where both succeed as Word.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_adc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  body: encode_adc(ops, true) XOR encode_adc(ops, false) == (1u32 << 29)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM ADC encoding sf 0 S 11010000 Rm 000000 Rn Rd; encoder/mod.rs:281-282
```

## encode_adc_invariant_arm_fields
- Tier: 4d
- Rationale: ARM ARM field layout for ADC (register): sf at 31, op=0 at 30, S at 29, opcode 11010000 at 28:21, Rm at 20:16, 000000 at 15:10, Rn at 9:5, Rd at 4:0. Differential is stronger (sibling); this invariant checks the documented bit fields against the SUT output without copying encode_adc's expression. Reference-as-full-word was rejected as it would reimplement the encoder; field extraction from the produced word is the ARM ARM contract.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ bool, set_flags ∈ bool. encode_adc([x/w rd,rn,rm], set_flags) = Word(w) ⇒ w[4:0]=rd ∧ w[9:5]=rn ∧ w[20:16]=rm ∧ w[31]=sf(is_64) ∧ w[29]=set_flags ∧ w[30]=0 ∧ w[28:21]=0b11010000 ∧ w[15:10]=0.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_adc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  body: let Word(w) = encode_adc(ops, set_flags) in (w & 0x1F) == rd && ((w >> 5) & 0x1F) == rn && ((w >> 16) & 0x1F) == rm && ((w >> 31) & 1) == sf && ((w >> 29) & 1) == s && ((w >> 30) & 1) == 0 && ((w >> 21) & 0xFF) == 0b11010000 && ((w >> 10) & 0x3F) == 0
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
evidence: ARM ARM ADC (register) encoding; encoder/mod.rs:1-7
```

## encode_adc_neg_too_few_operands
- Tier: 4e
- Rationale: ARM ADC requires three register operands; llvm-mc reports "too few operands for instruction" for `adc x0, x1`. get_reg on a missing index must Err. Documented error path: get_reg returns Err("expected register at operand N, got None"). Negative/error contract. Stronger oracles do not apply to the invalid-arity domain.
- Seed: llvm-mc `adc x0, x1` → error: too few operands
- Formal: ∀ ops with |ops| < 3, set_flags ∈ bool. encode_adc(ops, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, set_flags]
  domain: { ops: sequences of Reg of length 0..2 }
  relation:
    op: throws
    lhs: encode_adc(ops, set_flags)
    rhs: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  set_flags: { gen: bool }
expected_error: String
evidence: ARM ARM ADC requires Rd, Rn, Rm; llvm-mc "too few operands"; get_reg at encoder/mod.rs:956-966
```

## encode_adc_neg_non_register
- Tier: 4e
- Rationale: ARM ADC operands are registers only (no immediate, memory, or shift form). llvm-mc rejects `adc x0, x1, #1`. get_reg returns Err on non-Reg. Stronger oracles do not apply to this invalid domain.
- Seed: llvm-mc `adc x0, x1, #1` → error: invalid operand
- Formal: ∀ is_64, set_flags, which ∈ {0,1,2}, bad ∈ {Imm, Mem, Shift, Symbol, Cond}. encode_adc(ops with ops[which]=bad, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, bad, set_flags]
  relation:
    op: throws
    lhs: encode_adc([Reg(rd), Reg(rn), bad], set_flags)
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  set_flags: { gen: bool }
expected_error: String
evidence: ARM ARM ADC register form only; llvm-mc rejects immediate third operand
```

## encode_adc_neg_extra_shift
- Tier: 4e
- Rationale: ARM ADC has no shifted-register form (unlike ADD). llvm-mc rejects `adc x0, x1, x2, lsl #1` as invalid operand. Silent encoding that ignores the extra operand would assemble a different instruction than the source text. Assembler contract is gas-compatible textual assembly. Negative/error: extra Shift/Extend must Err.
- Seed: llvm-mc `adc x0, x1, x2, lsl #1` → error: invalid operand
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ bool, set_flags ∈ bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ {0..63}. encode_adc([Reg,Reg,Reg,Shift{kind,amt}], set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_64=false, set_flags=false, kind="lsl", amt=0
- Bug report: pbt-out/bug_reports/encode_adc_extra_shift_ignored.md

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, set_flags, kind, amt]
  relation:
    op: throws
    lhs: encode_adc([Reg,Reg,Reg,Shift{kind,amt}], set_flags)
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
  amt: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: ARM ARM ADC has no shift field (bits 15-10 fixed 000000); llvm-mc rejects extra lsl; assembler README.md:5-14
```

## encode_adc_neg_mixed_width
- Tier: 4e
- Rationale: ARM ADC requires all three registers the same width (all W or all X). llvm-mc rejects `adc x0, w1, x2`. A single sf bit taken only from Rd would silently encode a 64-bit ADC from mixed-width text. Negative/error: mixed x/w among the three registers must Err.
- Seed: llvm-mc `adc x0, w1, x2` → error: invalid operand
- Formal: ∀ rd,rn,rm ∈ {0..30}, widths ∈ {W,X}^3 with not-all-equal, set_flags ∈ bool. encode_adc([Reg(w0),Reg(w1),Reg(w2)], set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true, set_flags=false
- Bug report: pbt-out/bug_reports/encode_adc_mixed_width.md

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64, set_flags]
  domain: { not (rd64 == rn64 == rm64) }
  relation:
    op: throws
    lhs: encode_adc([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Reg(gpr(rm64,rm))], set_flags)
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
  set_flags: { gen: bool }
expected_error: String
evidence: ARM ARM ADC <Wd>,<Wn>,<Wm> or <Xd>,<Xn>,<Xm>; llvm-mc mixed-width error; assembler README.md:5-14
```

## encode_adc_neg_sp
- Tier: 4e
- Rationale: ARM ADC encoding uses register 31 as WZR/XZR, not WSP/SP. llvm-mc rejects `adc sp, x0, x1` and `adc x0, sp, x1`. Encoding SP as 31 would silently assemble `adc xzr, ...` — a different instruction. Negative/error: any of Rd/Rn/Rm being sp/wsp must Err.
- Seed: llvm-mc `adc sp, x0, x1` and `adc x0, sp, x1` → error: invalid operand
- Formal: ∀ which ∈ {0,1,2}, is_64 ∈ bool, set_flags ∈ bool, others GPR 0..30. encode_adc(ops with ops[which] = sp/wsp, set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, set_flags=false, a=0, b=0 (ops=[wsp, w0, w0])
- Bug report: pbt-out/bug_reports/encode_adc_sp_as_zr.md

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, set_flags, a, b]
  domain: { which: 0..2, a: 0..30, b: 0..30 }
  relation:
    op: throws
    lhs: encode_adc(ops_with_sp_at(which), set_flags)
    rhs: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  set_flags: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM ADC Rd/Rn/Rm are Wd/Xd not SP; llvm-mc rejects SP; parse_reg_num maps sp→31 which is XZR in this encoding
```

## encode_adc_neg_invalid_reg_name
- Tier: 4e
- Rationale: parse_reg_num rejects names outside x0-x31/w0-w31/sp/xzr/lr. Coverage-sweep gap: get_reg's parse_reg_num None path was not targeted. llvm-mc rejects `adc x32, x0, x1`. Negative/error.
- Seed: llvm-mc `adc x32, x0, x1` → error: invalid operand
- Formal: ∀ which ∈ {0,1,2}, set_flags ∈ bool, name ∈ {x32,w32,x99,w99,"",foo,r0,x,x-1}. encode_adc(ops with ops[which]=Reg(name), set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, set_flags, name]
  relation:
    op: throws
    lhs: encode_adc(ops_with_reg_name_at(which, name), set_flags)
    rhs: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  set_flags: { gen: bool }
expected_error: String
evidence: encoder/mod.rs:131-149 parse_reg_num; llvm-mc rejects x32
```

## encode_adc_neg_fp_reg
- Tier: 4e
- Rationale: ADC is integer data-processing (assembler README Data Processing list, ARM ARM W/X registers). llvm-mc rejects `adc d0, d1, d2`. parse_reg_num accepts d/s/q/v/h/b prefixes, so this path was untested. Coverage-sweep: FP register names at any operand must Err.
- Seed: llvm-mc `adc d0, d1, d2` → error: invalid operand
- Formal: ∀ which ∈ {0,1,2}, set_flags ∈ bool, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_adc(ops with ops[which]=Reg(prefix||n), set_flags) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, set_flags=false, prefix="d", n=0 (ops=[d0, x1, x2])
- Bug report: pbt-out/bug_reports/encode_adc_fp_reg.md

```property
function: encoder.data_processing.encode_adc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, set_flags, prefix, n]
  relation:
    op: throws
    lhs: encode_adc(ops_with_fp_at(which), set_flags)
    rhs: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  set_flags: { gen: bool }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: assembler README.md:214 Data Processing (GPR) vs FP/NEON; ARM ARM ADC Wd/Xd; llvm-mc rejects d/s/v forms
```
