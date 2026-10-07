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
