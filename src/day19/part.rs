use std::collections::HashMap;
use crate::day19::workflows::Var;

#[derive(Debug, PartialEq, Clone)]
pub struct Part {
    pub(crate) parts: HashMap<Var, usize>,
}

impl Part {
    pub fn parse(input: String) -> Self {
        let mut parts = HashMap::new();
        let input = input.replace("{", "").replace("}", "");
        let split = input.split(',');
        for s in split {
            let split = s.split('=');
            let mut split = split.clone();
            let var = split.next().unwrap().to_string();
            let value = split.next().unwrap().parse::<usize>().unwrap();
            parts.insert(Var::parse_var(var.as_str()), value);
        }
        Part {
            parts,
        }
    }
}