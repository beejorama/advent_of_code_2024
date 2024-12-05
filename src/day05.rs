use itertools::Itertools;
use regex::Regex;

pub fn run(input: &str) {
    // regex to identify rules lines
    let rule_regex = Regex::new(r"\d+\|\d+").unwrap();

    // partition the input lines into 2 categories - rules and updates
    let (rules, updates): (Vec<_>, Vec<_>) = input.lines()
    .into_iter()
    .partition(|line| rule_regex.is_match(line));

    // parse the rules into tuples
    let rules: Vec<(u8, u8)> = rules
    .into_iter()
    .map(|r| 
        r.split("|")
        .map(|n| n.parse::<u8>().unwrap())
        .collect_tuple().unwrap())
    .collect_vec();

    // parse the updates into u8s
    let updates: Vec<Vec<u8>> = updates
    .into_iter()
    .skip(1)
    .map(|r| 
        r.split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect_vec())
    .collect_vec();

    let mut running_total: usize = 0;
    let mut running_total_2: usize = 0;

    for mut update in updates {
        
        // evaluate each rule but if one fails, continue
        let mut is_valid = true;

        for rule in &rules {
            // skip over if it doesn't contain both values in the rule
            if !is_rule_relevant(&update, rule) {
                continue;
            }

            // otherwise evaluate the rule
            if !is_rule_met(&update, rule) {
                is_valid = false;
                break;
            }
        }

        if is_valid {
            // get middle page value and add to running_total
            running_total += middle_page_value(&update);
            continue; // continue so we don't count ones we don't want in part 2
        }

        // part 2
        let mut loop_count: usize = 0;
        loop {
            let mut all_rules_pass = true;
            loop_count += 1;
            println!("{loop_count}");

            for rule in &rules {
                // skip over if it doesn't contain both values in the rule
                if !is_rule_relevant(&update, rule) {
                    continue;
                }

                // otherwise evaluate the rule
                if !is_rule_met(&update, rule) {
                    reorder_update(&mut update, rule);
                    all_rules_pass = false;
                }
            }

            if all_rules_pass {
                break;
            }
        }

        running_total_2 += middle_page_value(&update);
    }

    println!("{running_total}");
    println!("{running_total_2}");
}

fn reorder_update(update: &mut Vec<u8>, rule: &(u8, u8)) {    
    // move rule.1 just after rule.0
    let position_1 = update.iter().position(|&c| c == rule.1).unwrap();
    update.remove(position_1);

    let position_0 = update.iter().position(|&c| c == rule.0).unwrap();
    update.insert(position_0 + 1, rule.1);
}

fn is_rule_relevant(update: &Vec<u8>, rule: &(u8, u8)) -> bool {
    return update.contains(&rule.0) && update.contains(&rule.1)
}

fn is_rule_met(update: &Vec<u8>, rule: &(u8, u8)) -> bool {
    if update.iter().position(|val| val == &rule.0).unwrap() > update.iter().position(|val| val == &rule.1).unwrap() {
        return false;
    }
    true
}

fn middle_page_value(update: &Vec<u8>) -> usize {
    let middle = update.len() / 2;
    return update.iter().nth(middle).unwrap().clone() as usize;
}