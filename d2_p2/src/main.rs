use std::env;
use std::fs;

fn main() {
    let filename = "../inputs/d2.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let cmdsPure: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut cmds = cmdsPure.to_vec();
            cmds[1]=noun; //noun
            cmds[2]=verb; //verb

            let mut n = 0;
            while(n<cmds.len()){
                let opt = cmds[n];
                if(opt!=1 && opt!=2){
                    //println!("found bad opt code {}: index {}", opt, n);
                    break;
                }

                let p1 = cmds[cmds[n+1] as usize];
                let p2 = cmds[cmds[n+2] as usize];
                let out = cmds[n+3];

                let result: i32;
                if(opt==1){
                    result = p1 + p2
                }
                else{
                    result = p1 * p2
                }
                // println!("index1 {}, index2 {}, index3 {}, val {}",p1,p2,out,result);
                cmds[out as usize] = result;
                n+=4;
            }
            println!("{:?}", cmds);
            println!("{}",cmds[0]);
            if(cmds[0]==19690720){
                println!("noun:{} verb:{} result:{}",noun,verb,(100*noun)+verb);
                break 'outer;
            }
        }
    }

    // println!("{:?}", cmds);
    //println!("{}",cmds[0]);
}