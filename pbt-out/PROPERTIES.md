# Properties: encode_ldrs

## encode_ldrs_diff_unsigned_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:12 same textual assembly as gas; ARM unsigned LDRSB/LDRSH). State machine rejected (pure function). Round-trip rejected (no in-tree decoder). Sibling encode_ldrsw / encode_ldr_str / encode_ldur_stur rejected (different jobs: word sign-extend / generic LDR/STR / generic unscaled).
- Seed: load_store.rs encode_ldrsw_diff_unsigned_llvm_mc
- Formal: ∀ size ∈ {0,1}, is_64 ∈ {false,true}, rt,rn ∈ 0..31, imm12 ∈ 0..4095. encode_ldrs([Reg(Rt), Mem{Rn|SP, imm12·(1<<size)}], size) = llvm-mc("ldrsb|ldrsh Rt, [Rn|SP, #(imm12·scale)]") as a 32-bit LE word.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: differential
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, imm12]
  domain:
    size: "0..1"
    is_64: bool
    rt: "0..31"
    rn: "0..31"
    imm12: "0..4095"
  body: sut_word(ops, size) == llvm_mc_word(asm)
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
evidence: README.md:12 README.md:221 encoder/mod.rs:334-335 ARM ARM LDRSB/LDRSH (unsigned immediate)
```

## encode_ldrs_diff_unscaled_pre_post_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for LDURSB/LDURSH (simm9 [-256,255]) and pre/post-index writeback. Same stronger-oracle rejection as unsigned.
- Seed: load_store.rs encode_ldrsw_diff_unscaled_pre_post_llvm_mc
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn ∈ 0..31, simm ∈ [-256,255], form ∈ {Mem, Pre, Post}. (form≠Mem ∧ rt=rn ∧ rn≠31) excluded (ARM unpredictable). encode_ldrs matches llvm-mc for the corresponding ldrsb/ldrsh assembly.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: differential
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, simm, form]
  domain:
    size: "0..1"
    simm: "-256..255"
    form: "0..2"
  body: sut_word(ops, size) == llvm_mc_word(asm)
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  form: { gen: int, min: 0, max: 2, type: u32 }
evidence: ARM ARM LDURSB/LDURSH / LDRSB/LDRSH pre/post-index; llvm-mc rejects writeback Rt==Rn (Rn!=SP)
```

## encode_ldrs_diff_regoff_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for register-offset LDRSB/LDRSH (option/S). Valid amounts: 0 for byte, {0,1} for half. Stronger oracles rejected as above.
- Seed: load_store.rs encode_ldrsw_diff_regoff_llvm_mc
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn,rm ∈ 0..31, extend ∈ {lsl,sxtx,uxtw,sxtw}, amount ∈ valid(size). encode_ldrs([Reg(Rt), MemRegOffset{Rn, Rm, extend, amount}], size) = llvm-mc(...) and option/S fields match ARM.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: differential
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, rm, ext_kind, amount_bit]
  domain:
    size: "0..1"
    ext_kind: "0..3"
    amount: "0 or size"
  body: sut_word(ops, size) == llvm_mc_word(asm)
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ext_kind: { gen: int, min: 0, max: 3, type: u32 }
  amount_bit: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Load/store register (register offset) option/S; llvm-mc shift of #0 (byte) or #0/#1 (half)
```

## encode_ldrs_arm_fields
- Tier: 4
- Rationale: Algebraic invariant unpacking ARM fields independently of the SUT packer. Differential already covers bit-identity; this pins size/opc/V/imm12/imm9/bits[11:10]. Stronger differential is a sibling property, not a replacement.
- Seed: load_store.rs encode_ldrsw_arm_fields
- Formal: ∀ valid unsigned encoding: size_field=size, [29:27]=111, V=0, [25:24]=01, opc=is_64?10:11, imm12=offset/scale, Rn,Rt. Unscaled: [25:24]=00 bit21=0 [11:10]=00 imm9=simm. Pre [11:10]=11; post [11:10]=01. Regoff bit21=1 [11:10]=10.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, imm12, simm, rm]
  domain:
    size: "0..1"
    imm12: "0..4095"
    simm: "-256..255 unscaled-only"
  body: unpack(encode_ldrs(ops, size)) matches ARM LDRSB/LDRSH fields
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM LDRSB/LDRSH / LDURSB/LDURSH encodings
```

## encode_ldrs_metamorphic_rt_rn_imm
- Tier: 3
- Rationale: Algebraic metamorphic: Rt+1 / Rn+1 / imm12+1 / pre⊕post / W↔X opc / B↔H size are independent field increments. Required metamorphic angle. Not a round-trip (no decoder).
- Seed: load_store.rs encode_ldrsw_metamorphic_rt_rn_imm
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn ∈ 0..30, imm12 ∈ 0..4094, simm ∈ [-256,255]. enc(rt+1)-enc(rt)=1; enc(rn+1)-enc(rn)=32; enc(imm12+1)-enc(imm12)=1<<10; pre ⊕ post = 0b10<<10; W vs X flips only opc bits[23:22] (11 vs 10); ldrsb vs ldrsh flips only size bits[31:30].
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, imm12, simm]
  domain:
    rt: "0..30"
    rn: "0..30"
    imm12: "0..4094"
    simm: "-256..255"
  body: enc(rt+1)-enc(rt)==1 && enc(rn+1)-enc(rn)==32 && enc(imm12+1)-enc(imm12)==(1<<10) && (pre^post)==(0b10<<10)
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm12: { gen: int, min: 0, max: 4094, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
evidence: ARM ARM field layout Rt[4:0] Rn[9:5] imm12[21:10] bits[11:10] opc[23:22] size[31:30]
```

## encode_ldrs_neg_arity_kinds
- Tier: 4
- Rationale: Negative/error contract: llvm-mc/gas reject 0/1 operands and a non-memory 2nd operand. encode_ldrs documents "ldrsb/ldrsh requires 2 operands" (load_store.rs:385) and "unsupported ldrsb/ldrsh operands" (load_store.rs:449).
- Seed: load_store.rs encode_ldrsw_neg_arity_kinds
- Formal: ∀ size ∈ {0,1}, rt ∈ 0..31, kind ∉ {Mem, MemPreIndex, MemPostIndex, MemRegOffset}. encode_ldrs([], size)=Err ∧ encode_ldrs([Reg(Rt)], size)=Err ∧ encode_ldrs([Reg(Rt), kind], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, rt, kind]
  domain:
    size: "0..1"
    kind: Imm|Cond|Barrier|Shift|Extend|Label|MemExpr|RegList|Symbol
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: load_store.rs:385 "ldrsb/ldrsh requires 2 operands"; llvm-mc rejects non-memory 2nd operand; README.md:12
```

## encode_ldrs_neg_invalid_regs
- Tier: 4
- Rationale: Negative/error contract from llvm-mc/gas/ARM: dest is Wt|Xt (31=WZR/XZR, never SP/WSP/SIMD); base is Xn|SP (not W, not XZR); W index requires uxtw/sxtw; pre/post Rt==Rn (Rn!=SP) is unpredictable and assemblers reject it.
- Seed: load_store.rs encode_ldrsw_neg_invalid_regs
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn. encode_ldrs rejects SP dest, WSP dest, SIMD dest, W base, XZR base, W index without extend, and pre-index Rt==Rn (rt≠31).
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0, rn=0, simd='d'; encode_ldrs([Reg("sp"), Mem{x0,0}], 0) = Ok(Word(0x3980001f))
- Bug report: pbt-out/bug_reports/encode_ldrs_sp_dest.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, simd]
  domain:
    size: "0..1"
    simd: "d|s|q|h|b"
  body: encode_ldrs(invalid_reg_ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" for SP/WSP/SIMD dest, W/XZR base, W index without uxtw/sxtw; unpredictable writeback Rt==Rn
```

## encode_ldrs_neg_offset_range_extra
- Tier: 4
- Rationale: Negative/error contract: llvm-mc rejects offsets outside unsigned pimm and simm9. Documented bounds sampled at bound±1: byte 4096 / -257; half 8191, 8192, -257.
- Seed: load_store.rs encode_ldrsw_neg_offset_range_extra
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn, off ∉ (unsigned pimm ∪ [-256,255]). encode_ldrs(Mem/Pre/Post with off)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0, rn=0, off=-257; encode_ldrs([Reg("w0"), Mem{x0,-257}], 0) = Ok(Word(0x38cff000))
- Bug report: pbt-out/bug_reports/encode_ldrs_offset_range.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, off]
  domain:
    off: "-257, 4096, 8191, 8192, i64::MIN, i64::MAX"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc "index must be an integer in range [-256, 255]" when not a valid unsigned pimm
```

## encode_ldrs_neg_extra
- Tier: 4
- Rationale: Negative/error contract: llvm-mc/gas reject a third operand. Strengthening: extra operand was bundled with offset-range and never reached after the offset fail.
- Seed: load_store.rs encode_ldrsw_neg_extra
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn ∈ 0..31, extra. encode_ldrs([Reg(Rt), Mem{Rn,0}, extra], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0, rn=0, extra=Reg("x2"); encode_ldrs three operands = Ok(Word(0x39c00000))
- Bug report: pbt-out/bug_reports/encode_ldrs_extra_operand.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, extra]
  domain:
    extra: Reg|Imm|Symbol|Mem
  body: encode_ldrs([Rt, Mem, extra], size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects a 3rd operand; README.md:12
```

## encode_ldrs_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for x31/w31, uppercase X/W/SP/XZR/WZR, and lr. Strengthening / coverage of parse_reg_num aliases. Same-job sibling gate unchanged.
- Seed: load_store.rs encode_ldrsw_diff_alt_spellings
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn ∈ 0..31, spelling ∈ {x31, uppercase, lr}. encode_ldrs(alias names) = llvm-mc(same aliases).
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: differential
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, spelling]
  domain:
    spelling: "x31/w31, uppercase, lr"
  body: sut_word(ops, size) == llvm_mc_word(asm)
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  spelling: { gen: int, min: 0, max: 2, type: u32 }
evidence: parse_reg_num maps x31/w31/XZR/WZR/LR/SP; llvm-mc accepts the same aliases
```

## encode_ldrs_neg_bad_extend_base
- Tier: 4
- Rationale: Negative/error contract: llvm-mc rejects lsl amount other than scale (byte #0, half #0/#1), uxtx, and invalid base names. Coverage of register-offset error paths.
- Seed: load_store.rs encode_ldrsw_neg_bad_extend_base
- Formal: ∀ size ∈ {0,1}, is_64, rt, rn,rm ∈ 0..30. encode_ldrs rejects MemRegOffset with lsl #(size+1 or 3), uxtx, and Mem base "foo".
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0, rn=0, rm=0, kind=0 (lsl #1); Ok(Word(0x38e07800))
- Bug report: pbt-out/bug_reports/encode_ldrs_bad_shift.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, rm, kind]
  domain:
    kind: "lsl#(size+1), lsl#3, uxtx, base foo"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc "expected lsl or sxtx with optional shift of #0" (byte) / "#0 or #1" (half); invalid base
```

## encode_ldrs_neg_invalid_name
- Tier: 4
- Rationale: Negative/error contract for parse_reg_num None (foo/x32/w32/empty/r0/x). coverage_gaps had no LLVM profraw; this is the manual arm-audit sweep of the invalid-name / invalid-base error path (get_reg / parse_reg_num).
- Seed: load_store.rs encode_ldrsw_neg_arity_kinds
- Formal: ∀ size ∈ {0,1}, is_64, rt ∈ 0..31, bad ∈ {foo, x32, w32, empty, r0, x}. encode_ldrs([Reg(bad), Mem{x1,0}], size)=Err ∧ encode_ldrs([Reg(Rt), Mem{bad,0}], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, bad]
  domain:
    bad: "foo|x32|w32|empty|r0|x"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: encoder/mod.rs:131 parse_reg_num returns None for non x/w/d/s/q/v/h/b prefixes and num>31
```

## encode_ldrs_neg_fp_dest
- Tier: 4
- Rationale: Negative/error contract; split from encode_ldrs_neg_invalid_regs so SIMD dest shrinks independently.
- Seed: load_store.rs encode_ldrsw_neg_invalid_regs
- Formal: ∀ size ∈ {0,1}, rt ∈ 0..31, simd ∈ {d,s,q,h,b}. encode_ldrs([Reg(simd||rt), Mem{x1,0}], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, rt=0, simd='d'; Ok(Word(0x39c00020))
- Bug report: pbt-out/bug_reports/encode_ldrs_fp_dest.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, rt, simd]
  domain:
    simd: "d|s|q|h|b"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for SIMD dest
```

## encode_ldrs_neg_w_base
- Tier: 4
- Rationale: Negative/error contract; split so W-base shrinks independently.
- Seed: load_store.rs encode_ldrsw_neg_invalid_regs
- Formal: ∀ size ∈ {0,1}, is_64, rt ∈ 0..31, rn ∈ 0..30. encode_ldrs([Reg(Rt), Mem{Wn,0}], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0, rn=0; Ok(Word(0x39c00000))
- Bug report: pbt-out/bug_reports/encode_ldrs_w_base.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn]
  domain:
    rn: "0..30"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for W base
```

## encode_ldrs_neg_xzr_base
- Tier: 4
- Rationale: Negative/error contract; split so XZR-base shrinks independently.
- Seed: load_store.rs encode_ldrsw_neg_invalid_regs
- Formal: ∀ size ∈ {0,1}, is_64, rt ∈ 0..31. encode_ldrs([Reg(Rt), Mem{xzr,0}], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0; Ok(Word(0x39c003e0))
- Bug report: pbt-out/bug_reports/encode_ldrs_xzr_base.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt]
  domain:
    rt: "0..31"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for XZR base
```

## encode_ldrs_neg_w_index
- Tier: 4
- Rationale: Negative/error contract; split so W-index-without-extend shrinks independently.
- Seed: load_store.rs encode_ldrsw_neg_invalid_regs
- Formal: ∀ size ∈ {0,1}, is_64, rt,rn,rm. encode_ldrs([Reg(Rt), MemRegOffset{Xn, Wm, None, None}], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0, rn=0, rm=0; Ok(Word(0x38e04800))
- Bug report: pbt-out/bug_reports/encode_ldrs_w_index.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt, rn, rm]
  domain:
    rm: "0..31"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc requires uxtw/sxtw on Wm
```

## encode_ldrs_neg_writeback_overlap
- Tier: 4
- Rationale: Negative/error contract; split so Rt==Rn writeback shrinks independently.
- Seed: load_store.rs encode_ldrsw_neg_invalid_regs
- Formal: ∀ size ∈ {0,1}, is_64, rt ∈ 0..30. encode_ldrs([Reg(Rt), MemPreIndex{x(rt), 4}], size)=Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: size=0, is_64=false, rt=0; Ok(Word(0x38c04c00))
- Bug report: pbt-out/bug_reports/encode_ldrs_writeback_overlap.md

```property
function: encoder.load_store.encode_ldrs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [size, is_64, rt]
  domain:
    rt: "0..30"
  body: encode_ldrs(ops, size).is_err()
generators:
  size: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc unpredictable writeback when Rt==Rn and Rn!=SP
```
