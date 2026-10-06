# PROPERTIES.md — round 16 (ARM assembler deep, non-NEON)

## p1_arm_asm_deep_valid_forms
- Status: passing — 22 families × 400 forms byte-exact vs llvm-mc (double-reference cross-check on every discrepancy). Four domain corrections en route, EACH of which exposed a real validation gap now filed: ldp imm7×8 range (→#792), W base register (→#791), movk W hw (→#793), SP in logical slots (→#795).
- Test file: src/backend/arm/mod.rs

## p2_arm_asm_deep_invalid_rejected
- Status: failing — shrunk counterexample `movk w19, #325050, lsl #0` (imm16 range, #792); width-mix and addressing classes carried by p2b and hand probes.
- Test file: src/backend/arm/mod.rs

## p2b_arm_width_mix_rejected
- Status: failing — every width-mixed direction silently accepted; shrunk `add x0, x0, w0`-class → #791.
- Test file: src/backend/arm/mod.rs
