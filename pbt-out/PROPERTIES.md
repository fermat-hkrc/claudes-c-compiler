# Properties: encode_umulh

## encode_umulh_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc. README claims the assembler "accepts the same textual assembly that GCC's gas would consume"; llvm-mc -triple=aarch64 is an independent GNU-style AArch64 assembler. State machine rejected: no lifecycle. Round-trip rejected: no in-tree UMULH decoder. encode_smulh fails the same-job gate (signed vs unsigned). Doc evidence: README.md:12; encoder/mod.rs:1-7; encoder/mod.rs:274; ARM ARM Data-processing (3 source) UMULH.
- Seed: neighbouring encode_umaddl_pbt encode_umaddl_diff_valid_gpr; README.md:214
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_umulh([Reg(xreg(rd)), Reg(xreg(rn)), Reg(xreg(rm))]) = Word(llvm-mc("umulh Xd, Xn, Xm"))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31" }
  relation:
    op: eq
    lhs: encode_umulh([Reg(xreg(rd)), Reg(xreg(rn)), Reg(xreg(rm))])
    rhs: llvm_mc_word("umulh Xd, Xn, Xm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:12 assembler accepts the same textual assembly that GCC's gas would consume; encoder/mod.rs:274 umulh dispatch; ARM ARM UMULH Xd,Xn,Xm
```

## encode_umulh_xor_smulh_u_bit
- Tier: 4c
- Rationale: Algebraic metamorphic. ARM ARM Data-processing (3 source) UMULH vs SMULH differ only in U (bit 23 / op31[2]). encode_smulh is a same-TU sibling with a different job (signed high multiply), so it is not a differential reference; the documented U-bit relation is a metamorphic transform. Stronger oracles: state machine no; round-trip no decoder; differential covered by encode_umulh_diff_valid_gpr.
- Seed: neighbouring encode_umaddl_xor_smaddl_u_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_umulh([Xd,Xn,Xm]) XOR encode_smulh([Xd,Xn,Xm]) = 1<<23
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31" }
  relation:
    op: eq
    lhs: encode_umulh(ops) XOR encode_smulh(ops)
    rhs: 1u32 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) UMULH op31=110 vs SMULH op31=010; llvm-mc umulh x0,x1,x2 XOR smulh x0,x1,x2 = 0x00800000
```

## encode_umulh_arm_fields
- Tier: 4d
- Rationale: Algebraic invariant of the ARM ARM bit layout for UMULH: sf=1, bits[30:21]=00 11011 110, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Stronger oracles covered separately. Doc evidence: data_processing.rs:691 comment; ARM ARM UMULH encoding.
- Seed: neighbouring encode_umaddl_arm_fields; data_processing.rs:691
- Formal: ∀ rd,rn,rm ∈ {0..31}. let w = encode_umulh([Xd,Xn,Xm]). w = 0x9BC07C00 | (rm<<16) | (rn<<5) | rd ∧ (w>>31)=1 ∧ ((w>>21)&0x3FF)=0b0011011110 ∧ ((w>>15)&1)=0 ∧ ((w>>10)&0x1F)=31
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31" }
  relation:
    op: eq
    lhs: encode_umulh([Reg(xreg(rd)), Reg(xreg(rn)), Reg(xreg(rm))])
    rhs: 0x9BC07C00 | (rm << 16) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:691 UMULH 1 00 11011 1 10 Rm 0 11111 Rn Rd; ARM ARM Data-processing (3 source)
```

## encode_umulh_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects UMULH with fewer than 3 operands ("too few operands"). get_reg on a missing index returns Err. Stronger oracles do not apply to the invalid domain. Doc evidence: llvm-mc probe `umulh x0, x1` errors; ARM ARM syntax UMULH Xd, Xn, Xm (3 registers).
- Seed: neighbouring encode_umaddl_neg_arity
- Formal: ∀ ops. |ops| < 3 ⇒ encode_umulh(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len 0..=2 of Operand" }
  relation:
    op: throws
    expr: encode_umulh(ops)
generators:
  len: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: llvm-mc umulh x0, x1 -> too few operands; ARM ARM UMULH Xd,Xn,Xm is a 3-register instruction
```

## encode_umulh_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects a 4th operand ("invalid operand"). GNU-style UMULH has exactly 3 registers. Stronger oracles do not apply. Doc evidence: llvm-mc probe `umulh x0, x1, x2, x3` errors; README.md:12 gas-compatible assembly.
- Seed: neighbouring encode_umaddl_neg_extra_operand
- Formal: ∀ rd,rn,rm ∈ {0..31}. ∀ extra ∈ Operand. encode_umulh([Xd,Xn,Xm, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=Reg("x0")  (umulh x0, x0, x0, x0)
- Bug report: pbt-out/bug_reports/encode_umulh_extra_operand.md

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31", extra: Operand }
  relation:
    op: throws
    expr: encode_umulh([Xd, Xn, Xm, extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: ["Reg", "Imm", "Shift"] }
expected_error: String
evidence: llvm-mc umulh x0, x1, x2, x3 -> invalid operand; ARM ARM UMULH takes exactly Xd,Xn,Xm
```

## encode_umulh_neg_wrong_width
- Tier: 4e
- Rationale: Negative/error contract. ARM ARM and llvm-mc require 64-bit GPRs only; any W register is "invalid operand". encode_umulh discards is_64 so this is the documented-width contract, not a restatement of the body. Doc evidence: llvm-mc `umulh w0, w1, w2` / mixed W errors; ARM ARM UMULH Xd, Xn, Xm.
- Seed: neighbouring encode_umaddl_neg_wrong_width
- Formal: ∀ rd,rn,rm ∈ {0..30}. ∀ rd64,rn64,rm64 ∈ Bool. ¬(rd64 ∧ rn64 ∧ rm64) ⇒ encode_umulh([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=false  (umulh w0, w0, w0)
- Bug report: pbt-out/bug_reports/encode_umulh_wrong_width.md

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: "0..=30", rn: "0..=30", rm: "0..=30", rd64: bool, rn64: bool, rm64: bool }
  relation:
    op: throws
    expr: encode_umulh([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: llvm-mc umulh w0,w1,w2 and mixed W/X all error; ARM ARM UMULH Xd,Xn,Xm (no 32-bit form)
```

## encode_umulh_neg_sp
- Tier: 4e
- Rationale: Negative/error contract. ARM ARM register 31 in this format is XZR, never SP. llvm-mc rejects SP/WSP in any UMULH slot. parse_reg_num maps sp/wsp to 31, which is the producing statement, not the contract. Doc evidence: llvm-mc `umulh sp, x1, x2` errors; ARM ARM UMULH uses XZR for 31.
- Seed: neighbouring encode_umaddl_neg_sp
- Formal: ∀ which ∈ {0,1,2}. ∀ is_64 ∈ Bool. ∀ a,b ∈ {0..30}. encode_umulh(ops with slot `which` = SP/WSP) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0  (umulh wsp, x0, x0)
- Bug report: pbt-out/bug_reports/encode_umulh_sp_as_zr.md

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain: { which: "0..=2", is_64: bool, a: "0..=30", b: "0..=30" }
  relation:
    op: throws
    expr: encode_umulh(ops_with_sp_at(which))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc umulh sp,x1,x2 / umulh x0,sp,x2 / umulh x0,x1,sp all error; ARM ARM register 31 is XZR not SP
```

## encode_umulh_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc over documented alternate GNU spellings of the same GPRs: x31/XZR/LR/uppercase. README gas-compatible contract. Documented bounds 0 and 31 sampled exactly via 0..=31. Stronger oracles rejected as in encode_umulh_diff_valid_gpr.
- Seed: neighbouring encode_umaddl_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}. ∀ dest/src spellings in {xN, x31, XZR, LR, uppercase}. encode_umulh(spelled regs) = Word(llvm-mc("umulh " + spelled))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, dest_spell, src_n_spell, src_m_spell]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31", dest_spell: "0..=4", src_n_spell: "0..=4", src_m_spell: "0..=4" }
  relation:
    op: eq
    lhs: encode_umulh([Reg(dest), Reg(src_n), Reg(src_m)])
    rhs: llvm_mc_word("umulh dest, src_n, src_m")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_n_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_m_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: llvm-mc accepts x31, XZR, LR, uppercase UMULH X0,X1,X2; README.md:12 gas-compatible; parse_reg_num maps lr->30, xzr->31, x31->31
```

## encode_umulh_neg_fp
- Tier: 4e
- Rationale: Sweep (coverage_gaps had no profraw; manual arm audit of get_reg / parse_reg_num prefixes). Negative/error contract: FP/SIMD names (d/s/q/v/h/b) are not UMULH operands. llvm-mc rejects them. parse_reg_num accepting those prefixes is the producing statement, not Doc evidence. Doc evidence: llvm-mc `umulh d0, d1, d2` errors; ARM ARM UMULH Xd,Xn,Xm; Operand::Reg docs list FP names as a different register class.
- Seed: neighbouring encode_umaddl_neg_fp
- Formal: ∀ which ∈ {0,1,2}. ∀ prefix ∈ {d,s,q,v,h,b}. ∀ n ∈ {0..31}. encode_umulh(ops with slot `which` = prefix+n) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0  (umulh d0, x1, x2)
- Bug report: pbt-out/bug_reports/encode_umulh_fp_as_gpr.md

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: "0..=2", prefix: "{d,s,q,v,h,b}", n: "0..=31" }
  relation:
    op: throws
    expr: encode_umulh(ops_with_fp_at(which))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc umulh d0,d1,d2 / s0,s1,s2 error; ARM ARM UMULH Xd,Xn,Xm
```

## encode_umulh_neg_nonreg
- Tier: 4e
- Rationale: Sweep. Negative/error contract: a non-Reg operand at any of the three GPR slots must Err. get_reg returns Err on Imm/Shift/Mem/Label/Symbol/Cond/RegArrangement. llvm-mc has no UMULH form with those operand kinds. Doc evidence: get_reg "expected register"; ARM ARM 3-register syntax.
- Seed: neighbouring encode_umaddl_neg_nonreg
- Formal: ∀ which ∈ {0,1,2}. ∀ bad ∉ Reg. encode_umulh(ops with slot `which` = bad) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: "0..=2", bad: "non-Reg Operand" }
  relation:
    op: throws
    expr: encode_umulh(ops_with_nonreg_at(which))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956 get_reg expected register; ARM ARM UMULH Xd,Xn,Xm
```

## encode_umulh_neg_invalid_name
- Tier: 4e
- Rationale: Sweep. Negative/error contract: invalid register names (foo, x32, w32, x, r0, empty) must Err. parse_reg_num returns None; llvm-mc rejects them. Doc evidence: parse_reg_num Option; llvm-mc invalid operand.
- Seed: neighbouring encode_umaddl_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2}. ∀ name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_umulh(ops with slot `which` = Reg(name)) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_umulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: "0..=2", name: "invalid register names" }
  relation:
    op: throws
    expr: encode_umulh(ops_with_invalid_name_at(which))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: parse_reg_num returns None for foo/x32/w32/x/r0/empty; llvm-mc rejects invalid register names
```
