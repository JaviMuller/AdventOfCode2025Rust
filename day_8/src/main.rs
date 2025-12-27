use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

const PROB_1_ITER : usize = 1000;

struct UnionFind {
    n_elem : usize,
    parent : Vec<usize>,
    rank   : Vec<usize>,
    s_size : Vec<usize>,
}

impl UnionFind {
    fn new(n_elem : usize) -> Self {
        let mut parent = Vec::with_capacity(n_elem);
        for i in 0..n_elem {
            parent.push(i);
        }

        Self {
            n_elem,
            parent,
            rank : vec![0; n_elem],
            s_size : vec![1; n_elem],
        }
    }
    
    fn find(&mut self, i : usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }

    fn union(&mut self, i : usize, j : usize) {
        let i_rep = self.find(i);
        let j_rep = self.find(j);

        if i_rep == j_rep { return }
        else if self.rank[i_rep] < self.rank[j_rep] {
            self.parent[i_rep] = j_rep;
            self.s_size[j_rep] += self.s_size[i_rep]
        } else if self.rank[i_rep] > self.rank[j_rep] {
            self.parent[j_rep] = i_rep;
            self.s_size[i_rep] += self.s_size[j_rep]
        } else {
            self.parent[i_rep] = j_rep;
            self.rank[j_rep] += 1;
            self.s_size[j_rep] += self.s_size[i_rep]
        }
    }

    // Once a circuit is formed, the head of all elements is the same,
    // and it has the size of the circuit
    fn one_group(&mut self) -> bool {
        let rep = self.find(0);
        self.s_size[rep] == self.n_elem
    }
}

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let coords : Vec<(u64, u64, u64)> = contents.trim()
        .lines()
        .map(|l| l.split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .next_tuple()
            .unwrap())
        .collect();
    
    let mut distances : Vec<(f64, usize, usize)> = Vec::with_capacity(coords.len() * (coords.len() - 1) / 2);

    for line in 0..coords.len() - 1 {
        for col in line + 1..coords.len() {
            distances.push((eucl_dist(coords[line], coords[col]), line, col));
        }
    }

    distances.sort_by(|(d1, _, _), (d2, _, _)| d1.total_cmp(d2));

    let mut links = UnionFind::new(coords.len());
    for i in 0..PROB_1_ITER {
        let (_, box1, box2) = distances[i];
        links.union(box1, box2);
        //if links.all_linked() { break }
    }

    let reps : Vec<usize> = (0..coords.len()).map(|x| links.find(x)).collect();
    let mut freq : HashMap<usize, u64> = HashMap::new();
    for rep in reps {
        freq.entry(rep)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let mut counts : Vec<u64> = freq.values().copied().collect();
    counts.sort();

    let mut it = counts.iter().rev();
    println!("(ANSWER 1) The product of the sizes of the three greatest circuits is: {}",
        it.next().unwrap() * it.next().unwrap() * it.next().unwrap());

    let mut res2 = 0;
    for i in PROB_1_ITER..distances.len() {
        let (_, box1, box2) = distances[i];
        links.union(box1, box2);
        if links.one_group() {
            res2 = coords[box1].0 * coords[box2].0;
            break
        }
    }

    println!("(ANSWER 2) The product of the x-coordinates of the last two linked boxes is: {res2}");
}

fn eucl_dist(coord1 : (u64, u64, u64), coord2 : (u64, u64, u64)) -> f64 {
    let (x1, y1, z1) = coord1;
    let (x2, y2, z2) = coord2;

    f64::sqrt(((x2.abs_diff(x1)).pow(2) + (y2.abs_diff(y1)).pow(2) + (z2.abs_diff(z1)).pow(2)) as f64)
}
