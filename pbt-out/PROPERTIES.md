# Properties: encode_neon_ldnr

## encode_neon_ldnr_diff_ld3r_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:12 same textual assembly as gas; README.md:235 ld3r). State machine rejected (pure function). Round-trip rejected (no in-tree decoder). Sibling encode_neon_ld1r / encode_neon_ld_st_single / encode_neon_ld_st_multi rejected (same-job gate). Restricted to n=3 because LD2R/LD4R encodings disagree with llvm-mc (see encode_neon_ldnr_diff_no_offset_llvm_mc).
- Seed: neon.rs encode_neon_tbl_pbt llvm-mc differential; encode_neon_ld1r no-offset encoding
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30 ∪ {SP}, post ∈ {false,true}. encode_neon_ldnr([RegList(consecutive wrap 3, T), Mem or MemPostIndex{#3·esize}], 3) = llvm-mc("ld3r {list}, [Xn|SP]{, #imm}").
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ldnr
oracle: differential
predicate:
  quantifier: forall
  vars: [t, rt, rn, post]
  domain:
    n: "3"
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30 or SP"
    post: bool
  body: sut_word(ops, 3) == llvm_mc_word(asm)
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  post: { gen: bool }
evidence: README.md:12 README.md:235 encoder/mod.rs:656 ARM AdvSIMD ld3r
```

## encode_neon_ldnr_diff_no_offset_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for no-offset LD2R/LD3R/LD4R. Same stronger-oracle rejection as ld3r. Fails for n=2/4 because S is encoded at bit 12 not bit 21.
- Seed: neon.rs encode_neon_tbl_pbt llvm-mc differential
- Formal: ∀ n ∈ {2,3,4}, T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30 ∪ {SP}. encode_neon_ldnr([RegList(list), Mem{Xn|SP, 0}], n) = llvm-mc("ldNr {list}, [Xn|SP]").
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=4, t="8b", rt=0, rn=0 — ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x0] SUT 0x0d40f000 vs llvm-mc 0x0d60e000
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_s_bit.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: differential
predicate:
  quantifier: forall
  vars: [n, t, rt, rn]
  domain:
    n: "{2,3,4}"
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30 or SP"
  body: sut_word(ops, n) == llvm_mc_word(asm)
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 README.md:235 encoder/mod.rs:655-657 ARM AdvSIMD ldNr replicate no-offset
```

## encode_neon_ldnr_diff_post_imm_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for immediate post-index. README.md:235 lists ld2r/ld3r/ld4r with post-index. Fails for n=2/4 (same S-bit bug).
- Seed: neon.rs encode_neon_ld1r MemPostIndex path
- Formal: ∀ n ∈ {2,3,4}, T valid, rt,rn. Let imm = n * esize(T). encode_neon_ldnr([RegList, MemPostIndex{Xn|SP, imm}], n) = llvm-mc("ldNr {list}, [Xn|SP], #imm").
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=4, t="8b", rt=0, rn=0 — ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x0], #4 SUT 0x0ddff000 vs llvm-mc 0x0dffe000
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_s_bit.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: differential
predicate:
  quantifier: forall
  vars: [n, t, rt, rn]
  domain:
    n: "{2,3,4}"
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30 or SP"
  body: sut_word(post_ops, n) == llvm_mc_word(post_asm)
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:235 neon.rs:1527 ARM AdvSIMD ldNr replicate post-index Rm=11111
```

## encode_neon_ldnr_diff_post_reg_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for register post-index. Parser leaves `[Xn], Xm` as Mem + Reg. gas/llvm-mc accept it. README.md:235 post-index.
- Seed: parser.rs:1808 merges only Mem+Imm, not Mem+Reg
- Formal: ∀ n ∈ {2,3,4}, T valid, rt, rn ∈ 0..30, rm ∈ 0..30. encode_neon_ldnr([RegList, Mem{Xn,0}, Reg(Xm)], n) = llvm-mc("ldNr {list}, [Xn], Xm").
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=2, t="8b", rt=0, rn=0, rm=0 — ld2r {v0.8b, v1.8b}, [x0], x0 SUT 0x0d40d000 vs llvm-mc 0x0de0c000
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_reg_post.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: differential
predicate:
  quantifier: forall
  vars: [n, t, rt, rn, rm]
  domain:
    n: "{2,3,4}"
    rm: "0..30"
  body: sut_word([list, mem, Reg(Xm)], n) == llvm_mc_word(asm)
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
evidence: README.md:235 llvm-mc ld2r {v0.8b,v1.8b},[x1],x2 = 0x0de2c020
```

## encode_neon_ldnr_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for uppercase V/X spellings (parse_reg_num lowercases). Fails for n=2/4 due to the S-bit bug, not due to spelling.
- Seed: load_store.rs encode_ldrs_diff_alt_spellings
- Formal: ∀ n ∈ {2,3,4}, T valid, rt,rn. encode_neon_ldnr with V/X uppercase names = llvm-mc("ldNr {V*.T}, [X*]").
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=4, t="8b", rt=0, rn=0 — same S-bit mismatch as no-offset
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_s_bit.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: differential
predicate:
  quantifier: forall
  vars: [n, t, rt, rn]
  body: sut_word(uppercase_ops, n) == llvm_mc_word(uppercase_asm)
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
evidence: README.md:12 parse_reg_num lowercases; llvm-mc accepts V0/X0
```

## encode_neon_ldnr_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM AdvSIMD replicate encoding and gas/llvm-mc KAT (S at bit 21, bit 12 = 0). Stronger differential already used; this pins field layout.
- Seed: gas KAT 0x0d60c020
- Formal: ∀ valid no-offset encoding w. bits[31]=0 ∧ bits[29:24]=001101 ∧ bit[22]=1 ∧ bit[12]=0 ∧ bits[15:13]=(110 if n∈{2} else 111) ∧ bit[21]=(1 if n∈{2,4} else 0) ∧ bits[11:10]=size(T) ∧ bit[30]=Q(T) ∧ bits[4:0]=rt ∧ bits[9:5]=rn ∧ bit[23]=0 ∧ bits[20:16]=0.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=4, t="8b", rt=0, rn=0 — bit21=0, expected 1
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_s_bit.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, t, rt, rn]
  body: arm_replicate_fields_hold(sut_word(ops, n), n, t, rt, rn, post=false)
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM AdvSIMD ld/st single structure replicate; gas/llvm-mc KAT ld2r {v0.8b,v1.8b},[x1]=0x0d60c020
```

## encode_neon_ldnr_metamorphic_rt_rn_q
- Tier: 4
- Rationale: Algebraic metamorphic: incrementing Rt/Rn or switching 8b↔16b (4h↔8h, 2s↔4s, 1d↔2d) must affect only the corresponding ARM field. Evidenced by ARM field map (Rt bits[4:0], Rn bits[9:5], Q bit 30).
- Seed: neon.rs encode_neon_dup_pbt metamorphic Rd/Rn/Q
- Formal: ∀ valid no-offset (n,T,rt,rn) with rt<31, rn<31. (w(rt+1) bits[4:0] = rt+1 ∧ other bits unchanged) ∧ (w(rn+1) bits[9:5] = rn+1 ∧ other bits unchanged) ∧ (wide T xor narrow T = 1<<30).
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ldnr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, t, rt, rn]
  domain:
    n: "{2,3,4}"
    t: "{8b,4h,2s,1d}"
    rt: "0..30"
    rn: "0..30"
  body: Rt/Rn fields increment independently; Q-pair flips only bit 30
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  t: { gen: oneof, values: ["8b","4h","2s","1d"] }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM AdvSIMD replicate Rt[4:0] Rn[9:5] Q[30]
```

## encode_neon_ldnr_neg_arity_kinds
- Tier: 4e
- Rationale: Negative/error contract. SUT documents "ldNr requires 2 operands", "expected register list", "expected memory operand". llvm-mc/gas reject missing operands, non-list dest, non-memory base.
- Seed: neon.rs encode_neon_tbl_pbt arity negatives
- Formal: ∀ n ∈ {2,3,4}. operands.len()<2 ∨ operands[0] not RegList ∨ operands[1] not Mem/MemPostIndex ⇒ encode_neon_ldnr is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, kind]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: neon.rs:1529 "ld{}r requires 2 operands"; neon.rs:1540 "expected register list"; neon.rs:1560 "expected memory operand"
```

## encode_neon_ldnr_neg_count_arr
- Tier: 4e
- Rationale: Negative/error contract. SUT documents expected n regs and supported arrangements. llvm-mc rejects wrong list length and unknown T.
- Seed: neon.rs encode_neon_ld1r unsupported arrangement
- Formal: ∀ n ∈ {2,3,4}. (list.len() ≠ n ∨ T ∉ {8b,16b,4h,8h,2s,4s,1d,2d}) ⇒ encode_neon_ldnr is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, count, t]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  count: { gen: int, min: 1, max: 5, type: u32 }
expected_error: String
evidence: neon.rs:1542 "expected {} regs"; neon.rs:1548 "unsupported arrangement"
```

## encode_neon_ldnr_neg_extra
- Tier: 4e
- Rationale: Negative/error contract from llvm-mc/gas (README.md:12): a surplus operand after a complete ldNr is invalid. A trailing GPR after [Xn] is register post-index (tested separately).
- Seed: neon.rs encode_neon_dup_neg_extra_operands
- Formal: ∀ valid 2-operand ldNr ops, extra ∈ {Cond, Shift, RegArrangement, Label}. encode_neon_ldnr(ops ++ [extra], n) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=2, t="8b", rt=0, rn=0, extra_kind=0 (Cond eq)
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_extra_operand.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, t, rt, rn, extra]
  body: encode_neon_ldnr(ops ++ [extra], n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  extra: { gen: oneof, values: ["cond","shift","arr","label"] }
expected_error: String
evidence: README.md:12 llvm-mc/gas reject a surplus operand on ld2r/ld3r/ld4r
```

## encode_neon_ldnr_neg_invalid_base
- Tier: 4e
- Rationale: Negative/error contract. ARM/gas/llvm-mc require base Xn|SP; reject W, XZR, x31, FP.
- Seed: load_store.rs encode_ldrs_neg_w_base / encode_ldrs_neg_xzr_base
- Formal: ∀ n ∈ {2,3,4}, T valid, rt ∈ 0..31, base ∈ {w0,wzr,wsp,xzr,x31,d0,s0,v0,q0}. encode_neon_ldnr([RegList, Mem{base,0}], n) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=2, t="8b", rt=0, base="w0"
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_w_base.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, t, rt, base]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  base: { gen: oneof, values: ["w0","wzr","wsp","xzr","x31","d0","s0","v0","q0"] }
expected_error: String
evidence: README.md:12 ARM ldNr base Xn|SP; llvm-mc rejects [w1]/[xzr]/[x31]/[d1]
```

## encode_neon_ldnr_neg_invalid_name
- Tier: 4e
- Rationale: Negative/error contract. parse_reg_num returns None for foo/x32/v32/r0/empty, so encode_neon_ldnr must Err.
- Seed: neon.rs encode_neon_dup invalid names
- Formal: ∀ n ∈ {2,3,4}, name ∈ {foo,x32,v32,r0,x,v,empty}, slot ∈ {list,base}. encode_neon_ldnr is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, name, slot]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  name: { gen: oneof, values: ["foo","x32","v32","r0","x","v",""] }
expected_error: String
evidence: encoder/mod.rs:131 parse_reg_num returns None outside x/w/d/s/q/v/h/b 0..31
```

## encode_neon_ldnr_neg_bad_post_imm
- Tier: 4e
- Rationale: Negative/error contract. gas/llvm-mc require post-index #imm = n*esize. Other immediates must Err.
- Seed: llvm-mc rejects ld2r {v0.8b,v1.8b}, [x1], #0 / #4 / #-1
- Formal: ∀ n ∈ {2,3,4}, T valid, imm ≠ n*esize(T). encode_neon_ldnr([RegList, MemPostIndex{Xn, imm}], n) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=2, t="8b", rt=0, rn=0, imm=-1
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_bad_post_imm.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, t, rt, rn, imm]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  imm: { gen: oneof, values: [-1, 0, 1, 5, 7, 64, 256] }
expected_error: String
evidence: llvm-mc/gas reject illegal post-index #imm; ARM post-index size = n*esize
```

## encode_neon_ldnr_neg_nonconsecutive
- Tier: 4e
- Rationale: Negative/error contract. gas/llvm-mc require consecutive wrapping same-T lists.
- Seed: llvm-mc rejects ld2r {v0.8b, v2.8b}, [x1]
- Formal: ∀ n ∈ {2,3,4}, skip≥1 such that second ≠ (rt+1) mod 32. encode_neon_ldnr(non-consecutive list, n) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=2, t="8b", rt=0, skip=1, rn=0 — {v0.8b, v2.8b}
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_nonconsecutive.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, t, rt, skip, rn]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  skip: { gen: int, min: 1, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc/gas require consecutive wrapping register lists
```

## encode_neon_ldnr_neg_mem_offset
- Tier: 4e
- Rationale: Negative/error contract. gas/llvm-mc reject [Xn, #imm] for ldNr (only [Xn] or post-index).
- Seed: llvm-mc rejects ld2r {v0.8b,v1.8b}, [x1, #0] and [x1, #4]
- Formal: ∀ n ∈ {2,3,4}, off ≠ 0. encode_neon_ldnr([RegList, Mem{Xn, off}], n) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: n=2, t="8b", rt=0, rn=0, off=-1
- Bug report: pbt-out/bug_reports/encode_neon_ldnr_mem_offset.md

```property
function: encoder.neon.encode_neon_ldnr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, t, rt, rn, off]
  body: encode_neon_ldnr(ops, n) is Err
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  off: { gen: oneof, values: [-1, 1, 4, 8, 16] }
expected_error: String
evidence: llvm-mc/gas reject [Xn, #imm] addressing on ld2r/ld3r/ld4r
```
