use std::env;
use std::fs;
use std::io::Read;
use num::integer::gcd;
use std::collections::HashSet;
use std::cmp::max;

fn main() {
    let filename = "../inputs/d10.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let size = 34;
    // let mut map: Vec<Vec<bool>> = vec![vec![false; size]; size];
    let mut map = HashSet::new();
    let mut x=0;
    let mut y=0;

    for line in lines {
        for c in line.chars() {
            if c=='#' {
                map.insert((x,y));
            }
            x+=1;
        }
        y+=1;
        x=0;
    } 

    let mut max_value = i32::min_value();

    let mut paths = HashSet::new();
    paths.insert((0,1));
    paths.insert((1,0));

    for y in 1..size {
        for x in 1..size {
            let dom = gcd(y,x);
            paths.insert((x/dom, y/dom));
        }
    } 

    let dir: Vec<(i32,i32)> = vec![(1,1),(1,-1),(-1,1),(-1,-1)];
    for astroid in map.clone() {
        let mut count = 0;
        //println!("astroid is {:?}",astroid);
        for p in paths.clone() {
            //println!("path is {:?}",p);
            for d in &dir{
                if((p.0==0 && d.0 == -1) || (p.1==0 && d.1 == -1)){
                    continue;
                }
                let mut x_tmp = astroid.0 + (p.0 * d.0);
                let mut y_tmp = astroid.1 + (p.1 * d.1);
                
                while (x_tmp>=0 && x_tmp<size && y_tmp>=0 && y_tmp<size){
                    // println!("{} {}", x_tmp, y_tmp);
                    if map.contains(&(x_tmp,y_tmp)){
                        // println!("found colision at {} {}", x_tmp,y_tmp);
                        count+=1;
                        break;
                    }
                    x_tmp+=(p.0 * d.0);
                    y_tmp+=(p.1 * d.1); 
                } 

            }
        }

        //println!("{:?} {}",astroid,count);
        max_value = max_value.max(count);
    }

    println!("{}", max_value);
}
