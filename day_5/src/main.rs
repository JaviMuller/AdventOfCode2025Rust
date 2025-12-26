use std::fs;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let (ranges, ids) = contents.trim_end().split_once("\n\n").unwrap();
    let mut valid : Vec<(u64, u64)> = Vec::new();
    
    for range in ranges.lines() {
        let (low, high) = range.split_once('-').unwrap();
        let low : u64 = low.parse().unwrap();
        let high : u64 = high.parse().unwrap();
        valid.push((low, high));
    }
    let valid = refine_ranges(&mut valid);

    let mut fresh = 0;
    for id in ids.lines().map(|x| x.parse::<u64>().unwrap()) {
        for (low, high) in &valid {
            if id >= *low && id <= *high {
                fresh += 1;
                break;
            }
        }
    }

    let mut fresh_ids = 0;
    for (low, high) in &valid {
        fresh_ids += high - low + 1;
    }

    println!("(ANSWER 1) The number of fresh products is: {fresh}");
    println!("(ANSWER 2) The number of fresh product IDs is: {fresh_ids}");
}

fn refine_ranges(ranges : &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.as_mut_slice().sort_by_key(|(low, _)| *low);
    let mut refined = Vec::new();

    let mut new_l = 0;
    let mut new_h = 0;
    let mut first = true;
    for (low, high) in ranges {
        if first {
            (new_l, new_h) = (*low, *high);
            first = false;
            continue;
        }
        if *low <= new_h + 1 && *high > new_h { new_h = *high }
        else if *low > new_h + 1 {
            refined.push((new_l, new_h));
            (new_l, new_h) = (*low, *high);
        }
    }
    refined.push((new_l, new_h));
    refined
}

