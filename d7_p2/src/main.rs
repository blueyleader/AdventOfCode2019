use std::env;
use std::fs;
use std::io::Read;
use std::cmp::max;

fn main() {
    const RADIX: u32 = 10;

    let filename = "../inputs/d7.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut cmds: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let mut max_value = i32::min_value();
    let amp_range = (5..10);
    for a in amp_range.clone(){
        for b in amp_range.clone(){
            if(a==b){
                continue;
            }
            for c in amp_range.clone(){
                if(a==c || b==c){
                    continue;
                }
                for d in amp_range.clone(){
                    if(a==d || b==d || c==d){
                        continue;
                    }
                    for e in amp_range.clone(){
                        if(a==e || b==e || c==e || d==e){
                            continue;
                        }
                        let phase = vec![a,b,c,d,e];
                        let mut phase_counter = vec![0,0,0,0,0];
                        let mut phase_mem = vec![cmds.clone(),cmds.clone(),cmds.clone(),cmds.clone(),cmds.clone()];
                        //println!("{:?}",phase);
                        let mut input = 0;
                        let looper = true;
                        'master: while looper {
                            for (n, n_counter) in phase.clone().iter().zip(0..5) {
                                //println!("{}",n);
                                let (input2, tmp_mem, tmp_counter, looper) = run_seq(phase_mem[n_counter].clone(), phase_counter[n_counter], vec![input], *n as i32);
                                phase_mem[n_counter] = tmp_mem;
                                phase_counter[n_counter] = tmp_counter;
                                if(!looper){
                                    break 'master;
                                }
                                input = input2;
                            }
                        }
                        //println!("{}",input);
                        max_value = max(max_value, input);
                    }
                }
            }
        }
    }
    println!("{}", max_value);
}

fn get_value(mode: i32, imm_value: i32, pos_value: i32) -> i32{
    if(mode==0){
        return pos_value;
    }
    return imm_value;
}

fn run_seq(mut cmds: Vec<i32>, looper: usize, mut input: Vec<i32>, phase: i32) -> (i32, Vec<i32>, usize, bool){
    //let mut input_count = 0;
    let mut output = String::new();

    const RADIX: u32 = 10;
    let mut n = looper;
    if(n==0){
        input.insert(0,phase);
    }
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
                cmds[out as usize] = input.remove(0);
                //output.clear();
                n+=2;
            },
            4 => { //output: p1->output
                //print!("{}",get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0)));
                output.push_str(&get_value(modes[0],cmds[n+1],*cmds.get(cmds[n+1] as usize).unwrap_or(&0)).to_string());
                // input.push(output.parse::<i32>().unwrap());
                n+=2;
                return (output.parse::<i32>().unwrap(), cmds, n, true);
                
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
    return (0, cmds, n, false);
}