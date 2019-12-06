use std::fs::read_to_string;
use std::collections::HashSet;

pub fn part01() -> i32 {
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");

    let mut result = 0;
    for line in file_content.lines() {
        let number:i32 = line.trim().parse()
            .expect("Not a number to parse");
        result = result + number;
        println!("{} ({})", number, line);
    }

    return result;
}

pub fn part02() ->i32 {
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");

    let mut hash_set = HashSet::new();
    let mut result = 0;
    let mut rounds = 0;


    loop {
        println!("next loop round ({})", rounds);

        for line in file_content.lines() {
            let number:i32 = line.trim().parse()
                .expect("Not a number to parse");
            let new_frequency = result + number;
            let new_frequency_str = (new_frequency).to_string();
            if hash_set.contains(&new_frequency_str) {
                return new_frequency;
            }else {
                hash_set.insert(new_frequency_str);
            }

            result = new_frequency;
        }
        rounds = rounds + 1;
    }
}

fn main() {
    println!("Hello, world!");
    part01();
//    part02();
}
