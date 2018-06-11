/// Initialize a RNG from a `u64` seed using `SplitMix64`.
macro_rules! from_splitmix {
    ($seed:expr) => { {
        let mut rng = ::SplitMix64::from_seed_u64($seed);
        Self::from_rng(&mut rng).unwrap()
    } }
}

/// Perform the ** operation used by some RNGs from the xoshiro family.
macro_rules! starstar {
    ($x:expr) => {
        $x.wrapping_mul(5).rotate_left(7).wrapping_mul(9)
    }
}

/// Implement a jump function for an RNG from the xoshiro family.
macro_rules! impl_jump {
    ($output:ty, $self:expr, [$j0:expr, $j1:expr]) => {
        const JUMP: [$output; 2] = [$j0, $j1];
        let mut s0 = 0;
        let mut s1 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= $self.s0;
                    s1 ^= $self.s1;
                }
                $self.next_u64();
            }
        }
        $self.s0 = s0;
        $self.s1 = s1;
    };
    ($output:ty, $self:expr, [$j0:expr, $j1:expr, $j2:expr, $j3:expr]) => {
        const JUMP: [$output; 4] = [$j0, $j1, $j2, $j3];
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= $self.s[0];
                    s1 ^= $self.s[1];
                    s2 ^= $self.s[2];
                    s3 ^= $self.s[3];
                }
                $self.next_u32();
            }
        }
        $self.s[0] = s0;
        $self.s[1] = s1;
        $self.s[2] = s2;
        $self.s[3] = s3;
    };
}

/// Implement the xoroshiro iteration.
macro_rules! impl_xoroshiro {
    ($self:expr) => {
        $self.s1 ^= $self.s0;
        $self.s0 = $self.s0.rotate_left(24) ^ $self.s1 ^ ($self.s1 << 16);
        $self.s1 = $self.s1.rotate_left(37);
    }
}

/// Implement the xoshiro iteration for `u32` output.
macro_rules! impl_xoshiro_u32 {
    ($self:expr) => {
        let t = $self.s[1] << 9;

        $self.s[2] ^= $self.s[0];
        $self.s[3] ^= $self.s[1];
        $self.s[1] ^= $self.s[2];
        $self.s[0] ^= $self.s[3];

        $self.s[2] ^= t;

        $self.s[3] = $self.s[3].rotate_left(11);
    }
}

/// Implement the xoshiro iteration for `u64` output.
macro_rules! impl_xoshiro_u64 {
    ($self:expr) => {
        let t = $self.s[1] << 17;

        $self.s[2] ^= $self.s[0];
        $self.s[3] ^= $self.s[1];
        $self.s[1] ^= $self.s[2];
        $self.s[0] ^= $self.s[3];

        $self.s[2] ^= t;

        $self.s[3] = $self.s[3].rotate_left(45);
    }
}
