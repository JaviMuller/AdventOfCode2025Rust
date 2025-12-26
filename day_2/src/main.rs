use std::fs;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let mut ranges : Vec<(u64, u64)> = Vec::new();

    for range in contents.split(',') {
        match range.split_once('-') {
            None => panic!("Invalid range"),
            Some((low, high)) => {
                let low = low.parse().expect("Invalid range");
                let high = high.trim_end().parse().expect("Invalid range"); // Deal with EOF
                ranges.push((low, high))
            }
        }
    }

    let new_ranges = refine_ranges(&ranges);

    let res1 = add_bad_ids_1(&new_ranges);
    let res2 = add_bad_ids_2(&ranges);
    
    println!("(ANSWER 1) The sum of all bad IDs is: {res1}");
    println!("(ANSWER 2) The sum of all bad IDs is: {res2}");
}

fn refine_ranges(ranges : &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_ranges = Vec::new();
    for (low, high) in ranges {
        let (low_str, high_str) = (low.to_string(), high.to_string());
        let (low_len, high_len) = (low_str.len(), high_str.len());

        if low_len == high_len && low_len % 2 == 1 { continue }
        // #digits -> 10^(number of zeroes), e.g. 35087 ->(5 dig.) 100000 
        let new_low = if low_len % 2 == 1 { 10u64.pow(low_len as u32) } else { *low };
        // #digits -> 10^(number of zeroes - 1) - 1, e.g. 35087 -> 9999
        let new_high = if high_len % 2 == 1 { 10u64.pow(high_len as u32 - 1) - 1 } else { *high };
        if new_high < new_low { continue } else { new_ranges.push((new_low, new_high)) };
    }
    new_ranges
}

fn add_bad_ids_1(ranges : &Vec<(u64, u64)>) -> u64 {
    let mut res = 0;

    for (low, high) in ranges {
        let (low_str, high_str) = (low.to_string(), high.to_string());
        let (low_len, high_len) = (low_str.len(), high_str.len());
        
        let low_h : u64 = low_str[..(low_len/2)].parse().unwrap();
        let high_h : u64 = high_str[..(high_len/2)].parse().unwrap();
        for half in low_h..=high_h {
            let bad_n = half * 10u64.pow(half.ilog(10) + 1) + half;
            if bad_n >= *low && bad_n <= *high { 
                res += bad_n;
            }
        }
    }
    res
}

fn add_bad_ids_2(ranges : &Vec<(u64, u64)>) -> u64 {
    let mut res = 0;

    for (low, high) in ranges {
        for id in (*low)..=(*high) {
            if bad_id(id) { res += id }
        }
    }
    res
}

fn bad_id(id : u64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();

    for i in 1..=(id_len/2) {
        if id_len % i == 0 {
            let bad_id : u64 = id_str[..i].repeat(id_len/i).parse().unwrap();
            if bad_id == id { return true }
        }
    }
    return false;
}
