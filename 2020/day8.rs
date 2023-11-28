use std::fs::read_to_string;
// use std::str::FromStr;

fn count_acc(instructions: &Vec<String>) -> i32 {
    let mut acc: i32 = 0;

    let a: usize = instructions.len();
    let mut it: usize = 0;
    let mut indices = Vec::new();

    while it < a {
        let x: Vec<&str> = instructions[it].split(" ").collect();
        // println!("{}", x[1])
        if x[0] == "acc" {
            acc += x[1].parse::<i32>().unwrap();
            it += 1;
        } else if x[0] == "jmp" {
            // println!("{}", x[1]);
            if x[1].contains("-") {
                it -= x[1].parse::<i32>().unwrap().abs() as usize;
            } else {
                it += x[1].parse::<usize>().unwrap();
            }
        } else if x[0] == "nop" {
            it += 1;
        }
        if indices.contains(&it) {
            break;
        }
        indices.push(it);

    }

    return acc;
}

fn main() {
    let data = read_to_string("input.txt").unwrap();
    let instructions: Vec<String> = data.split("\n").map(|line| line.to_string()).collect();

    let part1 = count_acc(&instructions);

    println!("{part1}");
}
