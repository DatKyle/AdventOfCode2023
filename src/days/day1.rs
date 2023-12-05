use regex::Regex;
mod util;

fn get_numebers_in_str(string: &str) -> u32 {
    let re = Regex::new(r"[A-Za-z]").unwrap();
    let numbers: String = re.replace_all(string, "").to_string();
    let first = numbers.chars().nth(0).expect("No Chars").to_string();
    let last = numbers.chars().nth_back(0).expect("No Chars").to_string();
    let mut number = first;
    number.push_str(&last);
    let number: u32 = number.parse().expect("msg");
    number
}

fn conver_word_to_number(line: &str) -> String {
    let mut number_line = line.replace("one", "o1e");
    number_line = number_line.replace("two", "t2o");
    number_line = number_line.replace("three", "t3e");
    number_line = number_line.replace("four", "f4r");
    number_line = number_line.replace("five", "f5e");
    number_line = number_line.replace("six", "s6x");
    number_line = number_line.replace("seven", "s7n");
    number_line = number_line.replace("eight", "e8t");
    number_line = number_line.replace("nine", "n9e");
    number_line
}

pub fn main() {
    const DAY: u8 = 1;
    println!("\n\n\nAdvent of Code day {DAY}");

    let contents = util::get_file_contents("data/day1/data.txt");

    let lines = contents.lines();

    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;

    for line in lines {
        sum_part1 += get_numebers_in_str(line);
        let new_line = conver_word_to_number(line);
        sum_part2 += get_numebers_in_str(&new_line);
    }
    println!("Part 1: {sum_part1}\nPart 2: {sum_part2}")
}
