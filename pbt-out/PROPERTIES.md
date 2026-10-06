# Properties: encode_stop

## encode_stop_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree STADD decoder; ST* is a one-way LD* alias with Rt=ZR). Sibling encode_cas / encode_swp / encode_ldop rejected (same-job gate: different LSE class / operand grammar). SUT-boundary: internal-helper, operands passed through from encode_instruction.
- Doc contract: load_store.rs:923 "/// Encode STADD/STCLR/STEOR/STSET and their release/byte/halfword variants." — asserted fingerprint 02b72f5c
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:379 encode_ldop_diff_llvm_mc
- Formal: ∀ op ∈ {stadd,stclr,steor,stset}, suf ∈ {ε,l,b,lb,h,lh}, Rs ∈ {0..31}, Rn ∈ {0..31}, wide ∈ {W,X if suf ∈ {ε,l} else W}. encode_stop(op+suf, [Reg(Rs), Mem{Xn|SP, 0}]) = llvm-mc("-triple=aarch64 -mattr=+lse", "op+suf Rs, [Xn|SP]")
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: differential
predicate:
  quantifier: forall
  vars: [op, suf, rs, rn, is_64]
  domain:
    op: "0..3"
    suf: "0..5"
    rs: "0..31"
    rn: "0..31"
    is_64: bool
  relation:
    op: eq
    lhs: encode_stop(mnemonic(op, suf), valid_ops(suf, rs, rn, is_64)).word
    rhs: llvm_mc_word(asm_line(op, suf, rs, rn, is_64))
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 5, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: encoder/mod.rs:3 32-bit words; encoder/mod.rs:1065-1068 dispatch; ARM ARM STADD alias of LDADD Rt=ZR
```

## encode_stop_arm_fields
- Tier: 4
- Rationale: ARM LDADD layout is an independent structural invariant (A=0, Rt=31 for the store alias). Weaker than differential (does not pin the exact word against llvm-mc) but checks every field. Round-trip rejected (no decoder).
- Doc contract: load_store.rs:924 "/// These are aliases for LDADD/LDCLR/LDEOR/LDSET with Rt=XZR (register 31)." — asserted fingerprint 5fcf94c4
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:398 encode_ldop_arm_fields
- Formal: ∀ valid (op,suf,Rs,Rn,wide). let w = encode_stop(...).word. bits[31:30]=size(suf,wide) ∧ bits[29:24]=0b111000 ∧ bit23=0 ∧ bit22=R(suf) ∧ bit21=1 ∧ bits[20:16]=Rs ∧ bit15=0 ∧ bits[14:12]=opc(op) ∧ bits[11:10]=0 ∧ bits[9:5]=Rn ∧ bits[4:0]=31
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, suf, rs, rn, is_64]
  domain:
    op: "0..3"
    suf: "0..5"
    rs: "0..31"
    rn: "0..31"
    is_64: bool
  relation:
    op: holds
    expr: arm_stadd_fields_hold(encode_stop(mnemonic(op, suf), valid_ops(suf, rs, rn, is_64)).word, op, suf, rs, rn, is_64)
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 5, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: load_store.rs:924 Rt=XZR; ARM ARM LDADD size 111000 A R 1 Rs 0 opc 00 Rn Rt with A=0 Rt=31
```

## encode_stop_metamorphic_regs_r_opc
- Tier: 4
- Rationale: Independent field isolation: mutating Rs/Rn/R/opc must change only that field; STADDL XOR STADD = 1<<22; STCLR/STEOR/STSET XOR STADD = opc<<12; uppercase matches lowercase. Stronger oracles already used above; this is the required metamorphic angle.
- Doc contract: load_store.rs:925 "/// STADD Ws, [Xn] encodes as LDADD Ws, WZR, [Xn]" — asserted fingerprint 0e17ede8
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:426 encode_ldop_metamorphic_regs_ar_opc
- Formal: ∀ Rs,Rn ∈ {0..30}, wide ∈ {W,X}. let b = encode_stop("stadd", [Rs,[Rn]]). (encode_stop("stadd",[Rs+1,[Rn]]) ⊕ b) ⊆ bits[20:16] ∧ (encode_stop("stadd",[Rs,[Rn+1]]) ⊕ b) ⊆ bits[9:5] ∧ encode_stop("staddl",...) ⊕ b = 1<<22 ∧ encode_stop("stclr",...) ⊕ b = 1<<12 ∧ encode_stop("steor",...) ⊕ b = 2<<12 ∧ encode_stop("stset",...) ⊕ b = 3<<12 ∧ encode_stop("STADD",...) = b
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, rn, is_64]
  domain:
    rs: "0..30"
    rn: "0..30"
    is_64: bool
  relation:
    op: holds
    expr: stadd_field_isolation(rs, rn, is_64)
generators:
  rs: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM LDADD field layout; load_store.rs:925 alias identity; mnemonic.to_lowercase at load_store.rs:937
```

## encode_stop_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc/gas reject a 3rd STADD operand. Body uses `operands.len() < 2`, so extra operands are currently accepted. Not a documented input-domain restriction on encode_stop itself.
- Doc contract: load_store.rs:926 "/// Variants: stadd/stclr/steor/stset, plus 'l' (release), 'b' (byte), 'h' (half)." — asserted fingerprint 2f5d3000
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:493 encode_ldop_neg_extra_operand
- Formal: ∀ valid (op,suf,Rs,Rn,wide), extra ∈ Operand. encode_stop(op+suf, [Reg(Rs), Mem{base,0}, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: failing
- Counterexample: encode_stop("stadd", [Reg("w0"), Mem{base:"x0", offset:0}, Reg("x2")])
- Bug report: pbt-out/bug_reports/encode_stop_extra_operand.md

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, suf, rs, rn, is_64, extra]
  domain:
    op: "0..3"
    suf: "0..5"
    rs: "0..31"
    rn: "0..31"
    is_64: bool
  relation:
    op: throws
    expr: encode_stop(mnemonic(op, suf), valid_ops(suf, rs, rn, is_64) ++ [extra])
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 5, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, options: [{ gen: const, value: "Reg" }, { gen: const, value: "Imm" }] }
expected_error: String
evidence: llvm-mc rejects extra STADD operand; encoder/mod.rs:3 32-bit words
```

## encode_stop_neg_sp_zr_base
- Tier: 3
- Rationale: ARM Rs is ZR not SP; Rn is Xn|SP not ZR. llvm-mc/gas reject SP/WSP as Rs and W/WSP/XZR/x31 as base. parse_reg_num maps both SP and XZR to 31, so the SUT currently cannot distinguish them.
- Doc contract: load_store.rs:925 "/// STADD Ws, [Xn] encodes as LDADD Ws, WZR, [Xn]" — asserted fingerprint 0e17ede8
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:515 encode_ldop_neg_sp_zr_base
- Formal: ∀ valid (op,suf), kind ∈ {SP-as-Rs, WSP-as-Rs, W-base, WSP-base, XZR-base, x31-base, WZR-base}. encode_stop(op+suf, ops(kind)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: failing
- Counterexample: encode_stop("stadd", [Reg("sp"), Mem{base:"x1", offset:0}])
- Bug report: pbt-out/bug_reports/encode_stop_sp_as_rs.md

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, suf, n, kind, is_64]
  domain:
    op: "0..3"
    suf: "0..5"
    n: "0..30"
    kind: "0..6"
    is_64: bool
  relation:
    op: throws
    expr: encode_stop(mnemonic(op, suf), sp_zr_base_ops(kind, n, suf, is_64))
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 5, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 6, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: llvm-mc rejects STADD SP and STADD with XZR base; ARM Rn=31 is SP not XZR
```

## encode_stop_neg_fp_xbyte
- Tier: 3
- Rationale: llvm-mc/gas require integer GPR Rs; STADDB/STADDH require W registers. get_reg accepts FP prefixes; size is taken from the 'b'/'h' suffix so X registers currently encode.
- Doc contract: load_store.rs:926 "/// Variants: stadd/stclr/steor/stset, plus 'l' (release), 'b' (byte), 'h' (half)." — asserted fingerprint 2f5d3000
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:592 encode_ldop_neg_mixed_fp_xbyte
- Formal: ∀ n ∈ {0..30}, kind ∈ {FP-Rs, STADDB-X, STADDH-X, STADDLB-X, STADDLH-X}. encode_stop(mnem(kind), ops(kind,n)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: failing
- Counterexample: encode_stop("stadd", [Reg("b0"), Mem{base:"x1", offset:0}])
- Bug report: pbt-out/bug_reports/encode_stop_fp_reg.md

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, kind, fp]
  domain:
    n: "0..30"
    kind: "0..5"
    fp: "b,h,s,d,q,v"
  relation:
    op: throws
    expr: encode_stop(mnem_fp_xbyte(kind), fp_or_xbyte_ops(kind, n, fp))
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
  fp: { gen: oneof, options: [{ gen: const, value: "b" }, { gen: const, value: "s" }, { gen: const, value: "d" }] }
expected_error: String
evidence: llvm-mc rejects STADD with FP Rs and STADDB with X registers
```

## encode_stop_neg_arity_and_shape
- Tier: 3
- Rationale: Documented form is 2 operands, second a bare [Xn]. Body returns Err for len < 2 or non-Mem second operand; this pins that documented error path.
- Doc contract: load_store.rs:925 "/// STADD Ws, [Xn] encodes as LDADD Ws, WZR, [Xn]" — asserted fingerprint 0e17ede8
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:666 encode_ldop_neg_arity_and_shape
- Formal: ∀ shape ∈ {[], [Rs], [Rs, Imm], [Rs, Symbol], [Rs, MemPreIndex], [Rs, MemPostIndex], [Rs, MemRegOffset], [Rs, Cond]}. encode_stop("stadd", ops(shape)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape, n, off]
  domain:
    shape: "0..7"
    n: "0..30"
    off: "-8..8"
  relation:
    op: throws
    expr: encode_stop("stadd", arity_shape_ops(shape, n, off))
generators:
  shape: { gen: int, min: 0, max: 7, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
  off: { gen: int, min: -8, max: 8, type: i64 }
expected_error: String
evidence: load_store.rs:928-935 len() < 2 and requires memory operand [Xn]
```

## encode_stop_neg_nonzero_offset
- Tier: 3
- Rationale: ARM/gas optional offset on LSE atomics can only be #0. Body matches `Mem { base, .. }` and ignores offset, so nonzero offsets currently encode.
- Doc contract: load_store.rs:925 "/// STADD Ws, [Xn] encodes as LDADD Ws, WZR, [Xn]" — asserted fingerprint 0e17ede8
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:721 encode_ldop_neg_nonzero_offset
- Formal: ∀ valid (op,suf,Rs,Rn,wide), off ∈ Z excluding 0. encode_stop(op+suf, [Reg(Rs), Mem{base, off}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: failing
- Counterexample: encode_stop("stadd", [Reg("w0"), Mem{base:"x0", offset:-1}])
- Bug report: pbt-out/bug_reports/encode_stop_nonzero_offset.md

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, suf, rs, rn, is_64, off]
  domain:
    op: "0..3"
    suf: "0..5"
    rs: "0..31"
    rn: "0..30"
    is_64: bool
    off: nonzero_i64
  relation:
    op: throws
    expr: encode_stop(mnemonic(op, suf), ops_with_offset(suf, rs, rn, is_64, off))
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 5, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  off: { gen: int, min: -4096, max: 4096, type: i64 }
expected_error: String
evidence: llvm-mc rejects STADD with nonzero Mem offset
```

## encode_stop_neg_invalid_name
- Tier: 3
- Rationale: Sweep of unparsable register/base names. parse_reg_num returns None for foo/x32/empty/r0; encode_stop must Err. Documented error path of get_reg/parse_reg_num.
- Doc contract: load_store.rs:923 "/// Encode STADD/STCLR/STEOR/STSET and their release/byte/halfword variants." — asserted fingerprint 02b72f5c
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:746 encode_ldop_neg_invalid_name
- Formal: ∀ slot ∈ {Rs, base}, name ∈ {foo, x32, w32, ε, r0, x-1, 31}. encode_stop("stadd", ops_with_name(slot, name)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, name]
  domain:
    slot: "0..1"
    name: "foo,x32,w32,empty,r0,x-1,31"
  relation:
    op: throws
    expr: encode_stop("stadd", ops_with_invalid_name(slot, name))
generators:
  slot: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, options: [{ gen: const, value: "foo" }, { gen: const, value: "x32" }] }
expected_error: String
evidence: load_store.rs:931 get_reg; load_store.rs:933 parse_reg_num None
```

## encode_stop_diff_alt_spellings
- Tier: 5
- Rationale: Sweep of ASCII case. mnemonic.to_lowercase is documented behavior; llvm-mc accepts STADD/Stadd. Differential vs llvm-mc on cased mnemonics.
- Doc contract: load_store.rs:936 "    let mn = mnemonic.to_lowercase();" — asserted fingerprint 54365599
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:781 encode_ldop_diff_alt_spellings
- Formal: ∀ valid (op,suf,Rs,Rn,wide), mode ∈ {upper, title, lower}. encode_stop(case(op+suf, mode), ops) = llvm-mc(case(op+suf, mode) + " Rs, [Xn]")
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: differential
predicate:
  quantifier: forall
  vars: [op, suf, rs, rn, is_64, mode]
  domain:
    op: "0..3"
    suf: "0..5"
    rs: "0..31"
    rn: "0..31"
    is_64: bool
    mode: "0..2"
  relation:
    op: eq
    lhs: encode_stop(cased(mnemonic(op, suf), mode), valid_ops(suf, rs, rn, is_64)).word
    rhs: llvm_mc_word(cased_asm(op, suf, rs, rn, is_64, mode))
generators:
  op: { gen: int, min: 0, max: 3, type: u32 }
  suf: { gen: int, min: 0, max: 5, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  mode: { gen: int, min: 0, max: 2, type: u32 }
evidence: load_store.rs:937 mnemonic.to_lowercase
```

## encode_stop_neg_unknown_op
- Tier: 3
- Rationale: Sweep of mnemonics that do not start with stadd/stclr/steor/stset. Body returns "unknown st atomic op".
- Doc contract: load_store.rs:947 "        return Err(format!(\"unknown st atomic op: {}\", mnemonic));" — asserted fingerprint f2617f8e
- Seed: src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:817 encode_ldop_neg_unknown_op
- Formal: ∀ name ∈ {stfoo, swp, cas, st, add, ε, ldadd}. encode_stop(name, [Reg("x0"), Mem{x1,0}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_stop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_stop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain:
    name: "stfoo,swp,cas,st,add,empty,ldadd"
  relation:
    op: throws
    expr: encode_stop(name, valid_ops(0, 0, 1, true))
generators:
  name: { gen: oneof, options: [{ gen: const, value: "stfoo" }, { gen: const, value: "ldadd" }] }
expected_error: String
evidence: load_store.rs:947-948 unknown st atomic op
```
