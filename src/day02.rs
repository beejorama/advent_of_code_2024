use std::cmp::{self, Ordering};

use itertools::Itertools;

pub fn run(input: &str) {

    let mut count_of_safe_reports = 0;
    let mut count_of_safe_reports_part_2 = 0;

    input
        .lines()
        .for_each(|line| {
            if report_is_safe(line) {
                count_of_safe_reports += 1;
            }
            if report_is_safe_part_2(line) {
                count_of_safe_reports_part_2 += 1;
            }
        });

    println!("{count_of_safe_reports}");
    println!("{count_of_safe_reports_part_2}");
}

fn report_is_safe(report: &str) -> bool {
    let report_vec: Vec<i8> = report
        .split_whitespace()
        .map(|f| f.parse::<i8>().unwrap())
        .collect();

    // if there are any duplicate numbers, just return false now
    let mut clone_vec = report_vec.clone();
    clone_vec.sort();
    clone_vec.dedup();

    if clone_vec.len() != report_vec.len() {
        return false;
    }

    // work out if we should be ascending or descending by comparing the first 2 elements
    let is_ascending: bool = report_vec.get(0).unwrap() < report_vec.get(1).unwrap();

    for (current, next) in report_vec.iter().tuple_windows() {
        if is_ascending {
            if !(current.cmp(next) == Ordering::Less
                && 1 <= (current - next).abs()
                && (current - next).abs() <= 3) {
                
                return false;
            }
        }
        else {
            if !(current.cmp(next) == Ordering::Greater
                && 1 <= (current - next).abs()
                && (current - next).abs() <= 3) {
                
                return false;
            }
        }
    }

    true
}

fn report_is_safe_part_2(report: &str) -> bool {

    // get vector of i8
    let report_vec: Vec<i8> = report
        .split_whitespace()
        .map(|f| f.parse::<i8>().unwrap())
        .collect();

    if !assess_sequence(&report_vec) {
        // iterate
        for i in 0..report_vec.len() {
            // remove an element
            let mut clone = report_vec.clone();
            clone.remove(i);
            if assess_sequence(&clone) {
                return true;
            }
        }
        return false;
    }

    true
}

fn assess_sequence(seq: &Vec<i8>) -> bool {
    // work out if valid ascending/descending sequence
    let mut ascending: Vec<u8> = Vec::new();
    let mut descending: Vec<u8> = Vec::new();
    let mut equal: Vec<u8> = Vec::new();
    let mut too_big: Vec<u8> = Vec::new();

    for i in 0..seq.len() - 1 {
        let current = seq.get(i).unwrap();
        let next = seq.get(i + 1).unwrap();

        match current.cmp(next) {
            Ordering::Greater => descending.push(i as u8),
            Ordering::Less => ascending.push(i as u8),
            Ordering::Equal => equal.push(i as u8)
        }

        let abs_diff = (current - next).abs();
        if  abs_diff > 3 {
            too_big.push(i as u8);
        }
    }

    let is_ascending = cmp::max(ascending.len(), descending.len()) == ascending.len();

    // if we have a gap which is too big, this sequence is invalid
    if too_big.len() > 0 {
        return false
    }

    // if we have >1 directional issue the sequence is certainly invalid
    let directional_issues = if is_ascending { [equal, descending].concat() } else { [equal, ascending].concat() };

    if directional_issues.len() > 0 {
        return false;
    }

    true
}