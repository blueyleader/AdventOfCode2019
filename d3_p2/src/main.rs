use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filename = "../inputs/d3.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    //let mut directions: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    //R990
    let directions: Vec<String> = lines[0].split(",").map(|s| s.to_string()).collect();
    let mut locations = HashMap::new();
    
    //let mut path1: Vec<(i32,i32,i32)>; // x, y, step
    //start at 0,0
    let mut masterX = 0;
    let mut masterY = 0;
    let mut step: i32 = 0;
    
    locations.insert((masterX,masterY),step);
    //locations.insert(format!("{}:{}", masterX, masterY));

    for dir in directions {
        // let val: String = dir.chars().skip(1).take(dir.len()-1).collect();
        let raw: String = dir.chars().skip(1).take(dir.len()-1).collect();//.parse::<i32>().unwrap();.
        let val = raw.parse::<i32>().unwrap();
        //println!("{}",val); 
        let mut rangeX = (masterX..masterX).collect::<Vec<i32>>();
        let mut rangeY = (masterY..masterY).collect::<Vec<i32>>();
        match dir.chars().nth(0).unwrap() {
           'R' => {
               rangeX = (masterX..masterX+val+1).collect::<Vec<i32>>();
               masterX+=val;
           },
           'D' =>{
                rangeY = (masterY-val..masterY+1).rev().collect::<Vec<i32>>();
                masterY-=val;
           },
           'L' => {
               rangeX = (masterX-val..masterX+1).rev().collect::<Vec<i32>>();
               masterX-=val;
           },
           'U' => {
               rangeY = (masterY..masterY+val+1).collect::<Vec<i32>>();
               masterY+=val;
           },
            _ => println!("UNKNOWN"),
        };

        for x in rangeX{
            if(!locations.contains_key(&(x, masterY))){
                step+=1;
                locations.insert((x, masterY),step);
            }
        }
        
        for y in rangeY{
            if(!locations.contains_key(&(masterX, y))){
                step+=1;
                locations.insert((masterX, y),step);
            }
        }
    }
        
    let directions: Vec<String> = lines[1].split(",").map(|s| s.to_string()).collect();
    let mut locations2 = HashMap::new();
    let mut masterX = 0;
    let mut masterY = 0;
    let mut step: i32 = 0;
    
    locations2.insert((masterX,masterY),step);
    let mut cross = HashSet::new();
    for dir in directions {
        // let val: String = dir.chars().skip(1).take(dir.len()-1).collect();
        let raw: String = dir.chars().skip(1).take(dir.len()-1).collect();//.parse::<i32>().unwrap();.
        let val = raw.parse::<i32>().unwrap();
        //println!("{}",val); 
        let mut rangeX = (masterX..masterX).collect::<Vec<i32>>();
        let mut rangeY = (masterY..masterY).collect::<Vec<i32>>();
        match dir.chars().nth(0).unwrap() {
           'R' => {
               rangeX = (masterX..masterX+val+1).collect::<Vec<i32>>();
               masterX+=val;
           },
           'D' =>{
                rangeY = (masterY-val..masterY+1).rev().collect::<Vec<i32>>();
                masterY-=val;
           },
           'L' => {
               rangeX = (masterX-val..masterX+1).rev().collect::<Vec<i32>>();
               masterX-=val;
           },
           'U' => {
               rangeY = (masterY..masterY+val+1).collect::<Vec<i32>>();
               masterY+=val;
           },
            _ => println!("UNKNOWN"),
        };

        for x in rangeX{
            if(!locations2.contains_key(&(x, masterY))){
                step+=1;
                locations2.insert((x, masterY),step);
            }

            if(locations.contains_key(&(x, masterY))){
                cross.insert((x, masterY));
            }
        }
        
        for y in rangeY{
            if(!locations2.contains_key(&(masterX, y))){
                step+=1;
                locations2.insert((masterX, y),step);
            }
            if(locations.contains_key(&(masterX, y))){
                cross.insert((masterX, y));
            }
        }

        // for x in rangeX{
        //     locations2.insert((x, masterY));
        //     if(locations.contains_key(&(x, masterY))){
        //         cross.insert((x, masterY));
        //     }
        // }
        // //masterX+=changeX;
        
        // for y in rangeY{
        //     locations2.insert((masterX, y));
        //     if(locations.contains_key(&(masterX, y))){
        //         cross.insert((masterX, y));
        //     }
        // }
        // masterY+=changeY;
    }
    // println!("{:?}",cross); 
    // println!("{:?}",locations);
    let mut min = std::i32::MAX;
    for (x, y) in cross {
        //println!("cross {}:{}",x,y);
        if(x!=0 || y!=0){
            let dis1=*locations.get(&(x, y)).unwrap();
            let dis2=*locations2.get(&(x, y)).unwrap();
            //println!("{} + {} = {}",dis1, dis2, dis1+dis2);
            if(dis1+dis2+1<min){
                min=dis1+dis2+1;
            }
        }
    }
    println!("{}",min);
}

// fn trace(steps: i32, x: i32, y: i32, mut loc: HashSet<(i32,i32)>) -> i32{
//     if( x==0 && y==0 ){
//         println!("found path {}",steps);
//         return steps;
//     }
//     //let mut tmpLoc = loc.clone();
//     let mut min = std::i32::MAX;

//     if(!loc.contains(&(x,y))){
//         // println!("point not found {}:{}",x,y);
//         return min;
//     }
//     println!("{}:{}",x,y);
//     loc.remove(&(x,y));
    
//     //right
//     let tmp = trace(steps+1,x+1,y,loc.clone());
//     if(tmp<min){
//         min=tmp;    
//     }
//     if(steps>min){
//         return min;
//     }
//     //down
//     let tmp = trace(steps+1,x,y-1,loc.clone());
//     if(tmp<min){
//         min=tmp;    
//     }
//     if(steps>min){
//         return min;
//     }
//     //left
//     let tmp = trace(steps+1,x-1,y,loc.clone());
//     if(tmp<min){
//         min=tmp;    
//     }
//     if(steps>min){
//         return min;
//     }
//     //up
//     let tmp = trace(steps+1,x,y+1,loc.clone());
//     if(tmp<min){
//         min=tmp;    
//     }    
//     return min;
// }