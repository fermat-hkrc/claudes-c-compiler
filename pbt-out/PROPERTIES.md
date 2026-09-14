# Properties: encode_logical

## encode_logical_diff_reg
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (gas-compatible AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree AND/ORR/EOR/ANDS decoder. encode_bic/orn/eon/bics fail the same-job sibling gate (N=1 vs N=0). Doc evidence: README.md:5-14 gas-compatible text; encoder/mod.rs:231-234 dispatch; ARM ARM Logical (shifted register) `sf opc 01010 shift 0 Rm imm6 Rn Rd`.
- Seed: encode_eon_pbt::encode_eon_diff_reg_llvm_mc (data_processing.rs)
- Formal: ∀ rd,rn,rm ∈ [0,31], sf ∈ {0,1}, opc ∈ {00,01,10,11}, shift ∈ {lsl,lsr,asr,ror}, amt ∈ [0, 31+32·sf]. encode_logical([Rd,Rn,Rm{,shift #amt}], opc) = llvm-mc("{and|orr|eor|ands} Rd, Rn, Rm{, shift #amt}") as little-endian u32, where register 31 is XZR/WZR.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, opc, kind, amt]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, opc: {0,1,2,3}, kind: {lsl,lsr,asr,ror}, amt: 0..(31+32*is_64) }
  relation:
    op: eq
    lhs: encode_logical([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm)), Shift{kind,amt}], opc)
    rhs: llvm_mc(mnemonic(opc)+" "+gpr(is_64,rd)+", "+gpr(is_64,rn)+", "+gpr(is_64,rm)+", "+kind+" #"+amt)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  kind: { gen: oneof, choices: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:231-234; ARM ARM Logical (shifted register)
```

## encode_logical_diff_imm
- Tier: 2
- Rationale: Same differential oracle over the bitmask-immediate form. Immediate values are constructed from ARM ARM (size, ones, immr) independently of encode_bitmask_imm. Rd=31 is SP for AND/ORR/EOR and XZR for ANDS. Rn=31 is XZR (llvm-mc rejects SP as Rn).
- Seed: encode_eon_pbt::encode_eon_diff_imm_llvm_mc (data_processing.rs)
- Formal: ∀ rd ∈ [0,31], rn ∈ [0,31], sf ∈ {0,1}, opc ∈ {00,01,10,11}, valid bitmask imm constructed from (size, ones, immr). encode_logical([Rd,Rn,Imm(imm)], opc) = llvm-mc("{and|orr|eor|ands} Rd, Rn, #imm").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, opc, size, ones, immr]
  domain: { rd: 0..31, rn: 0..31, is_64: bool, opc: {0,1,2,3}, size: {2,4,8,16,32,64}, ones: 1..(size-1), immr: 0..(size-1) }
  relation:
    op: eq
    lhs: encode_logical([Reg(rd_name), Reg(rn_name), Imm(bitmask_from_fields(size,ones,immr,is_64))], opc)
    rhs: llvm_mc(mnemonic(opc)+" "+rd_name+", "+rn_name+", #imm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  size: { gen: oneof, choices: [2, 4, 8, 16, 32, 64] }
  ones: { gen: int, min: 1, max: 63, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14; ARM ARM Logical (immediate); llvm-mc and sp, x0, #1 accepted / and x0, sp, #1 rejected
```

## encode_logical_diff_neon
- Tier: 2
- Rationale: Differential vs llvm-mc for the NEON three-same path (RegArrangement dest). ANDS is not a NEON instruction and is excluded from the valid domain. T in {8b,16b} only (llvm-mc rejects 4s/8h/etc.).
- Seed: encode_eon_pbt NEON is N/A; neon encode_neon_float_three_same_pbt differential shape
- Formal: ∀ vd,vn,vm ∈ [0,31], T ∈ {8b,16b}, opc ∈ {00,01,10}. encode_logical([Vd.T, Vn.T, Vm.T], opc) = llvm-mc("{and|orr|eor} Vd.T, Vn.T, Vm.T").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vn, vm, arr, opc]
  domain: { vd: 0..31, vn: 0..31, vm: 0..31, arr: {8b,16b}, opc: {0,1,2} }
  relation:
    op: eq
    lhs: encode_logical([RegArrangement{vN,arr} x3], opc)
    rhs: llvm_mc(mnemonic(opc)+" v"+vd+"."+arr+", v"+vn+"."+arr+", v"+vm+"."+arr)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  vm: { gen: int, min: 0, max: 31, type: u32 }
  arr: { gen: oneof, choices: ["8b", "16b"] }
  opc: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md NEON three-same and/orr/eor; neon.rs:297-319; ARM ARM Advanced SIMD logical; llvm-mc rejects and v0.4s and ands v0.16b
```

## encode_logical_metamorphic_opc
- Tier: 4c
- Rationale: ARM ARM places opc at bits [30:29] of both logical forms. At equal other fields, encode(opc_a) XOR encode(opc_b) = (opc_a XOR opc_b) << 29. Stronger differential is also used (diff_reg/diff_imm); this metamorphic check does not depend on llvm-mc and pins the opc field independently. State machine / round-trip rejected as above.
- Seed: encode_eon_pbt::encode_eon_metamorphic_n_bit_vs_eor
- Formal: ∀ valid GPR shifted-register operands, opc1 ≠ opc2 ∈ {0,1,2,3}. encode_logical(ops, opc1) XOR encode_logical(ops, opc2) = (opc1 XOR opc2) << 29.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, opc1, opc2]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, opc1: 0..3, opc2: 0..3 }
  relation:
    op: eq
    lhs: encode_logical(ops, opc1) XOR encode_logical(ops, opc2)
    rhs: (opc1 XOR opc2) << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  opc1: { gen: int, min: 0, max: 3, type: u32 }
  opc2: { gen: int, min: 0, max: 3, type: u32 }
evidence: ARM ARM Logical (shifted register) opc at bits [30:29]; encoder/mod.rs:231-234
```

## encode_logical_invariant_arm_fields
- Tier: 4d
- Rationale: Success-path word must match ARM ARM field layout. Weaker than differential (does not pin exact encoding against llvm-mc) but independently cites the ARM ARM bit positions. Shifted-register: bits[28:24]=01010, N=0 at bit 21. Immediate: bits[28:23]=100100.
- Seed: encode_eon_pbt::encode_eon_invariant_arm_fields
- Formal: ∀ valid shifted-register inputs. word[31]=sf, word[30:29]=opc, word[28:24]=01010, word[23:22]=shift, word[21]=0, word[20:16]=Rm, word[15:10]=imm6, word[9:5]=Rn, word[4:0]=Rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, opc, shift, amt]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, opc: 0..3, shift: 0..3, amt: 0..(31+32*is_64) }
  body: let w = encode_logical(ops, opc) in (w>>31)==sf AND ((w>>29)&3)==opc AND ((w>>24)&0x1F)==0b01010 AND ((w>>21)&1)==0 AND ((w>>16)&0x1F)==rm AND ((w>>10)&0x3F)==amt AND ((w>>5)&0x1F)==rn AND (w&0x1F)==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  shift: { gen: int, min: 0, max: 3, type: u32 }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Logical (shifted register) C4 encoding
```

## encode_logical_neg_arity
- Tier: 4e
- Rationale: Documented by llvm-mc ("too few operands for instruction") and the function comment "logical op requires 3 operands". Fewer than 3 operands must Err. Extra operands (4th not a Shift) are rejected by llvm-mc.
- Seed: encode_eon_pbt extra-operand / arity negatives
- Formal: ∀ ops with len < 3, opc ∈ {0,1,2,3}. encode_logical(ops, opc) is Err. ∀ valid 3-operand GPR/NEON plus a trailing non-Shift extra operand. encode_logical must Err (llvm-mc rejects a 4th operand that is not a shift).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, opc]
  domain: { ops: operand lists of length 0..2, or length 4 with 4th not Shift, opc: 0..3 }
  relation:
    op: throws
    lhs: encode_logical(ops, opc)
    rhs: String
expected_error: String
generators:
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc "too few operands for instruction"; data_processing.rs:456-458; llvm-mc rejects and x0, x1, x2, x3
```

## encode_logical_neg_invalid_imm
- Tier: 4e
- Rationale: ARM ARM logical immediate forbids 0 and all-ones (N/immr/imms cannot encode them). llvm-mc: "expected compatible register or logical immediate". SUT must Err for 0, all-ones (width-specific), and non-bitmask values such as 0x1234.
- Seed: encode_eon_pbt invalid-imm negatives
- Formal: ∀ rd,rn GPR, sf, opc, imm ∈ {0, all-ones(width), 0x1234, 0x1001 when not a valid bitmask}. encode_logical([Rd,Rn,Imm(imm)], opc) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, opc, imm]
  domain: { rd: 0..30, rn: 0..30, is_64: bool, opc: 0..3, imm: {0, ~0, 0x1234, 0x1001} filtered to invalid bitmasks }
  relation:
    op: throws
    lhs: encode_logical([Reg(rd), Reg(rn), Imm(imm)], opc)
    rhs: String
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  opc: { gen: int, min: 0, max: 3, type: u32 }
  imm: { gen: int, min: -1, max: 0x1234, type: i64 }
evidence: ARM ARM Logical (immediate) 0 and all-ones reserved; llvm-mc and x0, x1, #0 / #-1 / #0x1234 rejected; encode_bitmask_imm returns None for 0 and all-ones
```

## encode_logical_neg_rejected_operands
- Tier: 4e
- Rationale: llvm-mc / ARM ARM reject: SP in shifted-register form; mixed X/W; FP/SIMD names as GPR; shift amount out of range (32-bit >31, 64-bit >63); unknown shift kind; NEON T not in {8b,16b}; mismatched NEON arrangements; ANDS on NEON. Documented error contract is rejection (llvm-mc error).
- Seed: encode_eon_pbt SP / mixed-width / FP / shift-range / unknown-kind negatives
- Formal: ∀ each invalid category listed. encode_logical(ops, opc) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: extra `[w0,w0,w0,w0]` opc=0; SP `wsp,w0,w0` opc=0; mixed `w0,w0,x0` opc=0; FP `d0,x0,x0` opc=0; shift `w0,w0,w0,lsl #32`; unknown `lslx`; NEON T=`4s`; NEON mismatch `16b/8b/16b`; ANDS NEON `v0.8b` opc=11
- Bug report: pbt-out/bug_reports/encode_logical_extra_operand.md, pbt-out/bug_reports/encode_logical_sp_shifted.md, pbt-out/bug_reports/encode_logical_mixed_width.md, pbt-out/bug_reports/encode_logical_fp_as_gpr.md, pbt-out/bug_reports/encode_logical_shift_oob.md, pbt-out/bug_reports/encode_logical_unknown_shift.md, pbt-out/bug_reports/encode_logical_neon_bad_arr.md, pbt-out/bug_reports/encode_logical_neon_mismatch.md, pbt-out/bug_reports/encode_logical_ands_neon.md

```property
function: encode_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [category, ops, opc]
  domain: { category: {sp_shifted, mixed_width, fp_as_gpr, shift_oob, unknown_shift, neon_bad_arr, neon_mismatch, ands_neon} }
  relation:
    op: throws
    lhs: encode_logical(ops, opc)
    rhs: String
expected_error: String
generators:
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc rejects and sp, x0, x1; and x0, w1, x2; and x0, x1, d2; and w0, w1, w2, lsl #32; and x0, x1, x2, rorx #1; and v0.4s; and v0.16b, v1.8b, v2.16b; ands v0.16b
```

## encode_logical_metamorphic_sf
- Tier: 4c
- Rationale: Coverage-sweep. ARM ARM sf is bit 31 of both logical forms. At equal register numbers, encode(X) XOR encode(W) = 1<<31. Documented by ARM ARM Logical (shifted register) sf field.
- Seed: encode_eon_pbt / encode_logical_invariant_arm_fields sf check
- Formal: ∀ rd,rn,rm ∈ [0,30], opc ∈ {0,1,2,3}. encode_logical(X-ops, opc) XOR encode_logical(W-ops, opc) = 1<<31.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, opc]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, opc: 0..3 }
  relation:
    op: eq
    lhs: encode_logical(X-ops, opc) XOR encode_logical(W-ops, opc)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: ARM ARM Logical (shifted register) sf at bit 31
```

## encode_logical_neg_unsupported_third
- Tier: 4e
- Rationale: Coverage-sweep of the final Err("unsupported logical operands") arm. Operand 2 that is neither Imm nor Reg (Symbol/Mem/Label/Cond) must Err. Documented by the function's last return and llvm-mc rejecting non-register/non-imm thirds.
- Seed: encode_logical body data_processing.rs:509
- Formal: ∀ opc ∈ {0,1,2,3}, third ∈ {Symbol, Mem, Label, Cond}. encode_logical([Reg(x0), Reg(x1), third], opc) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [opc, third]
  domain: { opc: 0..3, third: {Symbol, Mem, Label, Cond} }
  relation:
    op: throws
    lhs: encode_logical([Reg(x0), Reg(x1), third], opc)
    rhs: String
expected_error: String
generators:
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: data_processing.rs:509 Err("unsupported logical operands"); llvm-mc rejects non-register/non-imm third
```

## encode_logical_neg_invalid_reg
- Tier: 4e
- Rationale: Coverage-sweep of get_reg / parse_reg_num None. Invalid names (foo, x32, w32, x, r0, empty) must Err.
- Seed: encode_eon_pbt invalid-name negatives
- Formal: ∀ opc, pos ∈ {0,1,2}, name ∈ {foo, x32, w32, x, r0, ""}. encode_logical with that name at pos is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_logical
oracle: negative_error
predicate:
  quantifier: forall
  vars: [opc, pos, name]
  domain: { opc: 0..3, pos: 0..2, name: {foo, x32, w32, x, r0, empty} }
  relation:
    op: throws
    lhs: encode_logical(ops with name at pos, opc)
    rhs: String
expected_error: String
generators:
  opc: { gen: int, min: 0, max: 3, type: u32 }
evidence: parse_reg_num returns None for non x/w/d/s/q/v/h/b prefixes and numbers >31; get_reg then Err
```
