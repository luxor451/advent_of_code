#[macro_use]
extern crate scan_fmt;
use divisors_fixed::Divisors;
use std::fs;

fn check_pattern(pattern: &str, id: &str, id_len: usize, pattern_len: usize) -> bool {
    let mut res = false;
    for i in 1..(id_len / pattern_len) {
        if id[i * pattern_len..(i + 1) * pattern_len] != *pattern {
            res = true;
            break;
        }
    }
    return res;
}

fn is_id_valid(id: &str) -> bool {
    let id_len = id.len();
    let mut divisors = id_len.divisors();
    divisors.pop();
    let mut res: bool = true;
    for div in divisors {
        let pattern_to_check = &id[0..div];
        if !check_pattern(pattern_to_check, id, id_len, div) {
            res = false;
            break;
        }
    }
    return res;
}

fn get_bad_id_in_range((a, b): (u64, u64)) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];
    for i in a..b + 1 {
        let is_valid = is_id_valid(&i.to_string());
        if !is_valid {
            res.push(i);
        }
    }
    return res;
}

fn main() {
    let string = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let contents: Vec<&str> = string.split(",").collect();
    let mut vec_of_range: Vec<(u64, u64)> = Vec::with_capacity(contents.len());
    for elem in contents {
        let range = scan_fmt!(elem, "{d}-{d}", u64, u64).unwrap();
        vec_of_range.push(range);
    }
    let mut res: u64 = 0;
    for elem in vec_of_range {
        let bad_ids = get_bad_id_in_range(elem);
        res += bad_ids.iter().sum::<u64>();
    }
    println!("Result: {}", res);
}
