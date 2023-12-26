use crate::utils::{Input, ProblemInput};
use std::{io::BufRead, time::Instant};
#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    24
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

#[derive(PartialEq, Clone, Copy, Debug)]
struct HailStone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}
impl HailStone {
    fn at_time(&self, t: f64) -> (f64, f64, f64) {
        (
            self.x + self.vx * t,
            self.y + self.vy * t,
            self.z + self.vz * t,
        )
    }
    fn adjust_velocity(&self, vx: f64, vy: f64, vz: f64) -> HailStone {
        HailStone {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx + vx,
            vy: self.vy + vy,
            vz: self.vz + vz,
        }
    }
}
fn parse(input: Vec<&str>) -> Vec<HailStone> {
    let mut result: Vec<HailStone> = Vec::new();

    for line in input {
        let line = line.replace(" @", ",");
        let line = line
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        result.push(HailStone {
            x: line[0].parse::<f64>().unwrap(),
            y: line[1].parse::<f64>().unwrap(),
            z: line[2].parse::<f64>().unwrap(),
            vx: line[3].parse::<f64>().unwrap(),
            vy: line[4].parse::<f64>().unwrap(),
            vz: line[5].parse::<f64>().unwrap(),
        });
    }
    result
}

fn does_intersect_2d(
    hailstone: &HailStone,
    other_hailstone: &HailStone,
) -> Option<(f64, f64, f64, f64)> {
    let t = (hailstone.x - other_hailstone.x) / other_hailstone.vx
        - (hailstone.y - other_hailstone.y) / other_hailstone.vy;
    let t = t / (hailstone.vy / other_hailstone.vy - hailstone.vx / other_hailstone.vx);
    let t_prime = (hailstone.x - other_hailstone.x + t * hailstone.vx) / other_hailstone.vx;
    let (x, y, _) = hailstone.at_time(t);
    Some((x, y, t, t_prime))
}

fn does_intersect_3d(
    hailstone: &HailStone,
    other_hailstone: &HailStone,
) -> Option<(f64, f64, f64, f64, f64)> {
    let t = (hailstone.x - other_hailstone.x) / other_hailstone.vx
        - (hailstone.y - other_hailstone.y) / other_hailstone.vy;
    let t = t / (hailstone.vy / other_hailstone.vy - hailstone.vx / other_hailstone.vx);
    let t_prime = (hailstone.x - other_hailstone.x + t * hailstone.vx) / other_hailstone.vx;
    let (x, y, z) = hailstone.at_time(t);
    let (_, _, z_prime) = other_hailstone.at_time(t_prime);
    if z.round() != z_prime.round() {
        return None;
    }
    Some((x, y, z, t, t_prime))
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let hailstones = parse(lines);

    const MIN: f64 = 200000000000000.0;
    const MAX: f64 = 400000000000000.0;

    let mut count = 0;
    for (i, hailstone) in hailstones.iter().enumerate() {
        for (_j, other_hailstone) in hailstones.iter().skip(i + 1).enumerate() {
            if let Some((x, y, t, t_prime)) = does_intersect_2d(hailstone, other_hailstone) {
                if (MIN..MAX).contains(&x) && (MIN..MAX).contains(&y) && t > 0.0 && t_prime > 0.0 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let hailstones = parse(lines);

    let top_five = hailstones.to_vec();
    // .iter()
    // //.take(50)
    // .copied()
    // .collect::<Vec<HailStone>>();
    let max_vel = 400;
    let min_vel = -400;
    let mut accurate = (0, 0, 0);
    for vx in min_vel..max_vel {
        for vy in min_vel..max_vel {
            for vz in 0..1 {
                let modified: Vec<HailStone> = top_five
                    .iter()
                    .map(|x| x.adjust_velocity(vx as f64, vy as f64, vz as f64))
                    .collect();

                let initial = *modified.first().unwrap();
                let mut found_it = true;
                let mut last_point = Option::<(f64, f64)>::None;
                for next in modified.iter().skip(1) {
                    if let Some((x, y, _, _)) = does_intersect_2d(&initial, next) {
                        match last_point {
                            Some((x_prime, y_prime)) => {
                                if x_prime.round() != x.round() || y_prime.round() != y.round() {
                                    found_it = false;
                                    break;
                                }
                            }
                            None => {
                                last_point = Some((x, y));
                            }
                        }
                    } else {
                        found_it = false;
                        break;
                    }
                }
                if found_it {
                    accurate = (vx, vy, 0);
                }
            }
        }
    }

    let vx = accurate.0;
    let vy = accurate.1;
    for vz in min_vel..max_vel {
        let modified: Vec<HailStone> = top_five
            .iter()
            .map(|x| x.adjust_velocity(vx as f64, vy as f64, vz as f64))
            .collect();

        let initial = *modified.first().unwrap();
        let mut found_it = true;
        let mut last_point = Option::<(f64, f64, f64)>::None;
        for next in modified.iter().skip(1) {
            if let Some((x, y, z, _, _)) = does_intersect_3d(&initial, next) {
                match last_point {
                    Some((x_prime, y_prime, z_prime)) => {
                        if x_prime.round() != x.round()
                            || y_prime.round() != y.round()
                            || z.round() != z_prime.round()
                        {
                            found_it = false;
                            break;
                        }
                    }
                    None => {
                        last_point = Some((x, y, z));
                    }
                }
            } else {
                found_it = false;
                break;
            }
        }
        if found_it {
            return last_point.unwrap().0 as i64
                + last_point.unwrap().1 as i64
                + last_point.unwrap().2 as i64;
        }
    }
    0
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct HailStone2 {
    x: i128,
    y: i128,
    z: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use rayon::vec;

    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 2;
    const PART_ONE_TEST: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    const PART_TWO_ANSWER: i64 = 47;
    const PART_TWO_TEST: &str = "19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3";

    #[test]
    #[should_panic]
    fn panics() {
        parse(vec!["19, A, 30 @ -2,  1, -2"]);
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let input = Input::new(input);
        let lines = input.get_data().lines();
        let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
        let hailstones = parse(lines);

        const MIN: f64 = 7.0;
        const MAX: f64 = 27.0;

        let mut count = 0;
        for (i, hailstone) in hailstones.iter().enumerate() {
            for (_j, other_hailstone) in hailstones.iter().skip(i + 1).enumerate() {
                if let Some((x, y, t, t_prime)) = does_intersect_2d(hailstone, other_hailstone) {
                    if (MIN..MAX).contains(&x)
                        && (MIN..MAX).contains(&y)
                        && t > 0.0
                        && t_prime > 0.0
                    {
                        count += 1;
                    }
                }
            }
        }

        assert_eq!(count, PART_ONE_ANSWER);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
