use std::{fs, collections::HashMap};

fn main() {
    let part1_input = fs::read_to_string("input_part1.txt").expect("The puzzle input exists.");
    let result = get_calibration_values_part1(&part1_input);
    // println!("The calibration values are {:?}.", result);
    println!("The sum of part1 is {}.", result.iter().sum::<u32>());
    
    let part2_input = fs::read_to_string("input_part2.txt").expect("The puzzle input exists.");
    let result = get_calibration_values_part2(&part2_input);
    // println!("The calibration values are {:?}.", result);
    println!("The sum of part2 is {}.", result.iter().sum::<u32>());
}

fn get_calibration_values_part1(input: &str) -> Vec<u32> {
    let mut ret : Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut number = String::from("");
        number.push(line.chars().find_map(|c| if c.is_ascii_digit() { Some(c) } else {None}).expect("At least 1 digit in each line"));
        number.push(line.chars().rev().find_map(|c| if c.is_ascii_digit() { Some(c) } else {None}).expect("At least 1 digit in each line"));
        ret.push(number.parse().unwrap());
    }
    ret
}


fn find_digit_or_digit_word(it: impl  Iterator<Item = (usize, char)>, src_input: &str) -> Option<char> {
    let num_names = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for (i, c) in it {
        if c.is_ascii_digit() {
            return Some(c);
        }
        for num_name in num_names.keys() {
            if src_input[i..].starts_with(num_name) {
                return Some(num_names[num_name]); 
            }
        }
    } 
    return None;
}

fn get_calibration_values_part2(input: &str) -> Vec<u32> {

    let mut ret : Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut number = String::new();
        number.push(find_digit_or_digit_word(line.char_indices(), line).unwrap());
        number.push(find_digit_or_digit_word(line.char_indices().rev(), line).unwrap());
        ret.push(number.parse().unwrap());
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = get_calibration_values_part1(input).iter().sum::<u32>();
        assert_eq!(result, 142);
    }

    #[test]
    fn part2() {
        let input = fs::read_to_string("./example_part2.txt");
        let result = get_calibration_values_part2(&input.unwrap()).iter().sum::<u32>();
        assert_eq!(result, 281);
    }

}
