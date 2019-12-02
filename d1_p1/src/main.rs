use std::env;
use std::fs;

fn main() {
    let filename = "../inputs/d1.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();//::<Vec<&str>>();
    
    let mut total = 0;
    for line in lines {
        //println!("{}",line);
        let mass = line.parse::<i32>().unwrap();
        let result = (mass/3) - 2;
        //println!("{}",result);
        total = total + result;

    }
    println!("{}",total);
}