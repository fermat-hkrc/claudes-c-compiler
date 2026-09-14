# Property ledger: encode_sxth

## encode_sxth_diff_valid_gpr
- Tier: 2
- Rationale: Strongest applicable oracle is Differential against llvm-mc (independent GNU-style AArch64 assembler). README asserts the built-in assembler "accepts the same textual assembly that GCC's gas would consume". State machine rejected: encode_sxth is a pure single-call encoder with no lifecycle. Round-trip rejected: no in-tree SXTH/SBFM decoder. encode_sxtb / encode_uxth fail the same-job gate (imms=7 / UBFM opc=10). encode_sbfm is the ARM ARM alias but shares get_reg and lives in the same crate, so it is algebraic.metamorphic not an independent differential.
- Seed: README.md:217 Extensions table; encoder/mod.rs:294 `"sxth" => encode_sxth`; llvm-mc KAT `sxth w0, w1` = 0x13003c20, `sxth x0, w1` = 0x93403c20
- Formal: ∀ rd,rn ∈ {0..31}, is_64 ∈ {false,true}. encode_sxth([Reg(gpr(is_64,rd)), Reg(W(rn))]) = Word(llvm-mc("sxth {X|W}d, Wn"))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..31, rn: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_sxth([Reg(gpr(is_64, rd)), Reg(wreg(rn))])
    rhs: llvm_mc("sxth " + gpr(is_64, rd) + ", " + wreg(rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:7-14 GNU-style gas-compatible assembler; encoder/mod.rs:1-7; ARM ARM SXTH alias of SBFM #0,#15
```

## encode_sxth_alias_sbfm
- Tier: 4c
- Rationale: ARM ARM documents SXTH as the assembler alias of SBFM Rd, Rn, #0, #15 (N=sf). Not an independent differential (encode_sbfm shares get_reg / same crate). Stronger differential vs llvm-mc is the sibling property above. Metamorphic required by standard tier in addition to differential.
- Seed: ARM ARM C6 SXTH; llvm-mc `sbfm w0, w1, #0, #15` canonicalizes to `sxth w0, w1`
- Formal: ∀ rd,rn ∈ {0..31}, is_64 ∈ {false,true}. encode_sxth([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]) = encode_sbfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(0), Imm(15)])
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..31, rn: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_sxth([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn))])
    rhs: encode_sbfm([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn)), Imm(0), Imm(15)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM C6 SXTH = SBFM #0,#15; llvm-mc canonicalizes sbfm ..., #0, #15 to sxth
```

## encode_sxth_arm_fields
- Tier: 4d
- Rationale: ARM ARM SBFM bit layout is an exact structural predicate on the success-path word: sf at 31, opc=00 at [30:29], 100110 at [28:23], N=sf at 22, immr=0 at [21:16], imms=15 at [15:10], Rn at [9:5], Rd at [4:0]. Weaker than differential / alias metamorphic; still pins the field packing independently of llvm-mc availability.
- Seed: ARM ARM SBFM encoding diagram; llvm-mc KAT 0x13003c20 / 0x93403c20
- Formal: ∀ rd,rn ∈ {0..31}, is_64 ∈ {false,true}. let w = encode_sxth([Reg(gpr(is_64,rd)), Reg(wreg(rn))]). w = (sf<<31) | (0b100110<<23) | (N<<22) | (15<<10) | (rn<<5) | rd with sf=N=is_64
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..31, rn: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_sxth([Reg(gpr(is_64, rd)), Reg(wreg(rn))])
    rhs: ((sf(is_64) << 31) | (0b100110 << 23) | (N(is_64) << 22) | (15 << 10) | (rn << 5) | rd)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM SBFM sf 00 100110 N immr imms Rn Rd with N=sf immr=0 imms=15 for SXTH
```

## encode_sxth_neg_arity
- Tier: 4e
- Rationale: llvm-mc rejects SXTH with fewer than 2 operands ("too few operands"). README gas-compatibility makes that the error contract. get_reg on a missing slot returns Err, which this property pins.
- Seed: llvm-mc `sxth` / `sxth w0` → error: too few operands
- Formal: ∀ ops with |ops| ∈ {0,1}. encode_sxth(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..1 }
  relation:
    op: throws
    lhs: encode_sxth(ops_of_len(len))
    rhs: Err
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects sxth / sxth w0 as too few operands; README.md:7-14 gas-compatible
```

## encode_sxth_neg_extra_operand
- Tier: 4e
- Rationale: ARM ARM SXTH has exactly two register operands. llvm-mc rejects a third operand. README gas-compatibility. Documented bound: arity = 2; extra at bound+1 must Err.
- Seed: llvm-mc `sxth w0, w1, x2` → invalid operand
- Formal: ∀ rd,rn ∈ {0..31}, extra ∈ Operand. encode_sxth([Reg(Wd), Reg(Wn), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_64=false, extra=Reg("x0")  (sxth w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_sxth_extra_operand.md

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    lhs: encode_sxth([Reg(wreg(rd)), Reg(wreg(rn)), extra])
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift] }
expected_error: String
evidence: llvm-mc rejects sxth w0, w1, x2; ARM ARM SXTH is two-operand; README.md:7-14
```

## encode_sxth_neg_wd_xn
- Tier: 4e
- Rationale: ARM ARM SXTH assembler syntax is Wd,Wn or Xd,Wn. llvm-mc rejects W dest with X source (`sxth w0, x1`). Dest-X with source-X is accepted (canonicalizes to Wn) and is covered by the differential / alias properties, not this negative contract.
- Seed: llvm-mc `sxth w0, x1` → invalid operand for instruction
- Formal: ∀ rd,rn ∈ {0..31}. encode_sxth([Reg(W(rd)), Reg(X(rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0  (sxth w0, x0)
- Bug report: pbt-out/bug_reports/encode_sxth_wd_xn.md

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    lhs: encode_sxth([Reg(wreg(rd)), Reg(xreg(rn))])
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects sxth w0, x1; ARM ARM SXTH <Wd>, <Wn> (source of 32-bit form is Wn)
```

## encode_sxth_neg_sp
- Tier: 4e
- Rationale: ARM ARM SXTH uses GPR encodings where register 31 is WZR/XZR, never SP/WSP. llvm-mc rejects SP/WSP in either slot. README gas-compatibility.
- Seed: llvm-mc `sxth sp, w1` / `sxth wsp, w1` / `sxth x0, sp` / `sxth x0, wsp` → invalid operand
- Formal: ∀ which ∈ {0,1}, is_64_sp ∈ {false,true}, a ∈ {0..30}. ops with SP/WSP at slot which ⇒ encode_sxth(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64_sp=false, a=0, dest64=false  (sxth wsp, w0)
- Bug report: pbt-out/bug_reports/encode_sxth_sp.md

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64_sp, a]
  domain: { which: 0..1, is_64_sp: bool, a: 0..30 }
  relation:
    op: throws
    lhs: encode_sxth(ops_with_sp_at(which))
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64_sp: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects SP/WSP as SXTH operands; ARM ARM register 31 is ZR not SP for SBFM/SXTH
```

## encode_sxth_neg_fp
- Tier: 4e
- Rationale: SXTH is a GPR bitfield alias. llvm-mc rejects FP/SIMD names (d/s/q/v/h/b) in either slot. parse_reg_num currently accepts those prefixes — this property asserts the gas-compatible rejection.
- Seed: llvm-mc `sxth d0, w1` / `sxth w0, d1` → invalid operand
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. ops with prefixN at slot which ⇒ encode_sxth(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0  (sxth d0, w1)
- Bug report: pbt-out/bug_reports/encode_sxth_fp.md

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    lhs: encode_sxth(ops_with_fp_at(which, prefix, n))
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects sxth d0, w1 and sxth w0, d1; README.md:7-14 gas-compatible GPR SXTH
```

## encode_sxth_diff_alt_spellings
- Tier: 2
- Rationale: Coverage-sweep (round 1). First differential generator under-sampled x31/w31, uppercase, LR, and the llvm-mc-accepted Xd,Xn form (canonicalizes to Xd,Wn). Same differential oracle and gas-compatibility evidence as encode_sxth_diff_valid_gpr.
- Seed: llvm-mc `sxth x31, w31` / `sxth x0, x1` / uppercase; ARM ARM register 31 is ZR
- Formal: ∀ rd,rn ∈ {0..31}, is_64 ∈ {false,true}, dest/src spellings in {canonical, x31/w31, XZR, LR, uppercase, X-source if is_64}. encode_sxth(ops) = Word(llvm-mc(asm))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, is_64: bool, dest_spell: 0..4, src_spell: 0..3 }
  relation:
    op: eq
    lhs: encode_sxth([Reg(dest_spelling), Reg(src_spelling)])
    rhs: llvm_mc("sxth " + dest_spelling + ", " + src_spelling)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc accepts x31/w31, uppercase, LR, and sxth Xd, Xn; README.md:7-14 gas-compatible
```

## encode_sxth_neg_nonreg
- Tier: 4e
- Rationale: Coverage-sweep (round 1). get_reg requires Operand::Reg at each slot; Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement must Err. Documented by get_reg contract and llvm-mc (non-register tokens are invalid SXTH operands).
- Seed: get_reg encoder/mod.rs:956 "expected register"; llvm-mc rejects non-register SXTH operands
- Formal: ∀ which ∈ {0,1}, bad ∉ Reg. encode_sxth(ops with bad at slot which) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non-Reg Operand }
  relation:
    op: throws
    lhs: encode_sxth(ops_with_nonreg_at(which, bad))
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, variants: [Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement] }
expected_error: String
evidence: encoder/mod.rs:956 get_reg requires Operand::Reg; llvm-mc rejects non-register SXTH operands
```

## encode_sxth_neg_invalid_name
- Tier: 4e
- Rationale: Coverage-sweep (round 1). parse_reg_num rejects names outside x0-x31/w0-w31/xzr/wzr/sp/lr. Documented bound: register number <= 31. Bound+1 (x32/w32) and non-alphabet names must Err.
- Seed: parse_reg_num encoder/mod.rs:131; llvm-mc rejects x32/foo
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, empty, x-1, x99, w}. encode_sxth with Reg(name) at slot which is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid GPR names }
  relation:
    op: throws
    lhs: encode_sxth(ops_with_name_at(which, name))
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, variants: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: parse_reg_num encoder/mod.rs:131 num<=31; llvm-mc rejects x32/foo as SXTH operands
```
