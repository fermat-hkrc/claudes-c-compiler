# Properties: encode_sxtw

## encode_sxtw_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc (independent AArch64 assembler). README claims the built-in assembler "accepts the same textual assembly that GCC's gas would consume"; encoder docstring claims 32-bit AArch64 words. State machine rejected: no lifecycle. Round-trip rejected: no in-tree SXTW decoder. encode_sxth/sxtb/uxtw fail same-job gate (different imms/opc). encode_sbfm is same-crate shared get_reg, so alias equality is metamorphic not differential.
- Seed: src/backend/arm/assembler/README.md:217; encoder/mod.rs:293; neighbouring encode_sxth_pbt; llvm-mc KAT `sxtw x0, w1` = 0x93407c20
- Formal: ∀ rd, rn ∈ {0..31}. encode_sxtw([Reg(x{rd}|xzr|lr), Reg(w{rn}|wzr)]) = llvm-mc("sxtw Xd, Wn") as little-endian u32
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_sxtw([Reg(xreg(rd)), Reg(wreg(rn))])
    rhs: llvm_mc_word("sxtw " + xreg(rd) + ", " + wreg(rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:8-12; encoder/mod.rs:1-7; encoder/mod.rs:293
```

## encode_sxtw_alias_sbfm
- Tier: 4
- Rationale: ARM ARM C6 documents SXTW Xd,Wn as the alias of SBFM Xd,Xn,#0,#31. encode_sbfm is the in-tree same-format sibling (not independent — shared get_reg), so this is algebraic.metamorphic, not differential. Cross-checked against llvm-mc which prints sbfm x0,x1,#0,#31 as sxtw x0,w1.
- Seed: data_processing.rs:856 comment "SXTW Xd, Wn -> SBFM Xd, Xn, #0, #31"; llvm-mc alias
- Formal: ∀ rd, rn ∈ {0..31}. encode_sxtw([Reg(x{rd}), Reg(w{rn})]) = encode_sbfm([Reg(x{rd}), Reg(x{rn}), Imm(0), Imm(31)]) = llvm-mc("sxtw Xd, Wn")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_sxtw([Reg(xreg(rd)), Reg(wreg(rn))])
    rhs: encode_sbfm([Reg(xreg(rd)), Reg(xreg(rn)), Imm(0), Imm(31)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:856; ARM ARM C6 SXTW alias of SBFM #0,#31; llvm-mc sbfm x0,x1,#0,#31 encodes as sxtw
```

## encode_sxtw_arm_fields
- Tier: 4
- Rationale: ARM ARM SBFM/SXTW field layout is an exact structural invariant of every success-path word. Weaker than differential (which already checks the whole word) but pins each named field so a packing slip is local. Documented bounds Rd/Rn 0..31 and imms=31 / immr=0 sampled exactly.
- Seed: ARM ARM C6 SXTW; llvm-mc encoding [0x20,0x7c,0x40,0x93] for x0,w1
- Formal: ∀ rd, rn ∈ {0..31}. let w = encode_sxtw([Reg(x{rd}), Reg(w{rn})]). w = 0x93407C00 | (rn<<5) | rd ∧ w[31]=1 ∧ w[30:29]=00 ∧ w[28:23]=100110 ∧ w[22]=1 ∧ w[21:16]=0 ∧ w[15:10]=31 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_sxtw([Reg(xreg(rd)), Reg(wreg(rn))])
    rhs: 0x93407C00 | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 SXTW = SBFM sf=1 N=1 immr=0 imms=31; llvm-mc sxtw x0,w1 = 0x93407c20
```

## encode_sxtw_neg_arity
- Tier: 4
- Rationale: llvm-mc and GNU as reject SXTW with fewer than 2 operands ("too few operands"). get_reg returns Err when the slot is missing. Negative/error contract for the documented 2-operand form.
- Seed: llvm-mc `sxtw` / `sxtw x0` → too few operands; encode_sxth_pbt arity property
- Formal: ∀ ops. |ops| < 2 ⇒ encode_sxtw(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_lists_of_len_0_or_1 }
  relation:
    op: throws
    expr: encode_sxtw(ops)
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: llvm-mc -triple=aarch64 `sxtw` / `sxtw x0` error too few operands
```

## encode_sxtw_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a third operand (`sxtw x0, w1, x2` → invalid operand). GNU-style SXTW has exactly two register operands. The assembler must Err rather than silently ignore extras.
- Seed: llvm-mc probe; encode_sxth_pbt extra-operand property
- Formal: ∀ rd, rn ∈ {0..31}, extra ∈ Operand. encode_sxtw([Reg(x{rd}), Reg(w{rn}), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=Reg("x0") — sxtw x0, w0, x0
- Bug report: pbt-out/bug_reports/encode_sxtw_extra_operand.md

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_sxtw([Reg(xreg(rd)), Reg(wreg(rn)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: [{gen: const, value: Imm(0)}, {gen: const, value: "Reg(x0)"}] }
expected_error: String
evidence: llvm-mc -triple=aarch64 `sxtw x0, w1, x2` error invalid operand
```

## encode_sxtw_neg_wd
- Tier: 4
- Rationale: ARM ARM SXTW has only the 64-bit dest form SXTW Xd, Wn. llvm-mc rejects `sxtw w0, w1` and `sxtw w0, x1`. 32-bit dest is outside the valid domain.
- Seed: llvm-mc probe; ARM ARM C6 SXTW <Xd>, <Wn>
- Formal: ∀ rd, rn ∈ {0..31}, src64 ∈ {false,true}. encode_sxtw([Reg(w{rd}), Reg(w/x{rn})]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, src64=false — sxtw w0, w0
- Bug report: pbt-out/bug_reports/encode_sxtw_wd.md

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, src64]
  domain: { rd: 0..31, rn: 0..31, src64: bool }
  relation:
    op: throws
    expr: encode_sxtw([Reg(wreg(rd)), Reg(gpr(src64, rn))])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  src64: { gen: bool }
expected_error: String
evidence: ARM ARM C6 SXTW <Xd>, <Wn> only; llvm-mc `sxtw w0, w1` error invalid operand
```

## encode_sxtw_neg_sp
- Tier: 4
- Rationale: ARM ARM SBFM/SXTW register 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `sxtw sp, w0` and `sxtw x0, sp` / `sxtw x0, wsp`.
- Seed: llvm-mc probe; parse_reg_num maps sp/wsp to 31
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other a valid SXTW GPR. encode_sxtw with SP/WSP at slot `which` is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64_sp=false, a=0 — sxtw wsp, w0
- Bug report: pbt-out/bug_reports/encode_sxtw_sp.md

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64_sp, a]
  domain: { which: 0..1, is_64_sp: bool, a: 0..30 }
  relation:
    op: throws
    expr: encode_sxtw(ops_with_sp_at(which))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64_sp: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM SBFM Rd/Rn are XZR not SP; llvm-mc `sxtw sp, w0` / `sxtw x0, wsp` error
```

## encode_sxtw_neg_fp
- Tier: 4
- Rationale: SXTW operands are GPRs. llvm-mc rejects FP/SIMD names (d/s/q/v/h/b). parse_reg_num currently accepts those prefixes, so the encoder must still reject them for this mnemonic.
- Seed: llvm-mc `sxtw d0, w1` / `sxtw x0, s1` error; encode_sxth_pbt FP property
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_sxtw with FP name at slot `which` is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — sxtw d0, w1
- Bug report: pbt-out/bug_reports/encode_sxtw_fp.md

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_sxtw(ops_with_fp_at(which, prefix, n))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, items: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 `sxtw d0, w1` / `sxtw x0, s1` error invalid operand
```

## encode_sxtw_diff_alt_spellings
- Tier: 2
- Rationale: Coverage-sweep of documented GNU-style spellings llvm-mc accepts: x31/w31, XZR, LR, uppercase, and Xd,Xn (canonicalized to Xd,Wn). Same differential oracle as encode_sxtw_diff_valid_gpr.
- Seed: llvm-mc probe sxtw x31,w31 / sxtw X0,W1 / sxtw x0,x1; encode_sxth_pbt alt-spellings
- Formal: ∀ rd, rn ∈ {0..31}, dest ∈ {xN, x31, XZR, LR, uppercase}, src ∈ {wN, w31, uppercase, xN}. encode_sxtw([Reg(dest), Reg(src)]) = llvm-mc("sxtw dest, src")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, dest_spell: 0..4, src_spell: 0..3 }
  relation:
    op: eq
    lhs: encode_sxtw([Reg(dest), Reg(src)])
    rhs: llvm_mc_word("sxtw " + dest + ", " + src)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc accepts x31/XZR/LR/uppercase/Xd,Xn; README GNU-style assembly
```

## encode_sxtw_neg_nonreg
- Tier: 4
- Rationale: Coverage-sweep: non-register Operand kinds at a GPR slot must Err (get_reg expects Operand::Reg). llvm-mc rejects immediates/shifts/mem/labels at SXTW register slots.
- Seed: encode_sxth_pbt nonreg; get_reg "expected register"
- Formal: ∀ which ∈ {0,1}, bad ∈ Operand \ Reg. encode_sxtw with `bad` at slot `which` is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non_reg_operand }
  relation:
    op: throws
    expr: encode_sxtw(ops_with_nonreg_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: get_reg encoder/mod.rs:956 expected register; llvm-mc rejects non-reg SXTW operands
```

## encode_sxtw_neg_invalid_name
- Tier: 4
- Rationale: Coverage-sweep: invalid register names (foo, x32, w32, x, r0, empty) must Err. parse_reg_num returns None outside x/w 0..31 and aliases.
- Seed: encode_sxth_pbt invalid_name; parse_reg_num
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_sxtw with Reg(name) at slot `which` is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_sxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_gpr_name }
  relation:
    op: throws
    expr: encode_sxtw(ops_with_name_at(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: parse_reg_num encoder/mod.rs:131 returns None for these names; llvm-mc rejects them
```
