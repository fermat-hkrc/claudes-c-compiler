# Properties: encode_neon_ld1r

## encode_neon_ld1r_diff_no_offset_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:12 same textual assembly as gas; README.md:235 ld1r). State machine rejected (pure function). Round-trip rejected (no in-tree decoder). Sibling encode_neon_ldnr / encode_neon_ld_st_single / encode_neon_ld_st_multi rejected (same-job gate).
- Doc contract: neon.rs:831 "Encode NEON LD1R: load single structure and replicate to all lanes" — asserted fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_pbt llvm-mc differential
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30 ∪ {SP}. encode_neon_ld1r([RegList({Vt.T}), Mem{Xn|SP, 0}]) = llvm-mc("ld1r {Vt.T}, [Xn|SP]").
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: differential
predicate:
  quantifier: forall
  vars: [t, rt, rn]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30 or SP"
  body: sut_word(ops) == llvm_mc_word(asm)
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 README.md:235 encoder/mod.rs:732 ARM AdvSIMD ld1r no-offset
```

## encode_neon_ld1r_diff_post_imm_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for immediate post-index. README.md:235 lists ld1r with post-index. Immediate #imm must equal esize(T).
- Doc contract: neon.rs:831 "Encode NEON LD1R: load single structure and replicate to all lanes" — asserted fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_pbt post-imm differential
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30 ∪ {SP}. Let imm = esize(T). encode_neon_ld1r([RegList({Vt.T}), MemPostIndex{Xn|SP, imm}]) = llvm-mc("ld1r {Vt.T}, [Xn|SP], #imm").
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: differential
predicate:
  quantifier: forall
  vars: [t, rt, rn]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30 or SP"
  body: sut_word(ops) == llvm_mc_word(asm)
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 README.md:235 encoder/mod.rs:732 ARM AdvSIMD ld1r imm post-index
```

## encode_neon_ld1r_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of ARM AdvSIMD replicate encoding. Weaker than differential; still pins field layout independently of llvm-mc. Stronger oracles rejected as above.
- Doc contract: neon.rs:831 "Encode NEON LD1R: load single structure and replicate to all lanes" — asserted fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_arm_fields
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..31, post ∈ {false,true}. Let w = encode_neon_ld1r(...). w[31]=0 ∧ w[30]=Q(T) ∧ w[29:24]=001101 ∧ w[23]=L ∧ w[22]=1 ∧ w[21]=0 ∧ w[20:16]=Rm ∧ w[15:13]=110 ∧ w[12]=0 ∧ w[11:10]=size(T) ∧ w[9:5]=Rn ∧ w[4:0]=Rt.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [t, rt, rn, post]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..31"
    post: bool
  body: fields(sut_word(ops)) match ARM LD1R layout
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  post: { gen: bool }
evidence: ARM AdvSIMD load/store single structure (replicate) LD1R opcode 110 S=0
```

## encode_neon_ld1r_metamorphic_rt_rn_q
- Tier: 4
- Rationale: Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; 8b vs 16b (4h vs 8h, 2s vs 4s, 1d vs 2d) flips only Q bit 30.
- Doc contract: neon.rs:831 "Encode NEON LD1R: load single structure and replicate to all lanes" — asserted fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_metamorphic_rt_rn_q
- Formal: ∀ T ∈ {8b,4h,2s,1d}, rt ∈ 0..30, rn ∈ 0..30. encode(rt+1) differs only in bits[4:0]; encode(rn+1) differs only in bits[9:5]; encode(wide T) ⊕ encode(narrow T) = 1<<30.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [t, rt, rn]
  domain:
    t: "{8b,4h,2s,1d}"
    rt: "0..30"
    rn: "0..30"
  body: rt+1 flips only bits[4:0]; rn+1 flips only bits[9:5]; wide vs narrow flips only Q
generators:
  t: { gen: oneof, values: ["8b","4h","2s","1d"] }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM AdvSIMD LD1R field layout Rt[4:0] Rn[9:5] Q[30]
```

## encode_neon_ld1r_neg_arity_kinds
- Tier: 5
- Rationale: Negative/error contract: fewer than 2 operands, non-RegList dest, non-Mem second operand must Err. llvm-mc/gas reject these forms.
- Doc contract: neon.rs:833 "LD1R {Vt.T}, [Xn]" — domain-restriction fingerprint b2ed7bb0
- Seed: neon.rs encode_neon_ldnr_neg_arity_kinds
- Formal: ∀ kind ∈ {empty, dest-only, bare-Reg dest, RegArrangement dest, swapped, Imm mem}, rt ∈ 0..31, rn ∈ 0..30. encode_neon_ld1r(ops_kind) is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, rt, rn]
  domain:
    kind: "0..5"
    rt: "0..31"
    rn: "0..30"
  body: encode_neon_ld1r(ops).is_err()
generators:
  kind: { gen: int, min: 0, max: 5, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: neon.rs:834-835 ld1r requires 2 operands; neon.rs:851 expected register list; neon.rs:883 expected [Xn] or post-index
```

## encode_neon_ld1r_neg_count_arr
- Tier: 5
- Rationale: Negative/error: list length != 1 and unsupported T must Err. llvm-mc rejects ld1r with two registers and unsupported arrangements. Quoted error strings declare those inputs invalid.
- Doc contract: neon.rs:841 "ld1r expects exactly one register in list" — domain-restriction fingerprint e54b103f
- Seed: neon.rs encode_neon_ldnr_neg_count_arr
- Formal: ∀ count ∈ {2,3,4,5}, T_bad ∈ {3s,8s,1s,b,h,2h,"",32b}, rt, rn. encode_neon_ld1r([RegList(count or T_bad), Mem]) is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [count, t_idx, rt, rn]
  domain:
    count: "2..5"
    t_idx: "0..7"
    rt: "0..31"
    rn: "0..30"
  body: encode_neon_ld1r(bad_T).is_err() AND encode_neon_ld1r(wrong_count).is_err()
generators:
  count: { gen: int, min: 2, max: 5, type: u32 }
  t_idx: { gen: int, min: 0, max: 7, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: neon.rs:841 expects exactly one register; neon.rs:863 unsupported arrangement
```

## encode_neon_ld1r_neg_extra
- Tier: 5
- Rationale: Negative/error: a surplus operand that llvm-mc/gas reject must Err. encode_neon_ld1r checks only operands.len() < 2 so extra is ignored.
- Doc contract: neon.rs:833 "LD1R {Vt.T}, [Xn]" — domain-restriction fingerprint b2ed7bb0
- Seed: neon.rs encode_neon_ldnr_neg_extra
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30, extra ∈ {Cond, Shift, RegArrangement, Label}. encode_neon_ld1r([RegList, Mem, extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: failing
- Counterexample: t="8b", rt=0, rn=0, extra_kind=0 (Cond "eq") — SUT Ok(0x0d40c000) vs Err
- Bug report: pbt-out/bug_reports/encode_neon_ld1r_extra_operand.md

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [t, rt, rn, extra_kind]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30"
    extra_kind: "0..3"
  body: encode_neon_ld1r([list, mem, extra]).is_err()
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: README.md:12 gas-compatible; llvm-mc rejects surplus operand after ld1r
```

## encode_neon_ld1r_neg_invalid_base
- Tier: 5
- Rationale: Negative/error: base must be Xn|SP. llvm-mc rejects W, WZR, WSP, XZR, x31, FP bases. parse_reg_num accepts them.
- Doc contract: neon.rs:883 "ld1r: expected [Xn] or [Xn], #imm memory operand" — domain-restriction fingerprint e848543f
- Seed: neon.rs encode_neon_ldnr_neg_invalid_base
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, base ∈ {w0,wzr,wsp,xzr,x31,d0,s0,v0,q0}. encode_neon_ld1r([RegList, Mem{base}]) is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: failing
- Counterexample: t="8b", rt=0, base="w0" — SUT Ok(0x0d40c000) vs Err
- Bug report: pbt-out/bug_reports/encode_neon_ld1r_invalid_base.md

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [t, rt, base]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    base: "{w0,wzr,wsp,xzr,x31,d0,s0,v0,q0}"
  body: encode_neon_ld1r(ops).is_err()
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  base: { gen: oneof, values: ["w0","wzr","wsp","xzr","x31","d0","s0","v0","q0"] }
expected_error: String
evidence: README.md:12 gas-compatible; llvm-mc requires Xn|SP base for ld1r
```

## encode_neon_ld1r_diff_post_reg_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for register post-index (README.md:235 ld1r with post-index). Trailing Xm after [Xn] is a valid ARM form; SUT ignores the extra GPR and encodes no-offset.
- Doc contract: neon.rs:831 "Encode NEON LD1R: load single structure and replicate to all lanes" — asserted fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_diff_post_reg_llvm_mc
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30, rm ∈ 0..30. encode_neon_ld1r([RegList({Vt.T}), Mem{Xn}, Reg(Xm)]) = llvm-mc("ld1r {Vt.T}, [Xn], Xm").
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: failing
- Counterexample: t="8b", rt=0, rn=0, rm=0 — SUT 0x0d40c000 vs llvm-mc 0x0dc0c000
- Bug report: pbt-out/bug_reports/encode_neon_ld1r_reg_post.md

```property
function: encoder.neon.encode_neon_ld1r
oracle: differential
predicate:
  quantifier: forall
  vars: [t, rt, rn, rm]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30"
    rm: "0..30"
  body: sut_word(ops) == llvm_mc_word(asm)
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
evidence: README.md:12 README.md:235 ARM AdvSIMD ld1r register post-index Rm=Xm L=1
```

## encode_neon_ld1r_neg_bad_post_imm
- Tier: 5
- Rationale: Negative/error: immediate post-index #imm must equal esize(T). llvm-mc rejects other values. SUT discards offset (neon.rs:876).
- Doc contract: neon.rs:876 "offset must match element size, not encoded separately" — limitation fingerprint 74bafa02
- Seed: neon.rs encode_neon_ldnr_neg_bad_post_imm
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt, rn, imm ∈ {-1,0,3,5,7,64,256} \ {esize(T)}. encode_neon_ld1r([RegList, MemPostIndex{imm}]) is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: failing
- Counterexample: t="8b", rt=0, rn=0, imm=-1 — SUT Ok(0x0ddfc000) vs Err
- Bug report: pbt-out/bug_reports/encode_neon_ld1r_bad_post_imm.md

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [t, rt, rn, imm]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30"
    imm: "{-1,0,3,5,7,64,256} minus esize(T)"
  body: encode_neon_ld1r(ops).is_err()
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: oneof, values: [-1, 0, 3, 5, 7, 64, 256] }
expected_error: String
evidence: README.md:12 llvm-mc rejects ld1r post-index #imm not equal to esize
```

## encode_neon_ld1r_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for uppercase V/X spellings. Strengthening round after first batch.
- Doc contract: neon.rs:831 "Encode NEON LD1R: load single structure and replicate to all lanes" — asserted fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_diff_alt_spellings
- Formal: ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..30, rn ∈ 0..30. encode_neon_ld1r([RegList({Vrt.T}), Mem{Xrn}]) = llvm-mc("ld1r {Vrt.T}, [Xrn]").
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: differential
predicate:
  quantifier: forall
  vars: [t, rt, rn]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..30"
    rn: "0..30"
  body: sut_word(ops) == llvm_mc_word(asm)
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: README.md:12 gas-compatible; llvm-mc accepts uppercase V/X
```

## encode_neon_ld1r_neg_invalid_name
- Tier: 5
- Rationale: Negative/error: invalid register names (foo/x32/v32/r0/x/v/empty) must Err. Sweep of documented rejection path.
- Doc contract: neon.rs:845 "invalid reg" — domain-restriction fingerprint c12eadcd
- Seed: neon.rs encode_neon_ldnr_neg_invalid_name
- Formal: ∀ T valid, name ∈ {foo,x32,v32,r0,x,v,""}, slot ∈ {0,1}. encode_neon_ld1r with that name in dest or base is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [t, name, slot]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    name: "{foo,x32,v32,r0,x,v,empty}"
    slot: "0..1"
  body: encode_neon_ld1r(ops).is_err()
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  name: { gen: oneof, values: ["foo","x32","v32","r0","x","v",""] }
  slot: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: neon.rs:845 parse_reg_num None; llvm-mc rejects invalid names
```

## encode_neon_ld1r_neg_mem_offset
- Tier: 5
- Rationale: Negative/error: [Xn, #imm] (nonzero offset Mem) is not a valid LD1R form. llvm-mc rejects it. Sweep of the offset:0 match arm.
- Doc contract: neon.rs:883 "ld1r: expected [Xn] or [Xn], #imm memory operand" — domain-restriction fingerprint e848543f
- Seed: neon.rs encode_neon_ldnr_neg_mem_offset
- Formal: ∀ T valid, rt, rn ∈ 0..30, off ∈ {-1,1,4,8,16}. encode_neon_ld1r([RegList, Mem{Xn, off}]) is Err.
- Test file: src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_ld1r
oracle: negative_error
predicate:
  quantifier: forall
  vars: [t, rt, rn, off]
  domain:
    t: "{8b,16b,4h,8h,2s,4s,1d,2d}"
    rt: "0..31"
    rn: "0..30"
    off: "{-1,1,4,8,16}"
  body: encode_neon_ld1r(ops).is_err()
generators:
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d"] }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  off: { gen: oneof, values: [-1, 1, 4, 8, 16] }
expected_error: String
evidence: neon.rs:866 Mem offset:0 only; llvm-mc rejects [Xn, #imm] for ld1r
```
