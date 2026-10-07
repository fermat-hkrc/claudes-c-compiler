# Round 48 final — last two mines + complete campaign close

## This round
#1173 enum values >int truncated to 0 (fresh mine C — miscompile class)
Extension comments: #1139 += member-access-on-nonstruct (mine H)

## Final mines
G) wide/unicode char runtime values: L'A'/L'中'/u/U constants, wchar_t/char16_t/
   char32_t arrays — ALL exact vs gcc. GREEN.
H) deep recursive types: 50-level nested structs + 100-node linked lists —
   valid form GREEN (42 4950 100 204 identical); wrong-depth member chain
   accepted (→ #1139 extension).

## COMPLETE MINE MAP (14 total)
encode(100%) constraints(78 forms) ABI(4-backend matrix) runtime GNU-extensions
driver(10) linker namespaces .s-interop enum-types(#1173) variadic-promotions
error-recovery multi-TU-static wide-chars deep-types
