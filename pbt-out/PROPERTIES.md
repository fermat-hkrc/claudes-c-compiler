# Properties: encode_bic

## encode_bic_diff_reg_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). In-tree BIC decoder does not exist so algebraic round-trip of a sibling decoder is unavailable. encode_logical is AND (N=0), a different job, so it fails the same-job sibling gate as a differential reference. Doc evidence: assembler README "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:674 bic dispatch; ARM ARM BIC shifted-register encoding.
- Seed: (none) — no existing tests for encode_bic
- Formal: ∀ rd,rn,rm ∈ 0..=31, ∀ is_64 ∈ Bool, ∀ shift ∈ {None} ∪ {lsl,lsr,asr,ror}×[0, width-1]. encode_bic([Reg(gpr), Reg(gpr), Reg(gpr), Shift?]) = Word(w) ∧ w = llvm-mc("bic Rd, Rn, Rm{, shift #amt}"). Register 31 is XZR/WZR, never SP.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, shift_kind, shift_amt]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_64: bool, shift: optional_aarch64_shift }
  relation:
    op: eq
    lhs: encode_bic([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm)), Shift?]) as Word
    rhs: llvm_mc("bic Rd, Rn, Rm{, shift #amt}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  shift_kind: { gen: oneof, options: [none, lsl, lsr, asr, ror] }
  shift_amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:674 bic dispatch; ARM ARM BIC shifted register sf 00 01010 shift N=1 Rm imm6 Rn Rd
```

## encode_bic_diff_imm_llvm_mc
- Tier: 7
- Rationale: Differential against llvm-mc for the documented BIC-immediate alias of AND-immediate. State machine rejected. Round-trip decoder unavailable. Doc evidence: function comment "BIC Xd, Xn, #imm -> AND Xd, Xn, #~imm"; ARM ARM BIC (immediate) is an alias of AND (immediate); README gas-compatible. Rd of 31 is SP/WSP (AND-immediate), not XZR.
- Seed: (none)
- Formal: ∀ rd ∈ {x0..x30,sp} ∪ {w0..w30,wsp}, ∀ rn ∈ {x0..x30,xzr} matching width, ∀ imm such that ~imm is a valid AArch64 bitmask. encode_bic([Reg(rd), Reg(rn), Imm(imm)]) = Word(llvm-mc("bic rd, rn, #imm"))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, rd_is_sp, imm]
  domain: { rd: u32_0_31, rn: u32_0_31, is_64: bool, imm: bic_imm_with_valid_inverted_bitmask }
  relation:
    op: eq
    lhs: encode_bic([Reg(rd_name), Reg(rn_name), Imm(imm)]) as Word
    rhs: llvm_mc("bic rd, rn, #imm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, type: i64 }
evidence: data_processing.rs:1008-1012 BIC imm alias of AND ~imm; ARM ARM AND-immediate Rd=SP; README.md:5-14
```

## encode_bic_diff_neon_llvm_mc
- Tier: 7
- Rationale: Differential against llvm-mc for NEON three-same BIC. ARM ARM BIC (vector) T is 8B or 16B only. State machine rejected. README.md:224 lists bic under NEON three-same. encode_neon_bic is the helper encode_bic calls, not an independent reference.
- Seed: (none)
- Formal: ∀ d,n,m ∈ 0..=31, ∀ T ∈ {8b,16b}. encode_bic([RegArrangement(vd,T), RegArrangement(vn,T), RegArrangement(vm,T)]) = Word(llvm-mc("bic vd.T, vn.T, vm.T"))
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: differential
predicate:
  quantifier: forall
  vars: [d, n, m, arr]
  domain: { d: u32_0_31, n: u32_0_31, m: u32_0_31, arr: {8b,16b} }
  relation:
    op: eq
    lhs: encode_bic([RegArrangement(vd,arr), RegArrangement(vn,arr), RegArrangement(vm,arr)]) as Word
    rhs: llvm_mc("bic vd.arr, vn.arr, vm.arr")
generators:
  d: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, options: [8b, 16b] }
evidence: README.md:224 NEON three-same bic; ARM ARM BIC vector 0 Q 0 01110 01 1 Rm 000111 Rn Rd T=8B|16B
```

## encode_bic_meta_imm_and_alias
- Tier: 4c
- Rationale: Algebraic metamorphic from the ARM ARM alias BIC imm = AND ~imm. Stronger differential is the sibling llvm-mc immediate property. encode_logical(opc=00) is the in-tree AND encoder (different mnemonic, same AND-immediate encoding job for the inverted immediate). State machine rejected.
- Seed: (none)
- Formal: ∀ rd,rn ∈ GPR-same-width, ∀ imm such that ~imm is a valid bitmask. encode_bic([Rd,Rn,Imm(imm)]) = encode_logical([Rd,Rn,Imm(~imm)], opc=00)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, imm]
  domain: { rd: u32_0_30, rn: u32_0_31, is_64: bool, imm: bic_imm_with_valid_inverted_bitmask }
  relation:
    op: eq
    lhs: encode_bic([Reg(rd), Reg(rn), Imm(imm)])
    rhs: encode_logical([Reg(rd), Reg(rn), Imm(~imm)], 0)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, type: i64 }
evidence: ARM ARM BIC (immediate) is an alias of AND (immediate); data_processing.rs:1008-1012
```

## encode_bic_neg_arity
- Tier: 4e
- Rationale: Negative/error contract from the explicit "bic requires 3 operands" check and llvm-mc "too few operands". Stronger differential does not apply to the invalid-arity domain. State machine rejected.
- Seed: (none)
- Formal: ∀ ops with len(ops) < 3. encode_bic(ops) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_lists_len_0_to_2 }
  relation:
    op: throws
    lhs: encode_bic(ops)
    error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: data_processing.rs:1014-1016 "bic requires 3 operands"; llvm-mc "too few operands for instruction"
```

## encode_bic_neg_mixed_width
- Tier: 4e
- Rationale: Negative/error contract from ARM ARM (all three GPRs must be the same width) and llvm-mc "invalid operand for instruction" on mixed x/w. encode_bic takes sf only from operand 0, so this is the contract llvm-mc/gas enforce. State machine rejected. Differential on valid same-width inputs is the sibling property.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ 0..=30, ∀ widths not all equal. encode_bic([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Reg(gpr(rm64,rm))]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true (bic w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_bic_mixed_width.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: u32_0_30, rn: u32_0_30, rm: u32_0_30, widths: not_all_equal }
  relation:
    op: throws
    lhs: encode_bic([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Reg(gpr(rm64,rm))])
    error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: ARM ARM BIC shifted-register all registers same width; llvm-mc rejects bic x0, w1, x2
```

## encode_bic_neg_sp_fp_regform
- Tier: 4e
- Rationale: Negative/error contract from ARM ARM BIC shifted-register (Rd/Rn/Rm of 31 are XZR not SP; FP/SIMD names are not GPRs) and llvm-mc rejects `bic sp,...` / `bic d0,...`. parse_reg_num maps sp and d/s/q/v/h/b prefixes to 0-31, so this checks the public gas contract. State machine rejected.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}. placing SP/WSP or an FP/SIMD name (d/s/q/v/h/b0..31) at operand `which` of a 3-register BIC yields Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, kind=0 (bic wsp, w0, w0)
- Bug report: pbt-out/bug_reports/encode_bic_sp_register_form.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..=2, bad: sp_or_fp_name }
  relation:
    op: throws
    lhs: encode_bic(ops_with_bad_at(which, bad))
    error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  bad: { gen: oneof, options: [sp, wsp, d0, s0, q0, v0, h0, b0, d31] }
expected_error: String
evidence: ARM ARM BIC shifted-register Rd/Rn/Rm are Xd/Wn not SP/FP; llvm-mc rejects bic sp, x0, x1 and bic d0, x1, x2
```

## encode_bic_neg_shift_range_neon_arr
- Tier: 4e
- Rationale: Negative/error contract from documented bounds: ARM ARM W-form shift amount in [0,31], X-form in [0,63] (bound and bound+1 must be sampled); ARM ARM BIC vector T is 8B|16B only (8h/4s/2d/4h/2s rejected by llvm-mc). Field masking of imm6 and Q=(arr==16b) would silently accept these. State machine rejected. Differential covers the valid side of each bound.
- Seed: (none)
- Formal: (1) ∀ 32-bit BIC with shift amount ∈ {32,33,63,64}. encode_bic is Err. (2) ∀ 64-bit BIC with shift amount ∈ {64,65,128}. encode_bic is Err. (3) ∀ T ∈ {8h,4h,4s,2s,2d}. encode_bic(Vd.T,Vn.T,Vm.T) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: is_64=false, kind=lsl, amt_w=32, which=0 (bic w0, w0, w0, lsl #32)
- Bug report: pbt-out/bug_reports/encode_bic_shift_out_of_range.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, amt, arr]
  domain: { amt: out_of_range_shift, arr: invalid_neon_T }
  relation:
    op: throws
    lhs: encode_bic(shifted_or_neon_invalid)
    error: String
generators:
  amt_w: { gen: oneof, options: [32, 33, 63, 64] }
  amt_x: { gen: oneof, options: [64, 65, 128] }
  arr: { gen: oneof, options: [8h, 4h, 4s, 2s, 2d] }
expected_error: String
evidence: ARM ARM BIC shifted-register imm6 range [0,31]/[0,63]; ARM ARM BIC vector T=8B|16B; llvm-mc rejects bic w0,w1,w2,lsl #32 and bic v0.8h,v1.8h,v2.8h
```

## encode_bic_neg_fp_reg
- Tier: 4e
- Rationale: Negative/error contract from ARM ARM (BIC shifted-register operands are GPRs) and llvm-mc rejecting d/s/q/v/h/b names. Split out of encode_bic_neg_sp_fp_regform so shrinking can witness FP independently of SP. parse_reg_num accepts those prefixes. State machine rejected.
- Seed: (none)
- Formal: ∀ which ∈ {0,1,2}, ∀ prefix ∈ {d,s,q,v,h,b}, ∀ n ∈ 0..=31. encode_bic with that FP name at operand `which` is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix=d, n=0 (bic d0, x1, x2)
- Bug report: pbt-out/bug_reports/encode_bic_fp_reg.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..=2, prefix: {d,s,q,v,h,b}, n: u32_0_31 }
  relation:
    op: throws
    lhs: encode_bic(ops_with_fp_at(which))
    error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM BIC shifted-register GPRs only; llvm-mc rejects bic d0, x1, x2; parse_reg_num in encoder/mod.rs:131-147 accepts d/s/q/v/h/b
```

## encode_bic_neg_invalid_neon_arr
- Tier: 4e
- Rationale: Negative/error contract from ARM ARM BIC (vector) T is 8B or 16B only; llvm-mc rejects 8h/4s/2d three-register form. encode_neon_bic sets Q only for 16b and silently encodes every other arrangement as 8b. Split out of encode_bic_neg_shift_range_neon_arr so shrinking can witness this independently.
- Seed: (none)
- Formal: ∀ d,n,m ∈ 0..=31, ∀ T ∈ {8h,4h,4s,2s,2d}. encode_bic(Vd.T, Vn.T, Vm.T) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: d=0, n=0, m=0, arr=8h (bic v0.8h, v1.8h, v2.8h)
- Bug report: pbt-out/bug_reports/encode_bic_invalid_neon_arr.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [d, n, m, arr]
  domain: { d: u32_0_31, n: u32_0_31, m: u32_0_31, arr: {8h,4h,4s,2s,2d} }
  relation:
    op: throws
    lhs: encode_bic([RegArrangement(vd,arr), RegArrangement(vn,arr), RegArrangement(vm,arr)])
    error: String
generators:
  d: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, options: [8h, 4h, 4s, 2s, 2d] }
expected_error: String
evidence: ARM ARM BIC vector T=8B|16B; README.md:224 NEON three-same bic; llvm-mc rejects bic v0.8h, v1.8h, v2.8h
```

## encode_bic_neg_imm_xzr_rd
- Tier: 4e
- Rationale: Negative/error contract from ARM ARM AND-immediate (BIC-imm alias): Rd of 31 is SP/WSP, not XZR/WZR. llvm-mc rejects `bic xzr, x0, #1` and `bic wzr, w0, #1`. get_reg maps both xzr and sp to 31, so the SUT cannot distinguish them. State machine rejected. Differential on valid SP-as-Rd immediates is encode_bic_diff_imm_llvm_mc.
- Seed: (none)
- Formal: ∀ is_64 ∈ Bool, ∀ rn ∈ 0..=30. encode_bic([Reg(xzr|wzr), Reg(gpr(is_64,rn)), Imm(1)]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: is_64=false, rn=0 (bic wzr, w0, #1)
- Bug report: pbt-out/bug_reports/encode_bic_imm_xzr_rd.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rn]
  domain: { is_64: bool, rn: u32_0_30 }
  relation:
    op: throws
    lhs: encode_bic([Reg(xzr_or_wzr), Reg(gpr(is_64,rn)), Imm(1)])
    error: String
generators:
  is_64: { gen: bool }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM AND-immediate Rd is Xd|SP not XZR; llvm-mc rejects bic xzr, x0, #1
```

## encode_bic_neg_invalid_imm
- Tier: 4e
- Rationale: Coverage-sweep of the documented bitmask-failure path (`cannot encode bitmask immediate for bic`). Differential vs llvm-mc on the invalid-imm domain: if llvm-mc rejects, SUT must Err; if llvm-mc accepts, encodings must match. State machine rejected. Stronger valid-imm differential is encode_bic_diff_imm_llvm_mc.
- Seed: (none)
- Formal: ∀ rd,rn ∈ 0..=30, ∀ is_64, ∀ imm ∈ {0,-1,5,9,0x11,i64::MIN}. llvm-mc("bic Rd, Rn, #imm") rejects ⇒ encode_bic is Err; accepts ⇒ encodings equal
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, imm]
  domain: { imm: known_non_bitmask_skew }
  relation:
    op: holds
    expr: llvm_mc_rejects(asm) => encode_bic(ops).is_err()
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: oneof, options: [0, -1, 5, 9, 0x11, i64_min] }
expected_error: String
evidence: data_processing.rs:1046 cannot encode bitmask immediate for bic; llvm-mc rejects #0 / all-ones / 0x5
```

## encode_bic_neg_unsupported_third
- Tier: 4e
- Rationale: Coverage-sweep of the documented `unsupported bic operands` path when operand 2 is neither Imm nor Reg. State machine rejected.
- Seed: (none)
- Formal: ∀ third ∈ {Mem, Symbol, Cond, Label, Barrier}. encode_bic([Reg, Reg, third]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, third]
  domain: { third: {Mem, Symbol, Cond, Label, Barrier} }
  relation:
    op: throws
    lhs: encode_bic([Reg, Reg, third])
    error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: data_processing.rs:1066 Err("unsupported bic operands")
```

## encode_bic_neg_invalid_rm
- Tier: 4e
- Rationale: Coverage-sweep of `invalid rm register for bic` when operand 2 is a Reg with an unparsable name. State machine rejected.
- Seed: (none)
- Formal: ∀ bad ∈ {x32,w32,x99,"",foo,r0,x}. encode_bic([Reg(x0), Reg(x1), Reg(bad)]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, bad]
  domain: { bad: invalid_reg_names }
  relation:
    op: throws
    lhs: encode_bic([Reg, Reg, Reg(bad)])
    error: String
generators:
  bad: { gen: oneof, options: [x32, w32, x99, empty, foo, r0, x] }
expected_error: String
evidence: data_processing.rs:1051 parse_reg_num(rm_name).ok_or("invalid rm register for bic")
```

## encode_bic_neg_unknown_shift_kind
- Tier: 4e
- Rationale: Coverage-sweep of the shift-kind match default (`_ => 0b00`). ARM ARM BIC shifted-register allows only lsl/lsr/asr/ror. Unknown kinds must Err, not encode as LSL. State machine rejected.
- Seed: (none)
- Formal: ∀ kind ∉ {lsl,lsr,asr,ror}. encode_bic([Rd, Rn, Rm, Shift{kind, amt}]) is Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: kind="lslx", amt=0, is_64=false, rd=rn=rm=0 (bic w0, w0, w0 with Shift lslx #0)
- Bug report: pbt-out/bug_reports/encode_bic_unknown_shift_kind.md

```property
function: encoder.data_processing.encode_bic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { kind: {lslx, rrx, rol, empty, "asr "} }
  relation:
    op: throws
    lhs: encode_bic([Reg, Reg, Reg, Shift{kind, amt}])
    error: String
generators:
  kind: { gen: oneof, options: [lslx, rrx, rol, empty, asr_space] }
  amt: { gen: oneof, options: [0, 1, 31] }
expected_error: String
evidence: ARM ARM BIC shifted-register shift in {lsl,lsr,asr,ror}; data_processing.rs:1053-1060 match default _ => 0b00
```
