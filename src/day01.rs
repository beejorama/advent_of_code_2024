use itertools::Itertools;

pub fn run(input: &str) {

    // set up arrays so we can sort them
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<i32> = line.split_ascii_whitespace().map(|location| location.parse::<i32>().unwrap()).collect();
        
        // consume the newly parsed values by deref
        left.push(*split_line.get(0).unwrap());
        right.push(*split_line.get(1).unwrap());
    }

    left.sort();
    right.sort();

    // logic part 1
    let mut difference: usize = 0;

    for i in 0..left.len() {
        
        let line_diff = (left.get(i).unwrap() - right.get(i).unwrap()).abs() as usize;
        difference += line_diff;
    }

    // logic part 2
    // options:
    //      calculate the score for each row
    //      save the previous number and calculation, then compare current number vs previous
    //      group vector
    let mut similarity_score: usize = 0;

    let grouped_locations = left.iter().chunk_by(|location_number| *location_number);

    for (key, chunk) in grouped_locations.into_iter() {
        let count_in_right: usize = right.iter().filter(|l| l == &key).count();

        similarity_score += *key as usize * count_in_right * chunk.count();
    }
            
    println!("{difference}");
    println!("{similarity_score}");
}