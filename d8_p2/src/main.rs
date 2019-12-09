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

    let mut final_image: Vec<Vec<i32>> = vec![vec![2;width];height];

    for lay in image {
        for h in 0..height{
            for w in 0..width{
                if(final_image[h][w]==2){
                    final_image[h][w]=lay[h][w]
                }
            }
        }
    }

    for r in final_image {
        for p in r {
            print!("{}",(if p==1 {"#"} else {" "}))
        }
        print!("\n");
    }

}
