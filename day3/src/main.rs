use regex::Regex;

// puzzle 1
fn scan_mul_values(input: &str) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut mul_values: Vec<(i32, i32)> = Vec::new();
    for mat in pattern.captures_iter(input) {
        let x = mat[1].parse::<i32>().unwrap();
        let y = mat[2].parse::<i32>().unwrap();
        mul_values.push((x, y));
    }
    return mul_values;
}

fn scan_enabled_instructions(input: &str) -> Vec<(i32, i32)> {
    // match all mul(x,y), don't() and do()
    let pattern = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let mut enabled_instructions = Vec::new();
    for mat in pattern.find_iter(input) {
        enabled_instructions.push(mat.as_str());
    }

    let mut valid_instructions = Vec::new();
    let mut is_valid = true;
    // only add instructions that are valid -> do() enables, don't() disables
    for instruction in enabled_instructions {
        if instruction == "don't()" {
            is_valid = false;
        } else if instruction == "do()" {
            is_valid = true;
        }
        if is_valid {
            valid_instructions.push(instruction);
        }
    }

    // convert to string and scan for mul(x,y)
    let valid_instruction_string = valid_instructions.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    return scan_mul_values(&valid_instruction_string);
    
}



fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    // Puzzle 1
    let mul_values = scan_mul_values(&input);
        
    println!("{:?}", mul_values.iter().map(|(x, y)| x * y).sum::<i32>());

    // Puzzle 2
    let enabled_instructions = scan_enabled_instructions(&input);
    println!("{:?}", enabled_instructions.iter().map(|(x, y)| x * y).sum::<i32>());

}
