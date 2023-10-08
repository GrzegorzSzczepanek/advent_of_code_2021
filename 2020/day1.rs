use std::fs;

fn main() {
    let file_contents = match fs::read_to_string("input.txt") {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };

    let passwords: Vec<String> = file_contents
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut valid: i32 = 0;

    for password in &passwords {
        let mut x = String::new();
        let mut y = String::new();
        let mut i: usize = 0;
        
        while let Some(c) = password.chars().nth(i) {
            if c == '-' {
                i += 1;
                break;
            }
            else {
                x.push(c);
            }
            i += 1;
        }
        
        while let Some(c) = password.chars().nth(i) {
            if c == ' ' {
                i += 1;
                break;
            }
            else {
                y.push(c);
            }
            i += 1;
        }
        let pass: &str = &password[(i+3)..password.len()]; 
        let num_x = x.parse::<i32>().unwrap();
        let num_y = y.parse::<i32>().unwrap(); 
        let required_character = password.chars().nth(i).unwrap();
        
        let mut matching: i32 = 0;
        for character in pass.chars() {
            // println!("character: {} | required_character: {}", character, required_character);
            if character == required_character {
                matching += 1;
            }
            
        }
        println!("Matching: {} | num_x: {} | num_y: {}", matching, num_x, num_y);
        if matching >= num_x && matching <= num_y{
                valid += 1;
            }
        // println!("{}", pass); 
        // println!("1: {};\n2: {};\n3: {};\n\n", x, y, required_character);
        
    }
    println!("{}", valid);
}
