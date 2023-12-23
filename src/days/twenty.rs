use crate::utils::{Input, ProblemInput};
use core::panic;
use std::collections::VecDeque;
use std::{collections::HashMap, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    20
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    //todo!("Implement day {} part one",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    //todo!("Implement day {} part two",the_day());
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}
fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut modules = parse_input(lines);
    modules.insert(
        "output".to_string(),
        Module::new("output".to_string(), ModuleType::Output, vec![]),
    );
    let keys = modules.keys().cloned().collect::<Vec<String>>();
    for key in keys {
        let module = modules.get_mut(key.as_str()).unwrap().clone();
        for &child in &module.outputs {
            if modules.get(child).is_none() {
                continue;
            }
            let module = modules.get_mut(child).unwrap();
            let tmp_state = module.state.clone();
            match tmp_state {
                StateType::Single(_) => {}
                StateType::Multi(mut state) => {
                    //let mut state = state.clone();
                    state.insert(key.to_string(), false);
                    module.state = StateType::Multi(state);
                }
            }
        }
    }
    let mut postives = 0;
    let mut negatives = 0;
    let mut i = 0;
    loop {
        let mut pulses: VecDeque<(String, String, bool)> = VecDeque::new();
        negatives += 1;
        pulses.push_back(("broadcaster".to_string(), "broadcaster".to_string(), false));
        loop {
            let (module_name, source, val) = match pulses.pop_front() {
                None => {
                    break;
                }
                Some(val) => val,
            };

            let module = modules.get_mut(&module_name).unwrap();
            let children = module.outputs.clone();
            let result = module.get_output_value(source, val);

            if result.is_some() {
                for child in children {
                    let child = child.to_string();
                    match result {
                        None => {}
                        Some(val) => {
                            match val {
                                true => {
                                    postives += 1;
                                }
                                false => {
                                    negatives += 1;
                                }
                            }
                            let child = modules.get_mut(&child);
                            if child.is_none() {
                                continue;
                            }
                            let child = child.unwrap();
                            pulses.push_back((child.name.clone(), module_name.clone(), val));
                        }
                    }
                }
            }
        }
        i += 1;
        if i == 1000 {
            break;
        }
    }
    // println!("Postives: {}, Negatives: {}", postives, negatives);
    postives * negatives
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut modules = parse_input(lines);
    modules.insert(
        "output".to_string(),
        Module::new("output".to_string(), ModuleType::Output, vec![]),
    );
    //println!("Modules: {:?}", modules);

    let keys = modules.keys().cloned().collect::<Vec<String>>();
    for key in keys {
        let module = modules.get_mut(key.as_str()).unwrap().clone();
        for &child in &module.outputs {
            if modules.get(child).is_none() {
                continue;
            }
            let module = modules.get_mut(child).unwrap();
            let tmp_state = module.state.clone();
            match tmp_state {
                StateType::Single(_) => {}
                StateType::Multi(mut state) => {
                    //let mut state = state.clone();
                    state.insert(key.to_string(), false);
                    module.state = StateType::Multi(state);
                }
            }
        }
    }

    // println!("Modules: {:?}", modules);

    let mut i = 0;

    let mut lcm_map: HashMap<String, i64> = HashMap::new();
    'outer: loop {
        let mut pulses: VecDeque<(String, String, bool)> = VecDeque::new();
        pulses.push_back(("broadcaster".to_string(), "broadcaster".to_string(), false));
        loop {
            let (module_name, source, val) = match pulses.pop_front() {
                None => {
                    break;
                }
                Some(val) => val,
            };
            let module = modules.get_mut(&module_name).unwrap();
            let children = module.outputs.clone();
            let result = module.get_output_value(source, val);
            if ["dh", "qd", "bb", "dp"].contains(&module_name.as_str()) {
                if let Some(true) = result {
                    if !lcm_map.contains_key(&module_name) {
                        lcm_map.insert(module_name.clone(), i + 1);
                    }
                }
                if lcm_map.len() == 4 {
                    break 'outer;
                }
            }
            if result.is_some() {
                for child in children {
                    let child = child.to_string();
                    match result {
                        None => {}
                        Some(val) => {
                            let child = modules.get_mut(&child);
                            if child.is_none() {
                                continue;
                            }
                            let child = child.unwrap();
                            pulses.push_back((child.name.clone(), module_name.clone(), val));
                        }
                    }
                }
            }
        }
        i += 1;
    }
    // println!("LCM Map: {:?}", lcm_map);
    let lcm_vals = lcm_map.values().cloned().collect::<Vec<i64>>();
    let mut rolling_lcm = *lcm_vals.first().unwrap();
    for lcm_val in lcm_vals.iter() {
        rolling_lcm = num::integer::lcm(rolling_lcm, *lcm_val);
    }

    // println!("LCM: {}", rolling_lcm);
    rolling_lcm
}

fn parse_input(input: Vec<&str>) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for line in input {
        let line = line.split(" -> ").collect::<Vec<&str>>();
        let name = line[0];
        let next = line[1];
        let next = next.split(", ").collect::<Vec<&str>>();
        match name {
            "broadcaster" => {
                let module = Module::new(name.to_string(), ModuleType::Broadcaster, next);
                modules.insert(name.to_string(), module);
            }
            other => match other.chars().next().unwrap() {
                '%' => {
                    let module = Module::new(name[1..].to_string(), ModuleType::FlipFlop, next);
                    modules.insert(name[1..].to_string(), module);
                }
                '&' => {
                    let mut module = Module::new(name[1..].to_string(), ModuleType::Con, next);
                    module.state = StateType::Multi(HashMap::new());
                    modules.insert(name[1..].to_string(), module);
                }
                _ => {
                    panic!("Invalid module name {}", other);
                }
            },
        }
    }
    modules
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    Broadcaster,
    Con,
    FlipFlop,
    Output,
}

#[derive(Debug, Clone)]
enum StateType {
    Single(bool),
    Multi(HashMap<String, bool>),
}
#[derive(Debug, Clone)]
struct Module<'a> {
    name: String,
    module_type: ModuleType,
    outputs: Vec<&'a str>,
    state: StateType,
    input_value: Option<HashMap<String, bool>>,
}

impl<'a> Module<'a> {
    fn new(name: String, module_type: ModuleType, outputs: Vec<&'a str>) -> Self {
        Self {
            name,
            module_type,
            outputs,
            input_value: None,
            state: StateType::Single(false),
        }
    }

    fn get_output_value(&mut self, source: String, value: bool) -> Option<bool> {
        let out = match self.module_type {
            // named broadcaster
            ModuleType::Broadcaster => Some(false),
            // & prefix
            ModuleType::Con => {
                let tmp_state = self.state.clone();
                if let StateType::Multi(mut previous_state) = tmp_state {
                    previous_state.insert(source, value);
                    let result = !previous_state.values().all(|x| *x);
                    self.state = StateType::Multi(previous_state.clone());
                    Some(result)
                } else {
                    None
                }
            }

            // % prefix
            ModuleType::FlipFlop => {
                if let StateType::Single(previous_state) = self.state {
                    if !value {
                        self.state = StateType::Single(!previous_state);
                        Some(!previous_state)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            // Final module named output
            ModuleType::Output => None, //Some(self.input_value.as_ref().unwrap().iter().next().unwrap().1.to_owned()),
        };
        self.input_value = None;
        out
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER_SIMPLE: i64 = 32000000;
    const PART_ONE_TEST_SIMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const PART_ONE_TEST_ANSWER: i64 = 11687500;
    const PART_ONE_TEST: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    const PART_TWO_ANSWER: i64 = 244055946148853;
    const PART_ONE_ANSWER: i64 = 800830848;

    const PANIC_INPUT: &str = "?braiod -> a";

    #[test]
    #[should_panic]
    fn panics() {
        let input = PANIC_INPUT;
        let input = ProblemInput::String(PANIC_INPUT);
        let input = Input::new(input);
        let lines = input.get_data().lines();
        let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
        let mut modules = parse_input(lines);
    }

    #[test]
    fn fn_test_parse() {
        let input = PART_ONE_TEST_SIMPLE;
        let result = parse_input(input.split("\n").collect::<Vec<&str>>());
        let first = result.get("broadcaster").unwrap();
        assert_eq!(first.name, "broadcaster");
        assert!(first.outputs.contains(&"a"));
        assert!(first.outputs.contains(&"b"));
        assert!(first.outputs.contains(&"c"));

        assert!(result.contains_key("a"));
        assert!(result.contains_key("b"));
        assert!(result.contains_key("c"));
        assert!(result.contains_key("inv"));
    }

    #[test]
    fn one_simple() {
        let input = ProblemInput::String(PART_ONE_TEST_SIMPLE);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER_SIMPLE);
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_TEST_ANSWER);
    }

    #[test]
    fn one_real() {
        let (result, _dur) = part_one();
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }
    #[test]
    fn two_real() {
        let (result, _dur) = part_two();
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
