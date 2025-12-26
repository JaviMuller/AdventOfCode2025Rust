use std::fs;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let mut grid : Vec<Vec<u64>> = Vec::new();
    for line in contents.lines() {
        grid.push(parse_line(line));
    }

    let mut new = accessible(&mut grid);
    let res1 = new;
    let mut res2 = 0;
    while new > 0 {
        res2 += new;
        new = accessible(&mut grid);
    }
    println!("(ANSWER 1) The number of accessible paper rolls is: {res1}");
    println!("(ANSWER 2) The total number of accessible paper rolls is: {res2}");
}

fn parse_line(line : &str) -> Vec<u64> {
    let line = line.trim();
    let len = line.len();
    let mut grid_line = vec![0; len];
    for (pos, char) in line.chars().enumerate() {
        if char == '@' { grid_line[pos] = 1 }
    }
    grid_line
}

fn accessible(grid : &mut Vec<Vec<u64>>) -> u64 {
    let mut accessible = 0;
    let mut remove : Vec<(usize, usize)> = Vec::new();
    for line in 0..grid.len() {
        for col in 0..grid[line].len() {
            if grid[line][col] == 0 { print!("."); continue }
            let mut adjacent = 0;
            // Top left
            if line > 0 && col > 0 { adjacent += grid[line-1][col-1] }
            // Top
            if line > 0 { adjacent += grid[line-1][col] }
            // Top right
            if line > 0 && col < grid[line].len() - 1 { adjacent += grid[line-1][col+1] }
            // Left
            if col > 0 { adjacent += grid[line][col-1] }
            // Right
            if col < grid[line].len() - 1 { adjacent += grid[line][col+1] }
            // Bot left
            if line < grid.len() - 1 && col > 0 { adjacent += grid[line+1][col-1] }
            // Bot
            if line < grid.len() - 1 { adjacent += grid[line+1][col] }
            // Bot right
            if line < grid.len() - 1 && col < grid[line].len() - 1 { adjacent += grid[line+1][col+1] }
            
            if adjacent < 4 {
                accessible += 1;
                remove.push((line, col));
            }
        }
    }
    // For the correctness of the first part, it cannot be done as soon as detected to be removed
    for (line, col) in remove {
        grid[line][col] = 0;
    }
    accessible
}
