
pub fn run(input: &str) {

    let mut running_total: usize = 0;
    let mut running_total_2: usize = 0;

    let lines: Vec<&str> = input.lines().collect();

    for i in 0..lines.len() {
        for (j, _x) in lines[i].match_indices('X') {
            running_total += count_xmas(&lines, i, j) as usize;
        }
    }

    for i in 1..lines.len() - 1 {
        for (j, _x) in lines[i].match_indices('A') {
            if is_xmas(lines.iter().skip(i - 1).take(3).collect(), j) {
                running_total_2 += 1;
            }
        }
    }

    println!("{running_total}");
    println!("{running_total_2}");
}

fn count_xmas(lines: &Vec<&str>, i: usize, j: usize) -> u8 {
    let mut count_for_x: u8 = 0;

    let i_increasing_breaks = i + 3 >= lines.len();
    let i_decreasing_breaks = i.checked_sub(3) == None;
    let j_increasing_breaks = j + 3 >= lines[i].len();
    let j_decreasing_breaks = j.checked_sub(3) == None;

    // i=j+
    if !j_increasing_breaks
    && lines[i].chars().nth(j.saturating_add(1)).unwrap() == 'M'
    && lines[i].chars().nth(j.saturating_add(2)).unwrap() == 'A'
    && lines[i].chars().nth(j.saturating_add(3)).unwrap() == 'S' {
        
        count_for_x += 1;
    }

    // i=j-
    if !j_decreasing_breaks
    && lines[i].chars().nth(j.saturating_sub(1)).unwrap() == 'M'
    && lines[i].chars().nth(j.saturating_sub(2)).unwrap() == 'A'
    && lines[i].chars().nth(j.saturating_sub(3)).unwrap() == 'S' {
    
        count_for_x += 1;
    }

    // i-j=
    if !i_decreasing_breaks
    && lines[i.saturating_sub(1)].chars().nth(j).unwrap() == 'M'
    && lines[i.saturating_sub(2)].chars().nth(j).unwrap() == 'A'
    && lines[i.saturating_sub(3)].chars().nth(j).unwrap() == 'S' {
    
        count_for_x += 1;
    }

    // i+j=
    if !i_increasing_breaks
    && lines[i.saturating_add(1)].chars().nth(j).unwrap() == 'M'
    && lines[i.saturating_add(2)].chars().nth(j).unwrap() == 'A'
    && lines[i.saturating_add(3)].chars().nth(j).unwrap() == 'S' {
    
        count_for_x += 1;
    }

    // i-j-
    if !i_decreasing_breaks && !j_decreasing_breaks
    && lines[i.saturating_sub(1)].chars().nth(j.saturating_sub(1)).unwrap() == 'M'
    && lines[i.saturating_sub(2)].chars().nth(j.saturating_sub(2)).unwrap() == 'A'
    && lines[i.saturating_sub(3)].chars().nth(j.saturating_sub(3)).unwrap() == 'S' {
    
        count_for_x += 1;
    }
        
    // i-j+
    if !i_decreasing_breaks && !j_increasing_breaks
    && lines[i.saturating_sub(1)].chars().nth(j.saturating_add(1)).unwrap() == 'M'
    && lines[i.saturating_sub(2)].chars().nth(j.saturating_add(2)).unwrap() == 'A'
    && lines[i.saturating_sub(3)].chars().nth(j.saturating_add(3)).unwrap() == 'S' {
    
        count_for_x += 1;
    }

    // i+j-
    if !i_increasing_breaks && !j_decreasing_breaks
    && lines[i.saturating_add(1)].chars().nth(j.saturating_sub(1)).unwrap() == 'M'
    && lines[i.saturating_add(2)].chars().nth(j.saturating_sub(2)).unwrap() == 'A'
    && lines[i.saturating_add(3)].chars().nth(j.saturating_sub(3)).unwrap() == 'S' {
    
        count_for_x += 1;
    }
        
    // i+j+
    if !i_increasing_breaks && !j_increasing_breaks
    && lines[i.saturating_add(1)].chars().nth(j.saturating_add(1)).unwrap() == 'M'
    && lines[i.saturating_add(2)].chars().nth(j.saturating_add(2)).unwrap() == 'A'
    && lines[i.saturating_add(3)].chars().nth(j.saturating_add(3)).unwrap() == 'S' {
    
        count_for_x += 1;
    }

    count_for_x
}

fn is_xmas(lines: Vec<&&str>, j: usize) -> bool {
    if j == 0 || j == lines[0].len() - 1 {
        return false;
    }

    let top_left = lines[0].chars().nth(j - 1).unwrap();
    let top_right = lines[0].chars().nth(j + 1).unwrap();
    let bottom_left = lines[2].chars().nth(j - 1).unwrap();
    let bottom_right = lines[2].chars().nth(j + 1).unwrap();

    if (top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M')
        && (bottom_left == 'M' && top_right == 'S' || bottom_left == 'S' && top_right == 'M') {
        return true;
    }

    return false;
}