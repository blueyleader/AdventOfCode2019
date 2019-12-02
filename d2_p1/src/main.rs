use std::env;
use std::fs;

fn main() {
    let filename = "../inputs/d2.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut cmds: Vec<i32> = contents.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    cmds[1]=12;
    cmds[2]=2;

    let mut n = 0;
    while(n<cmds.len()){
        let opt = cmds[n];
        if(opt!=1 && opt!=2){
            println!("found bad opt code {}: index {}", opt, n);
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

    // println!("{:?}", cmds);
    println!("{}",cmds[0]);
}
