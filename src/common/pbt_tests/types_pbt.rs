//! PBT properties (proptest) for types.rs — see pbt-out/PROPERTIES.md.
use crate::common::types::align_up;
use proptest::prelude::*;

// Property-based tests (proptest) for alignment helpers.
// Oracle: algebraic invariant — align_up(x, a) is the least multiple of
// a that is >= x (C alignment semantics; `a` is a power of two, as all
// C alignments are).
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P9: minimality laws of align_up for power-of-two alignments,
    /// including the documented overflow branch (returns offset unchanged).
    #[test]
    fn align_up_laws(x in any::<usize>(), k in 0usize..16) {
        let a = 1usize << k; // power-of-two alignment (1..=32768)
        let r = align_up(x, a);
        if x.checked_add(a - 1).is_some() {
            // normal branch: exact next multiple
            prop_assert!(r >= x, "r={} < x={}", r, x);
            prop_assert!(r % a == 0, "r={} not multiple of a={}", r, a);
            prop_assert!(r == 0 || r - a < x, "r={} not minimal for x={}, a={}", r, x, a);
        } else {
            // documented overflow guard: returns offset unchanged
            prop_assert_eq!(r, x);
        }
    }
}
