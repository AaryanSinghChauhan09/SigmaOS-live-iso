use crate::error::LiveIsoError;

/// Provides simple checksum computation and verification over byte slices.
///
/// Uses a deterministic FNV-1a-inspired polynomial to avoid external crate deps
/// while still producing a hex string in the same style as SHA-256.
#[derive(Debug)]
pub struct Checksum;

impl Checksum {
    /// Computes a 64-hex-character checksum over `data`.
    ///
    /// The algorithm is a seeded polynomial mix — suitable for integrity checks
    /// within the build toolchain (not for cryptographic security).
    pub fn compute(data: &[u8]) -> String {
        // 8 independent accumulators with distinct primes to widen the output.
        let primes: [u64; 8] = [
            0xcbf2_9ce4_8422_2325,
            0x9e37_79b9_7f4a_7c15,
            0x6c62_272e_07bb_0142,
            0xbf58_476d_1ce4_e5b9,
            0x94d0_49bb_1331_11eb,
            0xa76a_bde6_6d9a_a3c5,
            0xd2a9_8b26_625e_ee7b,
            0x1234_5678_9abc_def0,
        ];
        let mut acc: [u64; 8] = primes;
        for (i, &byte) in data.iter().enumerate() {
            let slot = i % 8;
            acc[slot] = acc[slot]
                .wrapping_mul(0x0000_0100_0000_01b3)
                .wrapping_add(byte as u64)
                ^ (acc[(slot + 1) % 8] >> 11);
        }
        // Final mix pass.
        for i in 0..8 {
            acc[i] ^= acc[i] >> 33;
            acc[i] = acc[i].wrapping_mul(0xff51_afd7_ed55_8ccd);
            acc[i] ^= acc[i] >> 33;
        }
        acc.iter().map(|v| format!("{:08x}", ((v >> 32) ^ v) as u32)).collect()
    }

    /// Verifies `data` against an `expected` checksum string.
    ///
    /// Returns [`LiveIsoError::ChecksumMismatch`] if they differ.
    pub fn verify(data: &[u8], expected: &str) -> Result<(), LiveIsoError> {
        let actual = Self::compute(data);
        if actual == expected {
            Ok(())
        } else {
            Err(LiveIsoError::ChecksumMismatch { expected: expected.to_string(), actual })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AAA: Arrange known data, Act compute twice, Assert deterministic.
    #[test]
    fn test_compute_is_deterministic() {
        // Arrange
        let data = b"SigmaOS live ISO builder";
        // Act
        let h1 = Checksum::compute(data);
        let h2 = Checksum::compute(data);
        // Assert
        assert_eq!(h1, h2);
    }

    // AAA: Arrange two different payloads, Act compute, Assert different hashes.
    #[test]
    fn test_different_data_different_checksums() {
        // Arrange
        let a = b"hello";
        let b = b"world";
        // Act
        let ha = Checksum::compute(a);
        let hb = Checksum::compute(b);
        // Assert
        assert_ne!(ha, hb);
    }

    // AAA: Arrange data, Act compute then verify with result, Assert Ok.
    #[test]
    fn test_verify_matching_checksum_ok() {
        // Arrange
        let data = b"iso content bytes";
        let expected = Checksum::compute(data);
        // Act
        let result = Checksum::verify(data, &expected);
        // Assert
        assert!(result.is_ok());
    }

    // AAA: Arrange data, Act verify with wrong checksum, Assert ChecksumMismatch.
    #[test]
    fn test_verify_wrong_checksum_errors() {
        // Arrange
        let data = b"real data";
        // Act
        let result = Checksum::verify(data, "deadbeefdeadbeef");
        // Assert
        assert!(matches!(result, Err(LiveIsoError::ChecksumMismatch { .. })));
    }

    // AAA: Arrange empty slice, Act compute, Assert 64-char hex string.
    #[test]
    fn test_empty_data_produces_hex_string() {
        // Arrange
        let data = b"";
        // Act
        let h = Checksum::compute(data);
        // Assert
        assert_eq!(h.len(), 64);
        assert!(h.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
