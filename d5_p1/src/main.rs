use std::env;
use std::fs;

fn main() {
    const RADIX: u32 = 10;

    let filename = "../inputs/d5.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut cmds: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut n = 0;
    while(n<cmds.len()){
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
                n+=4
            },
            2 => { //multiply: p1*p2->p3
                let p1 = get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0));
                let p2 = get_value(modes[1],cmds[n+2],*cmds.get(cmds[n+2] as usize).unwrap_or(&0));
                let out = cmds[n+3];
                cmds[out as usize] = p1 * p2;
                n+=4
            },
            3 => { //input: input->p1
                let out = cmds[n+1];
                cmds[out as usize] = 1; //TODO change to not hardcode for input
                n+=2
            },
            4 => { //output: p1->output
                print!("{}",get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0)));
                n+=2
            },
            99 => { //halt
                break;
            },
            _ => { //bad opt code
                println!("bad opt code");
                break;
            }
        }




        // if(opt!=1 && opt!=2){
        //     println!("found bad opt code {}: index {}", opt, n);
        //     break;
        // }

        // let p1 = cmds[cmds[n+1] as usize];
        // let p2 = cmds[cmds[n+2] as usize];
        // let out = cmds[n+3];

        // let result: i32;
        // if(opt==1){
        //     result = p1 + p2
        // }
        // else{
        //     result = p1 * p2
        // }
        // // println!("index1 {}, index2 {}, index3 {}, val {}",p1,p2,out,result);
        // cmds[out as usize] = result;
        // n+=4;
    }

    // println!("{:?}", cmds);
    //println!("{}",cmds[0]);
}

fn get_value(mode: i32, imm_value: i32, pos_value: i32) -> i32{
    if(mode==0){
        return pos_value;
    }
    return imm_value;
}