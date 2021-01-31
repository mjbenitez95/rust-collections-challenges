use rand::Rng;
use std::collections::HashMap;

const NUM_INTEGERS: u32 = 20;
const LIST_MIN: i32 = -100;
const LIST_MAX: i32 = 100;

fn create_random_list(num_integers: u32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<i32> = (0..num_integers)
        .map(|_| rng.gen_range(LIST_MIN, LIST_MAX + 1))
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
    let result = map
        .iter() // create an iteration through the map
        .max_by(|a, b| a.1.cmp(&b.1)) // find max by a.1 (where 1 is an index for value in (key,value)) compared with b.1
        .map(|(k, _v)| k); // map to (key, value) and return key

    // result will be an option where we always expect to have a Some. If none, error.
    match result {
        Some(num) => *num,
        None => panic!("Could not calculate a mode for the list of integers!"),
    }
}

fn calculate_median(numbers: &Vec<i32>) -> f32 {
    let num_elements = numbers.len();

    if num_elements % 2 == 0 {
        let middle = num_elements / 2;
        calculate_mean(&vec![numbers[middle - 1], numbers[middle]])
    } else {
        numbers[num_elements / 2] as f32
    }
}

const VOWELS: &str = "aeiou";

fn pig_latinize(word: &str) -> String {
    let (consonants, remains) = word.split_at(get_consonant_count(word));

    if consonants.is_empty() {
        format!("{}-hay", remains)
    } else {
        format!("{}-{}ay", remains, consonants)
    }
}

fn get_consonant_count(word: &str) -> usize {
    if word.is_empty() {
        return 0;
    }

    let mut count = 0;
    for i in 0..word.len() - 1 {
        let current = word.chars().nth(i).unwrap();
        let next = word.chars().nth(i + 1).unwrap();

        if VOWELS.contains(current) {
            return count;
        }
        if current == 'q' && next == 'u' {
            return count + 2;
        }
        count += 1;
        if next == 'y' {
            return count;
        }
    }

    return count;
}

fn main() {
    /*
        Given a list of integers, use a vector and return the mean (the average
        value), median (when sorted, the value in the middle position), and
        mode (the value that occurs most often; a hash map will be helpful
        here) of the list
    */
    let list_of_integers: Vec<i32> = create_random_list(NUM_INTEGERS);
    let mean = calculate_mean(&list_of_integers);
    let mode = calculate_mode(&list_of_integers);
    let median = calculate_median(&list_of_integers);

    println!("Problem 1:");
    println!("  {:?}.", list_of_integers);
    println!("  Mean: {}, Mode: {}, Median: {}.", mean, mode, median);

    println!("");

    /*
        Convert strings to pig latin. The first consonant of each word is moved
        to the end of the word and “ay” is added, so “first” becomes “irst-
        fay.” Words that start with a vowel have “hay” added to the end instead
        (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
        encoding!
    */
    let words_list: Vec<&str> = vec![
        "matthew", "went", "to", "the", "store", "and", "got", "a", "piece", "of",
        "candy", "then", "he", "went", "home", "to", "play", "some", "counter", "strike", "with",
        "his", "younger", "brother",
    ];
    for word in words_list {
        let pig_latinized = pig_latinize(&word);
        println!("The pig latin of {} is {}!", word, pig_latinized);
    }

    /*
        Using a hash map and vectors, create a text interface to allow a user
        to add employee names to a department in a company. For example, “Add
        Sally to Engineering” or “Add Amir to Sales.” Then let the user
        retrieve a list of all people in a department or all people in the
        company by department, sorted alphabetically.
    */
}
