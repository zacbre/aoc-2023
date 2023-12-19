
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Var {
    X, M, A, S
}
impl Var {
    pub fn parse_var(input: &str) -> Self {
        match input {
            "x" => Var::X,
            "m" => Var::M,
            "a" => Var::A,
            "s" => Var::S,
            _ => panic!("Invalid variable name: {}", input)
        }
    }
}

#[derive(Debug)]
pub enum Operand{
    Greater,
    Less
}

#[derive(Debug)]
pub struct Workflows {
    pub(crate) workflows: Vec<Workflow>,
}

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub steps: Vec<Step>
}

impl Workflow {
    pub fn parse_workflow(input: &str) -> Self {
        let found = input.find('{');
        let name = input[..found.unwrap()].to_string();
        let split = input[found.unwrap()+1..input.len()-1].split(',');
        let steps: Vec<_> = split.clone().enumerate().map(|(index, s)| Step::parse_step(s, index == &split.clone().count() - 1)).collect();
        // get the last step
        let next_workflow = if steps.iter().last().unwrap().is_only_next_workflow {
            steps.iter().last().unwrap().next_workflow.clone()
        } else {
            None
        };

        Workflow {
            name,
            steps,
        }
    }

    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }
}

#[derive(Debug)]
pub struct Step {
    pub(crate) var_to_check: Var,
    pub(crate) value_to_check: usize,
    pub(crate) operand: Operand,
    pub(crate) next_workflow: Option<String>,
    pub(crate) is_only_next_workflow: bool
}
impl Step {
    pub fn parse_step(input: &str, is_last: bool) -> Self {
        //a<2006:qkq
        // get the first variable name
        if is_last {
            // this is a final step
            return Step {
                var_to_check: Var::X,
                value_to_check: 0,
                operand: Operand::Greater,
                next_workflow: Some(input.to_string()),
                is_only_next_workflow: true
            }
        }

        let var = Var::parse_var(&input[..1]);
        let operand = match &input[1..2] {
            "<" => Operand::Less,
            ">" => Operand::Greater,
            _ => panic!("Invalid operand")
        };
        // get the value to check
        let found = input.find(':');
        let value = input[2..found.unwrap()].parse::<usize>().unwrap();
        // get the next workflow
        let next_workflow = input[found.unwrap()+1..].to_string();
        Step {
            var_to_check: var,
            value_to_check: value,
            operand,
            next_workflow: Some(next_workflow),
            is_only_next_workflow: false
        }
    }
}

impl Workflows {
    pub fn new() -> Workflows {
        Workflows {
            workflows: Vec::new(),
        }
    }

    pub fn add_workflow(&mut self, workflow: Workflow) {
        self.workflows.push(workflow);
    }

    pub fn get_workflow(&self, name: &str) -> Option<&Workflow> {
        self.workflows.iter().find(|w| w.name == name)
    }
}