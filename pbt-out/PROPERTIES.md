# Properties: encode_lui

## encode_lui_diff_imm_llvm_mc
- Tier: 4
- Rationale: Strongest applicable oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree LUI decoder). encode_auipc / encode_c_lui / encode_u rejected (different opcode / compressed / shared packer).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: (none)
- Formal: ∀ rd ∈ {x0..x31 ∪ ABI names}, ∀ imm ∈ [0, 1048575]. encode_lui([Reg(rd), Imm(imm)]) = Ok(Word(w)) ∧ w = llvm-mc("lui rd, imm")
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_name, imm: int(0..1048575) }
  relation:
    op: eq
    lhs: encode_lui([Reg(rd), Imm(imm)])
    rhs: llvm_mc_word("lui {rd}, {imm}")
generators:
  rd: { gen: string }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
evidence: encoder/mod.rs:3 README.md:304 llvm-mc -triple=riscv64
```

## encode_lui_isa_u_type
- Tier: 3
- Rationale: Algebraic invariant from README.md:356 / RISC-V U-type layout, independent of encode_u. Stronger differential is the sibling property; this pins opcode/rd/imm20 even if llvm-mc is unavailable.
- Doc contract: README.md:356 "U-type:  [          imm[31:12]           |  rd  | opcode]" — asserted fingerprint 8c9098fd
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ imm ∈ [0, 1048575]. let w = encode_lui([Reg(x{rd}), Imm(imm)]). w[6:0]=0b0110111 ∧ w[11:7]=rd ∧ w[31:12]=imm
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: u32(0..31), imm: int(0..1048575) }
  body: unpack_u(encode_lui([Reg(x{rd}), Imm(imm)])) == (0b0110111, rd, imm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
evidence: README.md:356 encoder/mod.rs:305
```

## encode_lui_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names, xN, zero/ra/sp/fp aliases encode the same rd field. Independent of llvm-mc. Stronger differential covers xN vs llvm-mc; this checks SUT alias table.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: encode_neg_pbt encode_neg_abi_xn_alias
- Formal: ∀ n ∈ 0..31, ∀ imm ∈ [0, 1048575]. encode_lui([Reg(x{n}), Imm(imm)]) = encode_lui([Reg(ABI[n]), Imm(imm)]) ∧ (n=8 ⇒ encode_lui([Reg("fp"), Imm(imm)]) equals both)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, imm]
  domain: { n: u32(0..31), imm: int(0..1048575) }
  relation:
    op: eq
    lhs: encode_lui([Reg(x{n}), Imm(imm)])
    rhs: encode_lui([Reg(ABI[n]), Imm(imm)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
evidence: encoder/mod.rs:146-191 parser.rs:22
```

## encode_lui_reloc_hi
- Tier: 4
- Rationale: Differential + reloc contract. `%hi(symbol)` is the documented Symbol arm (base.rs:12). llvm-mc emits R_RISCV_HI20; reloc-form word equals `lui rd, 0`.
- Doc contract: base.rs:12 "%hi(symbol)" — asserted fingerprint df4eaaa4
- Seed: (none)
- Formal: ∀ rd ∈ GPR names, ∀ sym ∈ identifier. encode_lui([Reg(rd), Symbol("%hi("+sym+")")]) = Ok(WordWithReloc{word, Hi20, symbol=sym, addend=0}) ∧ word = llvm-mc("lui rd, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, sym]
  domain: { rd: gpr_name, sym: identifier }
  body: reloc(encode_lui([Reg(rd), Symbol("%hi("+sym+")")])) == (llvm_mc_word("lui {rd}, 0"), Hi20, sym, 0)
generators:
  rd: { gen: string }
  sym: { gen: string }
evidence: base.rs:12 README.md:213 RelocType::Hi20 -> R_RISCV_HI20
```

## encode_lui_reloc_tprel_hi
- Tier: 4
- Rationale: Same as reloc_hi for `%tprel_hi`. Codegen globals.rs:32 emits `lui t0, %tprel_hi(name)`. llvm-mc fixup_riscv_tprel_hi20 / R_RISCV_TPREL_HI20.
- Doc contract: README.md:25 "Relocation modifier parsing (%pcrel_hi, %pcrel_lo, %hi, %lo, %tprel_*, %got_pcrel_hi, ...)" — asserted fingerprint a74b822a
- Seed: (none)
- Formal: ∀ rd ∈ GPR names, ∀ sym ∈ identifier. encode_lui([Reg(rd), Symbol("%tprel_hi("+sym+")")]) = Ok(WordWithReloc{word, TprelHi20, symbol=sym, addend=0}) ∧ word = llvm-mc("lui rd, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, sym]
  domain: { rd: gpr_name, sym: identifier }
  body: reloc(encode_lui([Reg(rd), Symbol("%tprel_hi("+sym+")")])) == (llvm_mc_word("lui {rd}, 0"), TprelHi20, sym, 0)
generators:
  rd: { gen: string }
  sym: { gen: string }
evidence: base.rs:16-18 globals.rs:32 README.md:216
```

## encode_lui_neg_imm_oob
- Tier: 3
- Rationale: Negative/error contract. llvm-mc rejects immediates outside [0, 1048575] (signed negatives, 1048576, i64 extremes). RISC-V U-immediate is 20 bits. SUT currently shifts/truncates; property asserts Err.
- Doc contract: (none) on encode_lui for the range — contract inferred from llvm-mc LUI operand diagnostic and README U-type 20-bit imm[31:12]
- Seed: (none)
- Formal: ∀ rd ∈ GPR names, ∀ imm ∈ i64 \ [0, 1048575]. encode_lui([Reg(rd), Imm(imm)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: failing
- Counterexample: encode_lui([Reg("x0"), Imm(-1)]) = Ok(Word(0xFFFFF037))
- Bug report: pbt-out/bug_reports/encode_lui_imm_oob.md

```property
function: encoder.encode_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_name, imm: i64_outside_0_1048575 }
  relation:
    op: throws
    expr: encode_lui([Reg(rd), Imm(imm)])
generators:
  rd: { gen: string }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc "integer in the range [0, 1048575]" README.md:356
```

## encode_lui_neg_extra
- Tier: 3
- Rationale: Negative/error. llvm-mc rejects a third operand. README / ISA LUI is two-operand (`lui rd, imm`). No operands.len() check in the SUT.
- Doc contract: README.md:304 "- **U-type**: lui, auipc." — asserted fingerprint 3c000870
- Seed: encode_neg_pbt encode_neg_neg_extra
- Formal: ∀ rd, ∀ imm ∈ [0, 1048575], ∀ extra. encode_lui([Reg(rd), Imm(imm), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: failing
- Counterexample: encode_lui([Reg("x0"), Imm(0), Imm(0)]) = Ok(Word(0x37))
- Bug report: pbt-out/bug_reports/encode_lui_extra_operand.md

```property
function: encoder.encode_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, extra]
  domain: { rd: gpr_name, imm: int(0..1048575), extra: Operand }
  relation:
    op: throws
    expr: encode_lui([Reg(rd), Imm(imm), extra])
generators:
  rd: { gen: string }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" README two-operand U-type
```

## encode_lui_neg_bad_modifier
- Tier: 3
- Rationale: Negative/error. llvm-mc accepts only `%hi` / `%tprel_hi` (or integer). Plain symbols, `%pcrel_hi`, `%lo`, `%got_pcrel_hi` are rejected. Comment base.rs:12 names `%hi(symbol)` as the intended form, not an exclusion of others; llvm-mc is the independent rejection contract.
- Doc contract: base.rs:12 "%hi(symbol)" — asserted fingerprint df4eaaa4
- Seed: (none)
- Formal: ∀ rd, ∀ s ∈ {bare ident, %pcrel_hi(ident), %lo(ident), %got_pcrel_hi(ident), %pcrel_lo(ident)}. encode_lui([Reg(rd), Symbol(s)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: failing
- Counterexample: encode_lui([Reg("x0"), Symbol("foo")]) = Ok(WordWithReloc{word: 0x37, Hi20, symbol: "foo", addend: 0})
- Bug report: pbt-out/bug_reports/encode_lui_bad_modifier.md

```property
function: encoder.encode_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, s]
  domain: { rd: gpr_name, s: bad_lui_symbol }
  relation:
    op: throws
    expr: encode_lui([Reg(rd), Symbol(s)])
generators:
  rd: { gen: string }
  s: { gen: string }
expected_error: String
evidence: llvm-mc "symbol with %hi/%tprel_hi modifier" base.rs:12
```

## encode_lui_neg_arity
- Tier: 3
- Rationale: Sweep — `_` / get_reg error paths for arity < 2. llvm-mc "too few operands". Documented two-operand U-type.
- Doc contract: base.rs:26 "lui: invalid operands" — asserted fingerprint dff3cf3a
- Seed: encode_neg_pbt encode_neg_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_lui(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: vec_len_lt_2 }
  relation:
    op: throws
    expr: encode_lui(ops)
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: llvm-mc "too few operands" base.rs:26 README.md:304
```

## encode_lui_neg_fp
- Tier: 3
- Rationale: Sweep — get_reg integer-register path. llvm-mc rejects FP dest for LUI.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7" — asserted fingerprint 00db3ff1
- Seed: (none)
- Formal: ∀ fp ∈ FP names, ∀ imm ∈ [0, 1048575]. encode_lui([Reg(fp), Imm(imm)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp, imm]
  domain: { fp: fp_name, imm: int(0..1048575) }
  relation:
    op: throws
    expr: encode_lui([Reg(fp), Imm(imm)])
generators:
  fp: { gen: string }
  imm: { gen: int, min: 0, max: 1048575, type: i64 }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" get_reg integer-only
```

## encode_lui_neg_bad_operand
- Tier: 3
- Rationale: Sweep — `_` arm of the operand-1 match (Label/Mem/Csr/Fence/SymbolOffset/MemSymbol).
- Doc contract: base.rs:26 "lui: invalid operands" — asserted fingerprint dff3cf3a
- Seed: (none)
- Formal: ∀ rd, ∀ bad ∈ Operand \ {Imm, Symbol, Reg}. encode_lui([Reg(rd), bad]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_lui
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, bad]
  domain: { rd: gpr_name, bad: non_imm_non_symbol }
  relation:
    op: throws
    expr: encode_lui([Reg(rd), bad])
generators:
  rd: { gen: string }
  bad: { gen: string }
expected_error: String
evidence: base.rs:26 `_ => Err("lui: invalid operands")`
```

## encode_lui_hi_addend
- Tier: 4
- Rationale: Sweep — `%hi(symbol+addend)` is accepted by llvm-mc as R_RISCV_HI20 against `symbol` with addend. extract_modifier_symbol keeps `foo+4` as the symbol name and hardcodes addend=0.
- Doc contract: base.rs:12 "%hi(symbol)" — asserted fingerprint df4eaaa4
- Seed: (none)
- Formal: ∀ rd, ∀ sym, ∀ a ≠ 0. encode_lui([Reg(rd), Symbol("%hi("+sym+sign(a)+")")]) = Ok(WordWithReloc{Hi20, symbol=sym, addend=a})
- Test file: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs
- Status: failing
- Counterexample: encode_lui([Reg("x0"), Symbol("%hi(foo+1)")]) reloc.symbol="foo+1" addend=0 (expected symbol="foo" addend=1)
- Bug report: pbt-out/bug_reports/encode_lui_hi_addend.md

```property
function: encoder.encode_lui
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, sym, addend]
  domain: { rd: gpr_name, sym: identifier, addend: i64_nonzero }
  relation:
    op: holds
    expr: reloc(encode_lui([Reg(rd), Symbol("%hi(sym+addend)")])).symbol == sym && reloc.addend == addend
generators:
  rd: { gen: string }
  sym: { gen: string }
  addend: { gen: int, type: i64 }
evidence: llvm-mc R_RISCV_HI20 foo 0x4 for lui x1, %hi(foo+4); README.md:25
```
