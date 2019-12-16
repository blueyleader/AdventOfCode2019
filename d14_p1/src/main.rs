use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use math::round;

#[derive(Debug, Clone)]
struct Formula {
    output: String,
    amount: i32,
    parts: Vec<(String, i32)>,
    
}

fn main() {
    let filename = "../inputs/d14.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let mut formulas: HashMap<String, Formula> = HashMap::new();
    let mut extras: HashMap<String, i32> = HashMap::new();
    for line in lines {
        // 4 BNVTZ, 13 KXFJ, 14 QRBK, 56 SJSLP, 18 SPFP, 9 WMVD, 12 JFXPH, 1 MHXF => 1 FUEL
        let parts: Vec<String> = line.split("=>").map(|x| x.to_string()).collect();
        let inputs: Vec<String> = parts[0].trim().split(",").map(|x| x.trim().to_string()).collect();
        let output: Vec<String> = parts[1].trim().split(" ").map(|x| x.trim().to_string()).collect();
        // println!("{:?}",parts); 
        // println!("{:?}",output); 
        let mut comp: Vec<(String, i32)> = Vec::new();
        for part in inputs {
            let split: Vec<String> = part.split(" ").map(|x| x.trim().to_string()).collect();
            comp.push((split[1].clone(), split[0].parse::<i32>().unwrap()));
        }
        // println!("{:?} {} {}",comp ,output[1], output[0]); 
        formulas.insert(output[1].clone(), Formula { output: output[1].clone(), amount: output[0].parse::<i32>().unwrap(), parts: comp});

    }

    // println!("{:?}",formulas); 
    let amount = build(formulas.clone(), extras, "FUEL".to_string(), 1);
    println!("{}", amount.0);

}

fn build(map: HashMap<String, Formula>, mut extras: HashMap<String, i32>, name: String, amount: i32) -> (i32, HashMap<String, i32>) {
    // println!("make {}",name);
    if(name=="ORE"){
        // println!("amount of ore {}",amount);
        return (amount as i32, extras);
    }

    let form = map.get(&name).unwrap();
    let mut count = 0;
    let have = extras.get(&name).unwrap_or(&0);
    let real_amount= amount-have;
    let rounded = round::ceil(real_amount as f64 /form.amount as f64, 0) as i32;
    if amount>*have {
        // println!("need more {} {}",real_amount, form.amount);
        extras.insert(name, (form.amount*rounded)-real_amount);

    }
    else{
        // println!("already have {:?}",extras);
        extras.insert(name, have-amount);
    }

    // println!("leftovers {:?}",extras);
    // println!("{}/{}={}",real_amount, form.amount, rounded);
    for part in form.parts.clone() {
        let (x, y) =build(map.clone(), extras, part.0, part.1 * rounded);
        extras = y;
        count += x;
    }

    return (count, extras);
}
