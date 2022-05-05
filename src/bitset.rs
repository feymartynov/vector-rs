use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

pub const BITS_SIZE: usize = 256;

pub static BITS_TABLE: [u64; BITS_SIZE] = {
    let mut bits_table = [0; BITS_SIZE];
    let mut i = 0;

    while i < BITS_SIZE {
        bits_table[i] = 1 << (i & 0x3f);
        i += 1;
    }

    bits_table
};

///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bitset([u64; 4]);

impl Bitset {
    pub fn new(bits: [u64; 4]) -> Self {
        Self(bits)
    }

    pub fn as_bits(&self) -> &[u64] {
        &self.0
    }

    pub fn load_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter().copied() {
            self.0[(byte >> 6) as usize] |= BITS_TABLE[byte as usize];
        }
    }

    pub fn to_bytes_dump(&self, bytes: &mut Vec<u8>) {
        bytes.clear();

        for i in 0..4 {
            if self.0[i] == 0 {
                continue;
            }

            let off = (i * 64) as u8;

            for x in 0..64 {
                if self.0[i] & (1 << x) != 0 {
                    bytes.push(x as u8 + off);
                }
            }
        }
    }

    pub fn to_bytes(&self, bytes: &mut [u8; 256]) -> usize {
        let mut n = 0;

        for i in 0..4 {
            let off = (i << 6) as u8;
            let mut v = 0;
            let mut rv = self.0[i];

            while rv != 0 {
                let mut x = rv.trailing_zeros();
                bytes[n] = (x + v) as u8 + off;
                n += 1;
                x += 1;
                v += x;
                rv = rv.checked_shr(x).unwrap_or(0);
            }
        }

        n
    }

    pub fn is_zero(&self) -> bool {
        self.0[0] | self.0[1] | self.0[2] | self.0[3] == 0
    }
}

impl BitAnd for Bitset {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] & rhs.0[0],
            self.0[1] & rhs.0[1],
            self.0[2] & rhs.0[2],
            self.0[3] & rhs.0[3],
        ])
    }
}

impl BitAndAssign for Bitset {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0[0] &= rhs.0[0];
        self.0[1] &= rhs.0[1];
        self.0[2] &= rhs.0[2];
        self.0[3] &= rhs.0[3];
    }
}

impl BitOr for Bitset {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] | rhs.0[0],
            self.0[1] | rhs.0[1],
            self.0[2] | rhs.0[2],
            self.0[3] | rhs.0[3],
        ])
    }
}

impl BitOrAssign for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0[0] |= rhs.0[0];
        self.0[1] |= rhs.0[1];
        self.0[2] |= rhs.0[2];
        self.0[3] |= rhs.0[3];
    }
}

impl BitXor for Bitset {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self([
            self.0[0] ^ rhs.0[0],
            self.0[1] ^ rhs.0[1],
            self.0[2] ^ rhs.0[2],
            self.0[3] ^ rhs.0[3],
        ])
    }
}

impl BitXorAssign for Bitset {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0[0] ^= rhs.0[0];
        self.0[1] ^= rhs.0[1];
        self.0[2] ^= rhs.0[2];
        self.0[3] ^= rhs.0[3];
    }
}
