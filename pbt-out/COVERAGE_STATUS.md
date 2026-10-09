# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_mov_imm_mem)
> Scope: HARD — `encode_mov_imm_mem` only (gp_integer.rs:238)
> Coverage evidence: file-level (symbol presence) + cargo execution evidence — `coverage_gaps` reported no .gcda/.profraw and NOT LINKED against unrelated OH binaries; ignored. Real evidence: `cargo test --lib encode_mov_imm_mem` linked and ran production `InstructionEncoder::encode` → `encode_mov_imm_mem`.

## Summary

| Metric | Value |
|--------|-------|
| Target function | encode_mov_imm_mem |
| Source file | gp_integer.rs |
| Properties | 9 (7 passing, 2 failing) |
| KAT / regression | 5 KAT pass, 2 KAT fail (segment), 2 regression fail |
| Bugs filed | 2 |
| Sweep | 1/1 STANDARD round complete |

## Function

| Function | Tested | Result |
|----------|--------|--------|
| encode_mov_imm_mem | yes | 7 pass / 2 fail properties; 2 bugs |

## Documented behaviors vs properties

| Behavior | Property | Status |
|----------|----------|--------|
| base+disp integer imm C6/C7 | diff_llvm_mc_base_disp | passing |
| SIB forms | diff_llvm_mc_sib | passing |
| all six segment overrides | diff_llvm_mc_segment | failing (bug) |
| ESP/EBP/abs edges | diff_edges_esp_ebp_abs | passing |
| opcode/modrm/imm trail invariant | invariant_opcode_modrm_imm | passing |
| same-mem metamorphic imm trail | meta_same_mem_imm_trail | passing |
| movl $sym reloc R_386_32 | diff_symbol_imm32 | passing |
| movb/movw $sym | diff_symbol_narrow | failing (bug) |
| SymbolMod/SymbolDiff reject | neg_symbol_mod_diff | passing |
