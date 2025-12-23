use std::fs;
use std::time::Instant;

fn read_file(path: &str) -> Vec<Vec<String>> {
    let string: String = fs::read_to_string(path).unwrap();
    let content: Vec<String> = string
        .split("\n")
        .filter(|x| *x != "")
        .map(|s| s.to_string())
        .collect();
    let content_divided: Vec<Vec<String>> = content
        .iter()
        .map(|x| {
            x.split(" ")
                .map(|s| s.to_string())
                .filter(|x| *x != "")
                .collect()
        })
        .collect();
    return content_divided;
}

fn part_one() {
    let content: Vec<Vec<String>> = read_file("inputs.txt");
    let n: usize = content.len();
    let m: usize = content[0].len();
    let mut res: u64 = 0;
    for j in 0..m {
        let mut res_of_this_collumn: u64 = content[0][j].parse::<u64>().unwrap();
        for i in 1..(n - 1) {
            let a: u64 = content[i][j].parse::<u64>().unwrap();
            if content[n - 1][j] == "+" {
                res_of_this_collumn += a;
            }
            if content[n - 1][j] == "*" {
                res_of_this_collumn *= a;
            }
        }
        res += res_of_this_collumn;
    }
    println!("Final res of part one: {res}");
}

fn read_file_two(path: &str) -> Vec<Vec<char>> {
    let string: String = fs::read_to_string(path).unwrap();
    let content: Vec<Vec<char>> = string
        .split("\n")
        .filter(|x| *x != "")
        .map(|s| s.chars().collect())
        .collect();

    let mut content = content;
    for elem in &mut content {
        elem.push(' ');
    }
    return content;
}

fn get_number(content: &Vec<Vec<char>>, j: usize, n: usize) -> u64 {
    let mut current_nb_vec: Vec<char> = Vec::new();
    for i in 0..(n - 1) {
        current_nb_vec.push(content[i][j]);
    }

    let current_nb_vec: Vec<char> = current_nb_vec
        .into_iter()
        .filter(|x| *x != ' ')
        .collect::<Vec<char>>();

    if current_nb_vec == [] {
        return 0;
    }

    let current_number: u64 = current_nb_vec
        .iter()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    return current_number;
}

fn part_two() {
    let content: Vec<Vec<char>> = read_file_two("inputs.txt");
    let n = content.len();
    let m = content[0].len();

    let all_operande = content[n - 1]
        .iter()
        .filter(|x| **x != ' ')
        .collect::<Vec<&char>>();

    let mut res = 0;
    let mut current_operande_id = 0;

    let mut vec_of_collumn: Vec<u64> = Vec::with_capacity(4);

    for j in 0..m {
        let current_operande = all_operande[current_operande_id];
        let current_nb = get_number(&content, j, n);
        println!("{:?}", current_nb);
        if current_nb == 0 {
            println!("{:?}", vec_of_collumn);
            if *current_operande == '+' {
                res += vec_of_collumn.iter().sum::<u64>();
            }
            if *current_operande == '*' {
                res += vec_of_collumn.iter().product::<u64>();
            }

            vec_of_collumn = Vec::with_capacity(4);
            current_operande_id += 1;
        } else {
            vec_of_collumn.push(current_nb);
        }
    }

    println!("Final res of part two: {:?}", res);
}

fn main() {
    let now = Instant::now();
    part_one();
    let elapsed = now.elapsed();
    println!("Part one done in: {:.2?}", elapsed);

    let now = Instant::now();
    part_two();
    let elapsed = now.elapsed();
    println!("Part two done in: {:.2?}", elapsed);
}
