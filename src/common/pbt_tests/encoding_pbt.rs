//! PBT properties (proptest) for encoding.rs — see pbt-out/PROPERTIES.md.
use crate::common::encoding::{bytes_to_string, decode_pua_byte};
use proptest::prelude::*;

// Property-based tests (proptest) for the PUA encoding contract.
// Oracle: algebraic round-trip / idempotence, per module doc contract.
use proptest::prelude::*;

/// Decode an encoded string back to raw bytes using the lexer's
/// `decode_pua_byte` at successive positions.
fn decode_all(s: &str) -> Vec<u8> {
    let input = s.as_bytes();
    let mut out = Vec::with_capacity(input.len());
    let mut pos = 0;
    while pos < input.len() {
        let (b, n) = decode_pua_byte(input, pos);
        out.push(b);
        pos += n;
    }
    out
}

/// True if `b` contains a literal UTF-8 encoding of a PUA code point in
/// U+E080..=U+E0FF — the collision range where the scheme is
/// information-losing by construction (documented scheme limit).
fn contains_pua_collision(b: &[u8]) -> bool {
    let mut i = 0;
    while i + 2 < b.len() {
        if b[i] == 0xEE
            && (b[i + 1] == 0x82 || b[i + 1] == 0x83)
            && (0x80..=0xBF).contains(&b[i + 2])
        {
            return true;
        }
        i += 1;
    }
    false
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P6: encode∘decode is the identity on inputs outside the PUA
    /// collision range (module doc: "encode ... then decode them back").
    #[test]
    fn pua_roundtrip(
        bytes in prop::collection::vec(any::<u8>(), 0..64)
            .prop_filter("BOM-prefixed (stripped by contract) or PUA-colliding", |b| {
                !(b.len() >= 3 && b[0..3] == [0xEF, 0xBB, 0xBF]) && !contains_pua_collision(b)
            })
    ) {
        let s = bytes_to_string(bytes.clone());
        prop_assert_eq!(decode_all(&s), bytes);
    }

    /// P7: encoding is idempotent — the encoded string is valid UTF-8 and
    /// re-encoding is a no-op (doc: "If the bytes are valid UTF-8,
    /// returns them as-is").
    #[test]
    fn encode_idempotent(
        bytes in prop::collection::vec(any::<u8>(), 0..64)
            .prop_filter("BOM-prefixed", |b| !(b.len() >= 3 && b[0..3] == [0xEF, 0xBB, 0xBF]))
    ) {
        let once = bytes_to_string(bytes);
        let twice = bytes_to_string(once.as_bytes().to_vec());
        prop_assert_eq!(once, twice);
    }
}
