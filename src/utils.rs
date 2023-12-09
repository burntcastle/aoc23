use std::io::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Error},
    path::Path,
};

#[derive(Clone, Copy)]
pub enum ProblemInput<'a> {
    File(&'a str),

    #[allow(unused)]
    String(&'a str),
}

// this abstracts the input between strings (for testing) and files (for real)
pub struct Input<'a> {
    data: ProblemInput<'a>,
}
impl Input<'_> {
    pub fn new(input: ProblemInput) -> Input {
        Input { data: input }
    }

    pub fn get_data(&self) -> Box<dyn BufRead + '_> {
        let result: Box<dyn BufRead> = match &self.data {
            ProblemInput::File(x) => {
                let path = Path::new(x);
                let file = match File::open(path) {
                    Err(why) => panic!(
                        "couldn't open {}: {}",
                        path.display(),
                        Error::to_string(&why)
                    ),
                    Ok(file) => file,
                };
                let reader = BufReader::new(file);
                Box::new(reader)
            }
            ProblemInput::String(x) => {
                let reader = BufReader::new(x.as_bytes());
                Box::new(reader)
            }
        };
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn input_file() {
        let input = ProblemInput::File("./inputs/test");
        let input = Input::new(input);
        let mut lines = input.get_data().lines();
        let line = lines.next().unwrap().unwrap();
        assert_eq!(line, "test");
    }
    #[test]
    fn input_string() {
        let input = ProblemInput::String("1721");
        let input = Input::new(input);
        let mut lines = input.get_data().lines();
        let line = lines.next().unwrap().unwrap();
        assert_eq!(line, "1721");
    }
    #[test]
    #[should_panic]
    fn panic() {
        let input = ProblemInput::File("./inputs/no_file");
        let input = Input::new(input);
        let mut lines = input.get_data().lines();
        let line = lines.next().unwrap().unwrap();
        assert_eq!(line, "test");
    }
}
