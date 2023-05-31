use crc_any::CRCu8;

pub fn crc5(bytes: &[u8]) -> u8 {
    // Poly (0x05), bits (5), initial (0x1f), final_xor (0x00), reflect (false).
    let mut crc = CRCu8::create_crc(0x05, 5, 0x1f, 0x00, false);
    crc.digest(bytes);
    crc.get_crc()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test a valid CRC5 invocation.
    #[test]
    fn crc5_correct() {
        // Chain inactive
        assert_eq!(crc5(&[0x53, 0x05, 0x00, 0x00]), 0x03);
        // Chippy
        assert_eq!(crc5(&[0x40, 0x05, 0x00, 0x00]), 0x1C);
        // Init 1
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00]),
            0x1C
        );
        // Init 2
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x84, 0x00, 0x00, 0x00, 0x00]),
            0x11
        );
        // Init 3
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x20, 0x00, 0x00, 0x00, 0x01]),
            0x02
        );
        // Init 4
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x3C, 0x80, 0x00, 0x80, 0x74]),
            0x10
        );
        // Init 5
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x68, 0xC0, 0x70, 0x01, 0x11]),
            0x00
        );
        // Init 6
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x28, 0x06, 0x00, 0x00, 0x0F]),
            0x18
        );
        // Baudrate 1.625Mbps
        assert_eq!(
            crc5(&[0x51, 0x09, 0x00, 0x18, 0x00, 0x00, 0x61, 0x31]),
            0x1C
        );
    }

    /// Test a CRC5 call that does not match.
    #[test]
    fn crc5_wrong() {
        // Chain inactive. This should not match - the expected result is the different.
        assert_ne!(crc5(&[0x53, 0x05, 0x00, 0x00]), 0x04);
    }
}
