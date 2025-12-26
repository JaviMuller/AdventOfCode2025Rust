use std::fs;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let mut dial = 50;
    let mut zeroes = 0;
    let mut clicks = 0;

    //println!("Initial Dial | Instruction | Final Dial | Round Clicks | Total Clicks");
    for line in contents.lines() {
        let rotation : i32 = line[1..].parse().expect("Rotation must be an integer");

        let new_clicks;
        //print!("{dial:^13}|{line:^13}|");
        if line.starts_with("L") {
            (dial, new_clicks) = rotate_left(dial, rotation)
        } else if line.starts_with("R") {
            (dial, new_clicks) = rotate_right(dial, rotation)
        } else {
            panic!("Rotation direction must be R or L")
        }
        
        if dial == 0 { zeroes += 1 };
        clicks += new_clicks;
        //println!("{dial:^12}|{new_clicks:^14}|{clicks:^13}");
    };
    
    println!("(ANSWER 1) The password is: {zeroes}");
    println!("(ANSWER 2) Using method 0x434C49434B, the password is: {clicks}")
}

fn rotate_left(dial : i32, rotation : i32) -> (i32, i32) {
    let res = dial - rotation;
    let new_dial = if res > 0 { res }
            else if res % 100 == 0 { 0 }
            else { 100 + (res % 100) };
    let clicks = if res > 0 { 0 }
            else if dial == 0 { - (res / 100) }
            else { - (res / 100) + 1 }; // +1 click when dial goes positive -> zero
    (new_dial, clicks)
}

fn rotate_right(dial : i32, rotation : i32) -> (i32, i32) {
    let res = dial + rotation;
    (res % 100, res / 100)
}

