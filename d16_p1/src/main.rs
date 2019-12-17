use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    const RADIX: u32 = 10;

    let filename = "../inputs/d16.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    

    let mut digits: Vec<i32> = contents.trim().chars().map(|x| x.to_digit(RADIX).unwrap() as i32).collect();
    // println!("{:?}", digits);
    let len = digits.len();
    let mut out: String = "".to_string();
    for x in 0..100{
        out = "".to_string();
        for n in 0..len{
            let mut sum =0;
            for(d,r) in digits.iter().zip(build_repeter(n,len)){
                sum += d*r;
            }
            out += &(sum%10).abs().to_string();
        }

        // println!("{}", out);
        digits = out.trim().chars().map(|x| x.to_digit(RADIX).unwrap() as i32).collect();
    }

    println!("{}", &out[..8]);
}

fn build_repeter(index: usize, len: usize) -> Vec<i32> {
    let base = vec![0,1,0,-1];
    let mut list: Vec<i32> = vec![0;len];
    let mut count = 1;
    
    let mut step = 0;
    for n in index..len {
        list[n]= base[count];
        step+=1;
        if step > index {
            step=0;
            count+=1;
            count=count%4;
        }
    }

    return list;
}