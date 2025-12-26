use std::fs;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");
    
    let mut sum_joltage_2_bat = 0;
    let mut sum_joltage_12_bat = 0;
    for line in contents.lines() {
        sum_joltage_2_bat += max_joltage(line, 2);
        sum_joltage_12_bat += max_joltage(line, 12);
    }

    println!("(ANSWER 1) The maximum joltage for two batteries is: {sum_joltage_2_bat}");
    println!("(ANSWER 2) The maximum joltage for twelve batteries is: {sum_joltage_12_bat}");
}

fn max_joltage(line : &str, batteries: usize) -> u64 {
    let line = line.trim_end();

    let mut start = 0;
    let mut dig_last;
    let mut joltage = 0;
    for i in (0..batteries).rev() {
        dig_last = 0;
        let mut new_pos = 0;
        for (pos, dig) in line[start..line.len() - i].chars().enumerate() {
            let dig = dig.to_digit(10).unwrap() as u64;
            if dig == 9 { (new_pos, dig_last) = (pos, dig); break }
            if dig > dig_last { (new_pos, dig_last) = (pos, dig) }
        }
        start = start + new_pos + 1;
        joltage = joltage * 10 + dig_last; 
    }
    joltage
}
