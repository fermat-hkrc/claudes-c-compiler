# PBT Campaign: second full-repo loop, station 1 (round 24)

> Deeper pass with the stronger oracles developed in rounds 15-23.

## Findings this station
- #808 (round 23 tail): riscv -S lacks .attribute arch (external assemblers reject).
- fpclass const battery: only signbit(-0.0) diverges — already #806 (class closed:
  repo-wide `as`-cast sweep found no sibling instances).
- Preprocessor round 2 (hex-float p-notation, macro-as-include-name, line-splice
  in strings, #line/__FILE__, recursion+undef cycles, zero-arg __VA_ARGS__,
  invalid ## paste): token-equivalent to gcc everywhere; the /##/ case is
  rejected by BOTH (ccc with a lesser diagnostic — not filed).
- copy_prop contracts (round-7 gap): transitive chains 0..80 (incl. > the
  documented 64-hop limit) and diamond multi-def — both PASS.
- Sema round 2 (10 gcc -fsyntax-only cases): 9 agree; same-scope enum
  redefinition accepted → filed.

## Module: second_loop_1
- [x] Scan/Test/Review — 3 new issues total this station (#806 #807 were round 22;
      this station: #808 + sema enum redefinition)
