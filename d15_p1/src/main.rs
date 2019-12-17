use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    const RADIX: u32 = 10;

    let filename = "../inputs/d15.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut cmds: Vec<i64> = contents.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    cmds.resize(10000,0);
    let mut n = 0;
    let mut relative = 0;
    let mut pos :(i64, i64) = (25,25);
    let mut map: HashMap<(i64,i64),i64> = HashMap::new();
    let mut min = i64::max_value();
    let step =0;
    while(n < cmds.len()){
        
        let opt = cmds[n] % 100; //rightmost two digits are opt code
        // println!("{} {}",n, opt);
        let tmp = (cmds[n]/100).to_string();
        let mut modes: Vec<i64> = tmp.chars().rev().map(|x| x.to_digit(RADIX).unwrap() as i64).collect(); // 0=position 1=immediate 2=relative none=position 
        modes.resize_with(4,Default::default);
        // println!("{:?}",cmds);
        
        match opt {
            1 => { //add: p1+p2->p3
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                cmds[out as usize] = p1 + p2;
                n+=4;
            },
            2 => { //multiply: p1*p2->p3
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                cmds[out as usize] = p1 * p2;
                n+=4;
            },
            3 => { //input: input->p1
                let out = get_value_out(cmds.clone(), modes[0], n+1, relative);
                n+=2;

                let moves = vec![(0,1),(0,-1),(1,0),(-1,0)];
                for (m, d) in moves.iter().zip(1..5) {
                    let x = pos.0 + m.0;
                    let y = pos.1 + m.1;
                    // println!("{}:{}",x,y);
                    if map.contains_key(&(x,y)){
                        continue;
                    }
                    println!("{}:{} {:?}",x,y,m);
                    cmds[out as usize] = d;
                    let (a, b) = robot_move(cmds.clone(), n.clone(), relative.clone(), map, (x,y), step+1, min);
                    if b<min {
                        min = b;
                    }
                    map = a;
                }
                println!("master out {}",min);
                break;
                //return (map, min);
            },
            4 => { //output: p1->output
                // println!("{}",get_value(cmds.clone(), modes[0], n+1, relative));
                let val=get_value(cmds.clone(), modes[0], n+1, relative);
                map.insert(pos,val);
                // match val {
                //     0 => {
                //         return (map, min);
                //     },
                //     2=> {
                //         return (map, step);
                //     }
                //     _ =>
                //     {

                //     }
                // }
                // println!("{}",min);
                map.insert(pos,get_value(cmds.clone(), modes[0], n+1, relative));

                n+=2;
            },
            5 => { //jump-if-true: if p1!=0 then p2->n
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                if(p1 != 0){
                    n = get_value(cmds.clone(), modes[1], n+2, relative) as usize;
                    continue;
                }
                n+=3;
            },
            6 => { //jump-if-false: if p1==0 then p2->n   
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                if(p1 == 0){
                    n = get_value(cmds.clone(), modes[1], n+2, relative) as usize;
                    continue;
                }
                n+=3;
            },
            7 => { //less than: if p1<p2 then 1->p3 else 0->p3 
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                if(p1<p2){
                    cmds[out as usize] = 1;
                }
                else{
                    cmds[out as usize] = 0;
                }
                n+=4;
            },
            8 => { //equals: if p1==p2 then 1->p3 else 0->p3
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                // println!("{} {} {}", p1, p2 ,out);
                if(p1==p2){
                    cmds[out as usize] = 1;
                }
                else{
                    cmds[out as usize] = 0;
                }
                n+=4;
            },
            9 => { //adjusts the relative base: relative += p1
                relative = add_usize_i64(relative, get_value(cmds.clone(), modes[0], n+1, relative));
                n+=2;
            },
            99 => { //halt
                break;
            },
            _ => { //bad opt code
                println!("bad opt code {}", opt);
                break;
            }
        }
    }    
}

fn get_value(cmds: Vec<i64>, mode: i64, position: usize, relative: usize) -> i64{
    match mode{
        0 => {
            // println!("position");
            return cmds[cmds[position] as usize] as i64;
        },
        1 => {
            // println!("immediate");
            return cmds[position] as i64;
        }
        2 => {
            // println!("relative");
            return cmds[add_usize_i64(relative, cmds[position])] as i64;
        },
        _ => {
            // println!("default position");
            return cmds[cmds[position] as usize] as i64;
        },
        
    }
}

fn get_value_out(cmds: Vec<i64>, mode: i64, position: usize, relative: usize) -> i64{
    
    match mode{
        2 => {
            // println!("relative");
            // return cmds[add_usize_i64(relative, cmds[position])] as i64;
            return add_usize_i64(relative, cmds[position]) as i64;
        },
        _ => {
            // println!("immediate");
            return cmds[position] as i64;
        } 
        
    }
}

fn add_usize_i64(u: usize, i: i64) -> usize {
    // println!("{} {}", u, i);
    if i.is_negative() {
        return u - i.wrapping_abs() as u32 as usize
    }
    return u + i as usize;
}

fn robot_move(mut cmds: Vec<i64>, mut n: usize, mut relative: usize, mut map :HashMap<(i64,i64),i64>, pos: (i64,i64), step: i64, mut min: i64) -> (HashMap<(i64,i64),i64>, i64){
    println!("depth {}",step);
    if(step>min) {
        println!("too deep return");
        return (map,min);
    }

    const RADIX: u32 = 10;
    while(n < cmds.len()){

        // println!("inside");
        let opt = cmds[n] % 100; //rightmost two digits are opt code
        // println!("{} {}",n, opt);
        let tmp = (cmds[n]/100).to_string();
        let mut modes: Vec<i64> = tmp.chars().rev().map(|x| x.to_digit(RADIX).unwrap() as i64).collect(); // 0=position 1=immediate 2=relative none=position 
        modes.resize_with(4,Default::default);
        // println!("{:?}",cmds);
        
        match opt {
            1 => { //add: p1+p2->p3
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                cmds[out as usize] = p1 + p2;
                n+=4;
            },
            2 => { //multiply: p1*p2->p3
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                cmds[out as usize] = p1 * p2;
                n+=4;
            },
            3 => { //input: input->p1
                let out = get_value_out(cmds.clone(), modes[0], n+1, relative);
                n+=2;

                let moves = vec![(0,-1),(0,1),(1,0),(-1,0)];
                for (m, d) in moves.iter().zip(1..5) {
                    let x = pos.0 + m.0;
                    let y = pos.1 + m.1;
                    println!("{}:{} {:?}",x,y,m);
                    if map.contains_key(&(x,y)){
                        println!("already mapped");
                        continue;
                    }
                    // println!("{}:{} 2",x,y);
                    cmds[out as usize] = d;
                    let (a, b) = robot_move(cmds.clone(), n.clone(), relative.clone(), map, (x,y), step+1, min);
                    if b<min {
                        min = b;
                        println!("found a min {}",min);
                    }
                    map = a;

                }
                return (map, min);
                // println!("{}",min);
                //return (map, min);
            },
            4 => { //output: p1->output
                // println!("{}",get_value(cmds.clone(), modes[0], n+1, relative));
                let val=get_value(cmds.clone(), modes[0], n+1, relative);
                println!("val is {} {:?}",val, pos);
                map.insert(pos,val);
                
                let mut printer = vec![vec![" ";50];50];
                for (key,value) in &map {
                    printer[key.1 as usize][key.0 as usize] = match value {
                        0=>"#",
                        1=>".",
                        2=>"Q",
                        _=>" ",
                    }
                    
                }
                if val==1{
                    printer[pos.1 as usize][pos.0 as usize]="D";
                }
                for row in printer {
                    for col in row {
                        print!("{}",col);
                    }
                    print!("\n");
                }
                print!("\n");
                match val {
                    0 => {
                        println!("return");
                        return (map, min);
                    },
                    2=> {
                        println!("return");
                        return (map, step);
                    }
                    _ =>
                    {
                        println!("dont return");
                    }
                }
                // println!("{}",min);
                //map.insert(pos,get_value(cmds.clone(), modes[0], n+1, relative));

                n+=2;
            },
            5 => { //jump-if-true: if p1!=0 then p2->n
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                if(p1 != 0){
                    n = get_value(cmds.clone(), modes[1], n+2, relative) as usize;
                    continue;
                }
                n+=3;
            },
            6 => { //jump-if-false: if p1==0 then p2->n   
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                if(p1 == 0){
                    n = get_value(cmds.clone(), modes[1], n+2, relative) as usize;
                    continue;
                }
                n+=3;
            },
            7 => { //less than: if p1<p2 then 1->p3 else 0->p3 
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                if(p1<p2){
                    cmds[out as usize] = 1;
                }
                else{
                    cmds[out as usize] = 0;
                }
                n+=4;
            },
            8 => { //equals: if p1==p2 then 1->p3 else 0->p3
                let p1 = get_value(cmds.clone(), modes[0], n+1, relative);
                let p2 = get_value(cmds.clone(), modes[1], n+2, relative);
                let out = get_value_out(cmds.clone(), modes[2], n+3, relative);
                // println!("{} {} {}", p1, p2 ,out);
                if(p1==p2){
                    cmds[out as usize] = 1;
                }
                else{
                    cmds[out as usize] = 0;
                }
                n+=4;
            },
            9 => { //adjusts the relative base: relative += p1
                relative = add_usize_i64(relative, get_value(cmds.clone(), modes[0], n+1, relative));
                n+=2;
            },
            99 => { //halt
                println!("halt?");
                break;
            },
            _ => { //bad opt code
                println!("bad opt code {}", opt);
                break;
            }
        }
    }
    return (map, min);

}

