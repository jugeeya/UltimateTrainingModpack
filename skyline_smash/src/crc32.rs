/// const crc32 implementation by leo60288

macro_rules! reflect {
    ($bits:expr, $value:expr) => {{
        let mut reflection = 0;
        let mut value = $value;
        let mut i = 0;

        while i < $bits {
            if (value & 0x01) == 1 {
                reflection |= 1 << (($bits - 1) - i)
            }

            value >>= 1;
            i += 1;
        }

        reflection
    }};
}

const fn make_table(poly: u32) -> [u32; 256] {
    let mut table = [0; 256];
    let top_bit = 1 << 31;
    let mut byte;

    let mut i = 0;
    while i <= 255 {
        byte = reflect!(8, i);

        let mut value = byte << 24;

        let mut j = 0;
        while j < 8 {
            if (value & top_bit) != 0 {
                value = (value << 1) ^ poly
            } else {
                value <<= 1
            }

            j += 1;
        }

        value = reflect!(32, value);

        table[i as usize] = value;

        i += 1;
    }

    table
}

const IEEE_TABLE: [u32; 256] = make_table(0x04C11DB7);

pub const fn crc32(bytes: &[u8]) -> u32 {
    let mut value = !0u32;
    let mut i = 0;
    while i < bytes.len() {
        value = (value >> 8) ^ (IEEE_TABLE[((value ^ (bytes[i] as u32)) & 0xFF) as usize]);
        i += 1;
    }

    !value
}

