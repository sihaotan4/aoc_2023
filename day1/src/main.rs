use std::fs;
use std::collections::HashMap;

const PATTERNS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

const NUMBERS: [u32; 18] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9
];

fn main() {
    let map = init_map(); 

    let input = read_data("data.txt");

    let result = sum_calibration_values(input, &map);

    println!("{}", result);
}

fn sum_calibration_values(input: Vec<String>, map: &HashMap<&str, u32>) -> u32 {
    let mut result: u32 = 0;

    for line in input {
        result += extract_calibration_value(&line, map);
    } 

    result
}

fn extract_calibration_value(s: &String, map: &HashMap<&str, u32>) -> u32 {
    let first = first_number(s, map);
    let last = last_number(s, map);
    (first*10) + last
}

fn read_data(filepath: &str) -> Vec<String> {
    let data = fs::read_to_string(filepath).expect("unable to read file");
    data.lines().map(String::from).collect::<Vec<String>>()
}

fn init_map() -> HashMap<&'static str, u32>{
    let mut map: HashMap<&str, u32> = HashMap::new();
    for i in 0..18 {
        map.insert(PATTERNS[i], NUMBERS[i]);
    }
    map
}

fn first_number(s: &String, map: &HashMap<&str, u32>) -> u32 {
    let mut smallest_position:usize = 9999;
    let mut result:u32 = 99; // unreachable state
    
    for (pattern, value) in map {
        match s.find(pattern) {
            Some(i) => {
                if i <= smallest_position {
                    result = *value;
                    smallest_position = i;
                }
            },
            None => {}
        };
    }
    if result == 99 {
        panic!("No match at all for string: {}", s) // unexpected for this puzzle
    }
    return result;
}

fn last_number(s: &String, map: &HashMap<&str, u32>) -> u32 {
    let mut largest_position: usize = 0;
    let mut result:u32 = 99;

    for (pattern, value) in map {
        match s.rfind(pattern) {
            Some(i) => {
                if i >= largest_position {
                    result = *value;
                    largest_position = i;
                }
            },
            None => {}
        }
    }
    if result == 99 {
        panic!("No match at all for string: {}", s) // unexpected for this puzzle
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_number() {
        let map = init_map();

        assert_eq!(first_number(&"two1nine".to_string(), &map.clone()), 2);
        assert_eq!(first_number(&"eightwothree".to_string(), &map.clone()), 8);
        assert_eq!(first_number(&"abcone2threexyz".to_string(), &map.clone()), 1);
        assert_eq!(first_number(&"xtwone3four".to_string(), &map.clone()), 2);
        assert_eq!(first_number(&"4nineeightseven2".to_string(), &map.clone()), 4);
        assert_eq!(first_number(&"zoneight234".to_string(), &map.clone()), 1);
        assert_eq!(first_number(&"7pqrstsixteen".to_string(), &map.clone()), 7);
    }

    #[test]
    fn test_last_number() {
        let map = init_map();

        assert_eq!(last_number(&"two1nine".to_string(), &map.clone()), 9);
        assert_eq!(last_number(&"eightwothree".to_string(), &map.clone()), 3);
        assert_eq!(last_number(&"abcone2threexyz".to_string(), &map.clone()), 3);
        assert_eq!(last_number(&"xtwone3four".to_string(), &map.clone()), 4);
        assert_eq!(last_number(&"4nineeightseven2".to_string(), &map.clone()), 2);
        assert_eq!(last_number(&"zoneight234".to_string(), &map.clone()), 4);
        assert_eq!(last_number(&"7pqrstsixteen".to_string(), &map.clone()), 6);
    }
}