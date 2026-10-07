# digraphs are unsupported — %:include / <% %> / <: :> rejected (C95 standard spellings; gcc/clang accept)
> #1172

## Obscure-features checklist sweep (18 items, zero new bugs)
- 16/18 identical to gcc: stdbool/u8-prefix/\e-GNU/complex-compound-assign/
  _Complex_I/__FUNCTION__/#warning/noreturn-attr/thread_local(C23)/
  digit-sep/typeof/nullptr/constexpr/auto/[[attr]]/typeof-unqualified
- 2 forward-compat divergences (ccc ACCEPTS C23 forms old-gcc default rejects —
  C23 legalizes them; NOT defects, recorded as default-std caveats):
  * static_assert(1, "msg") unqualified spelling
  * #if 0b101 binary literals in preprocessor expressions

## Combined with the earlier same-session sweep
-O3/-Os/-Ofast semantics, boundary-literal TYPE selection, __has_include,
-Werror promotion, -march acceptance — all green.

## VERDICT
#1172 (digraphs) was the last genuine defect found. After: 18-item feature
checklist + 6 flag probes + 2 new mines (namespace/.s-interop) + big-program
stress — all green or non-defect divergences. Saturation confirmed on both
axes: bug-density AND feature-coverage.

## Round-47b deepening — complete form-matrices for filed families
- #1144 context matrix (9 contexts): arithmetic-only trigger; array-declarator
  dimension ALSO broken (256-int stack slot for 7!) — comment posted
- #1118 HFA shape matrix: 8/8 directions bad (F1-F4, D1-D2, args+returns) — comment
- #1135 lvalue wave 2: +10 forms (compound assigns, pre/post, ternary/comma-as-lvalue) 7→17 — comment
- #1114 x86: args fully GREEN across mixed shapes — scope confirmed return-only
- #1140: +3 static non-const init forms (scalar/array/struct) — comment
- #1088: +function-call syntax in #if — comment
- #1147: pointer-to-typedef-VLA WORKS — scope narrowed — comment
