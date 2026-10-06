# Properties: encode_uxth

## encode_uxth_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent GNU-style AArch64 assembler). README.md:12 claims gas-compatible textual assembly; encoder/mod.rs:439 dispatches `"uxth"` to this symbol. State machine rejected (pure function). Round-trip rejected (no in-tree UXTH decoder). Sibling encode_sxth/sxtb/uxtb/uxtw rejected (same-job gate). encode_ubfm rejected as independent differential (shared get_reg / same crate). Valid domain is UXTH Rd, Wn with Rd in W or X (llvm-mc/gas canonicalize X dest to W; encoding is always 32-bit) and Wn a W GPR. Doc evidence: README.md:12, ARM ARM C6 UXTH = UBFM Wd,Wn,#0,#15.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_diff_valid_gpr
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31, dest64 ∈ {0,1}. let dest = GPR(dest64, rd); src = W(rn). encode_uxth([Reg(dest), Reg(src)]) = llvm_mc("uxth dest, src") as Word
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, dest64 = true, use_lr = false (uxth x0, w0); SUT=0xD3403C00 llvm-mc=0x53003C00
- Bug report: bug_reports/encode_uxth_x_dest.md

```property
function: encoder.encode_uxth
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest64]
  domain: { rd: 0..31, rn: 0..31, dest64: bool }
  relation:
    op: eq
    lhs: encode_uxth([Reg(gpr(dest64, rd)), Reg(wreg(rn))])
    rhs: llvm_mc("uxth " + gpr(dest64, rd) + ", " + wreg(rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_uxth_alias_ubfm
- Tier: 4
- Rationale: ARM ARM C6 UXTH is the alias of UBFM Wd, Wn, #0, #15. encode_ubfm is not an independent implementation (shared get_reg / same crate) so this is algebraic.metamorphic, not differential. Restricted to W registers because the 64-bit UBFM Xd,Xn,#0,#15 is UBFX, not a UXTH alias (llvm-mc). Cross-checked against llvm-mc.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_alias_sbfm
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31. encode_uxth([W(rd), W(rn)]) = encode_ubfm([W(rd), W(rn), Imm(0), Imm(15)]) = llvm_mc("uxth W(rd), W(rn)")
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_uxth([Reg(wreg(rd)), Reg(wreg(rn))])
    rhs: encode_ubfm([Reg(wreg(rd)), Reg(wreg(rn)), Imm(0), Imm(15)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 UXTH alias of UBFM Wd, Wn, #0, #15
```

## encode_uxth_arm_fields
- Tier: 4
- Rationale: ARM ARM bitfield layout of the UXTH/UBFM-#0,#15 32-bit form is an exact structural invariant: sf=0, opc=10, bits[28:23]=100110, N=0, immr=0, imms=15, Rn, Rd. Weaker than differential (does not check X-dest canonicalization). Documented bound imms=15 and N=sf=0 sampled exactly.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_arm_fields
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31. let w = encode_uxth([W(rd), W(rn)]). w = 0x53003C00 | (rn<<5) | rd ∧ w[31]=0 ∧ w[30:29]=10 ∧ w[28:23]=100110 ∧ w[22]=0 ∧ w[21:16]=0 ∧ w[15:10]=15 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_uxth([Reg(wreg(rd)), Reg(wreg(rn))])
    rhs: 0x53003C00 | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 UXTH encoding sf=0 opc=10 N=0 immr=0 imms=15
```

## encode_uxth_neg_arity
- Tier: 4
- Rationale: llvm-mc/gas reject UXTH with fewer than 2 operands. get_reg returns Err on missing slots; this is the documented assembler error contract (too few operands). Body has no explicit arity check, so the generator covers 0 and 1.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_arity
- Formal: ∀ ops. |ops| ∈ {0,1} ⇒ encode_uxth(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..1 }
  relation:
    op: throws
    expr: encode_uxth(ops_of_len(len))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: llvm-mc / gas reject UXTH with too few operands
```

## encode_uxth_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc/gas reject a third operand (`uxth w0, w1, x0` → invalid operand). Body has no operands.len() upper bound, so extra operands stay in the generator (not a domain restriction).
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_extra_operand
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31, extra ∈ Operand. encode_uxth([W(rd), W(rn), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, extra = Reg("x0")
- Bug report: bug_reports/encode_uxth_extra_operand.md

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_uxth([Reg(wreg(rd)), Reg(wreg(rn)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects uxth w0, w1, x0
```

## encode_uxth_neg_x_src
- Tier: 4
- Rationale: llvm-mc/gas reject UXTH with an X-register source (`uxth w0, x1` and `uxth x0, x1` → invalid operand / operand mismatch). get_reg discards Rn width, so X-source stays in the generator. ARM ARM source is Wn.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_wd_xn
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31, dest64 ∈ {0,1}. encode_uxth([Reg(GPR(dest64, rd)), Reg(X(rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, dest64 = false (uxth w0, x0)
- Bug report: bug_reports/encode_uxth_x_src.md

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest64]
  domain: { rd: 0..31, rn: 0..31, dest64: bool }
  relation:
    op: throws
    expr: encode_uxth([Reg(gpr(dest64, rd)), Reg(xreg(rn))])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest64: { gen: bool }
expected_error: String
evidence: llvm-mc / gas reject uxth *, Xn
```

## encode_uxth_neg_sp
- Tier: 4
- Rationale: Register 31 in UXTH/UBFM is ZR not SP. llvm-mc/gas reject `uxth wsp, w0` and `uxth w0, sp`. parse_reg_num maps SP and XZR both to 31; the body does not distinguish them, so SP stays in the generator.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp, wsp}, a ∈ 0..30. encode_uxth with SP/WSP in slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: failing
- Counterexample: which = 0, is_64_sp = false, a = 0, dest64 = false (uxth wsp, w0)
- Bug report: bug_reports/encode_uxth_sp.md

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64_sp, a]
  domain: { which: 0..1, is_64_sp: bool, a: 0..30 }
  relation:
    op: throws
    expr: encode_uxth(ops_with_sp_at(which))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64_sp: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects uxth wsp, w0; ARM ARM Rd/Rn are ZR not SP
```

## encode_uxth_neg_fp
- Tier: 4
- Rationale: UXTH operands are GPRs. llvm-mc rejects `uxth d0, w1`. parse_reg_num accepts FP prefixes (d/s/q/v/h/b); the body does not reject them, so FP stays in the generator.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_uxth with Reg(prefix+n) in slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: failing
- Counterexample: which = 0, prefix = "d", n = 0 (uxth d0, w1)
- Bug report: bug_reports/encode_uxth_fp.md

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_uxth(ops_with_fp_at(which))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects uxth d0, w1
```

## encode_uxth_diff_alt_spellings
- Tier: 2
- Rationale: Sweep: llvm-mc accepts w31/WZR/uppercase aliases of W dest and W src. X dest is the 64-bit-form bug already witnessed by encode_uxth_diff_valid_gpr; this property stays on W dest. Strengthening of the differential oracle over spelling variants.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_diff_alt_spellings
- Formal: ∀ rd, rn ∈ 0..31, dest_spell ∈ 0..3, src_spell ∈ 0..2. encode_uxth([Reg(W-spelling(rd)), Reg(W-spelling(rn))]) = llvm_mc("uxth dest, src")
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, dest_spell: 0..3, src_spell: 0..2 }
  relation:
    op: eq
    lhs: encode_uxth([Reg(w_spell(rd, dest_spell)), Reg(w_spell(rn, src_spell))])
    rhs: llvm_mc("uxth " + w_spell(rd, dest_spell) + ", " + w_spell(rn, src_spell))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 3, type: u32 }
  src_spell: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_uxth_neg_nonreg
- Tier: 4
- Rationale: Sweep: non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) must Err. get_reg returns Err on non-Reg; documented assembler error contract.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement}. encode_uxth with bad at slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non_reg_operand }
  relation:
    op: throws
    expr: encode_uxth(ops_with_nonreg_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: get_reg returns Err on non-Reg; llvm-mc requires GPR operands
```

## encode_uxth_neg_invalid_name
- Tier: 4
- Rationale: Sweep: unparsable register names (x32, foo, empty, r0) must Err via parse_reg_num.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_uxth with Reg(name) at slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_uxth(ops_with_name_at(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: parse_reg_num returns None for unparsable names
```

## encode_uxth_meta_rd_rn
- Tier: 4
- Rationale: Sweep: Rd n vs n+1 differs only in bits[4:0]; Rn n vs n+1 differs only in bits[9:5]. Algebraic metamorphic field isolation on the 32-bit form.
- Doc contract: (none) — encode_uxth has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:encode_sxtb_meta_rd_rn
- Formal: ∀ rd, rn ∈ 0..30. (encode_uxth(W(rd+1), W(rn)) ⊕ encode_uxth(W(rd), W(rn))) & ~0x1f = 0 ∧ (encode_uxth(W(rd), W(rn+1)) ⊕ encode_uxth(W(rd), W(rn))) & ~(0x1f<<5) = 0
- Test file: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxth
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..30, rn: 0..30 }
  relation:
    op: eq
    lhs: encode_uxth([Reg(wreg(rd+1)), Reg(wreg(rn))]) & !0x1f
    rhs: encode_uxth([Reg(wreg(rd)), Reg(wreg(rn))]) & !0x1f
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM C6 UXTH Rd bits[4:0] Rn bits[9:5]
```
