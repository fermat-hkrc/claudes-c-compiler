//! FxHash: a fast, non-cryptographic hash used by rustc.
//!
//! This replaces the default SipHash in HashMap/HashSet with a much faster
//! hash for compiler workloads where DoS resistance is unnecessary.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

/// Type aliases for HashMap/HashSet using FxHash.
pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;

const SEED: u64 = 0x517cc1b727220a95;

/// A speedy hash algorithm used within rustc. The hashmap in liballoc by
/// default uses SipHash which isn't quite as speedy as we want. In the
/// compiler we're not really worried about DOS attempts, so we use a fast
/// non-cryptographic hash.
#[derive(Default)]
pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5) ^ i;
        self.hash = self.hash.wrapping_mul(SEED);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // Process 8 bytes at a time
        let mut chunks = bytes.chunks_exact(8);
        for chunk in &mut chunks {
            let word = u64::from_ne_bytes(chunk.try_into().unwrap());
            self.add_to_hash(word);
        }
        // Handle remaining bytes
        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            let mut last = 0u64;
            for (i, &byte) in remainder.iter().enumerate() {
                last |= (byte as u64) << (i * 8);
            }
            self.add_to_hash(last);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

// ============================================================================
// Property-based tests (pi-pbt campaign, round 01_common — sweep round)
// ============================================================================

#[cfg(test)]
mod pbt_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]

        /// P15: typed write_uN is equivalent to writing the value's native-endian
        /// bytes, and 8-aligned splits of write() are transparent.
        #[test]
        fn pbt_p15_fxhash_write_consistency(
            v64 in any::<u64>(),
            v32 in any::<u32>(),
            v16 in any::<u16>(),
            v8 in any::<u8>(),
            head in prop::collection::vec(any::<u8>(), 0..24).prop_filter("8-multiple len", |v| v.len() % 8 == 0),
            tail in prop::collection::vec(any::<u8>(), 0..24),
        ) {
            let mut h1 = FxHasher::default();
            h1.write_u64(v64);
            let mut h2 = FxHasher::default();
            h2.write(&v64.to_ne_bytes());
            prop_assert_eq!(h1.finish(), h2.finish());

            let mut h1 = FxHasher::default();
            h1.write_u32(v32);
            let mut h2 = FxHasher::default();
            h2.write(&v32.to_ne_bytes());
            prop_assert_eq!(h1.finish(), h2.finish());

            let mut h1 = FxHasher::default();
            h1.write_u16(v16);
            let mut h2 = FxHasher::default();
            h2.write(&v16.to_ne_bytes());
            prop_assert_eq!(h1.finish(), h2.finish());

            let mut h1 = FxHasher::default();
            h1.write_u8(v8);
            let mut h2 = FxHasher::default();
            h2.write(&[v8]);
            prop_assert_eq!(h1.finish(), h2.finish());

            // Chunk-aligned split transparency.
            let mut whole = FxHasher::default();
            whole.write(&head);
            whole.write(&tail);
            let mut split = FxHasher::default();
            let joined: Vec<u8> = head.iter().chain(tail.iter()).copied().collect();
            split.write(&joined);
            prop_assert_eq!(whole.finish(), split.finish());
        }
    }
}
