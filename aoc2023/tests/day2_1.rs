#[cfg(test)]
mod tests {
    use aoc2023::day2_0::{get_game_sum, get_games, Game};

    #[test]
    fn day2_1() {
        let game = Game::new();
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let game_struct = get_games(input);
        assert_eq!(game_struct.id, 1);
        assert_eq!(get_game_sum(game_struct), (1, 1, 2, 3));
    }
}
