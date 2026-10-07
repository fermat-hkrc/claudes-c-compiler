# Round 46 — robustness fuzz + linker symbol semantics

## Bugs Found
### #1166 duplicate strong symbols link silently
- Formal: ∀ two objects defining the same strong symbol: link error required. Witness: gcc "multiple definition"; ccc links+runs (data form picks first TU; function form picks order-INDEPENDENTLY — internal heuristic).

## Extension comments
#1165 += -Wl,--whole-archive ignored (gcc errors on forced-pull undefined; ccc no-op)

## Verified-correct
- mutation fuzz: 600 mutated programs (token del/ins/rep incl. unicode/digraph junk) — zero ICE/panic/hang (p_mutation_no_ice, resident property)
- -E round-trip: ccc -E output re-compiles identically; gcc -E output feeds ccc (interop) — green
- missing main diagnosed; weak/strong resolution BOTH orders; tentative merges (round-33)
- archive lazy extraction (unused member with undefined ref NOT pulled); cross-member address initialization; .a member mutual deps both orders
