# Properties: encode_smull

## encode_smull_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc (independent AArch64 assembler). State machine rejected: encode_smull is a pure function with no lifecycle. Round-trip rejected: no in-tree SMULL decoder. encode_umull fails the same-job gate (unsigned twin, U=1). SUT-boundary: internal-helper of the GNU-style assembler; public contract is encoding `smull Xd, Wn, Wm`. Doc evidence: README.md:1-14 "accepts the same textual assembly that GCC's gas would consume"; README.md:214 lists smull; ARM ARM SMULL alias of SMADDL Xd,Wn,Wm,XZR; encoder/mod.rs:1-7.
- Seed: README.md:214 Data Processing table; encoder/mod.rs:247-256 dispatch; data_processing.rs:630 docstring. (none existing unit test)
- Formal: ∀ rd,rn,rm ∈ {0..31}, dest ∈ {xN, xzr if rd=31, lr if rd=30}. encode_smull([Reg(dest), Reg(wN rn), Reg(wN rm)]) = llvm-mc("smull dest, Wn, Wm") as a little-endian u32 word.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_smull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smull([Reg(x(rd)), Reg(w(rn)), Reg(w(rm))])
    rhs: llvm_mc("smull Xd, Wn, Wm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:1-14; README.md:214; ARM ARM SMULL=SMADDL Ra=XZR; encoder/mod.rs:247-256
```

## encode_smull_alias_smaddl_xzr
- Tier: 3
- Rationale: ARM ARM and the SUT docstring state SMULL Xd, Wn, Wm is the alias of SMADDL Xd, Wn, Wm, XZR. Algebraic metamorphic (alias equality) plus llvm-mc agreement. Stronger differential vs encode_smaddl is rejected as independent differential (shared get_reg / same TU) but the alias equality is an evidenced algebraic law. encode_umull rejected as same-job sibling.
- Seed: data_processing.rs:630 "Encode SMULL Xd, Wn, Wm -> SMADDL Xd, Wn, Wm, XZR"; llvm-mc prints `smull` for `smaddl ..., xzr`.
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_smull([Xrd, Wrn, Wrm]) = encode_smaddl([Xrd, Wrn, Wrm, XZR]) ∧ encode_smull(...) = llvm-mc("smull Xd, Wn, Wm") ∧ encode_smull(...) = llvm-mc("smaddl Xd, Wn, Wm, xzr").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_smull
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smull([Reg(x(rd)), Reg(w(rn)), Reg(w(rm))])
    rhs: encode_smaddl([Reg(x(rd)), Reg(w(rn)), Reg(w(rm)), Reg("xzr")])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:630; ARM ARM SMULL alias of SMADDL with Ra=XZR
```

## encode_smull_xor_umull_u_bit
- Tier: 3
- Rationale: ARM ARM Data-processing (3 source) documents SMULL (U=0) and UMULL (U=1) as the same format differing only at bit 23. Same-job differential rejected (signed vs unsigned). Metamorphic: XOR of the two encodings at equal register numbers is exactly 1<<23.
- Seed: data_processing.rs:635 vs 646 opcode comments (001 vs 101 at bits [23:21]).
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_smull([Xrd, Wrn, Wrm]) XOR encode_umull([Xrd, Wrn, Wrm]) = 1<<23.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_smull
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smull([Reg(x(rd)), Reg(w(rn)), Reg(w(rm))]) XOR encode_umull([Reg(x(rd)), Reg(w(rn)), Reg(w(rm))])
    rhs: 1u32 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) U bit; data_processing.rs:635,646
```

## encode_smull_arm_fields
- Tier: 3
- Rationale: ARM ARM SMADDL/SMULL field layout is an algebraic invariant on every success-path word. Weaker than differential (does not check agreement with an independent assembler) but pins each field so a swapped Rn/Rm cannot hide behind a matching llvm-mc skip.
- Seed: data_processing.rs:635 comment; llvm-mc KAT `smull x0, w1, w2` = 0x9b227c20.
- Formal: ∀ rd,rn,rm ∈ {0..31}. let w = encode_smull([Xrd, Wrn, Wrm]). w[31]=1 ∧ w[30:21]=0b0011011001 ∧ w[20:16]=rm ∧ w[15]=0 ∧ w[14:10]=0b11111 ∧ w[9:5]=rn ∧ w[4:0]=rd. Equivalently w = 0x9B207C00 | (rm<<16) | (rn<<5) | rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_smull
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smull([Reg(x(rd)), Reg(w(rn)), Reg(w(rm))])
    rhs: 0x9B207C00 | (rm << 16) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM SMADDL sf=1 U=0 o0=0 Ra=31; data_processing.rs:635
```

## encode_smull_diff_alt_spellings
- Tier: 2
- Rationale: Coverage-sweep differential. First valid-domain generator spelled ZR as xzr/wzr and used lowercase xN. parse_reg_num lowercases and accepts n=31 as ZR; llvm-mc accepts x31/w31, XZR/WZR, LR, and uppercase Xn/Wn. Same differential contract as encode_smull_diff_valid_gpr; generator skewed to those spellings.
- Seed: parse_reg_num encoder/mod.rs:131-146 (to_lowercase, num<=31); llvm-mc `smull x31, w31, w31` / `smull X0, W1, W2` / `smull LR, W1, W2`.
- Formal: ∀ rd,rn,rm ∈ {0..31}, dest_spell,src_spell covering {x31, XZR, LR, uppercase xN, xzr/xN} × {w31, uppercase wN, wzr/wN}. encode_smull([Reg(dest), Reg(src_n), Reg(src_m)]) = llvm-mc("smull dest, src_n, src_m").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_smull
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, dest_spell: 0..4, src_spell: 0..2 }
  relation:
    op: eq
    lhs: encode_smull([Reg(dest_spell(rd)), Reg(src_spell(rn)), Reg(src_spell(rm))])
    rhs: llvm_mc("smull dest, Wn, Wm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 2, type: u32 }
evidence: encoder/mod.rs:131-146 parse_reg_num; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_smull_neg_arity
- Tier: 4e
- Rationale: GNU as / llvm-mc reject SMULL with fewer than 3 operands ("too few operands"). get_reg on a missing index returns Err. Negative/error contract. Stronger oracles do not apply to the invalid domain.
- Seed: llvm-mc `smull` / `smull x0` / `smull x0, w1` → error: too few operands.
- Formal: ∀ ops with len(ops) ∈ {0,1,2} and slots filled with valid X/W regs or other Operand kinds. encode_smull(ops) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_smull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand lists of length 0..2 }
  relation:
    op: throws
    expr: encode_smull(ops)
expected_error: String
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 2 }
evidence: llvm-mc "too few operands for instruction"; get_reg encoder/mod.rs:956-966
```

## encode_smull_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects a 4th operand on scalar SMULL ("invalid operand for instruction"). The assembler contract is GNU-style assembly. Extra operands must Err, not be silently ignored. get_reg only reads indices 0..2 so the current body ignores extras — this is the law, not a characterizing test of the body.
- Seed: llvm-mc `smull x0, w1, w2, x3` and `smull x0, w1, w2, lsl #0` error.
- Formal: ∀ rd,rn,rm ∈ {0..31}, extra ∈ Operand. encode_smull([Xrd, Wrn, Wrm, extra]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=Reg("x0") — encode_smull returns Ok(Word(0x9b207c00))
- Bug report: pbt-out/bug_reports/encode_smull_extra_operand.md

```property
function: encoder.data_processing.encode_smull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_smull([Reg(x(rd)), Reg(w(rn)), Reg(w(rm)), extra])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Shift] }
evidence: llvm-mc rejects 4th operand; README.md:1-14 GNU-style assembly
```

## encode_smull_neg_wrong_width
- Tier: 4e
- Rationale: ARM ARM SMULL form is Xd, Wn, Wm only. llvm-mc rejects W dest and X sources. Mixed or inverted widths must Err. The body discards is_64 from get_reg and always sets sf=1, so this law is independent of the producing statement.
- Seed: llvm-mc `smull w0, w1, w2` / `smull x0, x1, x2` / `smull x0, w1, x2` error: invalid operand.
- Formal: ∀ rd,rn,rm ∈ {0..30}, rd64,rn64,rm64 ∈ bool. (rd64,rn64,rm64) ≠ (true,false,false) ⇒ encode_smull([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Reg(gpr(rm64,rm))]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=false — encode_smull(w0, w0, w0) returns Ok
- Bug report: pbt-out/bug_reports/encode_smull_wrong_width.md

```property
function: encoder.data_processing.encode_smull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, widths: not (X,W,W) }
  relation:
    op: throws
    expr: encode_smull([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Reg(gpr(rm64,rm))])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
evidence: ARM ARM SMULL Xd,Wn,Wm; llvm-mc invalid operand for W dest / X source
```

## encode_smull_neg_sp_fp_nonreg
- Tier: 4e
- Rationale: ARM ARM register 31 is XZR/WZR, never SP/WSP. FP/SIMD names (d/s/q/v/h/b) are not SMULL GPR operands. Non-register Operand kinds at any of the three slots are not registers. llvm-mc rejects all three classes. parse_reg_num currently maps sp/wsp to 31 and accepts FP prefixes — the law is the assembler contract, not the helper.
- Seed: llvm-mc `smull sp, w1, w2` / `smull x0, wsp, w2` / `smull d0, w1, w2` error.
- Formal: ∀ which ∈ {0,1,2}, bad ∈ {sp,wsp} ∪ {d,s,q,v,h,b}{0..31} ∪ {Imm,Mem,Shift,RegArrangement,Label,Symbol}. encode_smull(ops with slot `which` = bad, other slots valid X/W) = Err. Also ∀ invalid name ∈ {foo, x32, w32, x, r0, ""} encode_smull = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: SP which=0 is_64=false a=0 b=0 names=[wsp, w0, w0] returns Ok; FP which=0 prefix=d n=0 (d0, w1, w2) returns Ok. Sub-tests encode_smull_neg_nonreg and encode_smull_neg_invalid_name passed.
- Bug report: pbt-out/bug_reports/encode_smull_sp_as_zr.md; pbt-out/bug_reports/encode_smull_fp_as_gpr.md

```property
function: encoder.data_processing.encode_smull
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..2, bad: SP/WSP or FP/SIMD name or non-Reg Operand or invalid GPR name }
  relation:
    op: throws
    expr: encode_smull(ops_with_slot(which, bad))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  bad: { gen: oneof, options: [sp, wsp, fp_reg, non_reg, invalid_name] }
evidence: ARM ARM Rd/Rn/Rm are XZR/WZR never SP; llvm-mc rejects SP/FP/non-reg; README.md:1-14
```
