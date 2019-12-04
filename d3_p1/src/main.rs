use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filename = "../inputs/d3.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    //let mut directions: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    //R990
    let directions: Vec<String> = lines[0].split(",").map(|s| s.to_string()).collect();
    let mut locations = HashSet::new();
    
    //start at 0,0
    let mut masterX = 0;
    let mut masterY = 0;

    locations.insert((masterX,masterY));
    //locations.insert(format!("{}:{}", masterX, masterY));

    for dir in directions {
        // let val: String = dir.chars().skip(1).take(dir.len()-1).collect();
        let raw: String = dir.chars().skip(1).take(dir.len()-1).collect();//.parse::<i32>().unwrap();.
        let val = raw.parse::<i32>().unwrap();
        //println!("move {}",val);
        let mut rangeX = masterX..masterX;
        let mut rangeY = masterY..masterY; 
        match dir.chars().nth(0).unwrap() {
           'R' => {
               rangeX = masterX..masterX+val+1;
               masterX+=val;
           },
           'D' =>{
                rangeY = masterY-val..masterY+1;
                masterY-=val;
           },
           'L' => {
               rangeX = masterX-val..masterX+1;
               masterX-=val;
           },
           'U' => {
               rangeY = masterY..masterY+val+1;
               masterY+=val;
           },
            _ => println!("UNKNOWN"),
        };

        for x in rangeX{
            locations.insert((x, masterY));
        }
        
        for y in rangeY{
            locations.insert((masterX, y));
        }
    }
        
    let directions: Vec<String> = lines[1].split(",").map(|s| s.to_string()).collect();
    let mut masterX = 0;
    let mut masterY = 0;

    let mut cross = HashSet::new();
    for dir in directions {
        // let val: String = dir.chars().skip(1).take(dir.len()-1).collect();
        let raw: String = dir.chars().skip(1).take(dir.len()-1).collect();//.parse::<i32>().unwrap();.
        let val = raw.parse::<i32>().unwrap();
        //println!("{}",val); 
        let mut rangeX = masterX..masterX;
        let mut rangeY = masterY..masterY; 
        match dir.chars().nth(0).unwrap() {
           'R' => {
               rangeX = masterX..masterX+val+1;
               masterX+=val;
           },
           'D' =>{
                rangeY = masterY-val..masterY+1;
                masterY-=val;
           },
           'L' => {
               rangeX = masterX-val..masterX+1;
               masterX-=val;
           },
           'U' => {
               rangeY = masterY..masterY+val+1;
               masterY+=val;
           },
            _ => println!("UNKNOWN"),
        };

        //for x in masterX..(masterX+changeX+1){
        for x in rangeX {
            //println!("{}:{}",x,masterY);
            if(locations.contains(&(x, masterY))){
                cross.insert((x, masterY));
            }
        }
        // masterX+=changeX;
        
        //for y in masterY..(masterY+changeY+1){
        for y in rangeY {
            //println!("{}:{}",masterX,y);
            if(locations.contains(&(masterX, y))){
                cross.insert((masterX, y));
            }
        }
        // masterY+=changeY;
    }
    //println!("{:?}",cross);
    let mut min = std::i32::MAX;
    for (x, y) in cross {
        let val = x.abs() + y.abs();
        if ( val<min && val != 0){
            min = val;
        }
    }
   
    //println!("{:?}",locations);
    println!("{}",min);
}
