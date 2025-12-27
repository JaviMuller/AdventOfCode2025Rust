use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");
    
    let mut splits = 0;
    let mut it = contents.lines().map(|x| x.trim());

    let mut beams : Vec<bool> = it.next()
        .unwrap()
        .bytes()
        .map(|x| if x == b'S' { true } else { false })
        .collect();

    // Quantum splitters will spawn two timelines, but a single ray in each timeline
    let mut q_beams : HashMap<usize, u64> = HashMap::from([
        (beams.iter().position(|x| x == &true).unwrap(), 1)
    ]);

    for line in it {
        let line = line.as_bytes();

        // Non-quantum behaviour
        let mut new_beams = vec![false; line.len()];
        for i in 0..line.len() {
            if beams[i] && line[i] == b'.' { new_beams[i] = true }
            else if beams[i] && line[i] == b'^' {
                new_beams[i - 1] = true;
                new_beams[i + 1] = true;
                splits += 1;
            }
        }
        beams = new_beams;

        // Quantum behaviour 
        let mut add_after = (0, 0);
        let mut keys : Vec<usize> = q_beams.keys()
            .cloned()
            .collect();
        keys.sort();

        for pos in keys {
            let n_beams = *q_beams.get(&pos).unwrap();
            let mut new_add_after = (0, 0);

            // Quantum splitting -> Update previous positions (safe)
            if line[pos] == b'^' {
                q_beams.entry(pos - 1)
                    .and_modify(|v| *v += n_beams)
                    .or_insert(n_beams);
                q_beams.remove(&pos);
                new_add_after = (pos + 1, n_beams);
            }

            // Process add_after
            if add_after.0 <= pos && add_after.1 > 0 {
                q_beams.entry(add_after.0)
                    .and_modify(|v| *v += add_after.1)
                    .or_insert(add_after.1);
            }

            // Update add_after
            add_after = new_add_after;
        }

        // Process last add_after
        if add_after.1 > 0 {
            q_beams.entry(add_after.0)
                .and_modify(|v| *v += add_after.1)
                .or_insert(add_after.1);
        }
    }

    let timelines = q_beams.values()
        .fold(0, |acc, x| acc + x);
    
    println!("(ANSWER 1) The beam is splitted {splits} times");
    println!("(ANSWER 2) With quantum splitters, there are {timelines} timelines");
}
