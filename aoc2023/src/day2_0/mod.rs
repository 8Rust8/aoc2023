// use std::collections::HashMap;
#[allow(unused)]
use std::fs;

const FILEPATH: &str = "C:/repo_local/aoc/rust/aoc2023/data/sample/day2";

#[derive(Debug, PartialEq, PartialOrd , Clone)]
pub struct Round {
    pub round_id: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq, PartialOrd , Clone)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            rounds: Vec::new(),
        }
    }
}

pub fn get_day2(input: (u32, u32, u32)) -> Option<u32> {
    let content = fs::read_to_string(FILEPATH).unwrap_or_else(|e| {
        panic!("unable to read data from the file : {}", e);
    });

    let mut sum_2_1: u32 = 0;
    let mut sum_2_2: u32 = 0;

    for line in content.lines() {
        if !line.is_empty() {
            // println!("Line : {}", &line);
            let game = get_games(line);
            // println!("game # {} is  : {:?}", &game.id, &game);

            // get_valid_game_repeat is used for day 2 part 1
            // sum_2_1 = sum_2_1 + get_valid_game_repeat(game.clone(), input).unwrap_or(0);

            //
            sum_2_2 = sum_2_2 + get_min_power(game);

            // BELOW IS USED WHEN BALLS ARE NOT REPEATING
            // let game_acc = get_game_sum(game);
            // println!("line: {}", line);
            // println!("game acc : {:?}", game_acc);
            // println!("Sum: {}", sum);
            // sum = sum + get_valid_game(game_acc, input).unwrap_or(0);
            // println!("***************************");
        }
    }

    // return if sum_2_1 == 0 { None } else { Some(sum_2_1) };
    return  if sum_2_2 == 0 { None } else { Some(sum_2_2) };
}

pub fn get_games(game: &str) -> Game {
    let game_split = game.split(":").collect::<Vec<&str>>();

    if game_split.len() > 2 {
        panic!("The input string can only have one ':' ")
    }

    let game_str = game_split[0];

    let game_id: u32 = game_str
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            panic!("unable to parse the game id");
        });

    // println!("Game Id : {}", game_id);

    //  parsing the RGB cubes
    let color_cubes_tuple = game_split[1].split(";").collect::<Vec<&str>>();
    // println!("color_cubes_tuple : {:?}", color_cubes_tuple);

    let mut rounds_vec: Vec<Round> = Vec::new();
    let mut r = 0u32;
    for round in color_cubes_tuple {
        r += 1;
        let mut nr = 0;
        let mut ng = 0;
        let mut nb = 0;
        round.trim().split(",").for_each(|f| {
            let re: Vec<&str> = f.trim().split(" ").collect();
            // println!("------re ----- {:?}", re);
            let num = re[0]
                .trim()
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("unable to parse number and color"));

            let color = re[1].to_string();

            if color.trim() == "red" {
                nr += num;
            } else if color == "green" {
                ng += num;
            } else if color == "blue" {
                nb += num;
            } else {
                panic!("only Red, Green , Blue cubes are allowed")
            }
        });

        let round = Round {
            round_id: r,
            red: nr,
            green: ng,
            blue: nb,
        };
        // println!("round details : {:?}", round);
        rounds_vec.push(round);
    }

    Game {
        id: game_id,
        rounds: rounds_vec,
    }
}


// case when balls are kept back to the bag after each round
pub fn get_valid_game_repeat(game: Game, inrgb: (u32, u32, u32)) -> Option<u32> {
    let is_valid_game = game.rounds.iter().all(|r| {
        (inrgb.0 >= r.red) & (inrgb.1 >= r.green) & (inrgb.2 >= r.blue)
    });

    if is_valid_game {
        println!("valid game: {:?}", game);
        return Some(game.id);
    }
    None
}

// get_game_sum is not used in chapter 2
pub fn get_game_sum(game: Game) -> (u32, u32, u32, u32) {
    let mut nr = 0;
    let mut ng = 0;
    let mut nb = 0;
    game.rounds.iter().for_each(|r| {
        nr += r.red;
        ng += r.green;
        nb += r.blue;
    });
    println!("game id : {} , red: {} , green: {} , blue:{}", game.id, nr, ng, nb);
    (game.id, nr, ng, nb)
}

pub fn get_valid_game(idrgb: (u32, u32, u32, u32), inrgb: (u32, u32, u32)) -> Option<u32> {
    if (inrgb.0 >= idrgb.1) & (inrgb.1 >= idrgb.2) & (inrgb.2 >= idrgb.3) {
        println!("valid games : {:?}", idrgb);
        return Some(idrgb.0);
    }
    None
}


// day 2 part 2
pub fn get_min_power(game: Game) -> u32 {
    let mut rmax = 0;
    let mut gmax = 0;
    let mut bmax = 0;
    game.rounds.iter().for_each(|r| {
        if r.red > rmax {
            rmax = r.red;
        }
        if r.green > gmax {
            gmax = r.green;
        }
        if r.blue > bmax {
            bmax = r.blue;
        }
    });


    // println!("game id : {} , red: {} , green: {} , blue:{}", game.id, nr, ng, nb);
    rmax * gmax * bmax
}