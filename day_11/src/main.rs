use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let mut map : HashMap<&str, usize> = HashMap::new();
    let mut device = 0;
    let mut connections : HashMap<usize, Vec<usize>> = HashMap::new();
    for line in contents.trim_end().lines() {
        let (input, outputs) = line.split_once(": ").unwrap();
        let input : usize = match map.get(&input) {
            Some(v) => *v,
            None => { device += 1; map.insert(input, device); device }
        };
        let outputs : Vec<usize> = outputs
            .split_whitespace()
            .map(|id| match map.get(&id) {
                Some(v) => *v,
                None => { if id == "out" { 0 }
                          else { device += 1; map.insert(id, device); device } }
            })
            .collect();
       connections.insert(input, outputs); 
    }
    device += 1;

    let (p_you, p_dac, p_fft, p_svr) = (
        *map.get("you").unwrap(),
        *map.get("dac").unwrap(),
        *map.get("fft").unwrap(),
        *map.get("svr").unwrap()
    );

    let mut paths_out = vec![-1; device];
    paths_out[0] = 1;
    let mut paths_dac = vec![-1; device];
    paths_dac[0] = 0;
    paths_dac[p_dac] = 1;
    let mut paths_fft = vec![-1; device];
    paths_fft[0] = 0;
    paths_fft[p_fft] = 1;

    let you_to_out = find_paths(p_you, &mut paths_out, &connections);

    println!("(ANSWER 1) The number of paths you -> out is: {you_to_out}");

    let svr_to_dac = find_paths(p_svr, &mut paths_dac, &connections);
    let dac_to_fft = find_paths(p_dac, &mut paths_fft, &connections);
    let fft_to_out = find_paths(p_fft, &mut paths_out, &connections);

    let svr_to_fft = find_paths(p_svr, &mut paths_fft, &connections);
    let fft_to_dac = find_paths(p_fft, &mut paths_dac, &connections);
    let dac_to_out = find_paths(p_dac, &mut paths_out, &connections);
    
    let svr_out = if dac_to_fft == 0 {
        svr_to_fft * fft_to_dac * dac_to_out
    } else {
        svr_to_dac * dac_to_fft * fft_to_out
    };

    println!("(ANSWER 2) The number of paths svr -> out passing through dac and fft is: {svr_out}");
}

fn find_paths(input : usize, partial_sol : &mut Vec<i64>, connections : &HashMap<usize, Vec<usize>>) -> u64 {
    // Not computed
    if partial_sol[input] == -1 { 
        partial_sol[input] = connections.get(&input)
            .unwrap()
            .iter()
            .map(|out| find_paths(*out, partial_sol, connections))
            .sum::<u64>() as i64
    }
    partial_sol[input] as u64
}
