use std::fs;
use regex::Regex;
use pathfinding::prelude::dijkstra;
use good_lp::{variable, variables, constraint, Expression, Variable, SolverModel, Solution};
use good_lp::solvers::highs::highs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct LightCfg(Vec<bool>);  // States for our search algorithms

impl LightCfg {
    fn len(&self) -> usize {
        let LightCfg(lights) = self;
        lights.len()
    }

    fn successors(&self, buttons : &Vec<Vec<usize>>) -> Vec<(LightCfg, usize)> {
        let LightCfg(lights) = self;
        let mut res : Vec<(LightCfg, usize)> = Vec::with_capacity(buttons.len());
        for button in buttons {
            let mut new_lights = lights.clone();
            for pos in button {
                new_lights[*pos] = !new_lights[*pos];
            }
            res.push((LightCfg(new_lights), 1));
        }
        res
    }
}

fn main() {
    let file_path = "resources/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open the file");
    
    let re_light = Regex::new(r"\[([.#]+)\]").unwrap();
    let re_button = Regex::new(r"\(([\d,]+)\)").unwrap();
    let re_costs = Regex::new(r"\{([\d,]+)\}").unwrap();

    let mut button_presses_start = 0;
    let mut button_presses_joltage = 0;
    for line in contents.trim_end().lines() {
        let lights : LightCfg = LightCfg(re_light
            .captures(&line).unwrap()
            .get(1).unwrap()
            .as_str()
            .chars()
            .map(|x| match x {
                '#' => true,
                '.' | _ => false,
            })
            .collect());
        let buttons : Vec<Vec<usize>> = re_button
            .captures_iter(&line)
            .map(|x| x.extract())
            .map(|(_, [button])| button.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
            )
            .collect();
        let joltages : Vec<usize> = re_costs
            .captures(&line).unwrap()
            .get(1).unwrap()
            .as_str()
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let s_0 = vec![false; lights.len()];
        button_presses_start += dijkstra(
                &LightCfg(s_0),
                |cfg| cfg.successors(&buttons),
                |cfg| *cfg == lights
            ).unwrap().1;
        button_presses_joltage += joltage_solution(buttons, joltages);
    }
    println!("(ANSWER 1) The minimum number of button presses to start the machines is: {button_presses_start}");
    println!("(ANSWER 2) The minimum number of button presses to configure the machines is: {button_presses_joltage}");
}

fn joltage_solution(buttons: Vec<Vec<usize>>, joltages: Vec<usize>) -> usize {
    let mut problem = variables!();
    let variables : Vec<Variable> = problem.add_vector(variable().integer().min(0), buttons.len());
    let objective : Expression = variables.iter().sum();
    let mut model = problem.minimise(objective).using(highs);
    for pos in 0..joltages.len() {
        let joltage = joltages[pos];
        let mut constr_vars : Vec<Variable> = Vec::new();
        for but in 0..buttons.len() {
            if buttons[but].contains(&pos) {
                constr_vars.push(variables[but]);
            }
        }
        let sum : Expression = constr_vars.iter().sum();
        model = model.with(constraint!(sum == joltage as i32));
    }
    let sol = model.solve().unwrap();
    let mut presses = 0;
    for var in variables {
        presses += sol.value(var) as usize;
    }
    presses
}
