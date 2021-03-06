use byteorder::{ByteOrder, LittleEndian};
use rand_core;
use rand_core::le::read_u32_into;
use rand_core::impls::{fill_bytes_via_next, next_u64_via_u32};
use rand_core::{RngCore, SeedableRng};

/// A xoroshiro64* random number generator.
///
/// The xoroshiro64* algorithm is not suitable for cryptographic purposes, but
/// is very fast and has excellent statistical properties.
///
/// The algorithm used here is translated from [the `xoroshiro64star.c`
/// reference source code](http://xoshiro.di.unimi.it/xoroshiro64star.c) by
/// David Blackman and Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Xoroshiro64Star {
    s0: u32,
    s1: u32,
}

impl Xoroshiro64Star {
    /// Seed a `Xoroshiro64Star` from a `u64` using `SplitMix64`.
    pub fn from_seed_u64(seed: u64) -> Xoroshiro64Star {
        let mut s = [0; 8];
        LittleEndian::write_u64(&mut s, seed);
        Xoroshiro64Star::from_seed(s)
    }
}

impl RngCore for Xoroshiro64Star {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let r = self.s0.wrapping_mul(0x9E3779BB);
        impl_xoroshiro_u32!(self);
        r
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64_via_u32(self)
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

impl SeedableRng for Xoroshiro64Star {
    type Seed = [u8; 8];

    /// Create a new `Xoroshiro64Star`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u8; 8]) -> Xoroshiro64Star {
        assert!(seed != [0, 0, 0, 0, 0, 0, 0, 0],
            "Xoroshiro64Star::from_seed called with an all zero seed.");
        let mut s = [0; 2];
        read_u32_into(&seed, &mut s);

        Xoroshiro64Star {
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
        let mut rng = Xoroshiro64Star::from_seed([1, 0, 0, 0, 2, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro64star.c
        let expected = [
            2654435771, 327208753, 4063491769, 4259754937, 261922412, 168123673,
            552743735, 1672597395, 1031040050, 2755315674,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u32(), e);
        }
    }
}
