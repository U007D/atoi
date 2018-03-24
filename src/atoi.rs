// v2
#[allow(dead_code)]
pub fn atoi_v2(s: &str) -> u64 {
    s.chars()
     .fold(0u64, |acc, d| acc * 10 + d.to_digit(10).expect("invalid digit") as u64)
}

// v3
#[allow(dead_code)]
pub fn atoi_v3(s: &str) -> u64 {
    #[inline]
    fn n_pow_10(mut n: u64, mut pow: usize) -> u64 {
        n <<= pow;
        while pow > 0 {
            pow -= 1;
            n += n << 2;
        }
        n
    }

    #[allow(unused_imports)]
    use std::iter::Sum;

    s.chars()
     .rev()
     .enumerate()
     .map(|(i, d)| n_pow_10(d.to_digit(10).expect("invalid digit") as u64, i))
     .sum()
}

// v4
#[allow(dead_code)]
pub fn atoi_v4(s: &str) -> u64 {
    s.chars()
     .fold(0u64, |acc, d| (acc << 1) + (acc << 3) + d.to_digit(10).expect("invalid digit") as u64)
}

// v5
#[allow(dead_code)]
// How is this working?  (pow misused?!)
pub fn atoi_v5(s: &str) -> u64 {
    let mut pow = s.len() - 1;
    let mut res = 0u64;
    s.chars()
     .for_each(|c| {
         let mut d = (c as u8).wrapping_sub('0' as u8);
         d <<= pow;
         while pow > 0 {
             pow -= 1;
             d += d << 2;
         }
         res += d as u64;
     });
    res
}
