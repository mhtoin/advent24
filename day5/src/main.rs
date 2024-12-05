// puzzle 1
fn sum_middle_of_correct<'a>(rules: &Vec<Vec<&str>>, pages: &Vec<Vec<&'a str>>) -> usize {
    let mut sorted_pages = Vec::new();

    for (_i, page) in pages.iter().enumerate() {
       if check_if_sorted(&rules, &page) {
        sorted_pages.push(page.to_vec());
       }
    }

    sorted_pages
    .iter()
    .map(|page| page[page.len() / 2]
    .parse::<usize>()
    .unwrap())
    .sum::<usize>()
}

fn check_if_sorted(rules: &Vec<Vec<&str>>, page: &Vec<&str>) -> bool {
    for rule in rules.iter() {
        let left_part_index = rule[0].parse::<usize>().unwrap();
        let right_part_index = rule[1].parse::<usize>().unwrap();

        // search the page for the left part index
        let left_part = page.iter().position(|&x| x == left_part_index.to_string());
        let right_part = page.iter().position(|&x| x == right_part_index.to_string());

        if left_part.is_some() && right_part.is_some() {
             if left_part.unwrap() >= right_part.unwrap() {
                return false;
            }
        }
    }

    true
}

fn sort_pages<'a>(rules: &Vec<Vec<&str>>, pages: &Vec<Vec<&'a str>>) -> usize {
    let mut incorrectly_sorted_pages = Vec::new();

    for page in pages.iter() {
        let mut sorted_page = page.to_vec();
        let mut is_incorrectly_sorted = false;
        
        // Keep checking until we make a complete pass with no swaps needed
        'outer: loop {
            let mut made_swap = false;
            
            for rule in rules.iter() {
                let left_part_index = rule[0].parse::<usize>().unwrap();
                let right_part_index = rule[1].parse::<usize>().unwrap();

                let left_part = sorted_page.iter().position(|&x| x == left_part_index.to_string());
                let right_part = sorted_page.iter().position(|&x| x == right_part_index.to_string());

                if left_part.is_some() && right_part.is_some() {
                    if left_part.unwrap() >= right_part.unwrap() {
                        is_incorrectly_sorted = true;
                        made_swap = true;
                        let temp = sorted_page[left_part.unwrap()];
                        sorted_page[left_part.unwrap()] = sorted_page[right_part.unwrap()];
                        sorted_page[right_part.unwrap()] = temp;
                    }
                }
            }
            
            // If we made no swaps in this pass, the page is fully sorted
            if !made_swap {
                break 'outer;
            }
        }

        if is_incorrectly_sorted {
            incorrectly_sorted_pages.push(sorted_page);
        }
    }

    incorrectly_sorted_pages
    .iter()
    .map(|page| page[page.len() / 2]
    .parse::<usize>()
    .unwrap())
    .sum::<usize>()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = parts[0]
        .split("\n")
        .map(|rule| rule.split("|").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let pages = parts[1]
        .split("\n")
        .map(|page| page.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let  sum_of_middle_pages = sum_middle_of_correct(&rules, &pages);
    let sorted_pages_sum = sort_pages(&rules, &pages);
    
    println!("Sum of middle pages: {}", sum_of_middle_pages);
    
    println!("Sum of sorted pages: {}", sorted_pages_sum);
}

