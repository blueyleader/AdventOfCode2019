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

    let mut paths:HashSet<(i32,i32)> = HashSet::new();
    paths.insert((0,1));
    paths.insert((1,0));

    for y in 1..size {
        for x in 1..size {
            let dom = gcd(y,x);
            paths.insert((x/dom, y/dom));
        }
    } 
    // let path2:Vec<(i32,i32)> = paths.iter().map(|(x,y)| (*x as i32,*y as i32)).collect();
    // let mut sort_path:Vec<(i32,i32)> = paths.iter().map(|(x,y)| (*x,*y)).collect(); //.sort_by_key( |k| k.1 ).collect();
    // sort_path.sort_by_key(|k| if k.0==0 { i32::min_value() } else if k.1 == 0 { i32::max_value() } else { println!("{}", k.0 as f64/k.1 as f64);(k.0 as f64/k.1 as f64 * 10000.0) as i32} );

    let mut location = (0,0);
    let dir: Vec<(i32,i32)> = vec![(1,-1),(1,1),(-1,1),(-1,-1)];
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
        if(max_value==count){
            location = astroid;
        }
        
    }
    // println!("{:?} {}",location,max_value);
    let mut sort_path:Vec<(i32,i32)> = paths.iter().map(|(x,y)| (*x,*y)).collect();
    sort_path.sort_by_key(|k| if k.0==0 { i32::min_value() } else if k.1 == 0 { i32::max_value() } else {(k.0 as f64/k.1 as f64 * 10000.0) as i32} );

    let mut last = (0,0);
    let mut count = 0;
    'master: while(true){
        let dit_tmp = dir[0];
        for p in sort_path.clone() {
            if((p.0==0 && dit_tmp.0 == -1) || (p.1==0 && dit_tmp.1 == -1)){
                continue;
            }
            let mut x_tmp = location.0 + (p.0 * dit_tmp.0);
            let mut y_tmp = location.1 + (p.1 * dit_tmp.1);
            while (x_tmp>=0 && x_tmp<size && y_tmp>=0 && y_tmp<size){
                // println!("{} {}", x_tmp, y_tmp);
                if map.contains(&(x_tmp,y_tmp)){
                    // println!("found colision at {} {}", x_tmp,y_tmp);
                    map.remove(&(x_tmp,y_tmp));
                    last = (x_tmp,y_tmp);
                    count+=1;
                    // println!("{:?} {}", last, count);
                    if(count==200){
                        break 'master;
                    }
                    break;
                }
                x_tmp+=(p.0 * dit_tmp.0);
                y_tmp+=(p.1 * dit_tmp.1); 
            } 
        }
        if(count == 200){
            break;
        }

        let dit_tmp = dir[1];
        for p in sort_path.clone().iter().rev() {
            if((p.0==0 && dit_tmp.0 == -1) || (p.1==0 && dit_tmp.1 == -1)){
                continue;
            }
            let mut x_tmp = location.0 + (p.0 * dit_tmp.0);
            let mut y_tmp = location.1 + (p.1 * dit_tmp.1);
            while (x_tmp>=0 && x_tmp<size && y_tmp>=0 && y_tmp<size){
                // println!("{} {}", x_tmp, y_tmp);
                if map.contains(&(x_tmp,y_tmp)){
                    // println!("found colision at {} {}", x_tmp,y_tmp);
                    map.remove(&(x_tmp,y_tmp));
                    last = (x_tmp,y_tmp);
                    count+=1;
                    // println!("{:?} {}", last, count);
                    if(count==200){
                        break 'master;
                    }
                    break;
                }
                x_tmp+=(p.0 * dit_tmp.0);
                y_tmp+=(p.1 * dit_tmp.1); 
            } 
        }
        if(count == 200){
            break;
        }

        let dit_tmp = dir[2];
        for p in sort_path.clone() {
            if((p.0==0 && dit_tmp.0 == -1) || (p.1==0 && dit_tmp.1 == -1)){
                continue;
            }
            let mut x_tmp = location.0 + (p.0 * dit_tmp.0);
            let mut y_tmp = location.1 + (p.1 * dit_tmp.1);
            while (x_tmp>=0 && x_tmp<size && y_tmp>=0 && y_tmp<size){
                // println!("{} {}", x_tmp, y_tmp);
                if map.contains(&(x_tmp,y_tmp)){
                    // println!("found colision at {} {}", x_tmp,y_tmp);
                    map.remove(&(x_tmp,y_tmp));
                    last = (x_tmp,y_tmp);
                    count+=1;
                    // println!("{:?} {}", last, count);
                    if(count==200){
                        break 'master;
                    }
                    break;
                }
                x_tmp+=(p.0 * dit_tmp.0);
                y_tmp+=(p.1 * dit_tmp.1); 
            } 
        }
        if(count == 200){
            break;
        }

        let dit_tmp = dir[3];
        for p in sort_path.clone().iter().rev() {
            if((p.0==0 && dit_tmp.0 == -1) || (p.1==0 && dit_tmp.1 == -1)){
                continue;
            }
            let mut x_tmp = location.0 + (p.0 * dit_tmp.0);
            let mut y_tmp = location.1 + (p.1 * dit_tmp.1);
            while (x_tmp>=0 && x_tmp<size && y_tmp>=0 && y_tmp<size){
                // println!("{} {}", x_tmp, y_tmp);
                if map.contains(&(x_tmp,y_tmp)){
                    // println!("found colision at {} {}", x_tmp,y_tmp);
                    map.remove(&(x_tmp,y_tmp));
                    last = (x_tmp,y_tmp);
                    count+=1;
                    // println!("{:?} {}", last, count);
                    if(count==200){
                        break 'master;
                    }
                    break;
                }
                x_tmp+=(p.0 * dit_tmp.0);
                y_tmp+=(p.1 * dit_tmp.1); 
            } 
        }
        
    }
    // println!("{:?}",count);
    // println!("{:?}",last);
    println!("{}",last.0*100+last.1);


}
