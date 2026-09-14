# Properties: encode_mul

## encode_mul_diff_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MUL decoder). encode_madd rejected by same-job sibling gate for the 4-operand public mnemonic (MADD vs MUL). Doc evidence: README.md:5-14 gas-compatible assembly; README.md:214 lists mul; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:238-244 dispatch; ARM ARM Data-processing (3 source) MUL `sf 00 11011 000 Rm 0 11111 Rn Rd`; llvm-mc `-triple=aarch64`.
- Seed: encode_madd_pbt::encode_madd_diff_gpr
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ {false,true}. encode_mul([Rd, Rn, Rm]) = llvm-mc("mul Rd, Rn, Rm") where each name is xN/xzr if is_64 else wN/wzr.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    is_64: bool
  relation:
    op: eq
    lhs: encode_mul([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn)), Reg(gpr(is_64, rm))])
    rhs: llvm_mc("mul {Rd}, {Rn}, {Rm}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 README.md:214 encoder/mod.rs:238-244 ARM ARM Data-processing (3 source) MUL
```

## encode_mul_diff_alias_madd_zr
- Tier: 2
- Rationale: Documented alias: encode_mul comment data_processing.rs:589 "MUL Rd, Rn, Rm is MADD Rd, Rn, Rm, XZR". llvm-mc aliases madd Rd,Rn,Rm,ZR to mul. Differential vs llvm-mc of both mnemonics. encode_madd itself is a different job (4-operand public mnemonic) so not a same-job sibling of encode_mul. Doc evidence: data_processing.rs:589; ARM ARM MUL is MADD with Ra=XZR/WZR.
- Seed: encode_madd_pbt::encode_madd_diff_ra_zr_is_mul
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ {false,true}. encode_mul([Rd, Rn, Rm]) = llvm-mc("mul Rd, Rn, Rm") = llvm-mc("madd Rd, Rn, Rm, ZR").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    is_64: bool
  relation:
    op: eq
    lhs: encode_mul([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn)), Reg(gpr(is_64, rm))])
    rhs: llvm_mc("madd {Rd}, {Rn}, {Rm}, {ZR}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: data_processing.rs:589 ARM ARM MUL is MADD with Ra=31 encoder/mod.rs:245
```

## encode_mul_metamorphic_sf_bit
- Tier: 4
- Rationale: ARM ARM sf bit is the sole 64 vs 32 distinguisher of otherwise-identical MUL encodings. Stronger differential covers full-word agreement; this metamorphic isolates sf. Round-trip rejected (no decoder). Doc evidence: ARM ARM `sf 00 11011 000 Rm 0 11111 Rn Rd`.
- Seed: encode_madd_pbt::encode_madd_metamorphic_sf_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_mul(X-ops) XOR encode_mul(W-ops) = 1<<31 at equal register numbers.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
  relation:
    op: eq
    lhs: encode_mul(x_ops) XOR encode_mul(w_ops)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) MUL sf at bit 31
```

## encode_mul_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on success-path words. Stronger differential covers full-word; this invariant pins Ra=31 and o0=0 (the MUL vs MADD/MSUB distinguisher) and each register field. Doc evidence: ARM ARM `sf 00 11011 000 Rm 0 11111 Rn Rd`.
- Seed: encode_madd_pbt::encode_madd_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ {false,true}. let w = encode_mul(...). (w>>31)&1 = sf(is_64) ∧ (w>>21)&0x3FF = 0b0011011000 ∧ (w>>16)&0x1F = rm ∧ (w>>15)&1 = 0 ∧ (w>>10)&0x1F = 31 ∧ (w>>5)&0x1F = rn ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain:
    rd: 0..31
    rn: 0..31
    rm: 0..31
    is_64: bool
  relation:
    op: holds
    expr: arm_mul_fields(encode_mul(ops), rd, rn, rm, is_64)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Data-processing (3 source) MUL sf 00 11011 000 Rm 0 11111 Rn Rd
```

## encode_mul_diff_neon
- Tier: 2
- Rationale: encode_mul dispatches RegArrangement dest to encode_neon_mul (data_processing.rs:586-588). Differential vs llvm-mc for valid T. State machine rejected. Round-trip rejected (no in-tree NEON MUL decoder). Doc evidence: README.md:224 NEON three-same lists mul; ARM ARM Advanced SIMD MUL `0 Q 0 01110 size 1 Rm 10011 1 Rn Rd`; T in {8B,16B,4H,8H,2S,4S}; llvm-mc `-triple=aarch64`.
- Seed: encode_logical NEON differential (same file)
- Formal: ∀ vd,vn,vm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. encode_mul([Vd.T, Vn.T, Vm.T]) = llvm-mc("mul Vd.T, Vn.T, Vm.T").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vn, vm, t]
  domain:
    vd: 0..31
    vn: 0..31
    vm: 0..31
    t: {8b,16b,4h,8h,2s,4s}
  relation:
    op: eq
    lhs: encode_mul([RegArrangement(v{vd}, t), RegArrangement(v{vn}, t), RegArrangement(v{vm}, t)])
    rhs: llvm_mc("mul v{vd}.{t}, v{vn}.{t}, v{vm}.{t}")
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  vm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, variants: [{ gen: const, value: "8b" }, { gen: const, value: "16b" }, { gen: const, value: "4h" }, { gen: const, value: "8h" }, { gen: const, value: "2s" }, { gen: const, value: "4s" }] }
evidence: README.md:224 ARM ARM Advanced SIMD MUL vector T in {8B,16B,4H,8H,2S,4S} data_processing.rs:586-588
```

## encode_mul_neg_too_few
- Tier: 5
- Rationale: llvm-mc "too few operands for instruction" for arity < 3. get_reg on missing index returns Err. Documented ARM 3-operand form. Stronger oracles do not apply to the invalid domain. Doc evidence: ARM ARM three-register MUL; llvm-mc rejects `mul x0, x1`.
- Seed: encode_madd_pbt::encode_madd_neg_too_few
- Formal: ∀ n ∈ {0..2}, ops a length-n register list. encode_mul(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, r0, r1, is_64]
  domain:
    n: 0..2
    r0: 0..31
    r1: 0..31
    is_64: bool
  relation:
    op: throws
    expr: encode_mul(ops[..n])
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  r0: { gen: int, min: 0, max: 31, type: u32 }
  r1: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: ARM ARM MUL three registers; llvm-mc too few operands
```

## encode_mul_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc "invalid operand for instruction" for a fourth operand. ARM ARM MUL has exactly three registers. The encoder must reject extra operands rather than silently drop them. Doc evidence: ARM ARM three-register form; llvm-mc rejects `mul x0, x1, x2, x3`.
- Seed: encode_madd_pbt::encode_madd_neg_extra_operand
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ {false,true}, extra ∈ {Reg, Imm, Shift}. encode_mul([Rd,Rn,Rm,extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_64=false, extra=Reg("x0") i.e. mul w0, w0, w0, x0
- Bug report: pbt-out/bug_reports/encode_mul_extra_operand.md

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, extra]
  domain:
    rd: 0..30
    rn: 0..30
    rm: 0..30
    is_64: bool
    extra: Reg or Imm or Shift
  relation:
    op: throws
    expr: encode_mul([Rd, Rn, Rm, extra])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, variants: [{ gen: const, value: "Reg(x0)" }, { gen: const, value: "Imm(0)" }, { gen: const, value: "Shift(lsl,0)" }] }
expected_error: String
evidence: ARM ARM MUL three registers; llvm-mc invalid operand for fourth
```

## encode_mul_neg_mixed_width
- Tier: 5
- Rationale: ARM MUL uses a single sf bit for all three registers. llvm-mc rejects mixed X/W (`mul x0, w1, x2`). The encoder must Err when Rd/Rn/Rm are not all W or all X. Doc evidence: ARM ARM single sf; llvm-mc "invalid operand".
- Seed: encode_madd_pbt::encode_madd_neg_mixed_width
- Formal: ∀ rd,rn,rm ∈ {0..30}, width flags not all equal. encode_mul of mixed X/W names is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true i.e. mul w0, w0, x0
- Bug report: pbt-out/bug_reports/encode_mul_mixed_width.md

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain:
    rd: 0..30
    rn: 0..30
    rm: 0..30
    rd64: bool
    rn64: bool
    rm64: bool
  relation:
    op: throws
    expr: encode_mul(mixed_width_ops)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: ARM ARM single sf bit; llvm-mc rejects mixed X/W MUL
```

## encode_mul_diff_lr
- Tier: 2
- Rationale: `lr` is a documented 64-bit alias of X30 (parse_reg_num encoder/mod.rs:135; llvm-mc rewrites lr to x30). Differential vs llvm-mc. Strengthening round. Doc evidence: encoder/mod.rs:135 "lr" => 30; is_64bit_reg treats lr as 64-bit.
- Seed: encode_madd_pbt::encode_madd_diff_lr
- Formal: ∀ which ∈ {0..2}, a,b ∈ {0..30}. placing "lr" in slot which of mul xA, xB, xC (other slots GPR x0-x30) ⇒ encode_mul = llvm-mc of that asm.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: differential
predicate:
  quantifier: forall
  vars: [which, a, b]
  domain:
    which: 0..2
    a: 0..30
    b: 0..30
  relation:
    op: eq
    lhs: encode_mul(ops_with_lr_at(which, a, b))
    rhs: llvm_mc(mul_asm_with_lr_at(which, a, b))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
evidence: encoder/mod.rs:135 parse_reg_num lr => 30; llvm-mc lr alias
```

## encode_mul_neg_sp
- Tier: 5
- Rationale: ARM MUL encoding uses register 31 as WZR/XZR, never WSP/SP. llvm-mc rejects `mul wsp, w0, w0` and `mul sp, x0, x1`. Strengthening round after extra-operand / mixed-width failures. Doc evidence: ARM ARM register 31 is ZR; llvm-mc "invalid operand".
- Seed: encode_madd_pbt::encode_madd_neg_sp
- Formal: ∀ which ∈ {0..2}, is_64 ∈ {false,true}, a,b ∈ {0..30}. placing sp/wsp in slot which of an otherwise-valid 3-GPR MUL ⇒ encode_mul is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0 i.e. mul wsp, w0, w0
- Bug report: pbt-out/bug_reports/encode_mul_sp.md

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain:
    which: 0..2
    is_64: bool
    a: 0..30
    b: 0..30
  relation:
    op: throws
    expr: encode_mul(ops_with_sp_at(which, is_64, a, b))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM register 31 is ZR not SP; llvm-mc rejects mul wsp / mul sp
```

## encode_mul_neg_fp
- Tier: 5
- Rationale: Integer MUL is GPR-only. llvm-mc rejects `mul d0, x1, x2`. parse_reg_num accepts d/s/q/v/h/b prefixes. Strengthening round. Doc evidence: ARM ARM GPR-only 3-source; llvm-mc "invalid operand".
- Seed: encode_madd_pbt::encode_madd_neg_fp
- Formal: ∀ which ∈ {0..2}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. placing prefixN in slot which of mul x0,x1,x2 ⇒ encode_mul is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 i.e. mul d0, x1, x2
- Bug report: pbt-out/bug_reports/encode_mul_fp_reg.md

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain:
    which: 0..2
    prefix: {d,s,q,v,h,b}
    n: 0..31
  relation:
    op: throws
    expr: encode_mul(ops_with_fp_at(which, prefix, n))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  prefix: { gen: oneof, variants: [{ gen: const, value: "d" }, { gen: const, value: "s" }, { gen: const, value: "q" }, { gen: const, value: "v" }, { gen: const, value: "h" }, { gen: const, value: "b" }] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM GPR-only 3-source MUL; llvm-mc rejects mul d0, x1, x2
```

## encode_mul_neg_neon_d
- Tier: 5
- Rationale: ARM ARM Advanced SIMD MUL is UNDEFINED when size==11 (64-bit elements). llvm-mc rejects `mul v0.2d, ...` and `mul v0.1d, ...`. neon_arr_to_q_size accepts 1d/2d. Strengthening round. Doc evidence: ARM ARM MUL (vector) `if size == '11' then UNDEFINED`; llvm-mc "invalid operand".
- Seed: (none) — ARM size==11 bound, not covered by scalar madd/msub seeds
- Formal: ∀ vd,vn,vm ∈ {0..31}, T ∈ {1d,2d}. encode_mul([Vd.T, Vn.T, Vm.T]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: vd=0, vn=0, vm=0, t="1d" i.e. mul v0.1d, v0.1d, v0.1d
- Bug report: pbt-out/bug_reports/encode_mul_neon_d.md

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vn, vm, t]
  domain:
    vd: 0..31
    vn: 0..31
    vm: 0..31
    t: {1d,2d}
  relation:
    op: throws
    expr: encode_mul([RegArrangement(v{vd}, t), RegArrangement(v{vn}, t), RegArrangement(v{vm}, t)])
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  vm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, variants: [{ gen: const, value: "1d" }, { gen: const, value: "2d" }] }
expected_error: String
evidence: ARM ARM MUL vector size==11 UNDEFINED; llvm-mc rejects 1d/2d
```

## encode_mul_neg_invalid_reg
- Tier: 5
- Rationale: parse_reg_num returns None for names outside x0-x31/w0-w31/aliases; get_reg then Err. llvm-mc rejects `mul foo, x1, x2`. Strengthening round. Doc evidence: encoder/mod.rs:131-147 parse_reg_num; llvm-mc unknown operand.
- Seed: encode_msub_pbt::encode_msub_neg_invalid_reg
- Formal: ∀ which ∈ {0..2}, bad ∈ {x32,w32,x99,w99,"",foo,r0,x,x-1}. encode_mul with bad at slot which of mul x0,x1,x2 is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..2
    bad: {x32,w32,x99,w99,"",foo,r0,x,x-1}
  relation:
    op: throws
    expr: encode_mul(ops_with_bad_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  bad: { gen: oneof, variants: [{ gen: const, value: "x32" }, { gen: const, value: "w32" }, { gen: const, value: "foo" }, { gen: const, value: "r0" }, { gen: const, value: "" }] }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num; llvm-mc unknown operand
```

## encode_mul_neg_non_register
- Tier: 5
- Rationale: get_reg requires Operand::Reg. llvm-mc rejects Imm/Symbol/Mem/Shift/Cond/Label as MUL operands. Coverage-sweep (coverage_gaps had no profraw; manual arm audit of get_reg). Doc evidence: encoder/mod.rs:956-965 get_reg; ARM ARM three-register MUL; llvm-mc.
- Seed: encode_msub_pbt::encode_msub_neg_non_register
- Formal: ∀ which ∈ {0..2}, bad ∈ {Imm, Symbol, Mem, Shift, Cond, Label}. encode_mul with bad at slot which of mul x0,x1,x2 is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..2
    bad: {Imm, Symbol, Mem, Shift, Cond, Label}
  relation:
    op: throws
    expr: encode_mul(ops_with_non_reg_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  bad: { gen: oneof, variants: [{ gen: const, value: "Imm" }, { gen: const, value: "Symbol" }, { gen: const, value: "Mem" }] }
expected_error: String
evidence: encoder/mod.rs:956-965 get_reg; ARM ARM three-register MUL
```

## encode_mul_neg_neon_mismatch_t
- Tier: 5
- Rationale: ARM ARM Advanced SIMD MUL requires matching T on Vd, Vn, Vm. llvm-mc rejects `mul v0.8b, v0.8b, v0.16b`. encode_neon_mul uses dest arrangement only. Coverage-sweep. Doc evidence: ARM ARM MUL (vector) all three share T; llvm-mc "invalid operand".
- Seed: (none) — ARM matching-T contract, not in scalar madd/msub seeds
- Formal: ∀ vd,vn,vm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s} with not all equal. encode_mul([Vd.Td, Vn.Tn, Vm.Tm]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: vd=0, vn=0, vm=0, td="8b", tn="8b", tm="16b" i.e. mul v0.8b, v0.8b, v0.16b
- Bug report: pbt-out/bug_reports/encode_mul_neon_mismatch_t.md

```property
function: encoder.data_processing.encode_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vn, vm, td, tn, tm]
  domain:
    vd: 0..31
    vn: 0..31
    vm: 0..31
    td: {8b,16b,4h,8h,2s,4s}
    tn: {8b,16b,4h,8h,2s,4s}
    tm: {8b,16b,4h,8h,2s,4s}
  relation:
    op: throws
    expr: encode_mul([RegArrangement(v{vd}, td), RegArrangement(v{vn}, tn), RegArrangement(v{vm}, tm)])
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  vm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, variants: [{ gen: const, value: "8b" }, { gen: const, value: "16b" }] }
  tn: { gen: oneof, variants: [{ gen: const, value: "8b" }, { gen: const, value: "16b" }] }
  tm: { gen: oneof, variants: [{ gen: const, value: "8b" }, { gen: const, value: "16b" }] }
expected_error: String
evidence: ARM ARM MUL vector matching T; llvm-mc rejects mul v0.8b, v0.8b, v0.16b
```
