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
    return content;
}

fn get_max_lenght_of_number(operande_line: &Vec<char>) -> usize {
    let mut res = 1;
    while operande_line[res] == ' ' {
        res += 1;
    }
    return res - 1;
}

fn func(content: &Vec<Vec<char>>, j: usize, m: usize, n: usize, elem_index: usize) -> u64 {
    let mut current_nb_vec: Vec<char> = Vec::new();
    for i in 0..(n - 1) {
        current_nb_vec.push(content[i][j + elem_index * m]);
    }
    let current_nb_vec: Vec<char> = current_nb_vec
        .into_iter()
        .filter(|x| *x != ' ')
        .collect::<Vec<char>>();
    let current_number: u64 = current_nb_vec
        .iter()
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    return current_number;
}

fn part_two() {
    let content: Vec<Vec<char>> = read_file_two("inputs.txt");
    let n: usize = content.len();
    let max_nb_lenght = get_max_lenght_of_number(&content[n - 1]);
    let m: usize = content[0].len() / (max_nb_lenght + 1) + 1;
    let mut res: u64 = 0;
    for elem_index in 0..m {
        let mut res_collumn = func(&content, 0, m, n, elem_index);
        for j in 1..(m - 1) {
            let current_nb = func(&content, j, m, n, elem_index);
            if content[n - 1][elem_index * m] == '+' {
                res_collumn += current_nb;
            }
            if content[n - 1][elem_index * m] == '*' {
                res_collumn *= current_nb;
            }
        }
        res += res_collumn;
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
