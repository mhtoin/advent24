use std::collections::HashMap;

fn get_next_position(position: &str, direction: &str) -> Option<String> {
    let parts: Vec<usize> = position.split("-").map(|x| x.parse::<usize>().unwrap()).collect();
    let row = parts[0];
    let col = parts[1];
    match direction {
        // Check if row is 0 before subtracting
        "up" => if row > 0 { Some(format!("{}-{}", row - 1, col)) } else { None },
        "down" => Some(format!("{}-{}", row + 1, col)),
        // Check if col is 0 before subtracting
        "left" => if col > 0 { Some(format!("{}-{}", row, col - 1)) } else { None },
        "right" => Some(format!("{}-{}", row, col + 1)),
        _ => None,
    }
}

// puzzle 1
fn traverse_map(matrix: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut direction = "up";

    // find the row and row index of the first "^"
    let current_row = matrix.iter().position(|x| x.contains(&"^".to_string())).unwrap();
    let current_row_index = matrix[current_row].iter().position(|x| x == "^").unwrap();


    // each key is a string indicating the row and column 
    // the value is the character positioned in those coordinates
    
    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            map.entry(format!("{}-{}", i, j)).or_default().push(cell.clone());
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
        map.entry(current_position.clone()).or_default().push("x".to_string());
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
        map.entry(current_position.clone()).or_default().push("x".to_string());
        current_position = next_position;
        
    }
   }

   return map;
}

fn traverse_map_with_loop_detection(matrix: &Vec<Vec<String>>) -> HashMap<String, Vec<String>> {}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let matrix = input
        .split("\n")
        .map(|x| x.trim().chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    let mut direction = "up";

    // find the row and row index of the first "^"
    let starting_row = matrix.iter().position(|x| x.contains(&"^".to_string())).unwrap();
    let starting_row_index = matrix[starting_row].iter().position(|x| x == "^").unwrap();
    let traversed_map = traverse_map(&matrix);

   // count the number of x's in the map for puzzle 1 answer
   let number_of_xs = traversed_map.values().filter(|x| x.contains(&"x".to_string())).count();
   println!("{}", number_of_xs);

   // for puzzle 2, try inserting a new # in the positioned where the x is, one at a time
   // then traverse the map again and see if the guard gets stuck in a loop
   // loop happens if we hit a # again from the same position
   // thus, have a map where we store each # position and then when a # is hit, insert the direction we were traversing in

   // first, initialize a map of maps, where each key is a position and the value is a map of directions and whether they have been traversed
   let mut position_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

   for (key, value) in traversed_map.iter() {
    if value.contains(&"x".to_string()) && key != &format!("{}-{}", starting_row, starting_row_index) {
        // initialize the direction counts to 0
        position_map.entry(key.clone()).or_default().insert("up".to_string(), 0);
        position_map.entry(key.clone()).or_default().insert("down".to_string(), 0);
        position_map.entry(key.clone()).or_default().insert("left".to_string(), 0);
        position_map.entry(key.clone()).or_default().insert("right".to_string(), 0);

        // get a copy of the matrix, insert a # at the position of the current x
        let mut matrix_copy = matrix.clone();
        matrix_copy[key.split("-").next().unwrap().parse::<usize>().unwrap()][key.split("-").nth(1).unwrap().parse::<usize>().unwrap()] = "#".to_string();
        let traversed_map_copy = traverse_map(&matrix_copy);
        // check if the traversed map has a loop
        if traversed_map_copy.values().filter(|x| x.contains(&"x".to_string())).count() > 1 {
            println!("Loop detected at position: {}", key);
        }
    }
   }

   println!("{:?}", position_map);

 
}
