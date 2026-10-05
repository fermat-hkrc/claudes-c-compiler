# Properties: encode_neon_ld_st_single

## encode_neon_ld_st_single_diff_no_offset_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (gas agrees on KAT). State machine rejected (pure function). Round-trip rejected (no in-tree decoder). Sibling encode_neon_ld_st_multi / encode_neon_ld1r / encode_neon_ldnr rejected (different ARM class / mnemonic). README.md:12 claims gas-compatible assembly; encoder/mod.rs:1-7 claims 32-bit AArch64 words; dispatch encoder/mod.rs:733-741 routes ld1-4/st1-4 indexed lists here.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs:222 encode_neon_ld1r_diff_no_offset_llvm_mc
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0, max(sz)], rt,rn ∈ 0..31, load ∈ {0,1}. encode_neon_ld_st_single([{Vt.sz..Vt+n-1.sz}[idx], [Xn|SP]], load, n) = llvm-mc(`ldN|stN {Vt.sz, ...}[idx], [Xn|SP]`)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: differential
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..31, load: {0,1} }
  relation:
    op: eq
    lhs: encode_neon_ld_st_single(RegListIndexed(consecutive_wrap(rt,n,sz), idx), Mem(Xn|SP), load, n)
    rhs: llvm_mc(ldN_or_stN {Vt.sz..}[idx], [Xn|SP])
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
evidence: README.md:12 gas-compatible assembly; encoder/mod.rs:733-741 dispatch; neon.rs:899
```

## encode_neon_ld_st_single_diff_post_imm_llvm_mc
- Tier: 5
- Rationale: README.md:235 lists ld1-4/st1-4 with post-index. ARM Rm=11111 immediate post-index. Body already encodes MemPostIndex (neon.rs:943-946, 994-998) despite stale TODO at neon.rs:903. Same differential reference as no-offset.
- Doc contract: neon.rs:903 "TODO: add post-index form [Xn], #imm" — limitation fingerprint 34dfb7e2 (stale: body implements it; keep post-index in the success domain)
- Seed: encode_neon_ld1r_pbt.rs:238 encode_neon_ld1r_diff_post_imm_llvm_mc
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0, max(sz)], rt,rn ∈ 0..31, load ∈ {0,1}. encode_neon_ld_st_single([{Vt.sz..}[idx], [Xn|SP], #n*esize(sz)], load, n) = llvm-mc(`ldN|stN {Vt.sz..}[idx], [Xn|SP], #n*esize`)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: differential
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..31, load: {0,1} }
  relation:
    op: eq
    lhs: encode_neon_ld_st_single(RegListIndexed(consecutive_wrap(rt,n,sz), idx), MemPostIndex(Xn|SP, n*esize), load, n)
    rhs: llvm_mc(ldN_or_stN {Vt.sz..}[idx], [Xn|SP], #n*esize)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
evidence: README.md:235 with post-index; ARM Rm=11111; neon.rs:943-998
```

## encode_neon_ld_st_single_arm_fields
- Tier: 4
- Rationale: Weaker than differential; pins the ARM AdvSIMD single-structure layout independently of llvm-mc (bit31=0, bits[29:23]=0011010/0011011, L/R/Rm/opcode/S/size/Rn/Rt). Rejected stronger: state machine / round-trip as above.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs:254 encode_neon_ld1r_arm_fields
- Formal: ∀ valid (n,sz,idx,rt,rn,load,post). let w = encode_neon_ld_st_single(...). w[31]=0 ∧ w[29:23]=(post?0011011:0011010) ∧ w[22]=load ∧ w[21]=R(n) ∧ w[20:16]=(post?31:0) ∧ w[15:13]=opcode(sz,n) ∧ w[12]=S(sz,idx) ∧ w[11:10]=size(sz,idx) ∧ w[30]=Q(sz,idx) ∧ w[9:5]=rn ∧ w[4:0]=rt
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load, post]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..31, load: bool, post: bool }
  relation:
    op: holds
    expr: arm_fields_match(encode_neon_ld_st_single(list, mem, load, n, post))
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
  post: { gen: bool }
evidence: ARM AdvSIMD load/store single structure; neon.rs:950-1004
```

## encode_neon_ld_st_single_metamorphic_rt_rn_l_r
- Tier: 4
- Rationale: Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; load vs store flips only L bit 22; n=1 vs n=2 (same opcode family) flips only R bit 21. Weaker than differential; complementary field isolation.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs:287 encode_neon_ld1r_metamorphic_rt_rn_q
- Formal: ∀ sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], rt,rn ∈ 0..30. let w = encode(..., load=1, n=1). encode(rt+1) ⊕ w = 1 in bits[4:0] ∧ encode(rn+1) ⊕ w = 1 in bits[9:5] ∧ encode(load=0) ⊕ w = 1<<22 ∧ encode(n=2 consecutive) ⊕ w = 1<<21
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [sz, idx, rt, rn]
  domain: { sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..30, rn: 0..30 }
  relation:
    op: holds
    expr: (w_rt1 ^ w) == 1 && (w_rn1 ^ w) == (1<<5) && (w_st ^ w) == (1<<22) && (w_n2 ^ w) == (1<<21)
generators:
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM Rt/Rn/L/R field placement; neon.rs:950-1004
```

## encode_neon_ld_st_single_neg_arity_kinds
- Tier: 3
- Rationale: Documented form needs a RegListIndexed plus a memory operand (neon.rs:899-903 examples; neon.rs:905-911). Fewer than 2 operands, non-indexed list, non-memory second operand must Err. Negative/error contract; stronger oracles do not apply to invalid shapes.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs:320 encode_neon_ld1r_neg_arity_kinds
- Formal: ∀ ops ∈ {[], [list], [Reg, Mem], [RegArrangement, Mem], [Mem, list], [list, Imm]}. encode_neon_ld_st_single(ops, load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n, load]
  domain: { kind: 0..5, n: {1,2,3,4}, load: bool }
  relation:
    op: throws
    expr: encode_neon_ld_st_single(bad_ops(kind), load, n)
    error: String
generators:
  kind: { gen: int, min: 0, max: 5, type: u32 }
  n: { gen: int, min: 1, max: 4, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: neon.rs:905-911 arity and RegListIndexed requirement
```

## encode_neon_ld_st_single_neg_count_size_names
- Tier: 3
- Rationale: neon.rs:914-916 requires regs.len()==num_structs and returns Err otherwise; neon.rs:991 returns Err for element size outside {b,h,s,d}; parse_reg_num returns None for foo/v32/x32/r0/empty. These are documented invalid inputs (negative-error contract), not a narrowed valid domain.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs:342 encode_neon_ld1r_neg_count_arr
- Formal: ∀ n ∈ {1,2,3,4}. (len ≠ n ∨ sz ∉ {b,h,s,d} ∨ name ∉ valid v-regs) ⇒ encode_neon_ld_st_single(...) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, wrong_len, bad_sz, bad_name]
  domain: { n: {1,2,3,4}, wrong_len: 0..5, bad_sz: {8b,4s,q,""}, bad_name: {foo,v32,x32,r0,""} }
  relation:
    op: throws
    expr: encode_neon_ld_st_single(mismatched_or_bad, true, n)
    error: String
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  wrong_len: { gen: int, min: 0, max: 5, type: u32 }
  bad_sz: { gen: oneof, options: ["8b", "4s", "q", ""] }
  bad_name: { gen: oneof, options: ["foo", "v32", "x32", "r0", ""] }
expected_error: String
evidence: neon.rs:914-916 count; neon.rs:991 size; parse_reg_num invalid names
```

## encode_neon_ld_st_single_neg_extra
- Tier: 3
- Rationale: llvm-mc/gas reject a surplus operand. SUT checks only operands.len() < 2 (neon.rs:905), so extra is ignored — a negative-error contract the public assembler claims (README.md:12). Keep the domain; do not narrow.
- Doc contract: neon.rs:905 "if operands.len() < 2" — other fingerprint 7db664a5
- Seed: encode_neon_ld1r_pbt.rs:365 encode_neon_ld1r_neg_extra
- Formal: ∀ valid (n,sz,idx,rt,rn,load), extra ∈ {Cond, Shift, Label, RegArrangement}. encode_neon_ld_st_single([list, mem, extra], load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=1, sz=b, idx=0, rt=0, rn=0, load=false, extra=Cond("eq") — st1 {v0.b}[0], [x0] plus extra encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_extra_operand.md

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load, extra]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..30, extra: {Cond, Shift, Label, RegArrangement} }
  relation:
    op: throws
    expr: encode_neon_ld_st_single([list, mem, extra], load, n)
    error: String
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: README.md:12 gas-compatible; llvm-mc rejects surplus operand
```

## encode_neon_ld_st_single_neg_invalid_base
- Tier: 3
- Rationale: llvm-mc/gas reject W-base, XZR, x31, FP base. parse_reg_num accepts them (mod.rs:133-149). README.md:12 gas compatibility makes rejection the contract. Keep the domain.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs encode_neon_ld1r_neg_invalid_base
- Formal: ∀ base ∈ {w0,w31,wsp,xzr,x31,s0,d0,v0,q0}. encode_neon_ld_st_single([list, Mem(base)], load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=1, sz=b, idx=0, rt=0, load=false, base=w0 — st1 {v0.b}[0], [w0] encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_invalid_base.md

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, load, base]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, base: {w0,w31,wsp,xzr,x31,s0,d0,v0,q0} }
  relation:
    op: throws
    expr: encode_neon_ld_st_single([list, Mem(base)], load, n)
    error: String
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  load: { gen: bool }
  base: { gen: oneof, options: ["w0", "w31", "wsp", "xzr", "x31", "s0", "d0", "v0", "q0"] }
expected_error: String
evidence: README.md:12; llvm-mc rejects W/XZR/x31/FP base
```

## encode_neon_ld_st_single_diff_alt_spellings
- Tier: 5
- Rationale: README.md:12 gas-compatible assembly is case-insensitive on GNU as / llvm-mc for Vn.T / Xn. Differential vs llvm-mc on uppercase V/X and uppercase arrangement.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8
- Seed: encode_neon_ld1r_pbt.rs encode_neon_ld1r_diff_alt_spellings
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], rt,rn ∈ 0..30, load ∈ {0,1}. encode_neon_ld_st_single([{Vrt.SZ..}[idx], [Xrn]], load, n) = llvm-mc(`ldN|stN {Vrt.SZ..}[idx], [Xrn]`)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=1, sz=b, idx=0, rt=0, rn=0, load=false — st1 {V0.B}[0], [X0] SUT Err "unsupported element size for ld/st single: B"; llvm-mc encodes 0x0d000000
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_alt_spellings.md

```property
function: neon.encode_neon_ld_st_single
oracle: differential
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..30, rn: 0..30, load: bool }
  relation:
    op: eq
    lhs: encode_neon_ld_st_single(RegListIndexed(V_upper, sz_upper, idx), Mem(X_upper), load, n)
    rhs: llvm_mc(uppercase_asm)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
evidence: README.md:12 gas-compatible; llvm-mc accepts V0.B / X0
```

## encode_neon_ld_st_single_neg_index_oor
- Tier: 3
- Rationale: llvm-mc rejects lane index outside .b[0,15] / .h[0,7] / .s[0,3] / .d[0,1]. SUT masks index bits with no range check. Negative-error contract from the assembler ISA.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8 (examples use [0]; no OOR)
- Seed: encode_neon_dup_pbt encode_neon_dup_neg_index_oor
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx > max_lane(sz). encode_neon_ld_st_single([{Vt.sz..}[idx], [Xn]], load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=1, sz=b, rt=0, rn=0, load=false, idx=16 — st1 {v0.b}[16], [x0] encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_index_oor.md

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: max_lane(sz)+1 .. max_lane(sz)+8, rt: 0..31, rn: 0..30, load: bool }
  relation:
    op: throws
    expr: encode_neon_ld_st_single(RegListIndexed(rt,n,sz,idx), Mem(Xn), load, n)
    error: String
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 2, max: 23, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: llvm-mc "vector lane must be an integer in range"
```

## encode_neon_ld_st_single_neg_nonconsecutive
- Tier: 3
- Rationale: ARM ISA requires consecutive (wrapping) registers in the list. llvm-mc rejects non-sequential lists. neon.rs:918 TODO admits the encoder does not validate this on an input the API accepts.
- Doc contract: neon.rs:918 "TODO: validate that registers in the list are consecutive (ARM ISA requirement)" — limitation fingerprint ef1958b6
- Seed: encode_neon_ldnr_pbt test_encode_neon_ldnr_regression_nonconsecutive
- Formal: ∀ n ∈ {2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], list not consecutive-wrapping. encode_neon_ld_st_single([list, [Xn]], load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=2, sz=b, idx=0, rt=0, rn=0, load=false — st2 {v0.b, v2.b}[0], [x0] encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_nonconsecutive.md

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load]
  domain: { n: {2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..28, rn: 0..30, load: bool }
  relation:
    op: throws
    expr: encode_neon_ld_st_single(RegListIndexed(nonconsecutive, idx), Mem(Xn), load, n)
    error: String
generators:
  n: { gen: int, min: 2, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 28, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
expected_error: String
evidence: neon.rs:918 TODO; llvm-mc "registers must be sequential"
```

## encode_neon_ld_st_single_neg_reg_post
- Tier: 5
- Rationale: README.md:235 lists post-index. ARM register post-index is Rm=Xm, bit23=1. llvm-mc accepts `ldN {..}[idx], [Xn], Xm`. SUT ignores extra Reg (only Imm becomes post-index) and encodes no-offset.
- Doc contract: neon.rs:903 "TODO: add post-index form [Xn], #imm" — limitation fingerprint 34dfb7e2 (covers imm form; register post-index is the same addressing class)
- Seed: encode_neon_ld1r_pbt encode_neon_ld1r_diff_post_reg_llvm_mc
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], rt,rn,rm ∈ 0..30, load ∈ {0,1}. encode_neon_ld_st_single([list, [Xn], Xm], load, n) = llvm-mc(`ldN|stN {..}[idx], [Xn], Xm`)
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=1, sz=b, idx=0, rt=0, rn=0, rm=0, load=false — st1 {v0.b}[0], [x0], x0 SUT=0x0d000000 llvm-mc=0x0d800000
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_reg_post.md

```property
function: neon.encode_neon_ld_st_single
oracle: differential
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, rm, load]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..30, rm: 0..30, load: bool }
  relation:
    op: eq
    lhs: encode_neon_ld_st_single([list, Mem(Xn), Reg(Xm)], load, n)
    rhs: llvm_mc(ldN_or_stN {..}[idx], [Xn], Xm)
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
evidence: README.md:235 with post-index; ARM Rm=Xm bit23=1
```

## encode_neon_ld_st_single_neg_bad_post_imm
- Tier: 3
- Rationale: llvm-mc requires post-index #imm = n*esize. SUT binds Some(_offset) and always encodes Rm=11111, ignoring the value.
- Doc contract: neon.rs:903 "TODO: add post-index form [Xn], #imm" — limitation fingerprint 34dfb7e2
- Seed: encode_neon_ld1r_pbt encode_neon_ld1r_neg_bad_post_imm
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, imm ≠ n*esize(sz). encode_neon_ld_st_single([list, MemPostIndex(Xn, imm)], load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: failing
- Counterexample: n=1, sz=b, idx=0, rt=0, rn=0, load=false, imm=0 — st1 {v0.b}[0], [x0], #0 encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_ld_st_single_bad_post_imm.md

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load, imm]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..30, imm: {-1,0,1,3,5,7,9,64} }
  relation:
    op: throws
    expr: encode_neon_ld_st_single([list, MemPostIndex(Xn, imm)], load, n)
    error: String
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
  imm: { gen: int, min: -1, max: 64, type: i64 }
expected_error: String
evidence: README.md:235; llvm-mc rejects illegal post-index #imm
```

## encode_neon_ld_st_single_neg_mem_offset
- Tier: 3
- Rationale: llvm-mc rejects [Xn, #imm] (unsigned/pre-index) for single-structure LD/ST. SUT matches only Mem { offset: 0 }, so nonzero offset is Err. Negative-error contract.
- Doc contract: neon.rs:899 "Encode NEON LD/ST single structure (element):" — asserted fingerprint 175ab2c8 (examples use [x3])
- Seed: encode_neon_ldnr_pbt test_encode_neon_ldnr_regression_mem_offset
- Formal: ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, off ∈ {1,4,8,-4,16}. encode_neon_ld_st_single([list, Mem(Xn, off)], load, n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: neon.encode_neon_ld_st_single
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, sz, idx, rt, rn, load, off]
  domain: { n: {1,2,3,4}, sz: {b,h,s,d}, idx: 0..max_lane(sz), rt: 0..31, rn: 0..30, off: {1,4,8,-4,16} }
  relation:
    op: throws
    expr: encode_neon_ld_st_single([list, Mem(Xn, off)], load, n)
    error: String
generators:
  n: { gen: int, min: 1, max: 4, type: u32 }
  sz: { gen: oneof, options: ["b", "h", "s", "d"] }
  idx: { gen: int, min: 0, max: 15, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  load: { gen: bool }
  off: { gen: int, min: -4, max: 16, type: i64 }
expected_error: String
evidence: llvm-mc rejects [Xn, #imm] for single-structure LD/ST
```
