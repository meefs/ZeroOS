pub fn generate_random_bytes(seed_values: &[u64]) -> (u64, u64) {
    let mut r0 = seed_values.iter().fold(0u64, |acc, &v| acc ^ v);

    if r0 == 0 {
        r0 = 0x9E3779B97F4A7C15;
    }

    let mut r1 = r0.rotate_left(13) ^ 0x9E3779B97F4A7C15;

    r0 ^= r0 << 13;
    r0 ^= r0 >> 7;
    r0 ^= r0 << 17;
    r1 ^= r1 << 13;
    r1 ^= r1 >> 7;
    r1 ^= r1 << 17;

    (r0, r1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_bytes_deterministic() {
        let seed = [0x1234_5678_9ABC_DEF0u64];
        let (low1, high1) = generate_random_bytes(&seed);
        let (low2, high2) = generate_random_bytes(&seed);

        assert_eq!(low1, low2);
        assert_eq!(high1, high2);
    }

    #[test]
    fn test_generate_random_bytes_different_seeds() {
        let (low1, high1) = generate_random_bytes(&[0x1111_1111_1111_1111u64]);
        let (low2, high2) = generate_random_bytes(&[0x2222_2222_2222_2222u64]);

        assert_ne!(low1, low2);
        assert_ne!(high1, high2);
    }

    #[test]
    fn test_generate_random_bytes_multiple_entropy_sources() {
        let entropy = [
            0x1234_5678_9ABC_DEF0u64,
            0xFEDC_BA98_7654_3210u64,
            0xAAAA_BBBB_CCCC_DDDDu64,
        ];
        let (low, high) = generate_random_bytes(&entropy);

        assert_ne!(low, 0);
        assert_ne!(high, 0);

        assert_ne!(low, high);
    }

    #[test]
    fn test_generate_random_bytes_no_entropy() {
        let (low, high) = generate_random_bytes(&[]);
        assert_ne!(low, 0);
        assert_ne!(high, 0);
    }
}
