#[cfg(test)]
mod tests {
    use aoc2023::day1_0::get_sum;

    #[test]
    fn day1_1() {
        let input = [
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for i in input {
            assert_eq!(get_sum(i.0), i.1);
        }
    }
}
