pub fn safe_atoi_v2(s: &str) -> u64 {
    s.chars()
     .fold(0u64, |acc, d| acc * 10 + d.to_digit(10).expect("invalid digit") as u64)
}
