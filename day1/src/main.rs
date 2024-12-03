use puzzle1::read_lines;

// Puzzle 1
fn calculate_distance(vec_left: &Vec<i32>, vec_right: &Vec<i32>) -> i32 {
    let mut distance = 0;
    
    for i in 0..vec_left.len() {
        distance += (vec_left[i] - vec_right[i]).abs();
    }

    return distance;
}

// Puzzle 2
fn calculate_similarity(vec_left: &Vec<i32>, vec_right: &Vec<i32>) -> usize {
    let mut similarity = 0;

    for num in vec_left {
        let occurrences = vec_right.iter().filter(|&x| x == num).count();
        similarity += occurrences * (*num as usize);
    }

    return similarity;
}

fn main() {    
    let mut vec_left: Vec<i32> = Vec::new();
    let mut vec_right: Vec<i32> = Vec::new();
    
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(location_ids) = line {
                let command_split = location_ids.split_whitespace().collect::<Vec<&str>>();
                let left_id = command_split[0].parse::<i32>().unwrap();
                let right_id = command_split[1].parse::<i32>().unwrap();
                vec_left.push(left_id);
                vec_right.push(right_id);
            }
        }
    }

    vec_left.sort();
    vec_right.sort();

    let distance = calculate_distance(&vec_left, &vec_right);
    let similarity = calculate_similarity(&vec_left, &vec_right);

    println!("Distance: {}", distance);
    println!("Similarity: {}", similarity);
}
