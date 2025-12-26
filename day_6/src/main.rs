use std::fs;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");
    
    let mut it = contents.lines().rev();
    let symbols : Vec<&str> = it.next()
        .unwrap()
        .split_whitespace()
        .collect();

    let lines : Vec<&str> = it.rev().collect();

    let res1 = problem1(&symbols, &lines);
    let ans1 = res1.iter().fold(0, |acc, x| acc + x);

    let res2 = problem2(&symbols, &lines);
    let ans2 = res2.iter().fold(0, |acc, x| acc + x);

    println!("(ANSWER 1) The final answer is: {ans1}");
    println!("(ANSWER 2) The final (correct) answer is {ans2}");
}

fn problem1(symbols : &Vec<&str>, numbers : &Vec<&str>) -> Vec<u64> {
    let mut it = numbers.iter();
    let mut res : Vec<u64> = it.next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    for line in it {
        let values : Vec<u64> = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        for i in 0..res.len() {
            match symbols[i] {
                "*" => res[i] *= values[i],
                "+" => res[i] += values[i],
                _ => panic!("Operator must be + or *"),
            }
        }
    }

    res
}

fn problem2(symbols : &Vec<&str>, lines : &Vec<&str>) -> Vec<u64> {
    let mut symbol_it = symbols.iter();
    let mut res : Vec<u64> = Vec::new(); // Global result array 
    let mut res_pr = 0;    // Result for the current problem
    let mut new_pr = true; // Starting a new problem?
    let mut symbol = "";   // Symbol for the current problem
    for i in 0..lines[0].len() {
        let mut cur = 0;       // Current number
        let mut digit = false; // Has there been a digit in this column? No -> end problem

        for line in lines { // Get the number (vertical)
            match line.chars().nth(i).unwrap().to_digit(10) {
                Some(n) => {
                    if !digit { cur = n as u64; digit = true }
                    else { cur = cur * 10 + n as u64 }
                },
                None => {},
            }
        }

        
        if new_pr { // If new problem, start problem result
            res_pr = cur;
            new_pr = false;
            symbol = *symbol_it.next().unwrap();
            continue
        } else if digit { // If not new problem and there is a number, calculate new result
            match symbol {
                "*" => res_pr = res_pr * cur,
                "+" => res_pr = res_pr + cur,
                 _  => panic!("Operator must be + or *"),
            }
        } else { // If there is no number, push the current result and start a new problem
            res.push(res_pr);
            new_pr = true;
        }
    }
    res.push(res_pr);
    res
}
