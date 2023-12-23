use crate::utils::{Input, ProblemInput};
use kdam::tqdm;
use std::collections::HashMap;
use std::fmt::Display;
use std::{cmp::Ordering, collections::HashSet, fmt::Formatter, io::BufRead, time::Instant};

#[cfg(not(tarpaulin_include))]
fn the_day() -> u32 {
    22
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: i64,
    y: i64,
    z: i64,
}
impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    start: Location,
    end: Location,
    id: i64,
}
impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: {} -- {}]", self.id, self.start, self.end)
    }
}
impl Brick {
    fn new(start: Location, end: Location, id: i64) -> Brick {
        Brick { start, end, id }
    }

    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }
    fn get_lowest_location(&self) -> Location {
        let mut lowest = self.start;
        if self.end.z < self.start.z {
            lowest = self.end;
        }
        lowest
    }
    // fn get_highest_location(&self) -> Location {
    //     let mut highest = self.start;
    //     if self.end.z > self.start.z {
    //         highest = self.end;
    //     }
    //     highest
    // }
    fn get_all_locations(&self) -> Vec<Location> {
        let mut locations: Vec<Location> = Vec::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    locations.push(Location { x, y, z });
                }
            }
        }
        locations
    }
    fn get_above_faces(&self) -> Vec<Location> {
        match self.end.z.cmp(&self.start.z) {
            // horizontal plane
            Ordering::Equal => self.get_all_locations(),
            Ordering::Greater => vec![self.end],
            Ordering::Less => vec![self.start],
        }
    }
    fn get_below_faces(&self) -> Vec<Location> {
        let mut res = match self.end.z.cmp(&self.start.z) {
            // horizontal plane
            Ordering::Equal => self.get_all_locations(),
            Ordering::Greater => vec![self.start],
            Ordering::Less => vec![self.end],
        };
        // now convert to "below"
        for r in &mut res {
            r.z -= 1;
        }
        res
    }
}

fn parse(input: Vec<&str>) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        let line = line.split('~').collect::<Vec<&str>>();
        let start = line[0]
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let start = Location {
            x: start[0],
            y: start[1],
            z: start[2],
        };
        let end = line[1]
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let end = Location {
            x: end[0],
            y: end[1],
            z: end[2],
        };
        let loc = Brick::new(start, end, i as i64);
        bricks.push(loc);
    }
    bricks
}

fn do_part_one(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut bricks = parse(lines);

    bricks.sort_by(|a, b| a.get_lowest_location().z.cmp(&b.get_lowest_location().z));

    let max_x = bricks.iter().map(|b| b.end.x.max(b.start.x)).max().unwrap();
    let max_y = bricks.iter().map(|b| b.end.x.max(b.start.x)).max().unwrap();
    let mut new_bricks: Vec<Brick> = Vec::new();

    // create the floor
    let mut lower_surfaces: Vec<Location> = Vec::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            lower_surfaces.push(Location { x, y, z: 0 });
        }
    }

    for brick_start in bricks.iter() {
        let mut brick = *brick_start;
        let lower_faces = lower_surfaces.iter().collect::<HashSet<&Location>>();
        loop {
            let brick_lower_faces = brick.get_below_faces();
            let brick_lower_faces = brick_lower_faces.iter().collect::<HashSet<&Location>>();

            let intersection = lower_faces
                .intersection(&brick_lower_faces)
                .collect::<Vec<&&Location>>();
            if !intersection.is_empty() {
                break;
            }
            brick.move_down();
        }
        new_bricks.push(brick);
        lower_surfaces.extend(brick.get_above_faces());
    }

    // now need to build some sort of graph (i think)
    let mut can_disintergrate: HashSet<i64> = HashSet::new();
    let mut cannot_disintergrate: HashSet<i64> = HashSet::new();
    let mut upper_faces: HashMap<Location, i64> = HashMap::new();
    let mut lower_faces: HashMap<Location, i64> = HashMap::new();
    //let other_faces: HashMap<Location, i64> = HashMap::new();
    for brick in &new_bricks {
        upper_faces.extend(brick.get_above_faces().iter().map(|x| (*x, brick.id)));
        lower_faces.extend(brick.get_below_faces().iter().map(|x| (*x, brick.id)));
    }

    for brick in &new_bricks {
        let mut supporting: HashSet<i64> = HashSet::new();
        for face in brick.get_below_faces() {
            if upper_faces.contains_key(&face) {
                supporting.insert(*upper_faces.get(&face).unwrap());
            }
        }
        if supporting.len() > 1 || supporting.is_empty() {
            can_disintergrate.extend(supporting);
        } else if supporting.len() == 1 {
            cannot_disintergrate.extend(supporting);
        }

        let empty_surface = brick
            .get_above_faces()
            .iter()
            .all(|x| !lower_faces.contains_key(x));
        if empty_surface {
            can_disintergrate.insert(brick.id);
        }
    }

    can_disintergrate.retain(|x| !cannot_disintergrate.contains(x));
    //can_disintergrate.insert(new_bricks.last().unwrap().id);
    // println!("Can disintergrate: {:?}", can_disintergrate);
    can_disintergrate.len() as i64
}

fn do_part_two(input: Input) -> i64 {
    let lines = input.get_data().lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    let mut lower_surfaces_lookup: HashMap<i64, Vec<Location>> = HashMap::new();
    let mut bricks = parse(lines);

    bricks.sort_by(|a, b| a.get_lowest_location().z.cmp(&b.get_lowest_location().z));

    let max_x = bricks.iter().map(|b| b.end.x.max(b.start.x)).max().unwrap();
    let max_y = bricks.iter().map(|b| b.end.x.max(b.start.x)).max().unwrap();
    let mut new_bricks: Vec<Brick> = Vec::new();

    // stack the bricks

    //create the floor
    let mut lower_surfaces: Vec<Location> = Vec::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            lower_surfaces.push(Location { x, y, z: 0 });
        }
    }
    for (_i, brick_start) in bricks.iter().enumerate() {
        let mut brick = *brick_start;
        let lower_faces = lower_surfaces
            .iter()
            .copied()
            .collect::<HashSet<Location>>();

        loop {
            let brick_lower_faces = brick.get_below_faces();
            let brick_lower_faces = brick_lower_faces
                .iter()
                .copied()
                .collect::<HashSet<Location>>();

            let intersection = lower_faces
                .intersection(&brick_lower_faces)
                .collect::<Vec<&Location>>();
            if !intersection.is_empty() {
                break;
            }
            brick.move_down();
        }
        new_bricks.push(brick);
        lower_surfaces_lookup.insert(brick.id, lower_surfaces.clone());
        lower_surfaces.extend(brick.get_above_faces());
    }

    // now remove each one and count the movements.

    let bricks_to_remove = new_bricks.clone();

    // top to bottom

    let mut floor_lower_surfaces: Vec<Location> = Vec::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            floor_lower_surfaces.push(Location { x, y, z: 0 });
        }
    }

    let mut count = 0;
    for (i, brick_to_remove) in tqdm!(bricks_to_remove.iter().enumerate()) {
        // create the floor
        //let lower_surfaces: Vec<Location> = floor_lower_surfaces.clone();
        let mut bricks_that_moved: HashSet<i64> = HashSet::new();

        let mut lower_surfaces = lower_surfaces_lookup
            .get(&brick_to_remove.id)
            .unwrap()
            .clone();
        for brick_start in new_bricks.iter().skip(i + 1) {
            let mut brick = *brick_start;
            let lower_faces = lower_surfaces
                .iter()
                .copied()
                .collect::<HashSet<Location>>();
            loop {
                let brick_lower_faces = brick.get_below_faces();
                let brick_lower_faces = brick_lower_faces
                    .iter()
                    .copied()
                    .collect::<HashSet<Location>>();
                let intersection = lower_faces
                    .intersection(&brick_lower_faces)
                    .collect::<Vec<&Location>>();
                //println!("{:?}", intersection);
                if !intersection.is_empty() {
                    break;
                }
                brick.move_down();
                bricks_that_moved.insert(brick.id);
                //println!("\t{} moved", brick.id);
            }
            //println!("{} -> {}", brick_start, brick);
            //new_bricks.push(brick.clone());
            lower_surfaces.extend(brick.get_above_faces());
        }

        count += bricks_that_moved.len();
    }

    count as i64
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use crate::utils::ProblemInput;

    const PART_ONE_ANSWER: i64 = 5;
    const PART_ONE_TEST: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    const PART_ONE_TEST_2: &str = "0,0,1~0,1,1
1,1,1~1,1,1
0,0,2~0,0,2
0,1,2~1,1,2";

    const PART_ONE_ANSWER_3: i64 = 2;
    const PART_ONE_TEST_3: &str = "0,0,1~1,0,1
0,1,1~0,1,2
0,0,5~0,0,5
0,0,4~0,1,4";

    const PART_ONE_ANSWER_2: i64 = 3;

    const PART_TWO_ANSWER: i64 = 7;
    const PART_TWO_TEST: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn fn_() {
        let input = "123";
        let result = input.parse::<i32>().unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn one() {
        let input = ProblemInput::String(PART_ONE_TEST);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER);

        let input = ProblemInput::String(PART_ONE_TEST_2);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER_2);

        let input = ProblemInput::String(PART_ONE_TEST_3);
        let result = do_part_one(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_ONE_ANSWER_3);
    }

    #[test]
    fn two() {
        let input = ProblemInput::String(PART_TWO_TEST);
        let result = do_part_two(Input::new(input));
        println!("Result: {}", result);
        assert_eq!(result, PART_TWO_ANSWER);
    }
}
