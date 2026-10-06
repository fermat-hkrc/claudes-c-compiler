# Properties: encode_adrp

## encode_adrp_diff_symbol_word_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc on the ARM zero-page ADRP encoding. Reloc-form ADRP (GNU syntax) emits immhi=immlo=0; that word is the same encoding llvm-mc produces for `adrp Xd, #0`. State machine rejected (pure function). Round-trip rejected (no ADRP decoder). Linker reloc::encode_adrp rejected (patches displacement, different job). encode_adr rejected (op=0 / AdrPrelLo21). GNU as rejects `#imm`, so Imm is not in the differential domain; `#0` is used only as an independent encoder of the zero-displacement word.
- Doc contract: load_store.rs:681 "ADRP: 1 immlo[1:0] 10000 immhi[18:0] Rd" — asserted fingerprint 5b59f264
- Seed: load_store.rs:1146 encode_adr_diff_imm_llvm_mc
- Formal: ∀ rd ∈ {0..31}, ∀ suffix ∈ ℕ. let Xd = xreg(rd), word = encode_adrp([Reg(Xd), Symbol("s"+suffix)]).word in llvm_mc("adrp Xd, #0") = word
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adrp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, suffix]
  domain: { rd: u32 0..=31, suffix: u32 }
  relation:
    op: eq
    lhs: encode_adrp([Reg(xreg(rd)), Symbol(sym)]).word
    rhs: llvm_mc("adrp " + xreg(rd) + ", #0")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
evidence: load_store.rs:682 ARM ADRP encoding; README.md:12 GNU-style assembler; llvm-mc -triple=aarch64 -show-encoding
```

## encode_adrp_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from the ARM encoding comment: op=1, bits[28:24]=10000, Rd, reloc-form imm=0. Stronger differential covers the packed word vs llvm-mc; this unpacks fields against the ARM diagram (independent of llvm-mc). Round-trip rejected (no decoder).
- Doc contract: load_store.rs:681 "ADRP: 1 immlo[1:0] 10000 immhi[18:0] Rd" — asserted fingerprint 5b59f264
- Seed: load_store.rs:1163 encode_adr_roundtrip_arm_fields
- Formal: ∀ rd ∈ {0..31}, ∀ kind ∈ {Symbol, Label, SymbolOffset, Modifier-got}. let w = encode_adrp([Reg(xreg(rd)), op1]).word in unpack(w).op=1 ∧ unpack(w).opc=0b10000 ∧ unpack(w).rd=rd ∧ unpack(w).imm21=0
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adrp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, kind]
  domain: { rd: u32 0..=31, kind: {Symbol, Label, SymbolOffset, got} }
  relation:
    op: eq
    lhs: unpack_adrp(encode_adrp([Reg(xreg(rd)), op1]).word)
    rhs: (rd, imm21=0, op=1, opc=0b10000)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u32 }
evidence: load_store.rs:682
```

## encode_adrp_meta_rd_symbol
- Tier: 4
- Rationale: Metamorphic isolation from the ARM field layout: Rd occupies bits[4:0]; opcode/imm occupy the rest. Changing the symbol, addend, or GOT vs page reloc must not change the instruction word (reloc carries that data). Changing Rd must not change bits[31:5].
- Doc contract: load_store.rs:681 "ADRP: 1 immlo[1:0] 10000 immhi[18:0] Rd" — asserted fingerprint 5b59f264
- Seed: load_store.rs:1180 encode_adr_metamorphic_rd_imm_independent
- Formal: ∀ rd, rd2 ∈ {0..31}, ∀ s1, s2 symbols. let w(rd,s) = encode_adrp([Reg(xreg(rd)), Symbol(s)]).word in (w(rd,s1) xor w(rd,s2)) & 0x1f = 0 ∧ (w(rd,s1) xor w(rd2,s1)) & !0x1f = 0 ∧ w(rd, Symbol(s1)) = w(rd, Modifier{got,s1})
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adrp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rd2, s1, s2]
  domain: { rd: u32 0..=31, rd2: u32 0..=31, s1: symbol, s2: symbol }
  body: ((w(rd,s1) xor w(rd,s2)) & 0x1f == 0) && ((w(rd,s1) xor w(rd2,s1)) & !0x1f == 0) && (w(rd,Symbol(s1)) == w(rd,got(s1)))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  suffix1: { gen: int, min: 0, max: 1000, type: u32 }
  suffix2: { gen: int, min: 0, max: 1000, type: u32 }
evidence: load_store.rs:682
```

## encode_adrp_reloc_page21
- Tier: 4
- Rationale: README.md:256 AdrpPage21 = 275 = R_AARCH64_ADR_PREL_PG_HI21 for `adrp` page-relative. Symbol, Label, and SymbolOffset must produce WordWithReloc with that type, the given symbol, and the given addend (0 for Symbol/Label). GNU as / llvm-mc object output agrees (foo+8 → addend 8).
- Doc contract: README.md:256 "| `AdrpPage21` | 275 | `adrp` (page-relative, bits [32:12]) |" — asserted fingerprint 7a07f5f8
- Seed: load_store.rs:1211 encode_adr_symbol_reloc
- Formal: ∀ rd ∈ {0..31}, ∀ suffix, ∀ addend. encode_adrp([Xrd, Symbol(s)]) = WordWithReloc{0x90000000|rd, AdrpPage21, s, 0} ∧ encode_adrp([Xrd, Label(s)]) same ∧ encode_adrp([Xrd, SymbolOffset(s, addend)]) has addend
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adrp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, suffix, addend]
  domain: { rd: u32 0..=31, suffix: u32, addend: i64 }
  relation:
    op: eq
    lhs: encode_adrp([Reg(xreg(rd)), Symbol|Label|SymbolOffset]).reloc
    rhs: Relocation { AdrpPage21, symbol, addend }
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: README.md:256 AdrpPage21 ELF 275; load_store.rs:683-690
```

## encode_adrp_reloc_got
- Tier: 4
- Rationale: README.md:248/263 `adrp x0, :got:variable` → AdrGotPage21 ELF 311. Inline comment load_store.rs:659. GNU as / llvm-mc accept `:got:sym` and `:got:sym+addend` (R_AARCH64_ADR_GOT_PAGE, addend preserved). ModifierOffset is in the documented GNU domain; the generator includes it.
- Doc contract: README.md:263 "| `AdrGotPage21` | 311 | `adrp` via GOT |" — asserted fingerprint 754dd065; load_store.rs:659 "adrp x0, :got:symbol" — asserted fingerprint 8870dbf6
- Seed: load_store.rs:1211 encode_adr_symbol_reloc (GOT analogue)
- Formal: ∀ rd ∈ {0..31}, ∀ suffix, ∀ addend. encode_adrp([Xrd, Modifier{got,s}]) = WordWithReloc{0x90000000|rd, AdrGotPage21, s, 0} ∧ encode_adrp([Xrd, ModifierOffset{got,s,addend}]) = WordWithReloc{…, AdrGotPage21, s, addend}
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: failing
- Counterexample: encode_adrp([Reg("x0"), ModifierOffset{kind:"got", symbol:"g0", offset:0}]) -> Err("adrp needs symbol operand, got Some(ModifierOffset { kind: \"got\", symbol: \"g0\", offset: 0 })")
- Bug report: bug_reports/encode_adrp_got_modifier_offset.md

```property
function: encoder.load_store.encode_adrp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, suffix, addend]
  domain: { rd: u32 0..=31, suffix: u32, addend: i64 }
  relation:
    op: eq
    lhs: encode_adrp([Reg(xreg(rd)), Modifier{got}|ModifierOffset{got}]).reloc
    rhs: Relocation { AdrGotPage21, symbol, addend }
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: README.md:248,263; load_store.rs:659-670; GNU as / llvm-mc :got:sym[+addend]
```

## encode_adrp_neg_w_sp_fp
- Tier: 4e
- Rationale: ARM ADRP Rd is Xd (X31=XZR, not SP). GNU as and llvm-mc reject W registers, SP/WSP, and FP/SIMD destinations. README.md:12 claims gas syntax. No docstring excludes these; they are invalid per ARM/gas so the API must Err, not encode as Xd/XZR.
- Doc contract: (none) — function has no rustdoc restricting Rd; contract inferred from ARM ADRP <Xd> and README.md:12 gas agreement
- Seed: load_store.rs:1278 encode_adr_neg_w_reg / 1296 encode_adr_neg_sp / 1347 encode_adr_neg_fp_reg
- Formal: ∀ dest ∈ W-regs ∪ {sp,wsp} ∪ FP/SIMD, ∀ op1 ∈ valid-symbol-ops. encode_adrp([Reg(dest), op1]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: failing
- Counterexample: encode_adrp([Reg("w0"), Symbol("s0")]) -> Ok(WordWithReloc { word: 0x90000000, AdrpPage21, "s0", 0 })
- Bug report: bug_reports/encode_adrp_w_sp_fp.md

```property
function: encoder.load_store.encode_adrp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, op1]
  domain: { dest: W|SP|FP, op1: Symbol }
  relation:
    op: throws
    lhs: encode_adrp([Reg(dest), op1])
    rhs: Err
generators:
  kind: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ADRP <Xd>; README.md:12 gas; llvm-mc rejects w0/sp/d0
```

## encode_adrp_neg_bad_operands
- Tier: 4e
- Rationale: GNU as / llvm-mc reject missing operands, extra operands, `:lo12:` / `:got_lo12:` (wrong reloc modifier), `#imm` (gas: "bad expression"), and memory operands. README.md:12 gas contract. Body currently returns Err for Imm/Mem/lo12 (good) but ignores extra operands (likely bug). Keep extra in the domain.
- Doc contract: README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: load_store.rs:1323 encode_adr_neg_bad_operands / 1365 encode_adr_neg_modifier
- Formal: ∀ rd ∈ {0..30}, ∀ kind ∈ {empty, missing-op1, extra, lo12, got_lo12, Imm, Mem}. encode_adrp(ops(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: failing
- Counterexample: encode_adrp([Reg("x0"), Symbol("foo"), Reg("x0")]) -> Ok(WordWithReloc { word: 0x90000000, AdrpPage21, "foo", 0 })
- Bug report: bug_reports/encode_adrp_extra_operand.md

```property
function: encoder.load_store.encode_adrp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, rd]
  domain: { kind: {empty, missing, extra, lo12, got_lo12, Imm, Mem}, rd: u32 0..=30 }
  relation:
    op: throws
    lhs: encode_adrp(ops)
    rhs: Err
generators:
  kind: { gen: int, min: 0, max: 6, type: u32 }
  rd: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: README.md:12 gas; llvm-mc :lo12: "page or gotpage label reference expected"; gas `#imm` bad expression
```

## encode_adrp_symbol_misclassified
- Tier: 4
- Rationale: The function's own comment asserts that parser-misclassified Reg/Cond/Barrier names (s1, v0, d1, cc, lt, le, st, ld) are symbols for ADRP. Property: those operand kinds produce AdrpPage21 with that name as the symbol.
- Doc contract: load_store.rs:672 "Parser misclassifies symbol names that collide with register names (s1, v0, d1, etc.)," — asserted fingerprint 07ac48b7
- Seed: load_store.rs:1388 encode_adr_symbol_misclassified
- Formal: ∀ rd ∈ {0..31}, ∀ which ∈ {Reg, Cond, Barrier}, ∀ name ∈ {s1,v0,d1,cc,lt,le,st,ld}. encode_adrp([Xrd, which(name)]) = WordWithReloc{0x90000000|rd, AdrpPage21, name, 0}
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adrp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, which, name]
  domain: { rd: u32 0..=31, which: {Reg,Cond,Barrier}, name: {s1,v0,d1,cc,lt,le,st,ld} }
  relation:
    op: eq
    lhs: encode_adrp([Reg(xreg(rd)), misclassified]).reloc
    rhs: Relocation { AdrpPage21, name, 0 }
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: string }
evidence: load_store.rs:672-676
```

## encode_adrp_diff_alt_spellings
- Tier: 2
- Rationale: Coverage sweep for parse_reg_num aliases documented in encoder/mod.rs:283 (lr=30, ASCII case-fold, x0-x31). Word equals ADRP_BASE|rd with AdrpPage21.
- Doc contract: encoder/mod.rs:283 "Parse a register name to its 5-bit encoding number (0-30, 31 for sp/zr)." — asserted
- Seed: encode_ldr_str_diff_alt_spellings
- Formal: ∀ alias ∈ {lr, Xn, x31}. encode_adrp([Reg(alias), Symbol("foo")]).word = 0x90000000 | rd(alias) ∧ reloc = AdrpPage21
- Test file: src/backend/arm/assembler/encoder/encode_adrp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_adrp
oracle: differential
predicate:
  quantifier: forall
  vars: [which, n]
  domain: { which: {lr, uppercase, x31}, n: u32 0..=30 }
  relation:
    op: eq
    lhs: encode_adrp([Reg(alias), Symbol("foo")]).word
    rhs: 0x90000000 | rd
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
evidence: encoder/mod.rs:283 parse_reg_num
```
