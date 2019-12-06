use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Lines, empty};

fn detectTwoThree(my_str: &str) -> (u32, u32) {
    let char_map = count_chars(my_str);
    let mut count_map:HashSet<u32> = HashSet::new();

    for count in char_map.values() {
        count_map.insert(*count);
    }

    let mut result23 = ( 0, 0);

    if count_map.contains(&2) {
        result23.0 = 1;
    }

    if count_map.contains(&3) {
        result23.1 = 1;
    }

    return result23;
}

fn count_chars(value: &str) -> HashMap<char, u32>{
    let mut hash_map:HashMap<char, u32> = HashMap::new();

    for current_char in value.chars() {
        hash_map.entry(current_char).
            and_modify(|counter| { *counter += 1 })
            .or_insert(1);

    }

    hash_map
}
fn part01() {
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");

    let mut result = 0;
    let mut counter = (0,0);
    for line in file_content.lines() {
        let result = detectTwoThree(line.trim());
        counter.0 += result.0;
        counter.1 += result.1;
    }
    let result = counter.0 * counter.1;
    println!("result is {}", result);
}

fn part02() {
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");
    let (line_a, line_b) = findMismatchingLines(&file_content);
    let zipped_chars:Vec<_> = line_b.chars().zip(line_a.chars()).
        filter(|(a, b)| !a.eq(&b)).collect();
    println!("differences {:?}", zipped_chars);
}

fn findMismatchingLines(content: &str) -> (&str, &str) {
    let lines = content.lines();

    for (index, line_a) in lines.clone().enumerate() {
        for line_b in lines.clone().skip(index) {
            let result = countMismatches(line_a.trim(), line_b.trim());
            if result == 1 {
//                println!("(skipped {}), compare line a {} with line b {} ", index, line_a, line_b );
//                println!("result {:}", result);
                return (line_a, line_b);
            }
        }
    }
    return ("", "");
}
fn countMismatches(value_a: &str, value_b: &str) -> usize {
    let probe_a = value_a.chars().into_iter();
    let probe_b = value_b.chars().into_iter();
    return probe_a.zip(probe_b).into_iter()
        .map(|(a, b)| a.eq(&b))
        .filter(|result| result.eq(&false))
        .count();
}

fn main() {
    println!("Run Advent 02");
    part01();
    part02();
}
