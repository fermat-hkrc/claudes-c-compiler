# Properties: encode_ldtr_sized

## encode_ldtr_sized_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree LDTRB decoder). Same-job siblings encode_ldur_stur / encode_ldr_str rejected (LDUR/STUR/LDTR auto-size from W/X/SIMD vs explicit byte/half unprivileged; LDRB/STRB different addressing). Doc evidence: assembler README.md:11 "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:340-343 dispatch of ldtrh/sttrh/ldtrb/sttrb; ARM ARM LDTRB/LDTRH/STTRB/STTRH (unprivileged unscaled immediate).
- Seed: load_store.rs encode_ldur_stur_pbt encode_ldur_stur_diff_gpr_llvm_mc
- Formal: ∀ rt, rn ∈ {0..31}, simm ∈ [-256,255], is_load ∈ {0,1}, size ∈ {0,1}. encode_ldtr_sized([Reg(Wt), Mem{Xn|SP, simm}], is_load, size) = llvm-mc("{ldtrb|ldtrh|sttrb|sttrh} Wt, [Xn|SP{, #simm}]") as LE u32, where W31 dest is wzr, Rn=31 is sp.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldtr_sized
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, simm, is_load, size]
  domain: { rt: "0..=31", rn: "0..=31", simm: "-256..=255", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: eq
    lhs: encode_ldtr_sized(ops_wt_mem(rt, rn, simm), is_load, size)
    rhs: llvm_mc_word(asm_ldtr_sized)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:340-343; ARM ARM LDTRB/LDTRH/STTRB/STTRH size 111 0 00 opc 0 imm9 10 Rn Rt, simm9 in [-256,255]
```

## encode_ldtr_sized_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an independent invariant over the success path. Stronger differential covers the same valid domain vs llvm-mc; this unpacks size/V/opc/imm9/op2/Rn/Rt without copying the SUT packer. Stronger rejected as in encode_ldtr_sized_diff_llvm_mc. Documented simm9 bounds -256 and 255 sampled exactly.
- Seed: load_store.rs encode_ldur_stur_pbt unpack_ldur
- Formal: ∀ rt, rn ∈ {0..31}, simm ∈ [-256,255], is_load ∈ {0,1}, size ∈ {0,1}. Let w = encode_ldtr_sized([Reg(Wt), Mem{Xn|SP, simm}], is_load, size) as Word. Then bits[31:30]=size, bits[29:27]=111, V=0, bits[25:24]=00, bits[23:22]=(is_load?01:00), bit21=0, bits[20:12]=simm as i9, bits[11:10]=10, bits[9:5]=rn, bits[4:0]=rt.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldtr_sized
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rn, simm, is_load, size]
  domain: { rt: "0..=31", rn: "0..=31", simm: "-256..=255", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: holds
    expr: unpack_ldtr(w) == (size, 0, opc, simm, 0b10, rn, rt) && fixed_ldtr_bits(w)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM LDTRB/LDTRH/STTRB/STTRH encoding size 111 V=0 00 opc 0 imm9 10 Rn Rt; encoder/mod.rs:1-7
```

## encode_ldtr_sized_metamorphic_fields
- Tier: 4
- Rationale: ARM ARM places size at [31:30], opc at [23:22], Rt at [4:0], Rn at [9:5], imm9 at [20:12]. Independent of the SUT packer. Stronger differential already used; this checks field independence. Same-job sibling encode_ldur_stur rejected (different size selection).
- Seed: load_store.rs encode_ldrsw_pbt metamorphic Rt/Rn/imm
- Formal: ∀ rt ∈ {0..30}, rn ∈ {0..30}, simm ∈ [-256,254], is_load ∈ {0,1}. Let E(rt,rn,simm,load,sz) = encode_ldtr_sized Word. Then E(..., size=01) XOR E(..., size=00) = 1<<30; E(load=1) XOR E(load=0) = 1<<22; E(rt+1) - E(rt) = 1; E(rn+1) - E(rn) = 32; E(simm+1) XOR E(simm) has only bits[20:12] differing by +1 in the signed imm9 field.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldtr_sized
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, simm, is_load]
  domain: { rt: "0..=30", rn: "0..=30", simm: "-256..=254", is_load: "bool" }
  relation:
    op: holds
    expr: (E(sz=1) ^ E(sz=0) == 1<<30) && (E(load=1) ^ E(load=0) == 1<<22) && (E(rt+1) - E(rt) == 1) && (E(rn+1) - E(rn) == 32)
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  simm: { gen: int, min: -256, max: 254, type: i64 }
  is_load: { gen: bool }
evidence: ARM ARM LDTRB/LDTRH size at [31:30], opc at [23:22], imm9 at [20:12], Rn at [9:5], Rt at [4:0]
```

## encode_ldtr_sized_neg_arity
- Tier: 4
- Rationale: ARM syntax requires Wt and a memory operand. llvm-mc rejects fewer than 2 operands. Negative/error contract: encode_ldtr_sized must return Err. Stronger oracles do not apply to the invalid-arity domain.
- Seed: load_store.rs encode_ldur_stur_pbt arity checks
- Formal: ∀ ops with |ops| < 2, is_load ∈ {0,1}, size ∈ {0,1}. encode_ldtr_sized(ops, is_load, size) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldtr_sized
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, is_load, size]
  domain: { ops: "|ops|<2", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: throws
    expr: encode_ldtr_sized(ops, is_load, size)
expected_error: String
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM LDTRB Wt, [Xn|SP{, #simm}]; llvm-mc rejects missing operands; encoder docstring "ldtr/sttr requires 2 operands"
```

## encode_ldtr_sized_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc / gas reject a third operand on ldtrb/ldtrh/sttrb/sttrh. README.md:11 same textual assembly as gas. Negative contract: extra operand => Err. Documented arity is exactly 2.
- Seed: load_store.rs encode_ldur_stur_pbt test_encode_ldur_stur_regression_extra_operand
- Formal: ∀ rt, rn ∈ {0..31}, simm ∈ [-256,255], extra ∈ Operand, is_load ∈ {0,1}, size ∈ {0,1}. encode_ldtr_sized([Reg(Wt), Mem{Xn|SP, simm}, extra], is_load, size) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, simm=-256, is_load=false, size=0, extra=Reg("x2") — sttrb w0, [x0, #-256], x2
- Bug report: pbt-out/bug_reports/encode_ldtr_sized_extra_operand.md

```property
function: encoder.load_store.encode_ldtr_sized
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, simm, extra, is_load, size]
  domain: { rt: "0..=31", rn: "0..=31", simm: "-256..=255", extra: Operand, is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: throws
    expr: encode_ldtr_sized([Reg(Wt), Mem, extra], is_load, size)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:11 gas-compatible assembly; llvm-mc "invalid operand" on ldtrb w0, [x1], x2
```

## encode_ldtr_sized_neg_xt_dest
- Tier: 4
- Rationale: ARM ARM LDTRB/LDTRH/STTRB/STTRH dest/src is Wt only. llvm-mc rejects Xt/lr. Negative contract: X dest => Err. Stronger differential does not apply on this invalid domain.
- Seed: load_store.rs encode_ldrsw_pbt W-dest negative (inverse width)
- Formal: ∀ rt ∈ {0..31} with Xt/lr spelling, rn ∈ {0..31}, simm ∈ [-256,255], is_load ∈ {0,1}, size ∈ {0,1}. encode_ldtr_sized([Reg(Xt|lr), Mem{Xn|SP, simm}], is_load, size) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, simm=-256, is_load=false, size=0, use_lr=false — sttrb x0, [x0, #-256]
- Bug report: pbt-out/bug_reports/encode_ldtr_sized_xt_dest.md

```property
function: encoder.load_store.encode_ldtr_sized
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, simm, is_load, size]
  domain: { rt: "Xt|lr", rn: "0..=31", simm: "-256..=255", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: throws
    expr: encode_ldtr_sized([Reg(Xt), Mem], is_load, size)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM LDTRB <Wt>, [<Xn|SP>{, #<simm>}] ; llvm-mc "invalid operand" on ldtrb x0, [x1] and ldtrb lr, [x1]
```

## encode_ldtr_sized_neg_invalid_regs
- Tier: 4
- Rationale: llvm-mc rejects SP/WSP as Rt, SIMD/FP as Rt, W as base, XZR as base (Rn=31 is SP). README.md:11 gas-compatible. Negative contract: those register choices => Err.
- Seed: load_store.rs encode_ldur_stur_pbt SP / W-base / XZR / SIMD regressions
- Formal: ∀ kind ∈ {sp_rt, wsp_rt, fp_rt, w_base, xzr_base}, simm ∈ [-256,255], is_load ∈ {0,1}, size ∈ {0,1}. encode_ldtr_sized(ops(kind), is_load, size) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: kind=0, n=0, simm=-256, is_load=false, size=0 — sttrb sp, [x0, #-256] (also WSP, SIMD Rt, W base, XZR base)
- Bug report: pbt-out/bug_reports/encode_ldtr_sized_sp_as_rt.md; pbt-out/bug_reports/encode_ldtr_sized_fp_dest.md; pbt-out/bug_reports/encode_ldtr_sized_w_base.md; pbt-out/bug_reports/encode_ldtr_sized_xzr_base.md

```property
function: encoder.load_store.encode_ldtr_sized
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, simm, is_load, size]
  domain: { kind: "sp_rt|wsp_rt|fp_rt|w_base|xzr_base", simm: "-256..=255", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: throws
    expr: encode_ldtr_sized(ops_invalid_reg(kind), is_load, size)
expected_error: String
generators:
  simm: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Rt is Wt (31=WZR) never SP/SIMD; Rn is Xn|SP never W/XZR; llvm-mc rejects ldtrb sp/wsp/d0, [x1], ldtrb w0, [w1], ldtrb w0, [xzr]
```

## encode_ldtr_sized_neg_offset_and_form
- Tier: 4
- Rationale: ARM ARM simm9 in [-256, 255]; only unscaled [Xn{, #simm}]. llvm-mc rejects #256/#-257 and pre/post/reg-offset. Negative contract: out-of-range offset or non-Mem addressing => Err. Documented bounds -256 and 255 sampled at bound±1.
- Seed: load_store.rs encode_ldur_stur_pbt test_encode_ldur_stur_regression_imm9_range
- Formal: ∀ rt, rn ∈ {0..31}, is_load ∈ {0,1}, size ∈ {0,1}. (simm ∉ [-256,255] ⇒ encode_ldtr_sized([Reg(Wt), Mem{Xn|SP, simm}], ...) = Err) ∧ (addr ∈ {MemPreIndex, MemPostIndex, MemRegOffset, Imm, Symbol, Cond, Barrier} ⇒ encode_ldtr_sized([Reg(Wt), addr], ...) = Err).
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, is_load=false, size=0, use_offset=true, bad_offset=-257 — sttrb w0, [x0, #-257] (non-Mem form correctly Errs; offset wrap is the bug)
- Bug report: pbt-out/bug_reports/encode_ldtr_sized_imm9_range.md

```property
function: encoder.load_store.encode_ldtr_sized
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, bad_offset, bad_form, is_load, size]
  domain: { rt: "0..=31", rn: "0..=31", bad_offset: "i64 \\ [-256,255]", bad_form: "pre|post|regoff|Imm|Symbol", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: throws
    expr: encode_ldtr_sized(ops_bad_addr, is_load, size)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  bad_offset: { gen: int, min: -257, max: 256, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM simm9 in [-256,255], unscaled form only; llvm-mc "index must be an integer in range [-256, 255]" and "invalid operand" on pre/post/reg-offset
```

## encode_ldtr_sized_neg_bad_form
- Tier: 4
- Rationale: Sweep of documented unscaled-only addressing. ARM ARM LDTRB has no pre/post/register-offset form. llvm-mc rejects those. Isolated from the failing imm9-wrap property so the error path is independently evidenced.
- Seed: encode_ldtr_sized_neg_offset_and_form (form arm)
- Formal: ∀ rt, rn ∈ {0..31}, is_load ∈ {0,1}, size ∈ {0,1}, addr ∈ {MemPreIndex, MemPostIndex, MemRegOffset, Imm, Symbol, Label}. encode_ldtr_sized([Reg(Wt), addr], is_load, size) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldtr_sized
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, form, is_load, size]
  domain: { rt: "0..=31", rn: "0..=31", form: "pre|post|regoff|Imm|Symbol|Label", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: throws
    expr: encode_ldtr_sized([Reg(Wt), bad_form], is_load, size)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  form: { gen: int, min: 0, max: 5, type: u32 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM LDTRB unscaled [Xn|SP{, #simm}] only; llvm-mc invalid operand on pre/post/reg-offset
```

## encode_ldtr_sized_diff_w31_alias
- Tier: 2
- Rationale: Sweep of documented register-31 alias. ARM ARM / llvm-mc treat w31 as wzr. Differential vs llvm-mc plus equality of the two SUT spellings.
- Seed: encode_ldtr_sized_diff_llvm_mc
- Formal: ∀ rn ∈ {0..31}, simm ∈ [-256,255], is_load ∈ {0,1}, size ∈ {0,1}. encode_ldtr_sized([Reg("w31"), Mem{Xn|SP, simm}], ...) = encode_ldtr_sized([Reg("wzr"), Mem{Xn|SP, simm}], ...) = llvm-mc("{ldtrb|...} wzr, [Xn|SP{, #simm}]").
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldtr_sized
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, simm, is_load, size]
  domain: { rn: "0..=31", simm: "-256..=255", is_load: "bool", size: "{0b00,0b01}" }
  relation:
    op: eq
    lhs: encode_ldtr_sized([Reg("w31"), Mem], is_load, size)
    rhs: llvm_mc_word(asm_wzr)
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM register 31 is WZR; llvm-mc accepts w31 as wzr
```
