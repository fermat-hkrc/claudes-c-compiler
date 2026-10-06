//! Shared inline assembly constraint classification utilities.
//!
//! Used by the inline pass (symbol resolution), cfg_simplify (dead block detection),
//! and the backend (operand emission) to consistently identify immediate-only constraints.

/// Check whether a constraint string is purely immediate (no register or memory alternatives).
///
/// Returns true for constraints like "i", "n", "e" that ONLY accept compile-time constants.
/// Returns false for multi-alternative constraints like "ri", "Ir", "g" that also accept
/// registers or memory operands.
///
/// The constraint is first stripped of output/early-clobber modifiers ('=', '+', '&').
/// Named operand references like "[name]" are not immediate constraints.
pub fn constraint_is_immediate_only(constraint: &str) -> bool {
    let stripped = constraint.trim_start_matches(['=', '+', '&', '%']);
    if stripped.is_empty() {
        return false;
    }
    if stripped.starts_with('[') && stripped.ends_with(']') {
        return false;
    }
    // Must have at least one immediate letter
    let has_imm = stripped.chars().any(|c| matches!(c,
        'i' | 'I' | 'n' | 'N' | 'e' | 'E' | 'K' | 'M' | 'G' | 'H' | 'J' | 'L' | 'O'
    ));
    if !has_imm {
        return false;
    }
    // Must NOT have any register or memory alternative
    let has_reg_or_mem = stripped.chars().any(|c| matches!(c,
        'r' | 'q' | 'R' | 'l' |           // GP register
        'g' |                              // general (reg + mem + imm)
        'x' | 'v' | 'Y' |                 // FP register
        'a' | 'b' | 'c' | 'd' | 'S' | 'D' | // specific register
        'm' | 'o' | 'V' | 'p' | 'Q'       // memory
    ));
    !has_reg_or_mem && !stripped.chars().any(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod pbt_tests {
    use super::*;
    use proptest::prelude::*;

    /// P1: immediate-only constraints (i, n, I, N, etc.) → true; register
    /// constraints (r, q, etc.) → false; empty → false; named [x] → false;
    /// modifiers (=,+,&) stripped before checking.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]
        #[test]
        fn p_constraint_classification(
            letters in prop::collection::vec(any::<char>(), 1..5),
            mods in prop::collection::vec(any::<char>(), 0..3),
        ) {
            // Build a constraint string from random chars
            let raw: String = letters.into_iter().filter(|c| c.is_ascii_graphic()).collect();
            let prefixed: String = mods.into_iter()
                .filter(|c| "=+&%".contains(*c))
                .collect::<String>() + &raw;
            if prefixed.is_empty() { return Ok(()); }
            let result = constraint_is_immediate_only(&prefixed);
            // Check invariants:
            // 1. Pure register constraints → false
            let stripped = prefixed.trim_start_matches(['=', '+', '&', '%']);
            let has_imm = stripped.chars().any(|c| "inINeEKMGHJLO".contains(c));
            let has_reg = stripped.chars().any(|c| "rqRlgma".contains(c));
            if has_reg {
                prop_assert!(!result, "register constraint should not be imm-only: {}", prefixed);
            }
            // 2. Empty after strip → false
            if stripped.is_empty() {
                prop_assert!(!result);
            }
            // 3. Named [x] → false
            if stripped.starts_with('[') {
                prop_assert!(!result, "named constraint should not be imm-only: {}", prefixed);
            }
        }
    }

    /// P2: known constraints table (deterministic reference)
    #[test]
    fn p_known_constraints() {
        // Pure immediate
        assert!(constraint_is_immediate_only("i"));
        assert!(constraint_is_immediate_only("n"));
        assert!(constraint_is_immediate_only("I"));
        assert!(constraint_is_immediate_only("N"));
        // With modifiers
        assert!(constraint_is_immediate_only("=i"));
        assert!(constraint_is_immediate_only("+n"));
        // Register
        assert!(!constraint_is_immediate_only("r"));
        assert!(!constraint_is_immediate_only("q"));
        assert!(!constraint_is_immediate_only("=r"));
        assert!(!constraint_is_immediate_only("g"));  // general = reg+mem+imm
        // Mixed
        assert!(!constraint_is_immediate_only("ri"));
        assert!(!constraint_is_immediate_only("ir"));
        // Empty
        assert!(!constraint_is_immediate_only(""));
        assert!(!constraint_is_immediate_only("="));
        // Memory
        assert!(!constraint_is_immediate_only("m"));
        assert!(!constraint_is_immediate_only("o"));
        // Named
        assert!(!constraint_is_immediate_only("[name]"));
    }
}
