use std::env;
use std::fs;
use std::io::Read;
use std::cmp::max;

fn main() {
    const RADIX: u32 = 10;
    let width = 25;
    let height = 6;

    let filename = "../inputs/d8.txt";
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut pixels: Vec<i32> = contents.chars().map(|x| x.to_digit(RADIX).unwrap() as i32).collect(); // 0=position 1=immediate none=position 
    //println!("{:?}",pixels);
    
    let mut image: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut layer: Vec<Vec<i32>> = Vec::new();
    let mut row: Vec<i32> = Vec::new(); 

    let mut width_counter=0;
    let mut height_counter=0;
    for n in pixels {
        row.push(n);

        width_counter+=1;
        if(width == width_counter) {
            width_counter=0;
            height_counter+=1;
            layer.push(row.clone());
            row.clear();
        }
        if(height == height_counter){
            height_counter=0;
            image.push(layer.clone());
            layer.clear();
        }
    }

    let layer_count:Vec<usize> = image.iter().map(|x| x.iter().map(|y| y.iter().filter(|z| **z == 0).count()).sum()).collect();
    // println!("{:?}",layer_count);
    let mut index = 0;
    let mut min_val = usize::max_value();
    for (lay, x) in layer_count.iter().zip(0..layer_count.len()) {
        if(lay<&min_val) {
            min_val=*lay;
            index=x;
            // println!("new min {} {}", index, min_val);
        }
    }

    //println!("best index {} {}", index, min_val);

    let ones: usize = image[index].iter().map(|y| y.iter().filter(|z| **z == 1).count()).sum();
    let twos: usize = image[index].iter().map(|y| y.iter().filter(|z| **z == 2).count()).sum();
    //println!("{} {}", ones, twos);
    println!("{}", ones*twos);

}
