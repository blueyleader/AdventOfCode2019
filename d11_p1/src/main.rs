use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

enum Dir {
    UP, RIGHT, DOWN, LEFT
}

fn main() {
    const RADIX: u32 = 10;

    let filename = "../inputs/d11.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut cmds: Vec<i64> = contents.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    cmds.resize(10000,0);
    let mut n = 0;
    let mut relative = 0;

    let mut map:HashMap<(i64, i64), i64> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut dir = Dir::UP;
    let mut paint = true;

    while(n < cmds.len()){
        let opt = cmds[n] % 100; //rightmost two digits are opt code
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
                // let out = get_value_out(cmds.clone(), modes[0], n+1, relative);
                // let mut input = String::new();
                // std::io::stdin().read_line(&mut input); // as i64;
                // cmds[out as usize] = input.trim().parse::<i64>().unwrap();
                // n+=2;
                let out = get_value_out(cmds.clone(), modes[0], n+1, relative);
                cmds[out as usize] = *map.get(&(x,y)).unwrap_or(&0);
                n+=2;

            },
            4 => { //output: p1->output
                let val = get_value(cmds.clone(), modes[0], n+1, relative);
                if(paint){
                    map.insert((x,y), val);
                }
                else{ 
                    dir = match dir{
                        Dir::UP => {
                            if (val == 0){
                                Dir::RIGHT
                            }
                            else{ 
                                Dir::LEFT
                            }
                        },
                        Dir::RIGHT => {
                            if (val == 0){
                                Dir::DOWN
                            }
                            else{ 
                                Dir::UP
                            }
                        },
                        Dir::DOWN => {
                            if (val == 0){
                                Dir::LEFT
                            }
                            else{ 
                                Dir::RIGHT
                            }
                        },
                        Dir::LEFT => {
                            if (val == 0){
                                Dir::UP
                            }
                            else{ 
                                Dir::DOWN
                            }
                        },
                    };

                    match dir{
                        Dir::UP => y+=1,
                        Dir::RIGHT => x+=1,
                        Dir::DOWN => y-=1,
                        Dir::LEFT => x-=1,
                    };
                }
                paint = !paint;
                //print!("{}",get_value(cmds.clone(), modes[0], n+1, relative));
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

    println!("{}", map.len())
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
