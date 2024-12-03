use regex::{Regex, RegexBuilder};

pub fn run(input: &str) {

    let mut running_total: usize = 0;
    let mut running_total_part_2: usize = 0;

    let part_1_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let part_2_regex = RegexBuilder::new(r"(?:^|do\(\))(.*?)(?:don't\(\))")
        .multi_line(false)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    // regex
    for (_, [num1, num2]) in part_1_regex.captures_iter(input).map(|c| c.extract()) {
        running_total += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap()
    }


    for (_, [valid_part]) in part_2_regex.captures_iter(input).map(|c| c.extract()) {
        for (_, [num1, num2]) in part_1_regex.captures_iter(valid_part).map(|c| c.extract()) {
            running_total_part_2 += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap()
        }
    }

    println!("{running_total}");
    println!("{running_total_part_2}");
}