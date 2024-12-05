fn find_xmas_and_samx(matrix: &Vec<Vec<String>>) -> Vec<String> {
    const SEARCHED_WORDS: [&str; 2] = ["XMAS", "SAMX"];
    const WORD_LENGTH: usize = 4;
    let matrix_height = matrix.len();
    let mut found_words = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let row_length = row.len();
        // we need to check WORD_LENGTH in every possible directions - row, column, diagonal
        for j in 0..row_length {
            // check row
            if j + WORD_LENGTH <= row_length {  // Add this check
                let word = row[j..j+WORD_LENGTH].to_vec();
                if SEARCHED_WORDS.contains(&word.join("").as_str()) {
                    found_words.push(word.join(""));
                }
            }

            // check column [i..WORD_LENGTH][j]
            if i + WORD_LENGTH <= matrix_height {
                let column: String = (i..i+WORD_LENGTH)
                    .map(|x| matrix[x][j].as_str())
                    .collect::<Vec<&str>>()
                    .join("");
                if SEARCHED_WORDS.contains(&column.as_str()) {
                    found_words.push(column);
                }
            }

            // check diagonal to the right if possible
            if i + WORD_LENGTH <= matrix_height && j + WORD_LENGTH <= row_length {
                let diagonal: String = (0..WORD_LENGTH)
                    .map(|x| matrix[i+x][j+x].as_str())
                    .collect::<Vec<&str>>()
                    .join("");

                if SEARCHED_WORDS.contains(&diagonal.as_str()) {
                    found_words.push(diagonal);
                }
            }

            // check diagonal to the left if possible
            if i + WORD_LENGTH <= matrix_height && j >= (WORD_LENGTH - 1) {
                let diagonal: String = (0..WORD_LENGTH)
                    .map(|x| matrix[i + x][j - x].as_str())
                    .collect::<Vec<&str>>()
                    .join("");

                if SEARCHED_WORDS.contains(&diagonal.as_str()) {
                    found_words.push(diagonal);
                }
            }
        }
    }

    found_words
}

fn check_diagonals(matrix: &Vec<Vec<String>>, i: usize, j: usize) -> bool {
    let matrix_height = matrix.len();
    let row_length = matrix[0].len();
    let mut found_count = 0;
    const WORD_LENGTH: usize = 3;
    
    if i + WORD_LENGTH <= matrix_height && j + WORD_LENGTH <= row_length {
        let diagonal: String = (0..WORD_LENGTH)
            .map(|x| matrix[i+x][j+x].as_str())
            .collect::<Vec<&str>>()
            .join("");
        
        if ["MAS", "SAM"].contains(&diagonal.as_str()) {
            found_count += 1;
        }
    } 

    let left_starting_point = j + 2;

    if i + WORD_LENGTH <= matrix_height && j + 2 < row_length {
        let diagonal: String = (0..WORD_LENGTH)
            .map(|x| matrix[i + x][left_starting_point - x].as_str())
            .collect::<Vec<&str>>()
            .join("");
        
        if ["MAS", "SAM"].contains(&diagonal.as_str()) {
            found_count += 1;
        }
    } 

    found_count == 2
}

fn find_xmas_in_x_shape(matrix: &Vec<Vec<String>>) -> usize {
    let mut found_amount = 0;

    // iterate over the matrix, sliding a window in the shape of X
    // check if the window contains the searched words

    for (i, row) in matrix.iter().enumerate() {
        let row_length = row.len();
        for j in 0..row_length {
            if check_diagonals(matrix, i, j) {
                found_amount += 1;
            }
        }
    }

    found_amount
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let matrix = input
        .split("\n")
        .map(|x| x.trim().chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    
    let found_words = find_xmas_and_samx(&matrix);
    let found_x_shape = find_xmas_in_x_shape(&matrix);
    
    println!("{:?}", found_words.len());
    println!("{:?}", found_x_shape);
}
