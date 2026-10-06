# Properties: encode_uxtb

## encode_uxtb_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent GNU-style AArch64 assembler). README.md:12 claims gas-compatible textual assembly; encoder/mod.rs:442 dispatches `"uxtb"` to this symbol. State machine rejected (pure function). Round-trip rejected (no in-tree UXTB decoder). Sibling encode_sxth/sxtb/uxth/uxtw rejected (same-job gate). encode_ubfm rejected as independent differential (shared get_reg / same crate). Valid domain is UXTB Rd, Wn with Rd in W or X (llvm-mc/gas canonicalize X dest to W; encoding is always 32-bit) and Wn a W GPR. Doc evidence: README.md:12, ARM ARM C6 UXTB = UBFM Wd,Wn,#0,#7.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_diff_valid_gpr
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31, dest64 ∈ {0,1}. let dest = GPR(dest64, rd); src = W(rn). encode_uxtb([Reg(dest), Reg(src)]) = llvm_mc("uxtb dest, src") as Word
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, dest64 = true, use_lr = false (uxtb x0, w0); SUT=0xD3401C00 llvm-mc=0x53001C00
- Bug report: bug_reports/encode_uxtb_x_dest.md

```property
function: encoder.encode_uxtb
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest64]
  domain: { rd: 0..31, rn: 0..31, dest64: bool }
  relation:
    op: eq
    lhs: encode_uxtb([Reg(gpr(dest64, rd)), Reg(wreg(rn))])
    rhs: llvm_mc("uxtb " + gpr(dest64, rd) + ", " + wreg(rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_uxtb_alias_ubfm
- Tier: 4
- Rationale: ARM ARM C6 UXTB is the alias of UBFM Wd, Wn, #0, #7. encode_ubfm is not an independent implementation (shared get_reg / same crate) so this is algebraic.metamorphic, not differential. Restricted to W registers because the 64-bit UBFM Xd,Xn,#0,#7 is UBFX, not a UXTB alias (llvm-mc). Cross-checked against llvm-mc.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_alias_ubfm
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31. encode_uxtb([W(rd), W(rn)]) = encode_ubfm([W(rd), W(rn), Imm(0), Imm(7)]) = llvm_mc("uxtb W(rd), W(rn)")
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_uxtb([Reg(wreg(rd)), Reg(wreg(rn))])
    rhs: encode_ubfm([Reg(wreg(rd)), Reg(wreg(rn)), Imm(0), Imm(7)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 UXTB alias of UBFM Wd, Wn, #0, #7
```

## encode_uxtb_arm_fields
- Tier: 4
- Rationale: ARM ARM bitfield layout of the UXTB/UBFM-#0,#7 32-bit form is an exact structural invariant: sf=0, opc=10, bits[28:23]=100110, N=0, immr=0, imms=7, Rn, Rd. Weaker than differential (does not check X-dest canonicalization). Documented bound imms=7 and N=sf=0 sampled exactly.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_arm_fields
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31. let w = encode_uxtb([W(rd), W(rn)]). w = 0x53001C00 | (rn<<5) | rd ∧ w[31]=0 ∧ w[30:29]=10 ∧ w[28:23]=100110 ∧ w[22]=0 ∧ w[21:16]=0 ∧ w[15:10]=7 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_uxtb([Reg(wreg(rd)), Reg(wreg(rn))])
    rhs: 0x53001C00 | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 UXTB encoding sf=0 opc=10 N=0 immr=0 imms=7
```

## encode_uxtb_neg_arity
- Tier: 4
- Rationale: llvm-mc/gas reject UXTB with fewer than 2 operands. get_reg returns Err on missing slots; this is the documented assembler error contract (too few operands). Body has no explicit arity check, so the generator covers 0 and 1.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_arity
- Formal: ∀ ops. |ops| ∈ {0,1} ⇒ encode_uxtb(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..1 }
  relation:
    op: throws
    expr: encode_uxtb(ops_of_len(len))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: llvm-mc / gas reject UXTB with too few operands
```

## encode_uxtb_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc/gas reject a third operand (`uxtb w0, w1, x0` → invalid operand). Body has no operands.len() upper bound, so extra operands stay in the generator (not a domain restriction).
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_extra_operand
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31, extra ∈ Operand. encode_uxtb([W(rd), W(rn), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, extra = Reg("x0")
- Bug report: bug_reports/encode_uxtb_extra_operand.md

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_uxtb([Reg(wreg(rd)), Reg(wreg(rn)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects uxtb w0, w1, x0
```

## encode_uxtb_neg_x_src
- Tier: 4
- Rationale: llvm-mc/gas reject UXTB with an X-register source (`uxtb w0, x1` and `uxtb x0, x1` → invalid operand / operand mismatch). get_reg discards Rn width, so X-source stays in the generator. ARM ARM source is Wn.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_x_src
- Formal: ∀ rd ∈ 0..31, rn ∈ 0..31, dest64 ∈ {0,1}. encode_uxtb([Reg(GPR(dest64, rd)), Reg(X(rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, dest64 = false (uxtb w0, x0)
- Bug report: bug_reports/encode_uxtb_x_src.md

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest64]
  domain: { rd: 0..31, rn: 0..31, dest64: bool }
  relation:
    op: throws
    expr: encode_uxtb([Reg(gpr(dest64, rd)), Reg(xreg(rn))])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest64: { gen: bool }
expected_error: String
evidence: llvm-mc / gas reject uxtb *, Xn
```

## encode_uxtb_neg_sp
- Tier: 4
- Rationale: Register 31 in UXTB/UBFM is ZR not SP. llvm-mc/gas reject `uxtb wsp, w0` and `uxtb w0, sp`. parse_reg_num maps SP and XZR both to 31; the body does not distinguish them, so SP stays in the generator.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp, wsp}, a ∈ 0..30. encode_uxtb with SP/WSP in slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: failing
- Counterexample: which = 0, is_64_sp = false, a = 0, dest64 = false (uxtb wsp, w0)
- Bug report: bug_reports/encode_uxtb_sp.md

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64_sp, a]
  domain: { which: 0..1, is_64_sp: bool, a: 0..30 }
  relation:
    op: throws
    expr: encode_uxtb(ops_with_sp_at(which))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64_sp: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects uxtb wsp, w0; ARM ARM Rd/Rn are ZR not SP
```

## encode_uxtb_neg_fp
- Tier: 4
- Rationale: UXTB operands are GPRs. llvm-mc rejects `uxtb d0, w1`. parse_reg_num accepts FP prefixes (d/s/q/v/h/b); the body does not reject them, so FP stays in the generator.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_uxtb with Reg(prefix+n) in slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: failing
- Counterexample: which = 0, prefix = "d", n = 0 (uxtb d0, w1)
- Bug report: bug_reports/encode_uxtb_fp.md

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_uxtb(ops_with_fp_at(which))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects uxtb d0, w1
```

## encode_uxtb_diff_alt_spellings
- Tier: 2
- Rationale: Sweep: llvm-mc accepts w31/WZR/uppercase aliases of W dest and W src. X dest is the 64-bit-form bug already witnessed by encode_uxtb_diff_valid_gpr; this property stays on W dest. Strengthening of the differential oracle over spelling variants.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_diff_alt_spellings
- Formal: ∀ rd, rn ∈ 0..31, dest_spell ∈ 0..3, src_spell ∈ 0..2. encode_uxtb([Reg(W-spelling(rd)), Reg(W-spelling(rn))]) = llvm_mc("uxtb dest, src")
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_spell, src_spell]
  domain: { rd: 0..31, rn: 0..31, dest_spell: 0..3, src_spell: 0..2 }
  relation:
    op: eq
    lhs: encode_uxtb([Reg(w_spelling(rd, dest_spell)), Reg(w_spelling(rn, src_spell))])
    rhs: llvm_mc("uxtb dest, src")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 3, type: u32 }
  src_spell: { gen: int, min: 0, max: 2, type: u32 }
evidence: llvm-mc accepts w31/WZR/uppercase
```

## encode_uxtb_neg_nonreg
- Tier: 4
- Rationale: Sweep: non-register Operand kinds (Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement) at either slot must Err. get_reg returns Err on non-Reg; llvm-mc rejects them.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ non-Reg Operand. encode_uxtb with bad at slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non_reg_operand }
  relation:
    op: throws
    expr: encode_uxtb(ops_with_nonreg_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, variants: [Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement] }
expected_error: String
evidence: llvm-mc / get_reg reject non-register operands
```

## encode_uxtb_neg_invalid_name
- Tier: 4
- Rationale: Sweep: invalid register names (foo, x32, w32, empty, r0, …) must Err. parse_reg_num returns None; llvm-mc rejects them.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ invalid_names. encode_uxtb with Reg(name) at slot which is Err
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_register_name }
  relation:
    op: throws
    expr: encode_uxtb(ops_with_name_at(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, variants: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: llvm-mc / parse_reg_num reject unrecognized register names
```

## encode_uxtb_meta_rd_rn
- Tier: 4
- Rationale: Sweep: Rd and Rn occupy isolated fields. Incrementing Rd by 1 changes only bits[4:0]; incrementing Rn by 1 changes only bits[9:5]. Algebraic.metamorphic field isolation.
- Doc contract: (none) — encode_uxtb has no rustdoc or body comment
- Seed: src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:encode_uxth_meta_rd_rn
- Formal: ∀ rd, rn ∈ 0..30. let b = encode_uxtb([W(rd), W(rn)]); let d1 = encode_uxtb([W(rd+1), W(rn)]); let n1 = encode_uxtb([W(rd), W(rn+1)]). (d1 ⊕ b) ∧ ¬0x1F = 0 ∧ d1[4:0] = rd+1 ∧ (n1 ⊕ b) ∧ ¬(0x1F≪5) = 0 ∧ n1[9:5] = rn+1
- Test file: src/backend/arm/assembler/encoder/encode_uxtb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_uxtb
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..30, rn: 0..30 }
  relation:
    op: holds
    expr: rd_rn_fields_isolated(encode_uxtb, rd, rn)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM C6 UXTB Rd bits[4:0] Rn bits[9:5]
```
