use std::env;
use std::fs;

fn main() {
    const RADIX: u32 = 10;

    let filename = "../inputs/d5.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut cmds: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut n = 0;
    while(n < cmds.len()){
        let opt = cmds[n] % 100; //rightmost two digits are opt code
        let tmp = (cmds[n]/100).to_string();
        let mut modes: Vec<i32> = tmp.chars().rev().map(|x| x.to_digit(RADIX).unwrap() as i32).collect(); // 0=position 1=immediate none=position 
        modes.resize_with(4,Default::default);
        //println!("{:?}",modes);
        
        match opt {
            1 => { //add: p1+p2->p3
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));
                let p2 = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0));
                let out = cmds[n+3];
                cmds[out as usize] = p1 + p2;
                n+=4;
            },
            2 => { //multiply: p1*p2->p3
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));
                let p2 = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0));
                let out = cmds[n+3];
                cmds[out as usize] = p1 * p2;
                n+=4;
            },
            3 => { //input: input->p1
                let out = cmds[n+1];
                cmds[out as usize] = 5; //TODO change to not hardcode for input
                n+=2;
            },
            4 => { //output: p1->output
                print!("{}",get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0)));
                n+=2;
            },
            5 => { //jump-if-true: if p1!=0 then p2->n
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));  
                if(p1 != 0){
                    n = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0)) as usize;
                    continue;
                }
                n+=3;
            },
            6 => { //jump-if-false: if p1==0 then p2->n 
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));  
                if(p1 == 0){
                    n = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0)) as usize;
                    continue;
                }
                n+=3;
            },
            7 => { //less than: if p1<p2 then 1->p3 else 0->p3 
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));
                let p2 = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0));
                let out = cmds[n+3];
                if(p1<p2){
                    cmds[out as usize] = 1;
                }
                else{
                    cmds[out as usize] = 0;
                }
                n+=4;
            },
            8 => { //equals: if p1==p2 then 1->p3 else 0->p3
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));
                let p2 = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0));
                let out = cmds[n+3];
                if(p1==p2){
                    cmds[out as usize] = 1;
                }
                else{
                    cmds[out as usize] = 0;
                }
                n+=4;
            },

            99 => { //halt
                break;
            },
            _ => { //bad opt code
                println!("bad opt code");
                break;
            }
        }
    }
}

fn get_value(mode: i32, imm_value: i32, pos_value: i32) -> i32{
    if(mode==0){
        return pos_value;
    }
    return imm_value;
}