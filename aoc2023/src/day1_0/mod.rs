// use std::collections::HashMap;
#[allow(unused)]
use std::fs;

const FILEPATH: &str = "C:/repo_local/aoc/rust/aoc2023/data/sample/day1";

pub fn get_day1() -> Option<u32> {
    let content = fs::read_to_string(FILEPATH).unwrap_or_else(|e| {
        panic!("unable to read data from the file : {}", e);
    });

    let mut sum: u32 = 0;

    for line in content.lines() {
        if !line.is_empty() {
            // println!("Line : {}", &line);
            let translated_line = translate_line(line);
            // println!("Translated Line : {}", &translated_line);
            sum = sum + get_sum(&translated_line);
            // println!("Sum: {}", sum);
            // println!("***************************");
        }
    }

    return if sum == 0 { None } else { Some(sum) };
}

pub fn translate_line(line: &str) -> String {
    let line = line.to_string();
    println!("line : {}", line);
    let mut formatted_string = String::from("");
    let digits = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    let mut j: usize = 0;

    // line.chars().nth(0).unwrap().to_string()
    line.char_indices().fold("".to_string(), |mut acc, c| {
        let (ind, ch) = c;
        acc.push(ch);
        formatted_string.push(ch);

        for item in digits.iter() {
            let (k, v) = *item;
            // println!("K {}" , k);
            if acc[j..].contains(k) {
                // println!("before replacing formatted string : {}", formatted_string);
                formatted_string.push_str(v);
                j = ind;
            }
        }

        // println!("formatted string: {}", formatted_string);
        acc
    });

    formatted_string
}

pub fn get_sum(inline: &str) -> u32 {
    let searched = inline
        .chars()
        .clone()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c)
        .collect::<Vec<char>>();
    // println!("searched : {:?}", searched);
    let first = searched.get(0).unwrap().to_string().parse::<u32>().unwrap();
    let last = searched
        .get(searched.len() - 1)
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();
    let number = first * 10 + last;
    println!("Number : {}", number);
    number
}
