pub const BITS_SIZE: usize = 0xff + 1;

static BITS_TABLE: [u64; BITS_SIZE] = {
    let mut bits_table = [0; BITS_SIZE];
    let mut i = 0;

    while i < BITS_SIZE {
        bits_table[i] = 1 << (i & 0x3f);
        i += 1;
    }

    bits_table
};

#[derive(Debug, Default)]
pub struct Bitset([u64; 4]);

impl Bitset {
    pub fn new(bits: [u64; 4]) -> Self {
        Self(bits)
    }

    pub fn as_bits(&self) -> &[u64] {
        &self.0
    }

    pub fn bytes_to_bits(&mut self, bytes: &[u8]) {
        for byte in bytes.iter().copied() {
            self.0[(byte >> 6) as usize] |= BITS_TABLE[byte as usize];
        }
    }

    pub fn bits_to_bytes_dump(&self, mut bytes: Vec<u8>) -> Vec<u8> {
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

        bytes
    }

    pub fn bits_to_bytes(&self, mut bytes: Vec<u8>) -> Vec<u8> {
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

        bytes.truncate(n);
        bytes
    }
}
