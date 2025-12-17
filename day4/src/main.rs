use std::{fs, vec};
use std::time::Instant;

fn read_file(path: &str) -> Vec<Vec<char>> {
    let string: String = fs::read_to_string(path).unwrap();
    let contents: Vec<&str> = string.split("\n").filter(|x| *x != "").collect();
    let contents_as_char: Vec<Vec<char>> = contents
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    return contents_as_char;
}

fn get_neighbour(matrix_with_border: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<char> {
    let mut res: Vec<char> = Vec::with_capacity(8);

    res.push(matrix_with_border[i - 1][j + 1]);
    res.push(matrix_with_border[i - 1][j]);
    res.push(matrix_with_border[i - 1][j - 1]);

    res.push(matrix_with_border[i][j + 1]);
    res.push(matrix_with_border[i][j - 1]);

    res.push(matrix_with_border[i + 1][j + 1]);
    res.push(matrix_with_border[i + 1][j]);
    res.push(matrix_with_border[i + 1][j - 1]);

    res
}

fn is_roll_accesible(matrix_with_border: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let neighbour = get_neighbour(matrix_with_border, i, j);
    let mut count: u64 = 0;
    for i in 0..8 {
        if neighbour[i] == '@' {
            count += 1
        }
    }
    return count < 4;
}

fn add_border_to_matrix(matrix: &mut Vec<Vec<char>>, n: usize, m: usize) {
    let last_line: Vec<char> = vec!['.'; m + 2];
    let first_line: Vec<char> = vec!['.'; m + 2];
    for i in 0..n {
        matrix[i].insert(0, '.');
        matrix[i].push('.');
    }
    matrix.push(last_line);
    matrix.insert(0, first_line);
}

fn part_one() {
    let mut contents_as_char: Vec<Vec<char>> = read_file("input.txt");
    let n: usize = contents_as_char.len();
    let m: usize = contents_as_char[0].len();
    add_border_to_matrix(&mut contents_as_char, n, m);

    let mut final_res: u64 = 0;
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if contents_as_char[i][j] == '@' {
                if is_roll_accesible(&contents_as_char, i, j) {
                    final_res += 1;
                }
            }
        }
    }
    println!("final res part 1 = {:?}", final_res);
}

fn part_two() {
    let mut contents_as_char: Vec<Vec<char>> = read_file("input.txt");
    let n: usize = contents_as_char.len();
    let m: usize = contents_as_char[0].len();
    add_border_to_matrix(&mut contents_as_char, n, m);
    let mut final_res: u64 = 0;


    let mut removed_rolls: Vec<(usize, usize)>;
    let mut rolls_removed = 1;

    while rolls_removed != 0 {
        rolls_removed = 0;
        removed_rolls = vec![];
        for i    in 1..n + 1 {
            for j in 1..m + 1 {
                if contents_as_char[i][j] == '@' {
                    if is_roll_accesible(&contents_as_char, i, j) {
                        final_res += 1;
                        removed_rolls.push((i, j));
                        rolls_removed += 1;
                    }
                }
            }
        }   
        for (i,j) in removed_rolls {
            contents_as_char[i][j] = 'x';
        }
        
    }

    
    println!("final res part 2 = {:?}", final_res);
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
