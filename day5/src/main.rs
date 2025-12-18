use std::fs;
use std::time::Instant;
#[macro_use]
extern crate scan_fmt;

fn read_file(path: &str) -> (Vec<(usize, usize)>, Vec<u64>) {
    let string: String = fs::read_to_string(path).unwrap();
    let contents: Vec<&str> = string.split("\n\n").filter(|x| *x != "").collect();
    let range: Vec<(usize, usize)> = contents[0]
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| scan_fmt!(x, "{d}-{d}", usize, usize).unwrap())
        .collect();
    let items: Vec<u64> = contents[1]
        .split("\n")
        .filter(|x| *x != "")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    return (range, items);
}

fn is_id_in_this_range((a, b): (usize, usize), id: u64) -> bool {
    return (a as u64) <= id && id <= (b as u64);
}

fn is_id_in_one_range(ranges: &Vec<(usize, usize)>, id: u64) -> bool {
    let mut res = false;
    for range in ranges {
        if is_id_in_this_range(*range, id) {
            res = true;
            break;
        }
    }
    return res;
}

fn find_next_range_start(ranges_start: &Vec<usize>, b: usize) -> usize {
    for a in ranges_start {
        if b <= *a {
            return *a;
        }
    }
    return b;
}

fn part_one() {
    let (ranges, item_ids) = read_file("inputs.txt");

    let mut res: u32 = 0;
    for item in item_ids {
        if is_id_in_one_range(&ranges, item) {
            res += 1
        };
    }

    println!("Final res of part one : {:?}", res);
}

fn part_two() {
    let (ranges, _) = read_file("inputs.txt");

    let mut range_starts: Vec<usize> = ranges.iter().map(|(a, _)| *a).collect();
    range_starts.sort();
    let mut range_ends: Vec<usize> = ranges.iter().map(|(_, b)| *b).collect();
    range_ends.sort();

    let max = range_ends[range_ends.len() - 1];
    let min = range_starts[0];
    let mut res = (max - min) + 1;

    let mut next_a = 0;
    for b in range_ends {
        if b < next_a {
            continue;
        }
        if !is_id_in_one_range(&ranges, (b + 1) as u64) {
            next_a = find_next_range_start(&range_starts, b + 1);
            if next_a != b {
                res -= (next_a - b) - 1;
            };
        }
    }
    println!("Final res of part two : {:?}", res)
}

fn main() {
    let now = Instant::now();
    part_one();
    let elapsed = now.elapsed();
    println!("Part one done in : {:.2?}", elapsed);

    let now = Instant::now();
    part_two();
    let elapsed = now.elapsed();
    println!("Part two done in : {:.2?}", elapsed);
}
