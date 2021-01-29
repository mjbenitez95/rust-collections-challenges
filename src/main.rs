use rand::Rng;
use std::collections::HashMap;

fn create_random_list(num_integers: u32) -> Vec<i32> {
    vec![0, 1, 2, 3, 4, 5]
}

fn calculate_mean(numbers: &Vec<i32>) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn main() {
    // given a list of integers, use a vector and return the mean, median, and mode
    let num_integers = 30;
    let list_of_integers: Vec<i32> = create_random_list(num_integers);
    let mean = calculate_mean(&list_of_integers);

    println!("The mean of {:?} is: {}!", list_of_integers, mean);
}
