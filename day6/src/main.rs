use std::collections::HashMap;

fn get_next_position(position: &str, direction: &str) -> Option<String> {
    let parts: Vec<usize> = position
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let row = parts[0];
    let col = parts[1];
    match direction {
        // Check if row is 0 before subtracting
        "up" => {
            if row > 0 {
                Some(format!("{}-{}", row - 1, col))
            } else {
                None
            }
        }
        "down" => Some(format!("{}-{}", row + 1, col)),
        // Check if col is 0 before subtracting
        "left" => {
            if col > 0 {
                Some(format!("{}-{}", row, col - 1))
            } else {
                None
            }
        }
        "right" => Some(format!("{}-{}", row, col + 1)),
        _ => None,
    }
}

// puzzle 1
fn traverse_map(matrix: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut direction = "up";

    // find the row and row index of the first "^"
    let current_row = matrix
        .iter()
        .position(|x| x.contains(&"^".to_string()))
        .unwrap();
    let current_row_index = matrix[current_row].iter().position(|x| x == "^").unwrap();

    // each key is a string indicating the row and column
    // the value is the character positioned in those coordinates

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            map.entry(format!("{}-{}", i, j))
                .or_default()
                .push(cell.clone());
        }
    }

    let mut current_position = format!("{}-{}", current_row, current_row_index);
    let mut direction = "up";
    // start moving in starting direction
    // if up, check the cell above
    // if down, check the cell below
    // if left, check the cell to the left
    // if right, check the cell to the right

    while let Some(next_position) = get_next_position(&current_position, direction) {
        //println!("{}", next_position);
        if !map.contains_key(&next_position) {
            map.entry(current_position.clone()).or_default().pop();
            map.entry(current_position.clone())
                .or_default()
                .push("x".to_string());
            break;
        }

        if map.get(&next_position).unwrap().contains(&"#".to_string()) {
            //println!("{}", next_position);
            //println!("{:?}", map.get(&next_position).unwrap());

            direction = match direction {
                "up" => "right",
                "down" => "left",
                "left" => "up",
                "right" => "down",
                _ => "up",
            };
            // continue to the next iteration of the loop
            continue;
        } else {
            // once a position has been visited, replace the value with an x
            map.entry(current_position.clone()).or_default().pop();
            map.entry(current_position.clone())
                .or_default()
                .push("x".to_string());
            current_position = next_position;
        }
    }

    return map;
}

fn traverse_map_with_loop_detection(
    matrix: &Vec<Vec<String>>,
    position_map: &mut HashMap<String, HashMap<String, i32>>,
) -> bool {
    let mut map: HashMap<String, Vec<&str>> = HashMap::new();
    let current_row = matrix
        .iter()
        .position(|x| x.contains(&"^".to_string()))
        .expect("No starting point found");
    let current_row_index = matrix[current_row]
        .iter()
        .position(|x| x == "^")
        .expect("No starting point found");

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            map.entry(format!("{}-{}", i, j))
                .or_default()
                .push(cell);
        }
    }

    let mut current_position = format!("{}-{}", current_row, current_row_index);
    let directions = ["up", "right", "down", "left"];
    let mut direction_index = 0;

    while let Some(next_position) = get_next_position(&current_position, directions[direction_index]) {
        if !map.contains_key(&next_position) {
            map.entry(current_position.clone()).or_default().pop();
            map.entry(current_position.clone())
                .or_default()
                .push("x");
            return false;
        }

        if map.get(&next_position).unwrap().contains(&"#") {
            let current_direction_entry = position_map.entry(next_position.clone()).or_default();
            let current_count = current_direction_entry.get(directions[direction_index]).copied().unwrap_or(0);
            current_direction_entry.insert(directions[direction_index].to_string(), current_count + 1);

            if current_count >= 2 {
                return true;
            }

            direction_index = (direction_index + 1) % directions.len();
            continue;
        } else {
            map.entry(current_position.clone()).or_default().pop();
            map.entry(current_position.clone())
                .or_default()
                .push("x");
            current_position = next_position;
        }
    }

    false
}
fn main() {
    use std::time::Instant;
    
    let input = std::fs::read_to_string("input.txt").unwrap();
    let matrix = input
        .split("\n")
        .map(|x| {
            x.trim()
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    // find the row and row index of the first "^"
    let starting_row = matrix
        .iter()
        .position(|x| x.contains(&"^".to_string()))
        .unwrap();
    let starting_row_index = matrix[starting_row].iter().position(|x| x == "^").unwrap();
    let traversed_map = traverse_map(&matrix);

    // count the number of x's in the map for puzzle 1 answer
    let number_of_xs = traversed_map
        .values()
        .filter(|x| x.contains(&"x".to_string()))
        .count();
    println!("{}", number_of_xs);

    // for puzzle 2, try inserting a new # in the positioned where the x is, one at a time
    // then traverse the map again and see if the guard gets stuck in a loop
    // loop happens if we hit a # again from the same position
    // thus, have a map where we store each # position and then when a # is hit, insert the direction we were traversing in

    // first, initialize a map of maps, where each key is a position and the value is a map of directions and whether they have been traversed
    
    let now = Instant::now();
    let mut possible_loop_position_count = 0;
    for (key, value) in traversed_map.iter() {
        if value.contains(&"x".to_string())
            && key != &format!("{}-{}", starting_row, starting_row_index)
        {
            // Parse key once
            let key_parts: Vec<usize> = key.split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let row = key_parts[0];
            let col = key_parts[1];

            // Modify matrix in place
            let mut matrix_copy = matrix.clone();
            matrix_copy[row][col] = "#".to_string();

            // Initialize position_map only once
            let mut position_map: HashMap<String, HashMap<String, i32>> = HashMap::new();
            for (row, line) in matrix_copy.iter().enumerate() {
                for (index, cell) in line.iter().enumerate() {
                    if cell == "#" {
                        let pos = format!("{}-{}", row, index);
                        let inner_map = position_map.entry(pos).or_default();
                        inner_map.insert("up".to_string(), 0);
                        inner_map.insert("down".to_string(), 0);
                        inner_map.insert("left".to_string(), 0);
                        inner_map.insert("right".to_string(), 0);
                    }
                }
            }

            let loop_detected = traverse_map_with_loop_detection(&matrix_copy, &mut position_map);
            if loop_detected {
                possible_loop_position_count += 1;
                println!("Loop count: {}", possible_loop_position_count);
            }
        }
    }

    println!("Loop positions: {}", possible_loop_position_count);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
