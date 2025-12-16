use std::fs;


fn move_dial(direction : char, steps : i32, previous_rot : i32) -> (i32, i32) {
    let mut new_rotation = previous_rot;
    match direction {
        'L' => {new_rotation -= steps},
        'R' => {new_rotation += steps},
        _ => {()},
    };

    let as_crossed_zero;

    if new_rotation == 0 {
        as_crossed_zero = 1;
    } else if new_rotation < 0 {
        let mut add= 1;
        if previous_rot == 0 {
            add = 0;
        } 
        as_crossed_zero = add + (i32::abs(new_rotation / 100));
        new_rotation = (100 + (new_rotation % 100)) % 100
    } else {
        as_crossed_zero = i32::abs(new_rotation / 100);
        new_rotation = new_rotation % 100
    }


    return (new_rotation, as_crossed_zero);
}

fn main() {
    let first_rot : i32 = 50;
    let result_from_file: String = fs::read_to_string("input.txt").unwrap();

    let contents: Vec<&str> = result_from_file.split("\n").filter(|x| *x != "").collect::<Vec<&str>>();
    let mut answer: u32 = 0;
    let mut old_rot: i32 = first_rot;

    for elem in contents {
        let direction = elem.chars().next().unwrap();
        let steps: i32 = elem[1..].trim().parse().unwrap();
        let new_direction = move_dial(direction, steps, old_rot);
        let new_rot = new_direction.0;
        let as_crossed_zero = new_direction.1;
        answer += as_crossed_zero as u32;
        
        old_rot = new_rot;
        println!("current pose = {old_rot}")
    }

    println!("{answer}");
}