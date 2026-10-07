# Round 40 — constraint-layer + runtime-vein campaign

## Summary
Two NEW veins (avoiding teammate's encode territory): frontend C11 constraint checking and runtime behavior corners. 12 issues filed (#1133-#1144), 47 constraint violations + 1 miscompile witnessed.

## Bugs Found
### Constraint layer (gcc+clang double-rejected, ccc accepted)
- #1133 same-TU inline+non-inline redefinition — Formal: ∀ f defined inline AND non-inline in one TU: diagnostic required (6.7.4p7). Witness: inline body silently wins (f=1).
- #1134 & on bit-field — Formal: ∀ bit-field m: &m is Err (6.5.3.2p1). Witness: (char*)&x.b compiles.
- #1135 modifiable-lvalue ×7 — Formal: ∀ assignment/incdec target: modifiable lvalue (6.5.16p1/6.5.2.4p1). Witness: a=b, (int)x=5, f=g, const, enum, (x+1)++, 5--.
- #1136 sizeof(bit-field) — 6.5.3.4p1.
- #1137 statement-context ×7 — 6.8.x: break/continue/case/default placement, dup case/default, goto undefined. Reproduce: /tmp/neg2.sh.
- #1138 declaration ×6 — 6.2.1/6.7.x: same-scope redef, void var, static VLA, func returning array, dup struct tag, empty {}.
- #1139 expression-type ×6 — 6.5.x: *int, int[0], int(), void+1, struct-mismatch assign, const scalar.
- #1140 declaration/qualifier ×4 — 6.7.9p3 static non-const init SILENTLY ZERO-INITIALIZES (x=f() prints 0); restrict-non-ptr; multi storage; typedef redef.
- #1141 bit-field-decl ×3 — a:-1, a:99, float:3.
- #1142 linkage/label ×5 — static-after-nonstatic, int-then-double, inline conflict, dup label, goto-into-VLA.
- #1143 pp-directive-structure ×6 — #else/#endif w/o #if, #elif after #else, missing #endif, #define no name, #line garbage. Reproduce: /tmp/pp5.sh.

### Runtime miscompile
- #1144 sizeof(VLA) folds to 1024 in ANY arithmetic — Formal: ∀ VLA v of runtime length n: every expr uses runtime n*sizeof(T) (6.5.3.4p2). Witness: s1=28 ✓ bare; s2..s6 = 256/2048/1025/512/512 = 1024-folded. Reproduce: /tmp/vla9.c; proptest-regressions passes/mod.txt.

## Design Caveats (checks that EXIST — verified-correct rejects)
- main signature mismatch, negative/zero array size, enum duplicate value, excess initializers (array/scalar/struct), return-with-value in void fn, goto into plain block, static-after-extern conflict, #error, _Static_assert forms, missing #include — all correctly diagnosed. Doc evidence: /tmp/neg2.sh, /tmp/neg3.sh, /tmp/pp5.sh probe outputs (round-40 run logs).

## Verified-correct runtime surfaces (green, no bug)
- switch jump tables 300×2 levels (p_switch_tables_vs_gcc, resident property)
- escape maximal-munch (\x412/\0123/\777 byte-exact), K&R defs, flexible array members, _Atomic RMW (needs gcc -latomic oracle)
- sizeof side-effect suppression, short-circuit suppression, compound-literal per-iteration re-materialization, __LINE__/__func__
- va_copy independence, long double runtime arith at -O2, deep struct call chains (1KB ×3 hops), comma/chained assignment
- fixed-array sizeof arithmetic, integer promotions (uchar add, short<ushort), offsetof nested+array, _Alignas(32) enforcement, __builtin_types_compatible_p/__builtin_choose_expr

## Test Files Created
| File | Tests |
|------|-------|
| src/passes/mod.rs ::pbt_switch_tables | p_switch_tables_vs_gcc (300×2) |
| probe batteries /tmp/neg2.sh, neg3.sh, pp5.sh, rt*.c | 60+ forms |
