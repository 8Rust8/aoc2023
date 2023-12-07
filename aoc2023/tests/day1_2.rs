// cargo test -p aoc2023 tests_day1_2::day1_2

#[cfg(test)]
mod tests_day1_2 {
    use aoc2023::day1_0::translate_line;

    #[test]
    fn day1_2() {
        let input = [
            ("two1nine", 29, "two21nine9"),
            ("eightwothree", 83, "eight8wo2three3"),
            ("abcone2threexyz", 15, "abcone12three3xyz"),
            ("xtwone3four", 77, "xtwo2ne13four4"),
            ("4nineeightseven2", 42, "4nine9eight8seven72"),
            ("zoneight234", 14, "zone1ight8234"),
            ("7pqrstsixteen", 76, "7pqrstsix6teen"),
            ("eighttwo", 82, "eight8two2"),
        ];

        for i in input {
            assert_eq!(translate_line(i.0), i.2);
        }
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn negative_forced() {
        panic!("designed to panic");
    }
}
