use core::num;
use std::str;
use std::io::prelude::*;
use std::{io::{BufReader, Error}, path::Path, fs::File};

pub fn  part_one() -> u32 {
    
    let path = Path::new("./inputs/1/one");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    
    let reader = BufReader::new(file);
    let lines = reader.lines(); 

    let numbers = vec!["zero","one","two","three","four","five","six","seven","eight","nine"];
    let reverse_numbers: Vec<String> = numbers.clone().iter().map(|x| x.chars().rev().collect::<String>()).collect();

    let mut total  = 0;
    for l in lines {
        let line = l.unwrap();
        let mut char_vec: Vec<char> = (&line).chars().collect();
        let mut first_number = 0;
        let mut number_location = 0;

        for c in &char_vec {
            match c.to_digit(10){
                Some(x) => { first_number = x; break },
                None => { number_location += 1; } ,
            }
        }
   
        let mut second_number = 0;
        number_location = 0;
        char_vec.reverse();
        for c in &char_vec {
            match c.to_digit(10){
                Some(x) => { second_number = x; break },
                None => { number_location += 1; } ,
            }
        }
      
        total += (first_number*10) + second_number;
    }
    total
}



pub fn  part_two() -> u32 {
    
    let path = Path::new("./inputs/1/one");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::to_string(&why)),
        Ok(file) => file,
    };
    
    let reader = BufReader::new(file);
    let lines = reader.lines(); 

    let numbers = vec!["zero","one","two","three","four","five","six","seven","eight","nine"];
    let reverse_numbers: Vec<String> = numbers.clone().iter().map(|x| x.chars().rev().collect::<String>()).collect();
    let mut total  = 0;
    for l in lines {
        let line = l.unwrap();
        let mut char_vec: Vec<char> = (&line).chars().collect();
        let mut first_number = 0;
        let mut number_location = 0;

        for c in &char_vec {
            match c.to_digit(10){
                Some(x) => { first_number = x; break },
                None => { number_location += 1; } ,
            }
        }
        let mut i = 0 ;
        for num in &numbers {
            match line.find(num){
                Some(x) => {
                    if x < number_location{
                        first_number = i ;
                        number_location = x;
                    }                  
                },
                None => {}
            }
            i+=1;
        }

        let mut second_number = 0;
        number_location = 0;
        char_vec.reverse();
        for c in &char_vec {
            match c.to_digit(10){
                Some(x) => { second_number = x; break },
                None => { number_location += 1; } ,
            }
        }
        let line: String = char_vec.iter().collect();
        let mut i = 0 ;
        for num in &reverse_numbers {
            let num = num.as_str();
            
            match line.find(num){
                Some(x) => {
                    if x < number_location{
                        second_number = i;
                        number_location = x
                    }                  
                },
                None => {}
            }
            i+=1;
        }
        total += (first_number*10) + second_number;
    }
    total
}