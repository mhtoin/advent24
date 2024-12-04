fn find_xmas_and_samx(matrix: Vec<Vec<String>>) -> Vec<String> {
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
                    println!("Found {} at row {} at range {:?}", word.join(""), i, j..j+WORD_LENGTH);
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
                    println!("Found {} at column {} at range {:?}", column, i, j..j+WORD_LENGTH);
                    found_words.push(column);
                }
            }

            // check diagonal to the right if possible
            if i + WORD_LENGTH <= matrix_height && j + WORD_LENGTH <= row_length {
                let diagonal: String = (0..WORD_LENGTH)  // Changed from i..i+WORD_LENGTH to 0..WORD_LENGTH
                    .map(|x| matrix[i+x][j+x].as_str())  // Changed to use i+x instead of x
                    .collect::<Vec<&str>>()
                    .join("");

                if SEARCHED_WORDS.contains(&diagonal.as_str()) {
                    println!("Found {} at diagonal to the right {} at range {:?}", diagonal, i, j..j+WORD_LENGTH);
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
                    println!("Found {} at diagonal to the left {} starting at ({}, {})", diagonal, i, i, j);
                    found_words.push(diagonal);
                }
            }
        }
    }

    found_words
}

fn find_xmas_in_x_shape(matrix: Vec<Vec<String>>) -> Vec<String> {
    let mut found_words = Vec::new();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let matrix = input
        .split("\n")
        .map(|x| x.trim().chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    
    let found_words = find_xmas_and_samx(matrix);
    
    println!("{:?}", found_words.len());
}
