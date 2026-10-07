# Round 33 — linker_common multi-TU deep PBT

## Bugs Found
(none)

## Properties
| Property | Cases | Status |
|----------|-------|--------|
| p_multitu_link_vs_gcc (extern data, cross-fns, tentative defs, fn-ptr tables) | 200 | **passing** |
| weak symbols manual probe (common=77 weak_fn=1 weak_var=10 strong=101) | 1 | **matches gcc** |

## Design Caveats
- Multi-TU extern declarations in main are required (modern gcc rejects implicit decls) — generator contract, not SUT behavior.
