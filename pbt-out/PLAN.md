# PBT Campaign: encode_shift

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler module docstring `src/backend/x86/assembler/mod.rs:1-5`: native x86-64 assembler that translates AT&T-syntax assembly into ELF `.o`; replaces external `gcc -c`.
  - Assembler README `src/backend/x86/assembler/README.md:5-14`: translates AT&T-syntax assembly the compiler emits (plus musl/kernel coverage); not a full GAS replacement, but covers a broad swath of the x86-64 ISA.
  - README instruction table `src/backend/x86/assembler/README.md:567`: category 3 Shifts/Rotates lists `shl, shr, sar, rol, ror, rcl, rcr (b/w/l/q), shld, shrd (l/q)`.
  - Encoder module docstring `src/backend/x86/assembler/encoder/mod.rs:1-4`: encodes parsed x86-64 instructions into machine code bytes (REX, ModR/M, SIB, displacement).
  - Dispatch `encoder/mod.rs:196-200`: `"shlq"|"shll"|"shlw"|"shlb" => encode_shift(..., 4)`, shr /5, sar /7, rol /0, ror /1. `encoder/mod.rs:670-671`: rcl /2, rcr /3. Suffix-less `shl`/`sal` go through `encode_suffixless_shift` then `encode_shift`.
  - Body at `gp_integer.rs:756-833`: 1-operand form is implicit shift-by-1 (D0/D1); 2-operand imm uses D0/D1 when count==1 else C0/C1+imm8; CL form uses D2/D3; 66 prefix when size==2; REX via `emit_rex_unary`/`emit_rex_rm`.
  - Intel SDM Group 2: `/0` ROL, `/1` ROR, `/2` RCL, `/3` RCR, `/4` SHL/SAL, `/5` SHR, `/7` SAR. D0/D2/C0 = r/m8; D1/D3/C1 = r/m16/32/64. REX.W for 64-bit. GAS: omitted count is 1; AT&T operand order is count, dest. SAL is an alias of SHL.
  - Codegen caller `src/backend/x86/codegen/emit.rs:151-153`: `IrBinOp::Shl => ("shll","shlq")`, AShr sar, LShr shr.
  - Siblings encode_double_shift (SHLD/SHRD), encode_sse_shift, encode_avx_shift, encode_bmi2_shift (SHLX/SHRX/SARX) are different jobs.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `gp_integer.rs` has no existing test module; neighbouring x86 parser tests live in `parser.rs`. New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1499 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_shift_pbt` at the bottom of `src/backend/x86/assembler/encoder/gp_integer.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_shift (gp_integer.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in gp_integer.rs are indexed but not tested.
- **Oracle (encode_shift):** Differential (llvm-mc -triple=x86_64 -show-encoding) for valid AT&T GP shift/rotate forms (imm8 0..255 or %cl or omitted-count, dest matching mnemonic size b/w/l/q). State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_double_shift / encode_sse_shift / encode_avx_shift / encode_bmi2_shift fail the same-job gate. SAL vs SHL is the same job (/4) and is used as a metamorphic companion. SUT-boundary: internal-helper of the GNU-style x86-64 assembler; public contract is encoding GNU-style AT&T SHL/SHR/SAR/ROL/ROR/RCL/RCR text. Mapping: `encode_shift(ops, mnemonic, shift_op)` <-> AT&T `mnemonic src, dst` (or 1-operand dest).
- **Seeds:** README.md:567 Shifts/Rotates table; codegen/emit.rs:151-153 emits shll/shlq/sarl/sarq/shrl/shrq. No existing unit test of encode_shift. Seed: emit.rs:151.
- **State machine:** not applicable — encode_shift is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending bytes.

## Module: encode_shift
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of 1-op Reg/Mem/other, arity, Imm+Reg, Imm+Mem, CL, RIP reloc, mixed size, non-GP, segment, imm8 overflow — added one-operand non-r/m and RIP addend properties)
