use std::fs;
use itertools::Itertools;

// Strategy:
//   - Check rectangle area, if greater than current greatest, then check if intersects
// Important facts:
//   - There are no loops, either inner or outer:
//      
//      x ---------- x              x --------------- x
//      |            |              |                 |
//      |            |              |        x ---- x |
//      x -----------|----- x       |        |      | |
//                   |      |       |        x ------ x
//                   x ---- x       |               |
//                                  x ------------- x
//   - For each line, there are two possibilities for inside/outside:
//        1       x
//      x - x   1 | 2
//        2       x
//
//   -> If a rectangle is crossed by a line, it necessarily has a part in and another one out 
//      (as there are no loops)

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");

    let red_tiles : Vec<(u64, u64)> = contents.trim()
        .lines()
        .map(|v| v.split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .next_tuple().unwrap())
        .collect();

    let mut lines : Vec<((u64, u64), (u64, u64))> = Vec::with_capacity(red_tiles.len());
    let mut tile1 = red_tiles[0];
    let mut tile2;
    for i in 1..red_tiles.len() {
        tile2 = red_tiles[i];
        lines.push((tile1, tile2));
        tile1 = tile2
    }
    lines.push((tile1, red_tiles[0]));

    let mut largest = 0;
    let mut rg_largest = 0;
    
    for t1 in 0..red_tiles.len() {
        for t2 in t1..red_tiles.len() {
            let (x1, y1) = red_tiles[t1];
            let (x2, y2) = red_tiles[t2];
            let area = (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1);
            if area > largest { largest = area }
            if area > rg_largest && !crossing_interior(((x1, y1), (x2, y2)), &lines) { 
                rg_largest = area;
            }
        }
    }
    println!("(ANSWER 1) The size of the largest rectangle with red corners is: {largest}");
    println!("(ANSWER 2) The size of the largest red and green rectangle is: {rg_largest}");

}

fn crossing_interior(rect : ((u64, u64), (u64, u64)), lines : &Vec<((u64, u64), (u64, u64))>) -> bool {
    let mut cross = false;
    let ((r_x1, r_y1), (r_x2, r_y2)) = rect;

    let (r_x1, r_x2) = if r_x1 <= r_x2 { (r_x1, r_x2) } else { (r_x2, r_x1) };
    let (r_y1, r_y2) = if r_y1 <= r_y2 { (r_y1, r_y2) } else { (r_y2, r_y1) };

    for ((x1, y1), (x2, y2)) in lines {
        let (x1, x2) = if x1 <= x2 { (*x1, *x2) } else { (*x2, *x1) };
        let (y1, y2) = if y1 <= y2 { (*y1, *y2) } else { (*y2, *y1) };

        if x1 <= r_x1 && x2 > r_x1 && y1 > r_y1 && y1 < r_y2 ||
           x1 < r_x2 && x2 >= r_x2 && y1 > r_y1 && y1 < r_y2 ||
           y1 <= r_y1 && y2 > r_y1 && x1 > r_x1 && x1 < r_x2 ||
           y1 < r_y2 && y2 >= r_y2 && x1 > r_x1 && x1 < r_x2
        { cross = true; break }
    }
    cross
}
