use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{SeedableRng, RngCore, Error};

use Seed512;

/// A xoshiro512+ random number generator.
///
/// The xoshiro512+ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has good statistical properties, besides a low linear
/// complexity in the lowest bits.
///
/// The algorithm used here is translated from [the `xoshiro512plus.c`
/// reference source code](http://xoshiro.di.unimi.it/xoshiro512plus.c) by
/// David Blackman and Sebastiano Vigna.
#[derive(Debug, Clone)]
pub struct Xoshiro512Plus {
    s: [u64; 8],
}

impl Xoshiro512Plus {
    /// Seed a `Xoshiro512Plus` from a `u64` using `SplitMix64`.
    pub fn from_seed_u64(seed: u64) -> Xoshiro512Plus {
        from_splitmix!(seed)
    }

    /// Jump forward, equivalently to 2^256 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^256 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoshiro::Xoshiro512Plus;
    ///
    /// let rng1 = Xoshiro512Plus::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        impl_jump!(u64, self, [
            0x33ed89b6e7a353f9, 0x760083d7955323be, 0x2837f2fbb5f22fae,
            0x4b8c5674d309511c, 0xb11ac47a7ba28c25, 0xf1be7667092bcc1c,
            0x53851efdb6df0aaf, 0x1ebbc8b23eaf25db
        ]);
    }
}

impl SeedableRng for Xoshiro512Plus {
    type Seed = Seed512;

    #[inline]
    fn from_seed(seed: Seed512) -> Xoshiro512Plus {
        let mut state = [0; 8];
        read_u64_into(&seed.0, &mut state);
        Xoshiro512Plus { s: state }
    }
}

impl RngCore for Xoshiro512Plus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result_plus = self.s[0].wrapping_add(self.s[2]);
        impl_xoshiro_large!(self);
        result_plus
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let mut rng = Xoshiro512Plus::from_seed(Seed512(
            [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
             3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
             5, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
             7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0]));
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro512plus.c
        let expected = [
            4, 8, 4113, 25169936, 52776585412635, 57174648719367,
            9223482039571869716, 9331471677901559830, 9340533895746033672,
            14078399799840753678,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
