use std::fs::read_to_string;

fn count_trees(right: usize, down: usize, arr: &Vec<Vec<char>>, arr_len: usize) -> i64{
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut result: i64 = 0;

    while y < arr_len {
        if arr[y][x] == '#' {
            result += 1;
        }
        x = (x + right) % arr[0].len();
        y += down;
    }
    return result;
} 

fn main() {
    let data = read_to_string("input.txt").unwrap();

    let lines: Vec<String> = data.lines().map(|line| line.to_string())
        .collect();

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    
    let grid_len: usize = grid.len();
    let mut result: i64 = 1;
    let slopes: Vec<[usize; 2]> = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    
    for [right, down] in slopes {
        let trees_on_slope: i64 = count_trees(right, down, &grid, grid_len);
        // println!("{trees_on_slope}");
        result *= trees_on_slope;
    } 
    
    println!("{result}");
}
