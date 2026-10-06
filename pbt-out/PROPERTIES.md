# Properties: encode_sxtb

## encode_sxtb_diff_valid_gpr
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (GNU-style assembler README.md:12). State machine rejected (pure function). Round-trip rejected (no in-tree SXTB/SBFM decoder). encode_sxth / encode_sxtw / encode_uxtb rejected as independent differential (same-job gate: imms=15 / 32-bit-only / UBFM opc=10; shared get_reg). encode_sbfm rejected as independent differential (shared get_reg / same crate).
- Doc contract: (none)
- Seed: data_processing.rs:13049 encode_sxth_diff_valid_gpr
- Formal: ∀ rd, rn ∈ {0..31}, is_64 ∈ Bool. encode_sxtb([Reg(gpr(is_64,rd)), Reg(wreg(rn))]) = llvm-mc("sxtb Rd, Wn")
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..31, rn: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_sxtb([Reg(gpr(is_64,rd)), Reg(wreg(rn))])
    rhs: llvm_mc("sxtb Rd, Wn")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: README.md:12 gas-compat; README.md:217 Extensions lists sxtb; encoder/mod.rs:435 sxtb dispatch; llvm-mc 15.0.6
```

## encode_sxtb_alias_sbfm
- Tier: 4
- Rationale: ARM ARM C6 SXTB is the alias of SBFM Rd, Rn, #0, #7. Not an independent differential (encode_sbfm shares get_reg / same crate). Metamorphic: SUT SXTB equals SUT SBFM with #0,#7 and both equal llvm-mc.
- Doc contract: (none)
- Seed: data_processing.rs:13069 encode_sxth_alias_sbfm
- Formal: ∀ rd, rn ∈ {0..31}, is_64 ∈ Bool. encode_sxtb([Rd,Rn]) = encode_sbfm([Rd,Rn,#0,#7]) = llvm-mc("sxtb Rd, Wn")
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..31, rn: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_sxtb([Rd,Rn])
    rhs: encode_sbfm([Rd,Rn,Imm(0),Imm(7)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM C6 SXTB = SBFM Rd, Rn, #0, #7; README.md:217
```

## encode_sxtb_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout of SXTB/SBFM-#0,#7 is an exact structural invariant independent of llvm-mc: sf, opc=00, bits[28:23]=100110, N=sf, immr=0, imms=7, Rn, Rd.
- Doc contract: (none)
- Seed: data_processing.rs:13092 encode_sxth_arm_fields
- Formal: ∀ rd, rn ∈ {0..31}, is_64 ∈ Bool. word = encode_sxtb([Rd,Wn]) ⇒ (word>>31)=sf ∧ ((word>>29)&0b11)=0 ∧ ((word>>23)&0x3F)=0b100110 ∧ ((word>>22)&1)=sf ∧ ((word>>16)&0x1F)=0 ∧ ((word>>10)&0x3F)=7 ∧ ((word>>5)&0x1F)=rn ∧ (word&0x1F)=rd
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..31, rn: 0..31, is_64: bool }
  body: let w = encode_sxtb([Rd,Wn]) in (w>>31)==sf && ((w>>29)&0b11)==0 && ((w>>23)&0x3F)==0b100110 && ((w>>22)&1)==sf && ((w>>16)&0x1F)==0 && ((w>>10)&0x3F)==7 && ((w>>5)&0x1F)==rn && (w&0x1F)==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM C6 SXTB = SBFM sf 00 100110 N=sf immr=0 imms=7 Rn Rd
```

## encode_sxtb_neg_arity
- Tier: 3
- Rationale: llvm-mc / GNU as require two operands. get_reg on a missing slot returns Err, matching the documented assembler contract. Negative/error contract: too few operands must be rejected.
- Doc contract: (none)
- Seed: data_processing.rs:13113 encode_sxth_neg_arity
- Formal: ∀ ops. |ops| ∈ {0,1} ⇒ encode_sxtb(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..1 }
  relation:
    op: holds
    expr: encode_sxtb(ops_of_len(len)).is_err()
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: llvm-mc rejects too few operands; README.md:12 gas-compat; encoder/mod.rs:435 two-operand dispatch
```

## encode_sxtb_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a third operand for SXTB. Body has no arity upper bound (get_reg only reads slots 0 and 1). Negative/error: extra operand must Err. Domain is the documented assembler domain, not the passing one.
- Doc contract: (none)
- Seed: data_processing.rs:13136 encode_sxth_neg_extra_operand
- Formal: ∀ rd, rn ∈ {0..31}, is_64 ∈ Bool, extra ∈ Operand. encode_sxtb([Rd, Wn, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: failing
- Counterexample: encode_sxtb([Reg("w0"), Reg("w0"), Reg("x0")]) → Ok(Word(0x13001c00))
- Bug report: bug_reports/encode_sxtb_extra_operand.md

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, extra]
  domain: { rd: 0..31, rn: 0..31, is_64: bool, extra: Operand }
  relation:
    op: holds
    expr: encode_sxtb([Rd, Wn, extra]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, options: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects sxtb w0, w1, x0; README.md:12 gas-compat
```

## encode_sxtb_neg_wd_xn
- Tier: 3
- Rationale: ARM C6 / llvm-mc require Wn as the source; SXTB Wd, Xn is invalid. get_reg discards Rn width. Negative/error: Wd+Xn must Err.
- Doc contract: (none)
- Seed: data_processing.rs:13154 encode_sxth_neg_wd_xn
- Formal: ∀ rd, rn ∈ {0..31}. encode_sxtb([Reg(wreg(rd)), Reg(xreg(rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: failing
- Counterexample: encode_sxtb([Reg("w0"), Reg("x0")]) → Ok(Word(0x13001c00))
- Bug report: bug_reports/encode_sxtb_wd_xn.md

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: holds
    expr: encode_sxtb([Reg(wreg(rd)), Reg(xreg(rn))]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects sxtb w0, x0; ARM C6 source is Wn
```

## encode_sxtb_neg_sp
- Tier: 3
- Rationale: ARM C6 register 31 for SXTB/SBFM is ZR not SP. llvm-mc rejects SP/WSP. parse_reg_num maps SP and XZR both to 31. Negative/error: SP/WSP must Err.
- Doc contract: (none)
- Seed: data_processing.rs:13168 encode_sxth_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_sxtb with SP/WSP in slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: failing
- Counterexample: encode_sxtb([Reg("wsp"), Reg("w0")]) → Ok(Word(0x13001c1f))
- Bug report: bug_reports/encode_sxtb_sp.md

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64_sp]
  domain: { which: 0..1, is_64_sp: bool }
  relation:
    op: holds
    expr: encode_sxtb(ops_with_sp_at(which)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64_sp: { gen: bool }
expected_error: String
evidence: llvm-mc rejects sxtb wsp, w0 and sxtb sp, x0; ARM C6 Rd/Rn are ZR not SP
```

## encode_sxtb_neg_fp
- Tier: 3
- Rationale: llvm-mc rejects FP/SIMD registers as SXTB operands. parse_reg_num accepts d/s/q/v/h/b prefixes. Negative/error: FP/SIMD must Err.
- Doc contract: (none)
- Seed: data_processing.rs:13190 encode_sxth_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_sxtb with FP register prefix+n in slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: failing
- Counterexample: encode_sxtb([Reg("d0"), Reg("w1")]) → Ok(Word(0x13001c20))
- Bug report: bug_reports/encode_sxtb_fp.md

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: holds
    expr: encode_sxtb(ops_with_fp_at(which)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, options: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects sxtb d0, w1; README.md:12 gas-compat
```

## encode_sxtb_diff_alt_spellings
- Tier: 5
- Rationale: Sweep of documented aliases llvm-mc accepts: x31/w31, uppercase, LR, and X-source with 64-bit dest (canonicalized to Wn). Same differential oracle as encode_sxtb_diff_valid_gpr.
- Doc contract: (none)
- Seed: data_processing.rs:13210 encode_sxth_diff_alt_spellings
- Formal: ∀ rd, rn ∈ {0..31}, is_64 ∈ Bool, dest/src in {canonical, x31/w31, uppercase, LR, X-source-if-64}. encode_sxtb([Reg(dest), Reg(src)]) = llvm-mc("sxtb dest, src")
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, is_64: bool, dest_spell: 0..4, src_spell: 0..3 }
  relation:
    op: eq
    lhs: encode_sxtb([Reg(dest), Reg(src)])
    rhs: llvm_mc("sxtb dest, src")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc accepts x31/XZR/LR/uppercase and sxtb Xd, Xn; README.md:12
```

## encode_sxtb_neg_nonreg
- Tier: 3
- Rationale: Sweep: non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) must Err. get_reg already returns Err for non-Reg.
- Doc contract: (none)
- Seed: data_processing.rs:13246 encode_sxth_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement}. encode_sxtb with bad at slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non_reg_operand }
  relation:
    op: holds
    expr: encode_sxtb(ops_with_nonreg_at(which, bad)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: get_reg requires Operand::Reg; llvm-mc rejects non-register SXTB operands
```

## encode_sxtb_neg_invalid_name
- Tier: 3
- Rationale: Sweep: unparsable register names (x32, foo, empty, r0) must Err. parse_reg_num returns None.
- Doc contract: (none)
- Seed: data_processing.rs:13263 encode_sxth_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_sxtb with Reg(name) at slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_name }
  relation:
    op: holds
    expr: encode_sxtb(ops_with_name_at(which, name)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: parse_reg_num returns None for these names; llvm-mc rejects them
```

## encode_sxtb_meta_rd_rn
- Tier: 4
- Rationale: ARM field isolation: Rd+1 and Rn+1 change only bits[4:0] and bits[9:5]. Metamorphic layout identity independent of llvm-mc.
- Doc contract: (none)
- Seed: encode_mneg_pbt.rs encode_mneg_meta_rd_rn_rm
- Formal: ∀ rd, rn ∈ {0..30}, is_64 ∈ Bool. (encode_sxtb(rd+1,rn) XOR encode_sxtb(rd,rn)) & ~0x1F = 0 ∧ (encode_sxtb(rd,rn+1) XOR encode_sxtb(rd,rn)) & ~(0x1F<<5) = 0
- Test file: src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sxtb
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_64]
  domain: { rd: 0..30, rn: 0..30, is_64: bool }
  relation:
    op: holds
    expr: ((encode_sxtb(rd+1,rn) XOR encode_sxtb(rd,rn)) & !0x1F) == 0 && ((encode_sxtb(rd,rn+1) XOR encode_sxtb(rd,rn)) & !(0x1F<<5)) == 0
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM C6 SXTB Rd bits[4:0] Rn bits[9:5]
```
