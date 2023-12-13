#[cfg(test)]
mod tests {
    use aoc2023::day2_0::{Game, get_game_sum, get_min_power, Round};

    #[test]
    fn day2_2() {
        let r1 = Round {
            round_id: 1,
            red: 5,
            green: 5,
            blue: 5,
        };

        let r2 = Round {
            round_id: 2,
            red: 1,
            green: 2,
            blue: 1,
        };

        let g = Game { id: 0, rounds: vec![r1, r2] };
        let min_power = get_min_power(g);
        assert_eq!(min_power, 125);
    }
}

