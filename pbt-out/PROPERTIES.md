# Properties: encode_ldr_str_auto

## encode_ldr_str_auto_diff_gpr
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree LDR/STR decoder). encode_ldr_str rejected as differential sibling (callee; explicit size; shared get_reg). README.md:12 gas contract + encoder/mod.rs:484 size-from-register-width + load_store.rs:8 Wn/Xn size map. SUT-boundary: internal-helper of GNU-style assembler; encode_instruction passes operands through.
- Doc contract: load_store.rs:8 "Determine size from register: Wn -> 32-bit (size=10), Xn -> 64-bit (size=11)" — asserted fingerprint 368008af
- Seed: encode_ldr_str_pbt.rs:260 encode_ldr_str_kat_llvm_mc_x0_x1
- Formal: ∀ is_load ∈ {false,true}, rt ∈ [0,31], rn ∈ [0,31], imm12 ∈ [0,4095], width ∈ {W,X}. encode_ldr_str_auto([Reg(Rt), Mem{Xn|SP, imm12*(1<<size)}], is_load) = Word(llvm-mc(`ldr|str Rt, [Xn|SP, #pimm]`)) where size=10 for W and 11 for X, Rt≠SP
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, rt, rn, imm12, width]
  domain:
    is_load: bool
    rt: 0..31
    rn: 0..31
    imm12: 0..4095
    width: W_or_X
  body: sut_word(ops, is_load) == llvm_mc_word(asm)
generators:
  is_load: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
  width: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:12 encoder/mod.rs:484-486 load_store.rs:8
```

## encode_ldr_str_auto_diff_fp_sdq
- Tier: 5
- Rationale: Same differential vs llvm-mc for the FP sizes the function comment names (S/D/Q). encode_ldr_str SIMD sweep is not this symbol. Q must set is_128bit (opc 11/10, shift=4).
- Doc contract: load_store.rs:9 "FP: Sn -> 32-bit, Dn -> 64-bit, Qn -> 128-bit" — asserted fingerprint df05bd25
- Seed: encode_ldr_str_pbt.rs SIMD S/D/Q sweep (INVARIANTS.md encode_ldr_str)
- Formal: ∀ is_load ∈ {false,true}, rt ∈ [0,31], rn ∈ [0,31], imm12 ∈ [0,4095], pref ∈ {s,d,q}. encode_ldr_str_auto([Reg(pref+rt), Mem{Xn|SP, imm12*(1<<shift)}], is_load) = Word(llvm-mc(`ldr|str PrefRt, [Xn|SP, #pimm]`)) where (s→size=10 shift=2), (d→size=11 shift=3), (q→size=00 shift=4 is_128bit)
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, rt, rn, imm12, pref]
  domain:
    is_load: bool
    rt: 0..31
    rn: 0..31
    imm12: 0..4095
    pref: s_d_q
  body: sut_word(ops, is_load) == llvm_mc_word(asm)
generators:
  is_load: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
  pref: { gen: int, min: 0, max: 2, type: u32 }
evidence: load_store.rs:9 README.md:12
```

## encode_ldr_str_auto_diff_fp_bh
- Tier: 5
- Rationale: gas/llvm-mc accept `ldr/str Bt/Ht, [Xn|SP, #pimm]` (ARM SIMD&FP LDR/STR size=00/01 V=1). Function comment names S/D/Q but does not declare B/H invalid; parse_reg_num and is_fp_reg accept b/h; default-64-bit fallthrough is not an exclusion. README.md:12 gas contract.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: (none) — encode_ldr_str_pbt SIMD sweep covered S/D/Q only
- Formal: ∀ is_load ∈ {false,true}, rt ∈ [0,31], rn ∈ [0,31], imm12 ∈ [0,4095], pref ∈ {b,h}. encode_ldr_str_auto([Reg(pref+rt), Mem{Xn|SP, imm12*(1<<shift)}], is_load) = Word(llvm-mc(`ldr|str PrefRt, [Xn|SP, #pimm]`)) where (b→size=00 shift=0), (h→size=01 shift=1)
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: failing
- Counterexample: is_load=false, rt=0, rn=0, imm12=0, is_h=false → str b0, [x0] SUT=0xfd000000 llvm-mc=0x3d000000
- Bug report: bug_reports/encode_ldr_str_auto_byte_half_size.md

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, rt, rn, imm12, pref]
  domain:
    is_load: bool
    rt: 0..31
    rn: 0..31
    imm12: 0..4095
    pref: b_or_h
  body: sut_word(ops, is_load) == llvm_mc_word(asm)
generators:
  is_load: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
  pref: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:12 ARM ARM LDR immediate SIMD and FP Bt Ht
```

## encode_ldr_str_auto_size_v_bits
- Tier: 4
- Rationale: Algebraic invariant from ARM unsigned LDR/STR layout plus load_store.rs:8-9 size map. Independent of llvm-mc (unpack, not re-pack). Stronger differential is the sibling properties; this pins the auto-detect bits even if the reference is down. Domain includes B/H because parse_reg_num/is_fp_reg accept them and gas encodes them.
- Doc contract: load_store.rs:8 "Determine size from register: Wn -> 32-bit (size=10), Xn -> 64-bit (size=11)" — asserted fingerprint 368008af
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_arm_fields
- Formal: ∀ is_load, rt ∈ [0,30], rn ∈ [0,31], class ∈ {W,X,S,D,Q,B,H}. let w = encode_ldr_str_auto([Reg(class+rt), Mem{Xn|SP, 0}], is_load) as Word. unpack(w).size/V/opc match the ARM map: W→(10,0,load?01:00); X→(11,0,same); S→(10,1,same); D→(11,1,same); Q→(00,1,load?11:10); B→(00,1,load?01:00); H→(01,1,load?01:00). bits[29:27]=111, bits[25:24]=01, Rt=rt, Rn=rn
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: failing
- Counterexample: is_load=false, rt=0, rn=0, class=5 (B) -> size bits=11 expected 00
- Bug report: bug_reports/encode_ldr_str_auto_byte_half_size.md

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_load, rt, rn, class]
  domain:
    is_load: bool
    rt: 0..30
    rn: 0..31
    class: WXSDQBH
  body: unpack_unsigned(sut_word(ops, is_load)) == expected_size_v_opc(class, is_load, rt, rn)
generators:
  is_load: { gen: bool }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  class: { gen: int, min: 0, max: 6, type: u32 }
evidence: load_store.rs:8-9 ARM ARM LDR STR unsigned
```

## encode_ldr_str_auto_meta_load_store
- Tier: 4
- Rationale: Metamorphic: for a fixed Rt/Rn/offset=0, flipping is_load must flip only bit 22 (opc LSB) on GP and S/D/Q/B/H unsigned forms. W vs X at equal numbers differ only in size bits[31:30]. Stronger differential covers value equality; this isolates auto-detect independence of the load/store bit.
- Doc contract: encoder/mod.rs:484 "Loads/stores - size determined from register width" — asserted fingerprint e79e106a
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_meta_rt_rn_imm
- Formal: ∀ rt ∈ [0,30], rn ∈ [0,31], class ∈ {W,X,S,D,Q,B,H}. encode_ldr_str_auto(ops, true) XOR encode_ldr_str_auto(ops, false) = 1<<22. ∀ rt ∈ [0,30], rn ∈ [0,31]. encode_ldr_str_auto(W) XOR encode_ldr_str_auto(X) at offset 0 = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, class]
  domain:
    rt: 0..30
    rn: 0..31
    class: WXSDQBH
  body: (sut_word(ops, true) ^ sut_word(ops, false)) == (1u32 << 22)
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  class: { gen: int, min: 0, max: 6, type: u32 }
evidence: encoder/mod.rs:485-486 ARM opc load vs store
```

## encode_ldr_str_auto_neg_first_operand
- Tier: 4
- Rationale: Negative/error contract from load_store.rs:13 "ldr/str needs register operand". Empty slice, missing first operand, and non-Reg first operands (Imm/Mem/Symbol/Label/Shift/Cond) must return Err. Documented exact error path.
- Doc contract: load_store.rs:13 "ldr/str needs register operand" — asserted fingerprint 2ef6b00d
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_neg_arity_kinds
- Formal: ∀ is_load ∈ {false,true}, ops ∈ {[], [Imm(_)], [Mem{_}], [Symbol(_)], [Label(_)], [Shift{_}], [Cond(_)], [Reg(x0)]}. encode_ldr_str_auto(ops, is_load) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, kind]
  domain:
    is_load: bool
    kind: empty_or_nonreg_or_arity1
  body: encode_ldr_str_auto(ops, is_load).is_err()
generators:
  is_load: { gen: bool }
  kind: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: load_store.rs:13
```

## encode_ldr_str_auto_diff_aliases
- Tier: 5
- Rationale: Differential vs llvm-mc for Rt aliases the size-detect chain names as 64-bit GP (lr, xzr) and 32-bit GP (wzr) plus uppercase XZR/WZR/X0/W0 and x31/w31. SP as dest is a separate negative_error (llvm-mc rejects SP as Rt).
- Doc contract: load_store.rs:8 "Determine size from register: Wn -> 32-bit (size=10), Xn -> 64-bit (size=11)" — asserted fingerprint 368008af
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_kat_llvm_mc_x0_x1 (lr/xzr aliases)
- Formal: ∀ is_load, alias ∈ {lr, xzr, wzr, XZR, WZR, X0, W0, x31, w31}. encode_ldr_str_auto([Reg(alias), Mem{x1,0}], is_load) = Word(llvm-mc(`ldr|str alias, [x1]`))
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, alias]
  domain:
    is_load: bool
    alias: lr_xzr_wzr_XZR_WZR_X0_W0_x31_w31
  body: sut_word(ops, is_load) == llvm_mc_word(asm)
generators:
  is_load: { gen: bool }
  alias: { gen: int, min: 0, max: 8, type: u32 }
evidence: load_store.rs:8 README.md:12
```

## encode_ldr_str_auto_literal
- Tier: 5
- Rationale: LDR (literal) is a documented address form of ldr (not str). encode_ldr_str maps Symbol+is_load to WordWithReloc Ldr19 with opc from auto-detected size (W=00, X=01, S=00, D=01, Q=10). Differential on the reloc-form word vs llvm-mc ldr Rt, #0. STR+Symbol must Err (llvm-mc rejects).
- Doc contract: load_store.rs:7 "Auto-detect LDR/STR size from the first register operand." — asserted fingerprint 09f582bb
- Seed: encode_ldr_str_pbt.rs LDR literal Symbol sweep
- Formal: ∀ rt ∈ [0,30], class ∈ {W,X,S,D,Q}. encode_ldr_str_auto([Reg(class+rt), Symbol("foo")], true) = WordWithReloc { word matching llvm-mc `ldr Rt, #0` with imm19=0, reloc Ldr19 symbol foo addend 0 }. ∀ those, encode_ldr_str_auto(..., false) = Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, class]
  domain:
    rt: 0..30
    class: WXSDQ
  body: sut_literal(ops) == llvm_mc_word(asm_ldr_hash0) and reloc_is_ldr19_foo
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
  class: { gen: int, min: 0, max: 4, type: u32 }
evidence: load_store.rs:176-214 README.md Ldr19 ELF 273
```

## encode_ldr_str_auto_neg_v_reg
- Tier: 4
- Rationale: Sweep of the default-64-bit branch. llvm-mc/gas reject `ldr/str Vn, [Xn]` (bare V is not a scalar Rt). Operand::Reg documents v0-v31; the API accepts the name. README.md:12 gas contract requires rejection, not silent D-form.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: (none) — coverage_gaps file-level plus manual arm audit of the else default
- Formal: ∀ is_load ∈ {false,true}, rt ∈ [0,31], rn ∈ [0,31]. encode_ldr_str_auto([Reg("v"+rt), Mem{Xn|SP, 0}], is_load) = Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: failing
- Counterexample: is_load=false, rt=0, rn=0 → Ok(Word(0xfd000000))
- Bug report: bug_reports/encode_ldr_str_auto_bare_v.md

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, rt, rn]
  domain:
    is_load: bool
    rt: 0..31
    rn: 0..31
  body: encode_ldr_str_auto(ops_v, is_load).is_err()
generators:
  is_load: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc rejects ldr v0, [x1]
```

## encode_ldr_str_auto_diff_fp_alias
- Tier: 5
- Rationale: Sweep of GNU fp=X29 alias. llvm-mc/gas accept `ldr fp, [x1]` as X29. Size-detect default 64-bit would be correct if parse_reg_num mapped fp. README.md:12 gas contract.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: (none) — coverage_gaps file-level plus manual arm audit of aliases vs lr
- Formal: ∀ is_load ∈ {false,true}, rn ∈ [0,30]. encode_ldr_str_auto([Reg("fp"), Mem{Xn, 0}], is_load) = Word(llvm-mc(`ldr|str fp, [Xn]`))
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: failing
- Counterexample: is_load=false, rn=0 → Err("invalid register: fp")
- Bug report: bug_reports/encode_ldr_str_auto_fp_alias.md

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, rn]
  domain:
    is_load: bool
    rn: 0..30
  body: sut_word(ops_fp, is_load) == llvm_mc_word(asm)
generators:
  is_load: { gen: bool }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: README.md:12 llvm-mc ldr fp, [x1] == ldr x29, [x1]
```

## encode_ldr_str_auto_neg_sp_dest
- Tier: 4
- Rationale: ARM Rt=31 in LDR/STR is ZR not SP. llvm-mc/gas reject `ldr sp, [x0]`. load_store.rs:10 names sp as 64-bit for size detect; that does not make SP a valid Rt. README.md:12 gas contract requires Err.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_neg_invalid_regs SP dest
- Formal: ∀ is_load ∈ {false,true}, rn ∈ [0,31]. encode_ldr_str_auto([Reg("sp"), Mem{Xn|SP, 0}], is_load) = Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs
- Status: failing
- Counterexample: is_load=false, rn=0 → Ok(Word(0xfd00001f))
- Bug report: bug_reports/encode_ldr_str_auto_sp_dest.md

```property
function: encoder.load_store.encode_ldr_str_auto
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, rn]
  domain:
    is_load: bool
    rn: 0..31
  body: encode_ldr_str_auto(ops_sp, is_load).is_err()
generators:
  is_load: { gen: bool }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc rejects ldr sp, [x0]
```
