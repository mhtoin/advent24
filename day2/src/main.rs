use day2::read_lines;


// Returns the number of safe levels
// Puzzle 1
fn calculate_safe_levels() -> usize {
    let mut safe_levels: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(levels) = line {
                let levels_int = levels.split_whitespace().collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                
               let (_level, safe_level) = check_level(&levels_int);
               if safe_level {
                safe_levels.push(levels_int);
               }
            }
        }
    }

    return safe_levels.len();
}

fn check_level(levels: &Vec<i32>) -> (i32, bool) {
    let mut decreasing: bool = false;
    let mut increasing: bool = false;
    
    for i in 0..levels.len() {
        let current_level = levels[i];
        let next_level = if i + 1 < levels.len() { levels[i + 1] } else { -1 };

        let is_safe = check_level_safety(current_level, next_level, &mut decreasing, &mut increasing);
        if !is_safe {
            //println!("Not safe: {:?}", levels);
            return (current_level, false);
        }
    }

    return (0, true);
}

fn check_level_safety(current_level: i32, next_level: i32, decreasing: &mut bool, increasing: &mut bool) -> bool {
    //println!("Checking level safety: {:?} {:?}, {:?} {:?}", current_level, next_level, decreasing, increasing);
    
    if current_level > next_level {
        *decreasing = true;
    } else if current_level < next_level {
        *increasing = true;
    }

    if next_level == -1 {
        return true;
    }

    let diff = (current_level - next_level).abs();

    if (*decreasing && *increasing) || diff < 1 || diff > 3 {
        //println!("Conditions not safe: {:?} {:?}, {:?} {:?}, {:?}", current_level, next_level, decreasing, increasing, diff);
        return false;
    }

    return true;
}

// Puzzle 2
fn calculate_safe_levels_puzzle_2() -> usize {
    let mut safe_levels: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(levels) = line {
                let mut levels_int = levels.split_whitespace().collect::<Vec<&str>>().iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                let (level, safe_level) = check_level(&levels_int);

                if !safe_level {
                    // Find the index of the bad level instead of using the level value directly
                    let unsafe_index = levels_int.iter().position(|&x| x == level);
                    if let Some(unsafe_index) = unsafe_index {
                        let mut unsafe_removed = levels_int.clone();
                        unsafe_removed.remove(unsafe_index);

                        let (_level, safe_level) = check_level(&unsafe_removed);
                        if safe_level {
                            println!("Safe: {:?}", levels_int);
                            safe_levels.push(levels_int);
                        } else {
                            // try removing each index and see if level can be made safe
                            for i in 0..levels_int.len() {
                                let mut unsafe_removed = levels_int.clone();
                                unsafe_removed.remove(i);
                                let (_level, safe_level) = check_level(&unsafe_removed);
                                if safe_level {
                                    println!("Safe: {:?}", unsafe_removed);
                                    safe_levels.push(unsafe_removed);
                                }
                            }
                        }
                    }
                } else {
                    safe_levels.push(levels_int);
                }
            }
        }
    }

    return safe_levels.len();
}

fn main() {
    println!("Safe levels: {}", calculate_safe_levels());
    println!("Safe levels puzzle 2: {}", calculate_safe_levels_puzzle_2());
}