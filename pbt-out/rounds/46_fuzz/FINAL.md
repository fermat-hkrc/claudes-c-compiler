# Round 46 final — new-mine exploration + saturation verdict

## Filed this segment
#1167-#1170 (encode_call/tail/j_pseudo/jr), #1171 (encode_jump) — encode pseudo 100% swept

## New mines probed (both essentially GREEN)
A) namespace/scope semantics: 4 namespaces (tag/ordinary/typedef/label), block-extern
   shadowing, enum-tag-as-member reuse — ALL match gcc.
   Only find: nested duplicate label → extension comment on #1142.
B) .s interop: ccc -S output at -O2 for complex programs (bitfields/subnormals,
   builtins+complex, adversarial-optimizer, 4900-line) assembles under GNU as and
   links+runs — fully compatible.

## Big-program stress
4900-line valid program (3000-function chains, 1000 globals, 500 structs):
gcc 0.5s / ccc 6.4s — IDENTICAL OUTPUT (26561 7 7.5) at -O0 and -O2. Perf 13x slower, correct.

## Saturation verdict
Six original mines + 2 new: encode 100%, constraints ~95% (only same-root extensions
remain), ABI fully mapped, runtime green, GNU ext green, driver 100% mapped, linker
mapped. Consecutive green batches across 3 rounds. Remaining yield < 1/hour.
