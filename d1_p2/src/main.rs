use std::env;
use std::fs;

fn main() {
    // --snip--
    let filename = "../inputs/d1.txt";

    // println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();//::<Vec<&str>>();
    let mut total = 0;
    for line in lines {
        //println!("{}",line);
        
        let mut mass = line.parse::<i32>().unwrap();
        while(mass>0){
            let result = (mass/3) - 2;
            if(result>0){
                // println!("{}",result);
                total = total + result;
            }
            mass = result
        }
    }
    //println!("With text:\n{}", contents);
    println!("{}",total);
}