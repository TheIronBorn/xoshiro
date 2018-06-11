use rand_core;
use rand_core::le::read_u64_into;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{RngCore, SeedableRng};

use super::SplitMix64;

fn rotl(x: u64, k: u64) -> u64 {
    (x << k) | (x >> (64 - k))
}

fn starstar(s0: u64) -> u64 {
    rotl(s0.wrapping_mul(5), 7).wrapping_mul(9)
}

/// A Xoroshiro128** random number generator.
///
/// The Xoroshiro128** algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro128starstar.c`
/// reference source code](http://xoshiro.di.unimi.it/xoroshiro128starstar.c) by
/// David Blackman and Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Xoroshiro128StarStar {
    s0: u64,
    s1: u64,
}

impl Xoroshiro128StarStar {
    pub fn from_seed_u64(seed: u64) -> Xoroshiro128StarStar {
        let mut rng = SplitMix64::from_seed_u64(seed);
        Xoroshiro128StarStar::from_rng(&mut rng).unwrap()
    }

    /// Jump forward, equivalently to 2^64 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^64 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoshiro::Xoroshiro128StarStar;
    ///
    /// let rng1 = Xoroshiro128StarStar::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        const JUMP: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];
        let mut s0 = 0;
        let mut s1 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                }
                self.next_u64();
            }
        }
        self.s0 = s0;
        self.s1 = s1;
    }
}

impl RngCore for Xoroshiro128StarStar {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let r = starstar(self.s0);
        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(24) ^ self.s1 ^ (self.s1 << 16);
        self.s1 = self.s1.rotate_left(37);
        r
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for Xoroshiro128StarStar {
    type Seed = [u8; 16];

    /// Create a new `Xoroshiro128StarStar`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u8; 16]) -> Xoroshiro128StarStar {
        assert!(seed != [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            "Xoroshiro128StarStar::from_seed called with an all zero seed.");
        let mut s = [0; 2];
        read_u64_into(&seed, &mut s);

        Xoroshiro128StarStar {
            s0: s[0],
            s1: s[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let mut rng = Xoroshiro128StarStar::from_seed(
            [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro128starstar.c
        let expected = [
            5760, 97769243520, 9706862127477703552, 9223447511460779954,
            8358291023205304566, 15695619998649302768, 8517900938696309774,
            16586480348202605369, 6959129367028440372, 16822147227405758281,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
