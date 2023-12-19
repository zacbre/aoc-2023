use workflows::{Step, Workflow};
use crate::part::Part;
use crate::workflows::{Operand, Var, Workflows};

use rayon::prelude::*;
use rayon::iter::ParallelIterator;

mod input;
mod workflows;
mod part;

fn main() {}

fn parse(input: &str) -> (Workflows, Vec<Part>) {
    let mut workflows = Workflows::new();
    let mut lines = input.lines();
    let mut parts_list = Vec::new();
    let mut parts = false;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            parts = true;
            continue;
        }
        if !parts {
            let mut workflow = Workflow::parse_workflow(line);
            //println!("Workflow: {:?}", workflow);
            workflows.add_workflow(workflow);
            continue;
        }
        let parts = Part::parse(line.to_string());
        //println!("Parts: {:?}", parts);
        parts_list.push(parts);
    }
    (workflows, parts_list)
}

fn run_workflow(workflows: &Workflows, workflow: &Workflow, x: usize, m: usize, a: usize, s: usize) -> (bool, Option<String>) {
    //println!("{:?}", workflow.steps);
    for (index, step) in workflow.steps.iter().enumerate() {
        if step.is_only_next_workflow && step.next_workflow != Some("A".to_string()) && step.next_workflow != Some("R".to_string()) {
            //println!("Is last workflow, moving on to next one: {:?}", step.next_workflow);
            let next_workflow = workflows.get_workflow(step.next_workflow.as_ref().unwrap()).unwrap();
            return run_workflow(workflows, next_workflow, x, m, a, s);
        }

        let part_value = match step.var_to_check {
            Var::X => x,
            Var::M => m,
            Var::A => a,
            Var::S => s,
        };
        //println!("Workflow: {}, Step: {}, Part: {:?}, Part Value: {:?}, Operand: {:?}, To Check: {:?}", workflow.name, index+1, step.var_to_check, part_value, step.operand, step.value_to_check);
        let is_true = match step.operand {
            Operand::Greater => part_value > step.value_to_check,
            Operand::Less => part_value < step.value_to_check,
        };
        //println!("Part Value: {:?}, Is True: {:?}", part_value, is_true);
        if is_true {
            let next_workflow = step.next_workflow.clone().unwrap();
            return match next_workflow.as_str() {
                "A" => {
                    //println!("Accepted Part: {:?}", parts);
                    return (true, None)
                },
                "R" => {
                    //println!("Rejected Part: {:?}", parts);
                    return (false, None)
                },
                _ => {
                    let next_workflow = workflows.get_workflow(next_workflow.as_str());
                    match next_workflow {
                        Some(next_workflow) => {
                            //println!("Running next workflow: {:?}", next_workflow);
                            return run_workflow(workflows, next_workflow, x, m, a, s);
                        }
                        None => {
                            panic!("Workflow {:?} not found!", next_workflow);
                        }
                    }
                }
            };
        }
        //println!("Moving on to next step...");
    }
    (false, None)
}

fn run(workflows: &Workflows, parts: &Vec<Part>) -> Vec<(Part, bool)> {
    let mut passed: Vec<(Part, bool)> = Vec::new();
    for part in parts {
        // find the first workflow
        let x = part.parts.get(&Var::X).unwrap();
        let m = part.parts.get(&Var::M).unwrap();
        let a = part.parts.get(&Var::A).unwrap();
        let s = part.parts.get(&Var::S).unwrap();

        let result = run_workflow(&workflows, &workflows.get_workflow("in").unwrap(), *x, *m, *a, *s);
        passed.push((part.clone(), result.0));
    }
    passed
}

#[cfg(test)]
mod tests {
    use std::thread::available_parallelism;
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::IndexedParallelIterator;
    use rayon::iter::ParallelIterator;

    #[test]
    fn can_parse_example_input() {
        let parsed = super::parse(super::input::EXAMPLE_INPUT);
        assert_eq!(parsed.0.workflows.len(), 11);
        assert_eq!(parsed.1.len(), 5);
    }

    #[test]
    fn can_run_example_workflow() {
        let parsed = super::parse(super::input::EXAMPLE_INPUT);
        let result = super::run(&parsed.0, &parsed.1);
        println!("Result: {:?}", result);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].0, parsed.1[0]);
        assert_eq!(result[0].1, true);
        assert_eq!(result[1].0, parsed.1[1]);
        assert_eq!(result[1].1, false);
        assert_eq!(result[2].0, parsed.1[2]);
        assert_eq!(result[2].1, true);
        assert_eq!(result[3].0, parsed.1[3]);
        assert_eq!(result[3].1, false);
        assert_eq!(result[4].0, parsed.1[4]);
        assert_eq!(result[4].1, true);
    }

    #[test]
    fn can_run_workflow() {
        let parsed = super::parse(super::input::INPUT);
        let result = super::run(&parsed.0, &parsed.1);
        let answer: usize = result.iter().map(|(part, accepted)| {
            if !accepted {
                return 0;
            }
            part.parts.iter().map(|(var, value)| {
                value
            }).sum()
        }).sum();
        println!("Answer: {:?}", answer);
    }

    #[test]
    fn part_two() {
        let parsed = super::parse(super::input::INPUT);
        let workflow = &parsed.0.get_workflow("in").unwrap();
        // make ranges happen
        //4000 / cores
        let range = 4000 / available_parallelism().unwrap().get();
        let total_passed_parts: usize = (0..available_parallelism().unwrap().get()).into_par_iter().map(|i| {
            println!("Starting thread: {:?}", i);
            let start = i * range;
            let end = start + range;
            let passed_parts: usize = (start..end).into_par_iter().map(|x| {
                (0..4000).into_par_iter().map(|m| {
                    let val = (0..4000).into_par_iter().map(|a| {
                        (0..4000).into_par_iter().map(|s| {
                            let result = super::run_workflow(&parsed.0, workflow, x, m, a, s);
                            if result.0 {
                                1
                            } else {
                                0
                            }
                        }).sum::<usize>()
                    }).sum::<usize>();
                    println!("Thread {:?} finished another round, x: {}, m: {}", i, x, m);
                    val
                }).sum::<usize>()
            }).sum::<usize>();

            println!("Thread {:?} finished: {:?}", i, passed_parts);
            passed_parts
        }).sum();
        println!("Total passed parts: {:?}", total_passed_parts);
    }
}