# Properties: encode_uxtw

## encode_uxtw_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree UXTW decoder). Same-job siblings encode_sxtw / encode_uxth / encode_uxtb rejected (SBFM vs UBFM; different imms). encode_ubfm is same crate so alias equality is metamorphic, not independent differential. Doc evidence: README.md "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:296 `"uxtw" => encode_uxtw`; ARM ARM C6 UXTW = UBFM Xd, Xn, #0, #31.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_diff_valid_gpr
- Formal: ∀ rd, rn ∈ {0..31}. encode_uxtw([Reg(Xd), Reg(Wn)]) = llvm-mc("uxtw Xd, Wn") as little-endian u32, where X31/W31 are xzr/wzr.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, use_lr = false (uxtw x0, w0 → SUT 0x2A0003E0 vs llvm-mc 0xD3407C00)
- Bug report: pbt-out/bug_reports/encode_uxtw_mov_not_ubfm.md

```property
function: encoder.data_processing.encode_uxtw
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: "0..=31", rn: "0..=31" }
  relation:
    op: eq
    lhs: encode_uxtw([Reg(xreg(rd)), Reg(wreg(rn))])
    rhs: llvm_mc_word("uxtw {xreg(rd)}, {wreg(rn)}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:13; encoder/mod.rs:296; ARM ARM C6 UXTW alias of UBFM
```

## encode_uxtw_alias_ubfm
- Tier: 4
- Rationale: ARM ARM documents UXTW as the assembler alias of UBFM Xd, Xn, #0, #31 (equivalently UBFX Xd, Xn, #0, #32). encode_ubfm is the same crate, so this is algebraic.metamorphic, not independent differential. Stronger differential already claimed by encode_uxtw_diff_valid_gpr. Round-trip rejected (no decoder).
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_alias_sbfm
- Formal: ∀ rd, rn ∈ {0..31}. encode_uxtw([Reg(Xd), Reg(Wn)]) = encode_ubfm([Reg(Xd), Reg(Xn), Imm(0), Imm(31)]) = llvm-mc("uxtw Xd, Wn").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd = 0, rn = 0 (SUT 0x2A0003E0 vs UBFM 0xD3407C00)
- Bug report: pbt-out/bug_reports/encode_uxtw_mov_not_ubfm.md

```property
function: encoder.data_processing.encode_uxtw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: "0..=31", rn: "0..=31" }
  relation:
    op: eq
    lhs: encode_uxtw([Reg(xreg(rd)), Reg(wreg(rn))])
    rhs: encode_ubfm([Reg(xreg(rd)), Reg(xreg(rn)), Imm(0), Imm(31)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 UXTW alias of UBFM Xd, Xn, #0, #31; llvm-mc disassembles uxtw as ubfx #0,#32
```

## encode_uxtw_arm_fields
- Tier: 4
- Rationale: ARM ARM bitfield encoding of UXTW/UBFM with sf=1 opc=10 N=1 immr=0 imms=31. Weaker than differential; kept as an exact structural invariant that pins each field independently of llvm-mc parsing.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_arm_fields
- Formal: ∀ rd, rn ∈ {0..31}. let w = encode_uxtw([Reg(Xd), Reg(Wn)]). w = 0xD3407C00 | (rn << 5) | rd ∧ w[31]=1 ∧ w[30:29]=10 ∧ w[28:23]=100110 ∧ w[22]=1 ∧ w[21:16]=0 ∧ w[15:10]=31 ∧ w[9:5]=rn ∧ w[4:0]=rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd = 0, rn = 0 (SUT 0x2A0003E0 vs 0xD3407C00)
- Bug report: pbt-out/bug_reports/encode_uxtw_mov_not_ubfm.md

```property
function: encoder.data_processing.encode_uxtw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: "0..=31", rn: "0..=31" }
  relation:
    op: eq
    lhs: encode_uxtw([Reg(xreg(rd)), Reg(wreg(rn))])
    rhs: "0xD3407C00 | (rn << 5) | rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM C6 Bitfield UXTW/UBFM sf=1 opc=10 N=1 immr=0 imms=31
```

## encode_uxtw_neg_arity
- Tier: 4
- Rationale: llvm-mc rejects `uxtw` with fewer than 2 operands ("too few operands"). Negative/error contract from the GNU-style assembler surface. Stronger oracles do not apply to the invalid-arity domain.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_uxtw(ops) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len 0..=1" }
  relation:
    op: throws
    expr: encode_uxtw(ops)
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; get_reg errors when operand missing
```

## encode_uxtw_neg_extra_operand
- Tier: 4
- Rationale: ARM ARM UXTW takes exactly two registers. llvm-mc rejects a 3rd operand. Documented assembler arity; extra operand must Err.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_extra_operand
- Formal: ∀ rd, rn ∈ {0..31}. ∀ extra. encode_uxtw([Reg(Xd), Reg(Wn), extra]) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, extra = Reg("x0")
- Bug report: pbt-out/bug_reports/encode_uxtw_extra_operand.md

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra]
  domain: { rd: "0..=31", rn: "0..=31", extra: Operand }
  relation:
    op: throws
    expr: encode_uxtw([Reg(xreg(rd)), Reg(wreg(rn)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects "uxtw x0, w1, #0" / "uxtw x0, w1, w2"; ARM ARM two-operand alias
```

## encode_uxtw_neg_wd
- Tier: 4
- Rationale: ARM ARM assembler syntax is UXTW Xd, Wn only. llvm-mc rejects `uxtw w0, w1` ("invalid operand"). 32-bit dest is outside the valid domain.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_wd
- Formal: ∀ rd, rn ∈ {0..31}. ∀ src64 ∈ Bool. encode_uxtw([Reg(Wd), Reg(src)]) = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, src64 = false (uxtw w0, w0)
- Bug report: pbt-out/bug_reports/encode_uxtw_wd.md

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, src64]
  domain: { rd: "0..=31", rn: "0..=31", src64: bool }
  relation:
    op: throws
    expr: encode_uxtw([Reg(wreg(rd)), Reg(gpr(src64, rn))])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  src64: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on uxtw w0, w1; ARM ARM UXTW Xd, Wn
```

## encode_uxtw_neg_sp
- Tier: 4
- Rationale: Register 31 in the bitfield class is XZR/WZR, never SP/WSP. llvm-mc rejects `uxtw sp, w1` and `uxtw x0, wsp`.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_sp
- Formal: ∀ which ∈ {0,1}. ∀ is_64_sp ∈ Bool. ∀ a ∈ {0..30}. encode_uxtw with SP or WSP in slot `which` = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which = 0, is_64_sp = false, a = 0 (uxtw wsp, w0)
- Bug report: pbt-out/bug_reports/encode_uxtw_sp.md

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64_sp, a]
  domain: { which: "0..=1", is_64_sp: bool, a: "0..=30" }
  relation:
    op: throws
    expr: encode_uxtw(ops_with_sp)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64_sp: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects uxtw sp, w1 and uxtw x0, wsp; ARM ARM Rd/Rn are XZR/WZR at 31
```

## encode_uxtw_neg_fp
- Tier: 4
- Rationale: UXTW operands are GPRs. llvm-mc rejects FP/SIMD names (d/s/q/v/h/b). parse_reg_num currently accepts those prefixes, so this is a caller-reachable invalid domain.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_fp
- Formal: ∀ which ∈ {0,1}. ∀ prefix ∈ {d,s,q,v,h,b}. ∀ n ∈ {0..31}. encode_uxtw with Reg(prefix||n) in slot `which` = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which = 0, prefix = "d", n = 0 (uxtw d0, w1)
- Bug report: pbt-out/bug_reports/encode_uxtw_fp.md

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: "0..=1", prefix: "{d,s,q,v,h,b}", n: "0..=31" }
  relation:
    op: throws
    expr: encode_uxtw(ops_with_fp)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" on uxtw x0, d1; ARM ARM GPR-only UXTW
```

## encode_uxtw_diff_alt_spellings
- Tier: 2
- Rationale: Contract-surface sweep: llvm-mc accepts x31/w31, uppercase, LR, and X-source (canonicalizes to UBFX). Same differential oracle as encode_uxtw_diff_valid_gpr over spelling variants. Fails for the same MOV-vs-UBFM encoding bug.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_diff_alt_spellings
- Formal: ∀ rd, rn ∈ {0..31}. ∀ dest/src spellings in {xN, x31, XZR, LR, uppercase, Wn, Xn}. encode_uxtw([Reg(dest), Reg(src)]) = llvm-mc("uxtw dest, src").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd = 0, rn = 0, dest_spell = 0, src_spell = 0 (uxtw x0, w0 → 0x2A0003E0 vs 0xD3407C00)
- Bug report: pbt-out/bug_reports/encode_uxtw_mov_not_ubfm.md

```property
function: encoder.data_processing.encode_uxtw
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_spell, src_spell]
  domain: { rd: "0..=31", rn: "0..=31", dest_spell: "0..=4", src_spell: "0..=3" }
  relation:
    op: eq
    lhs: encode_uxtw([Reg(dest), Reg(src)])
    rhs: llvm_mc_word("uxtw {dest}, {src}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc accepts x31/XZR/LR/uppercase and uxtw Xd, Xn
```

## encode_uxtw_neg_nonreg
- Tier: 4
- Rationale: Contract-surface sweep: non-register operand kinds (Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement) at either GPR slot must Err. get_reg already documents "expected register".
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_nonreg
- Formal: ∀ which ∈ {0,1}. ∀ bad ∉ Reg. encode_uxtw with `bad` in slot `which` = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: "0..=1", bad: "non-Reg Operand" }
  relation:
    op: throws
    expr: encode_uxtw(ops_with_nonreg)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, variants: [Imm, Shift, Mem, Label, Symbol, Cond, RegArrangement] }
expected_error: String
evidence: get_reg "expected register at operand"; llvm-mc rejects non-register UXTW operands
```

## encode_uxtw_neg_invalid_name
- Tier: 4
- Rationale: Contract-surface sweep: invalid register names (foo, x32, w32, x, r0, empty, x-1, x99, w) must Err via parse_reg_num.
- Seed: data_processing.rs encode_sxtw_pbt encode_sxtw_neg_invalid_name
- Formal: ∀ which ∈ {0,1}. ∀ name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_uxtw with Reg(name) in slot `which` = Err(_).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_uxtw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: "0..=1", name: "{foo,x32,w32,x,r0,\"\",x-1,x99,w}" }
  relation:
    op: throws
    expr: encode_uxtw(ops_with_invalid_name)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, variants: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: parse_reg_num returns None for names outside x/w0..31 and aliases
```
