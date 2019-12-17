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
    let mut tmp= "".to_string();
    for n in 0..7 {
        tmp+= &digits[n].to_string();
    }
    let skip = tmp.parse::<usize>().unwrap();
    println!("{}", skip);
    let mut large_d:Vec<i32> = Vec::new();

    for n in 0..10000{
        large_d.append(&mut digits.clone());
    }
    digits = large_d.clone();
    // println!("{:?}", digits);
    
    digits = digits.iter().skip(skip as usize).map(|x| *x).collect();
    
    // let len2 = digits.len();
    let len = digits.len();
    println!("digit len:{} skip:{}", len,skip as usize);

    let mut out: String = "".to_string();
    for x in 0..100{
        println!("{}",x);
        out = "".to_string();

        let mut sum = 0;
        // for n in (skip..len).rev() {
        for n in digits.iter().rev() {
            //println!("n {}",n);
            sum = (sum + n) % 10;
            out = sum.to_string() + &out;
        }
        // println!("{}",out);





        // for n in skip..len{
        //     println!("{} {}/{}",x,n,len);
        //     let mut sum =0;
        //     for(d,r) in digits.iter().zip(build_repeter(n,len).iter().skip(skip as usize)){
        //         sum += d*r;
        //     }
        //     out += &(sum%10).abs().to_string();
        // }

        // println!("{}", out);
        digits = out.trim().chars().map(|x| x.to_digit(RADIX).unwrap() as i32).collect();
    }

    // println!("{:?}",digits);
    println!("{}", &out[..8]);

    // let mut tmp= "".to_string();
    // for n in skip..skip+8 {
    //     tmp+= &digits[n as usize].to_string();
    // }
    // println!("digit len:{} skip:{}", len,skip as usize);
    // for n in digits.iter().skip(skip as usize).take(8){
    //     println!("digit {}", n);
    //     tmp+=&n.to_string();
    // }
    // println!("{}", tmp);
    // println!("{:?}", digits.iter().skip(skip as usize).take(8));
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