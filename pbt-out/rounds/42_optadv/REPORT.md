# Round 42 — adversarial optimizer + GNU-extension semantics dive

## Bugs Found
### #1150 block-scope static function definition accepted (gcc/clang reject)
- Formal: ∀ block-scope fn def: no static storage class. Witness: compiles clean vs both references' errors.
### #1151 nested function support inverted vs gcc
- Formal: nested fn extension — plain forms (void(void)/params/mid-block) rejected by ccc while gcc accepts with static links (computes 110); static form accepted while both references reject. Reproduce: /tmp/nest*.c.
### #1152 asm symbol-rename labels break the link
- Formal: ∀ asm-label rename: definition AND calls use the new name. Witness: "undefined symbols: my_renamed_fn" vs gcc printing 15.

## Verified-correct (no bug)
- adversarial optimizer: branchless abs/min/max at INT_MIN, guarded overflow checks, rotates r∈{0..33}, induction strength reduction — 400-case PBT ×3 levels (p_opt_adversarial, resident)
- memory-graph fuzzer: aliasing index writes, fn-ptr side effects, box-ptr chains — 300 ×3 levels (p_memgraph, resident)
- false-rejection battery: 28 valid-C forms (complex declarators incl. fn-ret-ptr-to-arr, 8KB strings, 200-deep ternaries, _Atomic(int), _Thread_local, anon-nest-3) — zero false rejects
- GNU extensions byte-exact: statement expressions, typeof-in-expr, __builtin_constant_p, labels-as-values + computed goto, packed/aligned attributes (P=7/offs 1,5/A16 enforce)
- _Thread_local semantics; printf arg-order divergence = unspecified (C11 6.5.2.2p10-ish) — caveat, not bug
- (double)2^64→u64 UB divergence — caveat

## Design Caveats
- gcc rejects block-scope static fn defs; ccc accepting is #1150 (not caveat)
- printf multi-call arg order differs gcc (RTL) vs ccc (LTR) — unspecified evaluation order, both conforming

## Test Files Created
src/passes/mod.rs ::pbt_opt_adversarial (400), ::pbt_memgraph (300) — resident properties
