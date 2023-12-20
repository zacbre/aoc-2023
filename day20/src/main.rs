use std::collections::{BTreeMap, VecDeque};

mod input;

fn main() {}

#[derive(PartialEq, Debug, Clone)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
    None
}

#[derive(PartialEq, Debug, Clone)]
struct Module {
    pub name: String,
    pub state: bool,
    pub outputs: Vec<String>,
    pub module_type: ModuleType,
    pub memory: BTreeMap<String, Pulse>
}

#[derive(PartialEq, Debug, Clone)]
struct ModuleList {
    pub modules: BTreeMap<String, Module>
}

fn parse(input: &str) -> ModuleList {
    let mut modules: BTreeMap<String, Module> = input.lines().map(|line| {
        let mut parts = line.split(" -> ");
        let module_name = &parts.next().unwrap();
        let module_type = match &module_name[..1] {
            "%" => ModuleType::FlipFlop,
            "&" => ModuleType::Conjunction,
            _ => ModuleType::Broadcaster
        };
        let module_name = if module_name.starts_with("%") || module_name.starts_with("&") {
            &module_name[1..]
        } else {
            module_name
        };
        let outputs = parts.next().unwrap();
        let outputs = outputs.split(",").map(|output| output.trim().to_string()).collect::<Vec<String>>();
        (module_name.to_string(), Module {
            name: module_name.to_string(),
            state: false,
            outputs,
            module_type,
            memory: Default::default(),
        })
    }).collect();
    // populate memory for conjunction connections
    for (name, module) in modules.clone().iter() {
        for next_module_name in &module.outputs {
            if !modules.contains_key(next_module_name) {
                continue;
            }
            let next_module = modules.get(next_module_name).unwrap();
            if next_module.module_type == ModuleType::Conjunction {
                let next_module = modules.get_mut(next_module_name).unwrap();
                next_module.memory.insert(name.to_string(), Pulse::Low);
            }
        }
    }
    ModuleList {
        modules
    }
}

impl Module {
    pub fn pulse(&mut self, sender: String, pulse: &Pulse) -> Pulse {
        match self.module_type {
            ModuleType::FlipFlop => {
                if pulse == &Pulse::Low && self.state == false {
                    self.state = true;
                    Pulse::High
                } else if pulse == &Pulse::Low && self.state == true {
                    self.state = false;
                    Pulse::Low
                } else {
                    Pulse::None
                }
            },
            ModuleType::Conjunction => {
                self.memory.insert(sender.clone(), pulse.clone());
                // println!("(conj) {} -{:?}-> {}", &sender, pulse, self.name);
                if self.memory.values().all(|pulse| *pulse == Pulse::High) {
                    //println!("(conj) {} memory: {:?} => {:?}", self.name, self.memory, Pulse::Low);
                    Pulse::Low
                } else {
                    //println!("(conj) {} memory: {:?} => {:?}", self.name, self.memory, Pulse::High);
                    Pulse::High
                }
            },
            ModuleType::Broadcaster => {
                *pulse
            }
        }
    }
}

fn process_modules(modules: &mut ModuleList, start_module: Option<String>, reset_module: Option<String>) -> (usize, usize, bool) {
    let mut signals_to_process: VecDeque<(String, String, Pulse)> = VecDeque::new();
    match start_module {
        Some(s) => {
            signals_to_process.push_back(("broadcaster".to_string(), s, Pulse::Low));
        }
        None => {
            signals_to_process.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    while let Some((previous, output, value)) = signals_to_process.pop_front() {
        match value {
            Pulse::High => high_pulses += 1,
            Pulse::Low => low_pulses += 1,
            Pulse::None => continue,
        };
        //println!("{} -{:?}-> {}", previous, value, output);

        match &reset_module {
            Some(r) => {
                if *r == output && value == Pulse::High {
                    return (low_pulses, high_pulses, true);
                }
            },
            _ => {}
        }

        if !modules.modules.contains_key(&output) {
            continue;
        }

        let new_output = modules.modules.get_mut(&output).unwrap();
        let new_pulse = new_output.pulse(previous, &value);

        for sub_module in &new_output.outputs {
            signals_to_process.push_back((output.to_string(), sub_module.to_string(), new_pulse));
        }
    }
    (low_pulses, high_pulses, false)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_file(file: &str) -> String {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src");
        path.push(file);
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn can_parse_input() {
        let parsed = parse(input::EXAMPLE_INPUT);
        assert_eq!(parsed.modules.len(), 5);
        assert_eq!(parsed.modules.get("broadcaster").unwrap().outputs.len(), 3);
        assert_eq!(parsed.modules.get("inv").unwrap().outputs.len(), 1);
        assert_eq!(parsed.modules.keys().collect::<Vec<_>>(), vec![&"a", &"b", &"broadcaster", &"c", &"inv"]);
        assert_eq!(parsed.modules.get("inv").unwrap().module_type, ModuleType::Conjunction);
        assert_eq!(parsed.modules.get("broadcaster").unwrap().module_type, ModuleType::Broadcaster);
        assert_eq!(parsed.modules.get("a").unwrap().module_type, ModuleType::FlipFlop);
    }

    #[test]
    fn can_process_example_input() {
        let mut parsed = parse(input::EXAMPLE_INPUT);
        let mut low = 0;
        let mut high = 0;
        for _ in 0..1000 {
            let (low_res, high_res, _) = process_modules(&mut parsed, None, None);
            low += low_res;
            high += high_res;
        }
        assert_eq!(low, 8000);
        assert_eq!(high, 4000);
        assert_eq!(low * high, 32000000);
    }

    #[test]
    fn can_process_example_input_2() {
        let mut parsed = parse(input::EXAMPLE_INPUT_2);
        let mut low = 0;
        let mut high = 0;
        for _ in 0..1000 {
            let (low_res, high_res, _) = process_modules(&mut parsed, None, None);
            low += low_res;
            high += high_res;
        }
        assert_eq!(low, 4250);
        assert_eq!(high, 2750);
        assert_eq!(low * high, 11687500);
    }

    #[test]
    fn can_process_input() {
        let input = get_test_file("input.txt");
        let mut parsed = parse(input.as_str());
        let mut low = 0;
        let mut high = 0;
        for _ in 0..1000 {
            let (low_res, high_res, _) = process_modules(&mut parsed, None, None);
            low += low_res;
            high += high_res;
        }
        assert_eq!(low, 17198);
        assert_eq!(high, 45824);
        assert_eq!(low * high, 788081152);
    }

    #[test]
    fn can_process_input_pt2() {
        let input = get_test_file("input.txt");
        let mut parsed = parse(input.as_str());
        let broadcaster = parsed.modules.get("broadcaster").unwrap().clone();
        let rx_parent = parsed.modules.iter().find_map(|(name, module)| module.outputs.contains(&"rx".to_string()).then(|| name.clone())).unwrap();
        let presses = broadcaster.outputs.iter().map(|t| {
            for it in 1.. {
                let (_, _, terminated) = process_modules(&mut parsed, Some(t.to_string()), Some(rx_parent.clone()));
                if terminated {
                    return it;
                }
            }
            unreachable!()
        }).into_iter().reduce(lcm).unwrap();
        assert_eq!(presses, 224602011344203);
    }
}
