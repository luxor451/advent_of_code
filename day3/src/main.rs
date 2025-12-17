use std::fs;
use std::time::Instant;

fn find_max_and_index(input: Vec<char>) -> (char, usize) {
    let mut max: u32 = input[0].to_digit(10).unwrap();
    let mut max_index: usize = 0;
    for i in 1..input.len() {
        let current_number = input[i].to_digit(10).unwrap();
        if max < current_number {
            max = current_number;
            max_index = i;
        }
    }
    return (input[max_index], max_index);
}


fn find_max_per_banks(nb_of_battery: usize, bank: Vec<char>) -> u64 {
    let mut last_index = 0;
    let mut char_vec_of_number: Vec<char> = Vec::with_capacity(nb_of_battery);
    for i in (0..nb_of_battery).rev() {
        let elem_to_shearch = &bank[last_index + 1..bank.len() - i];
        let (max, max_index) = find_max_and_index(elem_to_shearch.to_vec());
        last_index = max_index + last_index;
        char_vec_of_number.push(max);
    }
    let max_string = char_vec_of_number.into_iter().collect::<String>();
    let max_number = max_string.parse::<u64>();
    return max_number.unwrap();
}

fn main() {
    let now = Instant::now();
    let string = fs::read_to_string("input.txt").unwrap();
    let contents: Vec<&str> = string.split("\n").filter(|x| *x != "").collect();
    let contents_as_char = contents
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut res: Vec<u64> = Vec::new();

    for bank in contents_as_char {
        let max_nb = find_max_per_banks(12, bank.to_vec());
        res.push(max_nb);
    }
    println!("final res:\n{:?}", res.iter().sum::<u64>());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
