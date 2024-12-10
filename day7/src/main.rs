use strum_macros::EnumIter;
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, EnumIter)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, a: i128, b: i128) -> i128 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => {
                let a_str = a.to_string();
                let b_str = b.to_string();
                let result = format!("{}{}", a_str, b_str);
                result.parse::<i128>().unwrap()
            }
        }
    }
}

// Puzzle 1
fn get_valid_values_add_multiply(test_value: &i128, operands: &Vec<i128>) -> i128 {
    let possible_positions = operands.len() - 1;
    let operations_count = 2_i128.pow(possible_positions as u32);

    // Try each possible combination of operations
    for i in 0..operations_count {
        let mut operators = Vec::new();
            
        // Convert the number to a sequence of operations
        // Each bit represents Add (0) or Multiply (1)
        for pos in 0..possible_positions {
            let operator = if (i & (1 << pos)) == 0 {
                Operator::Add
            } else {
                Operator::Multiply
                };
            operators.push(operator);
        }
            
        // Evaluate the expression
        let mut result = operands[0];
        for (idx, &operator) in operators.iter().enumerate() {
            result = operator.apply(result, operands[idx + 1]);

            if result == *test_value {
                //println!("Test value: {}, Result: {}, Operators: {:?}", test_value, result, operators);
                return *test_value;
            }
        }
    }
    return 0;
}

fn get_valid_values_concatenate(test_value: &i128, operands: &Vec<i128>) -> i128 {
    let possible_positions = operands.len() - 1;
    let operations_count = 3_i128.pow(possible_positions as u32);

    // Try each possible combination of operations
    for i in 0..operations_count {
        let mut operators = Vec::new();
        
        for pos in 0..possible_positions {
            let operator = match (i / 3_i128.pow(pos as u32)) % 3 {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concatenate,
                _ => unreachable!(),
            };
            operators.push(operator);
        }

        // Evaluate the expression
        let mut result = operands[0];
        for (idx, &operator) in operators.iter().enumerate() {
            result = operator.apply(result, operands[idx + 1]);

            if result == *test_value {
            //println!("Test value: {}, Result: {}, Operators: {:?}", test_value, result, operators);
                return *test_value;     
            }
        }

        
    }
    0  // Return 0 if no match found
}

fn main() {
    use std::time::Instant;
    let start = Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut sum_of_possible_values = 0;
    let mut sum_of_possible_values_concatenate = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let parts = line.split(": ").collect::<Vec<&str>>();
        let test_value = parts[0].parse::<i128>().unwrap();
        let operands = parts[1].split(" ").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();
        
        
        let result = get_valid_values_add_multiply(&test_value, &operands);
        if result != 0 {
            sum_of_possible_values += result;
        }

        let result = get_valid_values_concatenate(&test_value, &operands);
        if result != 0 {
            sum_of_possible_values_concatenate += result;
        }
    }

    println!("Sum of possible values: {}", sum_of_possible_values);
    println!("Sum of possible values concatenate: {}", sum_of_possible_values_concatenate);
    println!("Time taken: {:?}", start.elapsed().as_secs_f32());
}