# Properties: encode_neon_ld_st_multi

## encode_neon_ld_st_multi_diff_no_offset_llvm_mc
- Tier: 3
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:12 gas-compatible assembler; README.md:235 lists ld1-4/st1-4). State machine rejected (pure function). Round-trip rejected (no in-tree multiple-structures decoder). Sibling encode_neon_ld_st_single rejected (same-job gate: single-element encoding).
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:280
- Formal: ∀ n∈{1,2,3,4}, T∈valid(n), rt,rn∈0..31, load∈𝔹. encode_neon_ld_st_multi(RegList({v_rt.T .. consecutive wrap n_regs}), Mem[Xn|SP], load, n) = llvm-mc(`ldn/stn {v_rt.T..}, [Xn|SP]`) where n_regs=n for n≥2 else n_regs∈{1,2,3,4}; valid(1)={8b,16b,4h,8h,2s,4s,1d,2d}, valid(n≥2)={8b,16b,4h,8h,2s,4s,2d}
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: differential
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load]
  domain: { n: structs_1_4, t: valid_arr(n), rt: v0_31, rn: x0_sp, load: bool }
  body: sut_word([reg_list(rt,n_regs,t), mem0(rn)], load, n) == llvm_mc(asm_no_offset)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ld_st_multi_diff_post_imm_llvm_mc
- Tier: 3
- Rationale: Same differential vs llvm-mc for the documented post-index form (README.md:235 "with post-index"). Immediate must equal n_regs*(Q?16:8).
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:302
- Formal: ∀ n,T,rt,rn,load in the valid domain. encode_neon_ld_st_multi(RegList, MemPostIndex(Xn|SP, #n_regs*(Q?16:8)), load, n) = llvm-mc(`ldn/stn {..}, [Xn|SP], #imm`)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: differential
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load]
  domain: { n: structs_1_4, t: valid_arr(n), rt: v0_31, rn: x0_sp, load: bool }
  body: sut_word([reg_list(rt,n_regs,t), mem_post(rn, legal_imm)], load, n) == llvm_mc(asm_post_imm)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
evidence: src/backend/arm/assembler/README.md:235
```

## encode_neon_ld_st_multi_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM AdvSIMD load/store multiple structures layout cited in neon.rs:1075-1089 and encoder/mod.rs:1-7. Stronger differential already used above; this pins field placement independently of llvm-mc.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:326
- Formal: ∀ valid no-offset or legal-imm-post inputs. word bit31=0, bit30=Q(T), bits[29:24]=001100, bit23=post, bit22=L, bit21=0, Rm=0 or 31, opcode per neon.rs:1061-1065, size=size(T), Rn, Rt
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load, post]
  domain: { n: structs_1_4, t: valid_arr(n), rt: v0_31, rn: x0_sp, load: bool, post: bool }
  body: fields(encode_neon_ld_st_multi(...)) match ARM AdvSIMD multiple-structures layout
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
  post: { gen: bool }
evidence: src/backend/arm/assembler/encoder/neon.rs:1061
```

## encode_neon_ld_st_multi_metamorphic_rt_rn_l
- Tier: 4
- Rationale: Metamorphic: incrementing Rt/Rn or flipping load/store must change only the corresponding field (ARM layout). Required STANDARD metamorphic/differential (differential already present; this strengthens).
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:360
- Formal: ∀ T∈valid(1), rt,rn∈0..30. encode(rt+1,rn,load) differs only in bits[4:0]; encode(rt,rn+1,load) differs only in bits[9:5]; encode(rt,rn,!load) XOR encode(rt,rn,load) = 1<<22
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [t, rt, rn]
  domain: { t: valid_arr(1), rt: 0..30, rn: 0..30 }
  body: (w_rt xor w) masked to bits[4:0] AND (w_rn xor w) masked to bits[9:5] AND (w_st xor w) == 1<<22
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:1088
```

## encode_neon_ld_st_multi_neg_arity_kinds
- Tier: 4
- Rationale: Negative/error contract: llvm-mc/gas reject fewer than 2 operands and non (RegList, Mem) kinds. SUT returns Err for operands.len()<2, non-RegList dest, non-Mem second operand (neon.rs:1009-1057).
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:384
- Formal: ∀ n∈{1,2,3,4}, load∈𝔹. operands empty | dest-only | Reg dest | RegArrangement dest | swapped | Imm second ⇒ encode_neon_ld_st_multi = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n, load]
  domain: { kind: 0..5, n: structs_1_4, load: bool }
  body: encode_neon_ld_st_multi(bad_ops(kind), load, n).is_err()
generators:
  kind: { gen: int, min: 0, max: 5, type: u32 }
  n: { gen: int, min: 1, max: 4, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1009
```

## encode_neon_ld_st_multi_neg_count_arr_names
- Tier: 4
- Rationale: llvm-mc rejects unsupported arrangement, invalid register names, LD1 with 0 or 5 regs, and LD2/3/4 with list length ≠ n. SUT documents opcode only for those counts (neon.rs:1061-1065). Wrong list length is in the documented assembler domain.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:408
- Formal: ∀ n∈{1,2,3,4}. unsupported T | invalid name ∈{foo,v32,x32,r0,ε} | LD1 n_regs∉{1,2,3,4} | LD2/3/4 n_regs≠n ⇒ encode_neon_ld_st_multi = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: n=2, wrong_len=1, t_idx=0, name_idx=0, rn=0 (ld2 {v0.16b}, [x0] encodes)
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_wrong_reg_count.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, wrong_len, t_idx, name_idx, rn]
  domain: { n: structs_1_4, wrong_len: 1..5, t_idx: 0..4, name_idx: 0..4, rn: 0..30 }
  body: encode_neon_ld_st_multi(bad_count_or_arr_or_name, true, n).is_err()
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  wrong_len: { gen: int, min: 1, max: 5, type: u32 }
  t_idx: { gen: int, min: 0, max: 4, type: u32 }
  name_idx: { gen: int, min: 0, max: 4, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1061
```

## encode_neon_ld_st_multi_neg_extra
- Tier: 4
- Rationale: llvm-mc/gas reject a surplus non-post-index operand (README.md:12). SUT only inspects operands[2] when it is Imm or Reg (neon.rs:1078-1086) and otherwise falls through. Extra Cond/Shift/Label/RegArrangement is in the documented assembler domain and must Err.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:461
- Formal: ∀ valid no-offset ops, extra∈{Cond,Shift,RegArrangement,Label}. encode_neon_ld_st_multi(ops++[extra], load, n) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: n=1, n_regs=1, t=8b, rt=0, rn=0, load=false, extra_kind=0 (st1 {v0.8b}, [x0], eq encodes)
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_extra_operand.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load, extra_kind]
  domain: { n: structs_1_4, extra_kind: 0..3 }
  body: encode_neon_ld_st_multi(ops ++ [extra], load, n).is_err()
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ld_st_multi_neg_invalid_base
- Tier: 4
- Rationale: llvm-mc rejects W/XZR/x31/FP bases for AdvSIMD multiple-structure addressing (base is Xn|SP only). parse_reg_num maps those names to a number; the function's own comment does not declare them invalid, so they stay in domain.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:487
- Formal: ∀ n,T,rt,load, base∈{w0,w31,wsp,xzr,x31,s0,d0,v0,q0}. encode_neon_ld_st_multi(list, Mem{base}, load, n) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: n=1, n_regs=1, t=8b, rt=0, load=false, base=w0 (st1 {v0.8b}, [w0] encodes as [x0])
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_invalid_base.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, load, base]
  domain: { n: structs_1_4, base: {w0,w31,wsp,xzr,x31,s0,d0,v0,q0} }
  body: encode_neon_ld_st_multi(list, Mem{base}, load, n).is_err()
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ld_st_multi_diff_reg_post_llvm_mc
- Tier: 3
- Rationale: Sweep: register post-index `[Xn], Xm` is a documented ARM addressing mode (README.md:235 post-index; neon.rs:1078-1086). Differential vs llvm-mc.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:605
- Formal: ∀ valid inputs, rm∈0..30. encode_neon_ld_st_multi([RegList, Mem0, Reg(Xm)], load, n) = llvm-mc(`ldn/stn {..}, [Xn|SP], Xm`)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: differential
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, rm, load]
  domain: { n: structs_1_4, t: valid_arr(n), rm: 0..30 }
  body: sut_word([list, mem0(rn), Reg(Xm)], load, n) == llvm_mc(asm_reg_post)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
evidence: src/backend/arm/assembler/README.md:235
```

## encode_neon_ld_st_multi_diff_alt_spellings
- Tier: 3
- Rationale: Sweep: gas/llvm-mc accept uppercase V/X and arrangement (README.md:12). parse_reg_num lowercases names; neon_arr_to_q_size does not.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:510
- Formal: ∀ valid inputs. encode_neon_ld_st_multi(RegList({Vrt.T_upper..}), Mem[XN], load, n) = llvm-mc(uppercase spelling)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: ld3 {V2.2S, V3.2S, V4.2S}, [X10] — SUT Err "unsupported NEON arrangement: 2S"
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_uppercase_arrangement.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: differential
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load]
  domain: { n: structs_1_4, t: valid_arr(n), rt: 0..30, rn: 0..30 }
  body: sut_word(uppercase_ops, load, n) == llvm_mc(uppercase_asm)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ld_st_multi_neg_nonconsecutive
- Tier: 4
- Rationale: Sweep: llvm-mc reports "registers must be sequential"; ARM ISA requires consecutive wrapping lists. SUT uses only regs[0].
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:578
- Formal: ∀ n∈{2,3,4}, T∈valid(n), rt∈0..28, rn∈0..30, load∈𝔹. list with regs[1] skipped ⇒ encode_neon_ld_st_multi = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: n=2, t=8b, rt=0, rn=0, load=false (st2 {v0.8b, v2.8b}, [x0] encodes as {v0.8b, v1.8b})
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_nonconsecutive.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, t, rt, rn, load]
  domain: { n: {2,3,4}, t: valid_arr(n), rt: 0..28, rn: 0..30 }
  body: encode_neon_ld_st_multi(nonconsecutive_list, load, n).is_err()
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 28, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ld_st_multi_neg_bad_post_imm
- Tier: 4
- Rationale: Sweep: ARM immediate post-index amount is implicit (n_regs*(Q?16:8)); llvm-mc rejects any other #imm. SUT binds `_imm` and always uses Rm=11111.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:643
- Formal: ∀ valid inputs, bad≠legal_imm. encode_neon_ld_st_multi(RegList, MemPostIndex(Xn, #bad), load, n) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: n=1, n_regs=1, t=8b, rt=0, rn=0, load=false, bad=0 (st1 {v0.8b}, [x0], #0 encodes as #8)
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_bad_post_imm.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load, bad]
  domain: { n: structs_1_4, bad: illegal_post_imm }
  body: encode_neon_ld_st_multi(list, mem_post(rn, bad), load, n).is_err()
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:235
```

## encode_neon_ld_st_multi_neg_mem_offset
- Tier: 4
- Rationale: Sweep: `[Xn, #imm]` is not a valid multiple-structure addressing mode (llvm-mc rejects). SUT matches only Mem { offset: 0 }.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: encode_neon_ld_st_single_pbt.rs:664
- Formal: ∀ valid inputs, off∈{1,4,8,-4,16}. encode_neon_ld_st_multi(RegList, Mem{Xn, off}, load, n) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, n_regs, t, rt, rn, load, off]
  domain: { n: structs_1_4, off: {1,4,8,-4,16} }
  body: encode_neon_ld_st_multi(list, Mem{rn, off}, load, n).is_err()
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  n_regs: { gen: int, min: 1, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1031
```

## encode_neon_ld_st_multi_neg_1d_ldn
- Tier: 4
- Rationale: Sweep: llvm-mc rejects .1d for LD2/ST2/LD3/ST3/LD4/ST4 (only LD1/ST1 allow .1d). neon_arr_to_q_size accepts 1d for all n.
- Doc contract: neon.rs:1007 "Common encoder for LD1/ST1 (multiple structures)" — asserted fingerprint 1717cd25
- Seed: (none) — llvm-mc rejection of ld2 {v0.1d, v1.1d}, [x0]
- Formal: ∀ n∈{2,3,4}, rt,rn,load. encode_neon_ld_st_multi(RegList({v.1d}×n), Mem[Xn], load, n) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs
- Status: failing
- Counterexample: n=2, rt=0, rn=0, load=false (st2 {v0.1d, v1.1d}, [x0] encodes)
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_multi_1d_ldn.md

```property
function: encoder.encode_neon_ld_st_multi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rt, rn, load]
  domain: { n: {2,3,4}, rt: v0_31, rn: 0..30 }
  body: encode_neon_ld_st_multi(reg_list(rt, n, "1d"), mem0(rn), load, n).is_err()
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```
