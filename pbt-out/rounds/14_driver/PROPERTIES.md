# PROPERTIES.md — round 14 (src/driver)

## p1_driver_mode_plumbing
- Tier: 3
- Doc contract: cli.rs mode table (-E PreprocessOnly, -S AssemblyOnly, -c ObjectOnly, default Full)
- Formal: ∀ (target, mode) ∈ {x86-64, i686} × {-E, -S, -c, Full}, ∀ tiny C source: the driver produces the mode's artifact, well-formed (text contains the function identifier; object is ET_REL with the right machine; executable exits 0).
- Test file: src/driver/mod.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: driver::Driver::{parse_cli_args, run}
oracle: reference
predicate:
  quantifier: forall
  vars: [target, mode, ret_value]
  domain: { target: {x86_64, i686}, mode: {E, S, c, Full}, ret_value: int }
  relation:
    op: holds
    expr: "artifact_exists_and_well_formed(Driver.run([argv0(target), mode_flag(mode), tiny_c(ret_value)]))"
generators:
  target: { gen: bool }
  mode: { gen: int, min: 0, max: 3, type: u8 }
  ret_value: { gen: int, min: 0, max: 120, type: u8 }
evidence: src/driver/cli.rs:249-251 mode flags; driver pipeline modes
```
