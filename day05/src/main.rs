use std::fs::read_to_string;
use regex::Regex;
use std::hash::Hash;
use std::collections::HashMap;

const PROBE:&str = "dabAcCaCBAcCcaDA";

fn main() {
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");
//    part01(PROBE);
    part02(&file_content);
}

fn reacting(char_a:char, char_b:char) -> bool {
    let char_a = char_a as i32;
    let char_b = char_b as i32;
    let diff = char_a - char_b;
    return i32::abs(diff) == 32;
}

fn reduce_units(units:&str, reactor: &dyn Fn(char, char) -> bool) -> String {
    println!("processing units {}", units.len());

    let mut collected_chars: Vec<char> = Vec::new();
    let mut last_unit = ' ';
    let mut reacted_count = 0;

    for (index, unit) in units.chars().enumerate() {
        if !unit.is_alphabetic() {
            continue;
        }

        if reactor(last_unit,unit) {
            let removed = collected_chars.pop().unwrap_or(' '); //remove last unit
//            let last_chars:Vec<char> = collected_chars.iter().skip(collected_chars.len() - 10).collect();

//            println!("Removed char {} {:?}", removed, last_chars);
//            println!("removed {} & {}", removed, unit);
            reacted_count += 1;
//            last_unit = ' ';
            last_unit = *collected_chars.iter().last().unwrap_or(&'-');
//            println!("Reset last unit to {}", last_unit);
            continue;
        }else{
            collected_chars.push(unit);
        }

        let value_string:String = collected_chars.clone().into_iter().collect();
        last_unit = unit;
    }

    let value = b'a' - b'A';
    let value_string:String = collected_chars.into_iter().collect();

    if reacted_count > 0 {
        println!("Reacted {} times, remaining units: {}", reacted_count, value_string.len());
        return reduce_units(&value_string, reactor);
    }else {
        return value_string;
    }

}
fn clear_unit(input:&str, unit1:char, unit2:char) -> String {
    let value1 = String::from(input).replace(unit1, "");
    let value2 = value1.replace(unit2, "");
    return value2;
}

fn run_cleared_by(input:&str, unit: char) -> (usize, String, char) {
    println!("\n--- run for {}", unit);
    let re = Regex::new(&format!("[{}|{}]", unit.to_uppercase(), unit.to_lowercase())).unwrap();

    let input = re.replace_all(input, "");
    let result = reduce_units(&input, &reacting);
    return (result.len(), result, unit);
}
fn part02(input:&str) {
    let char_list = "abcdefghijklmnopqrstuvwxyz";
    let mut result_map:HashMap<char, (String, usize, char)> = HashMap::new();

    for single_char in char_list.chars() {
        let (size, result, unit) = run_cleared_by(input, single_char);
//        println!("result (no {}): {} with length of {}", unit, result, size);
        result_map.insert(unit, (result, size, unit));
    }

    let smallest = result_map.values().min_by(|(_,size_a, _), (_, size_b, _)| {
        size_a.cmp(size_b)
    });
    match smallest {
        None => println!("Something wrong"),
        Some((value, size, unit)) => println!("\nFinal Result:\n\tReduced by {}: \n\tvalue: {} \n\twith size: {}", unit, value, size)
    }



}

fn part01(input:&str) {
    let result = reduce_units(input, &reacting);
    let r = &result[result.len() - 3..];
    println!("result {} with length of {}", result, result.len());
}
