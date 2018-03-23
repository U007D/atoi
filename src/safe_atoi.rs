pub fn safe_atoi(s: &str) -> u64 {
    let pow10: [u64; 20] = [
        10000000000000000000,
        1000000000000000000,
        100000000000000000,
        10000000000000000,
        1000000000000000,
        100000000000000,
        10000000000000,
        1000000000000,
        100000000000,
        10000000000,
        1000000000,
        100000000,
        10000000,
        1000000,
        100000,
        10000,
        1000,
        100,
        10,
        1,
    ];

    let b = s.as_bytes();
    let mut len = s.len();
    let mut i: usize = 20 - len;
    let mut idx: usize = 0;
    let mut result = 0;

    unsafe {
        while len >= 4 {
            let d1 = (b.get_unchecked(idx) - b'0') as u64;
            let mut r1 = d1 * pow10.get_unchecked(i);

            let d2 = (b.get_unchecked(idx + 1) - b'0') as u64;
            let mut r2 = d2 * pow10.get_unchecked(i + 1);

            let d3 = (b.get_unchecked(idx + 2) - b'0') as u64;
            let mut r3 = d3 * pow10.get_unchecked(i + 2);

            let d4 = (b.get_unchecked(idx + 3) - b'0') as u64;
            let mut r4 = d4 * pow10.get_unchecked(i + 3);

            i += 4;
            idx += 4;
            len -= 4;

            result += r1 + r3 + r2 + r4;
        }

        for _ in 0..len {
            let d = (b.get_unchecked(idx) - b'0') as u64;
            let r = d * pow10.get_unchecked(i);

            idx += 1;
            i += 1;

            result += r;
        }

        return result
    }
}
