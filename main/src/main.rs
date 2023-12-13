use aoc2023::day1_0;
use aoc2023::day2_0::get_day2;

fn main() {
    // let day1 = day1_0::get_day1().unwrap();
    // println!("day1 result : {}", day1);

    // let day2_1 = match get_day2((12, 13, 14)) {
    //     None => 0,
    //     Some(n) => n,
    // };
    // println!("day2_1 result : {}", day2_1);

    let day2_2 = match get_day2((12, 13, 14)) {
        None => 0,
        Some(n) => n,
    };
    println!("day2_1 result : {}", day2_2);
}
