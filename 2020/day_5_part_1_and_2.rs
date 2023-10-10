use std::fs::read_to_string;

fn part_1(data: &Vec<String>) -> i32 {
   
    let mut longest: i32 = 0;   
    for ticket in data {
        let mut i: usize = 0;
        let mut upper_row: i32 = 127;
        let mut bottom_row: i32 = 0;
        let mut upper_column: i32 = 7;
        let mut lower_column: i32 = 0;
        
        while i < ticket.len() {
            if ticket.chars().nth(i).unwrap()  == 'F' {
                upper_row = (upper_row + bottom_row) / 2;
            }
            else if ticket.chars().nth(i).unwrap()  == 'B' {
                bottom_row = (bottom_row + upper_row) / 2 + 1;
            }
            else if ticket.chars().nth(i).unwrap() == 'R' {
                lower_column = (upper_column + lower_column) / 2 + 1;
            }
            else if ticket.chars().nth(i).unwrap() == 'L' {
                upper_column = (upper_column + lower_column) / 2;
            }
            i += 1;
        }
        if longest < upper_row * 8 + upper_column {
            longest = upper_row * 8 + upper_column;
        }
       
    }
    longest
}

fn part_2( data: &Vec<String> ) -> i16 {
    let mut ids: Vec<i16> = Vec::new();    

    for ticket in data {
        let mut i: usize = 0;
        let mut upper_row: i16 = 127;
        let mut bottom_row: i16 = 0;
        let mut upper_column: i16 = 7;
        let mut lower_column: i16 = 0;
                while i < ticket.len() {
            // println!("{}", ticket.chars().nth(i).unwrap());
            if ticket.chars().nth(i).unwrap()  == 'F' {
                upper_row = (upper_row + bottom_row) / 2;
                // println!("{upper_row}");
            }
            else if ticket.chars().nth(i).unwrap()  == 'B' {
                bottom_row = (bottom_row + upper_row) / 2 + 1;
                // println!("{bottom_row}");
            }
            else if ticket.chars().nth(i).unwrap() == 'R' {
                lower_column = (upper_column + lower_column) / 2 + 1;
                // println!("{lower_column}");
            }
            else if ticket.chars().nth(i).unwrap() == 'L' {
                upper_column = (upper_column + lower_column) / 2;
                // println!("up : {upper_column}");
            }
            i += 1;
        }
    ids.push(upper_row * 8 + upper_column)
    }
    ids.sort();
    let mut index: usize = 128;
    while index < ids.len() - 127 {
        if ids[index] != ids[index - 1] + 1 {
            // println!("");
            return ids[index] - 1;
        }

        index += 1;
    }
   
    0
}
    

    fn main() {
        // println!("{}", 127 / 2);
        let data: Vec<String> = read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect();
        
        println!("{}", part_1(&data));

        // part_2(&data);
        println!("{}", part_2(&data)); 
    }
