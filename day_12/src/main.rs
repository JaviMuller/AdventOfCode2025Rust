use std::fs;
use itertools::iproduct;
use z3::{Solver, SatResult, ast::Int, ast::Bool};

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");
    
    let mut it = contents.trim_end().rsplit("\n\n");
    let problems = it.next().unwrap();
    let mut shapes : Vec<Vec<Vec<bool>>> = Vec::new();
    for shape in it {
        let mut lines = shape.lines();
        lines.next(); // Bypass the number of the shape
        shapes.push(
            lines.map(|line| line.chars()
                    .map(|c| if c == '#' { true } else { false })
                    .collect())
            .collect());
    }

    let shapes : Vec<Vec<Vec<Vec<bool>>>> = shapes.iter().rev()
        .map(|s| rotations_reflections(s))
        .collect();
    let shapes_pos : Vec<Vec<Vec<(usize, usize)>>> = shapes.iter()
        .map(|t| t.iter()
            .map(|sh| shape_as_pos(sh))
            .collect())
        .collect();

    let mut solvable = 0;
    // Model each of the problems using mxn integer variables with the number being the id
    // of the present -> 32, 43, 76, 87, 12 (1-32 -> shape 1, 33-76 -> shape 2, ...)
    for problem in problems.lines() {
        let (size, presents) = problem.split_once(": ").unwrap();
        let (width, height) = size.split_once("x").unwrap();
        let (width, height) = (width.parse::<u64>().unwrap(), height.parse::<u64>().unwrap());
        let presents : Vec<u64> = presents.split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let n_presents : u64 = presents.iter().sum();
        let aggr_presents_size : u64 = shapes_pos.iter().zip(presents.iter())
            .map(|(sh, n)| sh[0].len() as u64 * *n)
            .sum();
        // Practical solution
        // (either a lot of free space or can't fit even if arrangement left no empty spaces)
        if aggr_presents_size < width * height { solvable += 1 }
        // Principled solution
        // if (width / 3) * (height / 3) > n_presents { solvable += 1 }
        // else if (width * height) >= aggr_presents_size {
        //     let solver = model_problem(width, height, &presents, &shapes_pos);
        //     match solver.check() {
        //         SatResult::Sat => { solvable += 1 },
        //         _ => { }
        //     }
        // }
    }
    println!("(ANSWER) The number of solvable problems is: {solvable}");
}

fn rotations_reflections(shape : &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let mut res : Vec<Vec<Vec<bool>>> = vec![];
    let mut new_shape = shape.clone();
    for _ in 0..3 {
        res.push(new_shape.clone());
        new_shape = rotate_right(&new_shape);
    }
    res.push(new_shape);

    new_shape = reflect_horizontal(shape);
    for _ in 0..3 {
        res.push(new_shape.clone());
        new_shape = rotate_right(&new_shape);
    }
    res.push(new_shape);
    res.sort();
    res.dedup();
    res
}

fn rotate_right(shape : &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rot_shape : Vec<Vec<bool>> = (0..shape[0].len()).map(|_| Vec::new()).collect();
    for line in (0..shape.len()).rev() {
        for col in 0..shape[line].len() {
            rot_shape[col].push(shape[line][col]);
        }
    }
    rot_shape
}

fn reflect_horizontal(shape : &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut refl_shape : Vec<Vec<bool>> = (0..shape.len()).map(|_| Vec::new()).collect();

    for i in 0..shape.len() {
        refl_shape[i] = shape[shape.len() - i - 1].clone();
    }
    refl_shape
}

fn shape_as_pos(shape : &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for (w, h) in iproduct!(0..shape.len(), 0..shape[0].len()) {
        if shape[w][h] { res.push((w, h)) }
    }
    res
}

fn model_problem(width : u64, height : u64, presents : &Vec<u64>, shapes : &Vec<Vec<Vec<(usize, usize)>>>) -> Solver {
    let mut solver = Solver::new();
    let variables : Vec<Vec<Int>> = 
        (0..width).map(|w| 
            (0..height).map(|h| 
                Int::fresh_const(&format!("{{{w},{h}}}")))
                .collect())
            .collect();
    let mut present_id = 0;
    for t in 0..presents.len() {
        model_present_type(&mut solver, &variables, present_id, presents[t], &shapes[t]);
        present_id += presents[t];
    }
    // All variables must belong to a present or be empty (0)
    iproduct!(0..width, 0..height).for_each(|(w, h)| solver.assert(Bool::and(&[
        variables[w as usize][h as usize].ge(Int::from_u64(0)),
        variables[w as usize][h as usize].le(Int::from_u64(present_id))])));
    // For unsat core checking
    // iproduct!(0..width, 0..height).for_each(|(w, h)| solver.assert_and_track(Bool::and(&[
    //     variables[w as usize][h as usize].ge(Int::from_u64(0)),
    //     variables[w as usize][h as usize].le(Int::from_u64(present_id))]),
    // &Bool::fresh_const(&format!("valid_{{{w},{h}}}"))));
    solver
}

fn model_present_type(solver : &mut Solver, variables : &Vec<Vec<Int>>, highest_id : u64, amount : u64, shapes : &Vec<Vec<(usize, usize)>>) {
    // Need to model
    let max_id = highest_id + amount;
    let max_w = variables.len();
    let max_h = variables[0].len();
    for (w, h) in iproduct!(0..max_w, 0..max_h) {
        let pos_is_of_type_t : Bool = Bool::and(&[
            variables[w][h].gt(Int::from_u64(highest_id)),
            variables[w][h].le(Int::from_u64(max_id))]);
        let mut variant_configs : Vec<Bool> = Vec::new();
        for variant in 0..shapes.len() {
            'root: for (root_w, root_h) in &shapes[variant] {
                let mut concrete_config : Vec<Bool> = Vec::new();
                // Check that given a fixed root, all other positions are within bounds
                for (rel_w, rel_h) in &shapes[variant] {
                    // Skip root
                    if (rel_w, rel_h) == (root_w, root_h) { continue }
                    // If out of bounds, jump to next configuration
                    match ((w + rel_w).checked_sub(*root_w), (h + rel_h).checked_sub(*root_h)) {
                        (Some(abs_w), Some(abs_h)) => {
                            if abs_w < max_w && abs_h < max_h {
                                concrete_config.push(variables[abs_w][abs_h].eq(&variables[w][h]));
                            } else { continue 'root }
                        },
                        (_,_) => continue 'root
                    }
                }
                variant_configs.push(Bool::and(&concrete_config[..]));
            }
        }
        let all_configs : Bool = Bool::or(&variant_configs[..]);
        solver.assert(Bool::implies(&pos_is_of_type_t, all_configs));
        // For unsat core checking
        // solver.assert_and_track(Bool::implies(&pos_is_of_type_t, all_configs), &Bool::fresh_const(&format!("{{{w},{h}}}-[{}-{max_id}]=config", highest_id + 1)));
    }
    // The number of positions that equal any id are the size of the id present in units
    for id in highest_id + 1..=max_id {
        let vars_equal_id : Vec<Int> = iproduct!(0..max_w, 0..max_h)
            .map(|(w, h)| Bool::ite(
                &variables[w][h].eq(Int::from_u64(id)),
                &Int::from_u64(1),
                &Int::from_u64(0)))
            .collect();
        let size_id = Int::from_u64(shapes[0].len() as u64);
        solver.assert(Int::add(&vars_equal_id[..]).eq(size_id));
        // For unsat core checking
        // solver.assert_and_track(Int::add(&vars_equal_id[..]).eq(size_id), &Bool::fresh_const(&format!("#cells_{id}")));
    }
}
