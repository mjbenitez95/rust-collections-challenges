use rand::Rng;
use std::collections::HashMap;

const NUM_INTEGERS: u32 = 30;
const LIST_MIN: i32 = -10;
const LIST_MAX: i32 = 10;

fn create_random_list(num_integers: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<i32> = (0..num_integers)
        .map(|_| rng.gen_range(LIST_MIN, LIST_MAX))
        .collect();
    vals.sort();
    vals
}

fn calculate_mean(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn calculate_mode(numbers: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, u32> = HashMap::new();
    for num in numbers {
        let count = map.entry(*num).or_insert(0);
        *count += 1;
    }
    get_highest_value(&map)
}

fn get_highest_value(map: &HashMap<i32, u32>) -> i32 {
    let result = map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k);
    match result {
        Some(num) => *num,
        None => -1000,
    }
}

fn calculate_median(numbers: &Vec<i32>) -> f32 {
    2.5
}

fn main() {
    // given a list of integers, use a vector and return the mean, median, and mode
    let list_of_integers: Vec<i32> = create_random_list(NUM_INTEGERS);
    let mean = calculate_mean(&list_of_integers);
    let mode = calculate_mode(&list_of_integers);
    let median = calculate_median(&list_of_integers);

    println!("With {:?}, we have: ", list_of_integers);
    println!("  Mean: {}.", mean);
    println!("  Mode: {}.", mode);
    println!("  Median: {}.", median);
}
