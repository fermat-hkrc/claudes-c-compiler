# PBT Campaign: parser deep (round 18)

## Scan findings
- Probe battery: 27 adversarial declarator/initializer/type forms vs gcc -fsyntax-only
  (function pointers, arrays-of-pointers-to-functions, K&R-adjacent storage
  classes, designated initializers incl. ranges/overwrites, compound
  literals, anonymous struct/union initializers, bitfields with holes,
  variadic prototypes, _Generic, __LINE__ in array sizes, typedef-of-function).
- Result: 26/27 agree with gcc. One divergence → issue #799:
  `sizeof(enum { X, Y })` — ccc parses the sizeof but the inline enum's
  enumerators do not become visible in the enclosing scope ('Y' undeclared);
  gcc/clang accept (the enum-specifier in a type-name declares the constants
  in the enclosing scope).
- Round 04's 15 properties + this battery: the parser surface is solid.

## Module: parser_deep
- [x] Scan (27-form gcc-differential battery)
- [x] Plan (battery + issue triage)
- [x] Test (26/27 agree)
- [x] Review (#799 filed)
