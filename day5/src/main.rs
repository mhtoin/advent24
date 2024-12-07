use std::collections::{HashMap, HashSet, VecDeque};


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

fn sort_pages<'a>(rules: &'a Vec<Vec<&'a str>>, pages: &Vec<Vec<&'a str>>) -> usize {
    let mut incorrectly_sorted_pages = Vec::new();

    for page in pages.iter() {
        // Build adjacency list and in-degree count for numbers in this page
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        let page_numbers: HashSet<&&str> = page.iter().collect();

        // Initialize in-degree for all numbers in the page
        for &num in page_numbers.iter() {
            graph.entry(*num).or_default();
            in_degree.entry(*num).or_insert(0);
        }

        // Build the graph from rules that apply to this page
        for rule in rules {
            let left = rule[0];
            let right = rule[1];
            if page_numbers.contains(&left) && page_numbers.contains(&right) {
                graph.entry(left).or_default().push(right);
                *in_degree.entry(right).or_insert(0) += 1;
            }
        }

        // Find nodes with no incoming edges (in-degree = 0)
        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&node, _)| node)
            .collect();

        let mut sorted_result = Vec::new();
        let mut original_order = page.clone();

        // Perform topological sort
        while let Some(node) = queue.pop_front() {
            sorted_result.push(node);
            
            if let Some(neighbors) = graph.get(node) {
                for &next in neighbors {
                    *in_degree.get_mut(next).unwrap() -= 1;
                    if in_degree[next] == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }

        // Check if the page needs sorting
        if sorted_result.len() == page.len() && sorted_result != original_order {
            println!("Checking page: {:?}", page);
            incorrectly_sorted_pages.push(sorted_result);
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
    use std::time::Instant;
    let now = Instant::now();
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
    //let  sum_of_middle_pages = sum_middle_of_correct(&rules, &pages);
    let sorted_pages_sum = sort_pages(&rules, &pages);
    
    //println!("Sum of middle pages: {}", sum_of_middle_pages);
    
    println!("Sum of sorted pages: {:?}", sorted_pages_sum);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

