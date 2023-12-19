use nom::IResult;

use crate::utils::{Input, ProblemInput};
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    time::Instant,
};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    19
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Part {
    data: HashMap<char, i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum WorkFlowResult {
    Accept,
    Reject,
    Next(String),
}

impl WorkFlowResult {
    fn new(result: &str) -> Self {
        match result {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Next(result.to_string()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkFlowOperator {
    key: char,
    value: i64,
    comparitor: std::cmp::Ordering,
    result: WorkFlowResult,
}
impl WorkFlowOperator {
    fn new(key: char, op: char, value: i64, result: &str) -> Self {
        let comparitor = match op {
            '<' => std::cmp::Ordering::Less,
            '>' => std::cmp::Ordering::Greater,
            _ => panic!("Invalid operator: {}", op),
        };
        Self {
            key,
            value,
            comparitor,
            result: WorkFlowResult::new(result),
        }
    }
    // fn to_string(&self) -> String {
    //     let com = match self.comparitor {
    //         std::cmp::Ordering::Less => "<",
    //         std::cmp::Ordering::Greater => ">",
    //         std::cmp::Ordering::Equal => "=",
    //     };
    //     format!("{}{}{}:{:?}", self.key, com, self.value, self.result)
    // }
    fn do_workflow(&self, part: &Part) -> Option<WorkFlowResult> {
        let val = match part.data.get(&self.key) {
            Some(val) => val,
            None => return None,
        };

        let result = self.comparitor == val.cmp(&self.value);
        match result {
            true => Some(self.result.clone()),
            false => None,
        }
    }
    fn get_inverse(&self) -> Self {
        let mut op = self.clone();
        match self.comparitor {
            std::cmp::Ordering::Less => {
                op.comparitor = std::cmp::Ordering::Greater;
                op.value -= 1;
            }
            std::cmp::Ordering::Greater => {
                op.comparitor = std::cmp::Ordering::Less;
                op.value += 1;
            }
            _ => panic!("Invalid operator: {:?}", self.comparitor),
        };

        op
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkFlow {
    operators: Vec<WorkFlowOperator>,
    final_op: WorkFlowResult,
}

impl WorkFlow {
    fn send(&self, part: Part) -> WorkFlowResult {
        for op in &self.operators {
            let result = op.do_workflow(&part);
            match result {
                Some(result) => return result,
                None => continue,
            }
        }
        self.final_op.clone()
    }
}

//overall parser to parse the input into a hashmap of WorkFlows and a vector of Parts
fn parse_input(input: Vec<&str>) -> (HashMap<&str, WorkFlow>, Vec<Part>) {
    let mut workflows: HashMap<&str, WorkFlow> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    let mut empty = false;
    for row in input {
        if row.is_empty() {
            empty = true;
            continue;
        }
        if !empty {
            let (_input, (key, workflow)) = workflow_parser(row).unwrap();
            workflows.insert(key, workflow);
        } else {
            let (_input, part) = part_parser(row).unwrap();
            parts.push(part);
        }
    }
    (workflows, parts)
}

// parse the input into a WorkFlow struct including a vector of WorkFlowOperators and a WorkFlowResult
fn workflow_parser(input: &str) -> IResult<&str, (&str, WorkFlow)> {
    let mut workflow = WorkFlow {
        operators: vec![],
        final_op: WorkFlowResult::Reject,
    };
    let (input, key) = nom::bytes::complete::take_until1("{")(input)?;
    let (input, _) = nom::bytes::complete::tag("{")(input)?;
    let input = input.replace('}', "");
    let input = input.split(',');

    for item in input {
        if item.contains(':') {
            let (_input, op) = workflow_operator_parser(item).unwrap();
            workflow.operators.push(op);
        } else {
            workflow.final_op = WorkFlowResult::new(item);
        }
    }
    Ok(("", (key, workflow)))
}

// parse the input into a WorkFlowOperator struct
fn workflow_operator_parser(input: &str) -> IResult<&str, WorkFlowOperator> {
    let (input, key) = nom::character::complete::anychar(input)?;
    let (input, op) = nom::character::complete::anychar(input)?;
    let (input, val) = nom::character::complete::digit1(input)?;
    let (input, _s) = nom::bytes::complete::tag(":")(input)?;
    let (input, result) = nom::character::complete::alpha1(input)?;
    Ok((
        input,
        WorkFlowOperator::new(key, op, val.parse::<i64>().unwrap(), result),
    ))
}

// parse the input into a Part struct
fn part_parser(input: &str) -> IResult<&str, Part> {
    let (input, _) = nom::bytes::complete::tag("{")(input)?;
    let (input, _x) = nom::bytes::complete::tag("x=")(input)?;
    let (input, x) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(",")(input)?;
    let (input, _m) = nom::bytes::complete::tag("m=")(input)?;
    let (input, m) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(",")(input)?;
    let (input, _a) = nom::bytes::complete::tag("a=")(input)?;
    let (input, a) = nom::character::complete::digit1(input)?;
    let (input, _) = nom::bytes::complete::tag(",")(input)?;
    let (input, _s) = nom::bytes::complete::tag("s=")(input)?;
    let (input, s) = nom::character::complete::digit1(input)?;
    //let (input, _) = nom::character::complete::line_ending(input)?;
    let mut part = Part {
        data: HashMap::new(),
    };
    part.data.insert('x', x.parse::<i64>().unwrap());
    part.data.insert('m', m.parse::<i64>().unwrap());
    part.data.insert('a', a.parse::<i64>().unwrap());
    part.data.insert('s', s.parse::<i64>().unwrap());
    Ok((input, part))
}

fn find_result(workflows: &HashMap<&str, WorkFlow>, part: Part) -> bool {
    let mut key = "in".to_string();
    loop {
        let workflow = workflows.get(key.as_str()).unwrap();
        match workflow.send(part.clone()) {
            WorkFlowResult::Accept => return true,
            WorkFlowResult::Reject => return false,
            WorkFlowResult::Next(next) => key = next.clone(),
        }
    }
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (workflows, parts) = parse_input(lines);
    let mut total = 0;
    for part in parts {
        let result = find_result(&workflows, part.clone());
        if result {
            total += part.data.values().sum::<i64>();
        }
    }
    total
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let (workflows, _parts) = parse_input(lines);

    // generate all possible routes to accept
    let results = search_to_accept("in", &workflows, vec![]).unwrap();

    let mut supertotal = 0;

    // for each route, generate a hashmap of all possible values for each variable
    for line in results {
        let mut res: HashMap<char, HashSet<i64>> = HashMap::new();

        //start with all possible values for each variable in a hashset
        let all_nums = (1..4001).collect::<HashSet<i64>>();
        res.insert('x', all_nums.clone());
        res.insert('m', all_nums.clone());
        res.insert('a', all_nums.clone());
        res.insert('s', all_nums.clone());

        // iterate over each step in the route and remove all values that are not possible
        for (_key, op) in line {
            // if the operator is the final operator, skip it
            if op.key == '*' {
                continue;
            }

            // remove all impossible values from the set.
            match op.comparitor {
                std::cmp::Ordering::Less => {
                    for i in op.value..4001 {
                        res.get_mut(&op.key).unwrap().remove(&i);
                    }
                }
                std::cmp::Ordering::Greater => {
                    for i in 1..op.value + 1 {
                        res.get_mut(&op.key).unwrap().remove(&i);
                    }
                }
                _ => panic!("Invalid operator: {:?}", op.comparitor),
            }
        }
        // calculate the total number of possible values for each variable and multiply them together
        let total: i64 = res.values().map(|x| x.iter().len() as i64).product();
        supertotal += total;
    }
    supertotal
}

// recursive function to search for all valid routes to accept and then return those routes as a vector of vectors of WorkFlowOperators
fn search_to_accept<'a>(
    key: &'a str,
    workflows: &'a HashMap<&str, WorkFlow>,
    route: Vec<(&'a str, WorkFlowOperator)>,
) -> Option<Vec<Vec<(&'a str, WorkFlowOperator)>>> {
    let mut result: Vec<Vec<(&'a str, WorkFlowOperator)>> = vec![];
    let workflow = workflows.get(key).unwrap();
    let mut sub_route = route.clone();

    // iterate over the operators; if there is a next step, recursively call this.
    for op in workflow.operators.iter() {
        match &op.result {
            WorkFlowResult::Next(next) => {
                let mut tmp = sub_route.clone();
                tmp.push((key, op.clone()));
                // If there is as valid next step, seach recursively
                if let Some(valid) = search_to_accept(next.as_str(), workflows, tmp) {
                    // add the recursion result to the result set
                    result.extend(valid);
                }
            }
            // if the result is accept, add the route to the result set
            WorkFlowResult::Accept => {
                let mut tmp = sub_route.clone();
                tmp.push((key, op.clone()));
                result.push(tmp);
            }
            // if the result is reject, do nothing
            _ => {}
        }
        //push the inverse route as the next step
        sub_route.push((key, op.get_inverse()));
    }
    // now check the final op
    match &workflow.final_op {
        WorkFlowResult::Next(next) => {
            sub_route.push((key, WorkFlowOperator::new('*', '<', 4001, next.as_str())));
            if let Some(valid) = search_to_accept(next.as_str(), workflows, sub_route.clone()) {
                result.extend(valid);
            }
        }
        // if the result is accept, add the route to the result set; note the janky '*' operator; this is because the final op is recorded
        // as a WorkFlowResult and not as a WorkFlowOperator.  This is a hack to get around that.
        WorkFlowResult::Accept => {
            sub_route.push((key, WorkFlowOperator::new('*', '<', 4001, "A")));
            result.push(sub_route);
        }
        // if the result is reject, do nothing
        _ => {}
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}
#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;
    const PART_ONE_ANSWER: i64 = 19114;
    const PART_ONE_TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    const PART_TWO_ANSWER: i64 = 167409079868000;
    const PART_TWO_TEST_HARD: &str = "in{a<2000:pop,A}
pop{a<1000:R,R}";

    const PART_TWO_TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    #[should_panic]
    fn panic_workflow() {
        let input = "Panic!";
        WorkFlowOperator::new('x', '!', 100, "A");
    }
    #[test]
    #[should_panic]
    fn panic_part_two() {
        let input = ProblemInput::String("kmt{x=2294:tlp,s>566:R,x=1268:A,A}");
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }

    #[test]
    fn fn_part_parser() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        let (_input, result) = part_parser(input).unwrap();

        let mut expected = Part {
            data: HashMap::new(),
        };
        expected.data.insert('x', 787);
        expected.data.insert('m', 2655);
        expected.data.insert('a', 1222);
        expected.data.insert('s', 2876);
        assert_eq!(result, expected);
    }

    #[test]
    fn fn_workflow_operator_parser() {
        let input = "x<3010:R";
        let (_input, result) = workflow_operator_parser(input).unwrap();
        assert!(result.key == 'x');
        assert!(result.comparitor == std::cmp::Ordering::Less);
        assert!(result.value == 3010);
        assert!(result.result == WorkFlowResult::Reject);
    }

    #[test]
    fn fn_workflow_parser() {
        let input = "kmt{x>2294:tlp,s>566:R,x>1268:A,A}";
        let (_input, (key, result)) = workflow_parser(input).unwrap();
        //assert!(result.op == 'x');
        assert!(key == "kmt");
        assert!(result.final_op == WorkFlowResult::Accept);
    }

    #[test]
    // tests a number of possible routes to accept aiming for the final result of Accept, Reject or Next
    fn fn_test_workflow() {
        let input = "{x=787,m=2655,a=1222,s=2876}";
        let (_input, part) = part_parser(input).unwrap();

        let input = "{x=787,m=2655,a=1222,s=276}";
        let (_input, part_end) = part_parser(input).unwrap();

        let input = "{x=7087,m=2655,a=1222,s=276}";
        let (_input, part_some) = part_parser(input).unwrap();

        let input = "kmt{x>2294:tlp,s>566:R,x>1268:A,A}";
        let (_input, (key, workflow)) = workflow_parser(input).unwrap();

        let result = workflow.send(part);
        let result_end = workflow.send(part_end);
        let result_some = workflow.send(part_some);
        assert!(result == WorkFlowResult::Reject);
        assert!(result_end == WorkFlowResult::Accept);
        assert!(result_some == WorkFlowResult::Next(("tlp").to_string()));
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}

#[cfg(not(tarpaulin_include))]
pub fn part_one() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_one(input), now.elapsed())
}

#[cfg(not(tarpaulin_include))]
pub fn part_two() -> (i64, std::time::Duration) {
    let now = Instant::now();
    let path = format!("./inputs/{}", the_day());
    let input = ProblemInput::File(path.as_str());
    let input = Input::new(input);
    (do_part_two(input), now.elapsed())
}
