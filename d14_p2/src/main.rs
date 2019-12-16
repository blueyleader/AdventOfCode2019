use std::env;
use std::fs;
use std::io::Read;
use std::collections::HashMap;
use math::round;

#[derive(Debug, Clone)]
struct Formula {
    output: String,
    amount: i128,
    parts: Vec<(String, i128)>,
    
}

fn main() {
    let filename = "../inputs/d14.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();
    let mut formulas: HashMap<String, Formula> = HashMap::new();
    let mut extras: HashMap<String, i128> = HashMap::new();
    for line in lines {
        // 4 BNVTZ, 13 KXFJ, 14 QRBK, 56 SJSLP, 18 SPFP, 9 WMVD, 12 JFXPH, 1 MHXF => 1 FUEL
        let parts: Vec<String> = line.split("=>").map(|x| x.to_string()).collect();
        let inputs: Vec<String> = parts[0].trim().split(",").map(|x| x.trim().to_string()).collect();
        let output: Vec<String> = parts[1].trim().split(" ").map(|x| x.trim().to_string()).collect();
        // println!("{:?}",parts); 
        // println!("{:?}",output); 
        let mut comp: Vec<(String, i128)> = Vec::new();
        for part in inputs {
            let split: Vec<String> = part.split(" ").map(|x| x.trim().to_string()).collect();
            comp.push((split[1].clone(), split[0].parse::<i128>().unwrap()));
        }
        // println!("{:?} {} {}",comp ,output[1], output[0]); 
        formulas.insert(output[1].clone(), Formula { output: output[1].clone(), amount: output[0].parse::<i128>().unwrap(), parts: comp});

    }
    let (amount,_,_) = build(formulas.clone(), extras.clone(), "FUEL".to_string(), 1, 0);
    // println!("{}", amount);
    let can_make = (1000000000000 as i128/amount as i128);
    // println!("{} {}", amount, can_make);
    let mut upper =0;
    'master : for n in (can_make..).step_by(1000) {
        let (amount,x,y) = build(formulas.clone(), extras.clone(), "FUEL".to_string(), n, 0);
        // println!("{}", n);
        if y>1000000000000 {
            upper = n-1000;
            //println!("{}", n-1);
            break 'master;
        }
    }
    for n in (upper..=upper+1000) {
        let (amount,x,y) = build(formulas.clone(), extras.clone(), "FUEL".to_string(), n, 0);
        println!("{}", n);
        if y>1000000000000 {
            //upper = n-1;
            println!("{}", n-1);
            break;
        }
    }
}

fn build(map: HashMap<String, Formula>, mut extras: HashMap<String, i128>, name: String, amount: i128, mut used: i128) -> (i128, HashMap<String, i128>,i128) {
    //println!("make {} {} {}",name, amount, used);
    if(name=="ORE"){
        // println!("amount of ore {}",amount);
        return (amount as i128, extras, used + amount as i128);
    }

    let form = map.get(&name).unwrap();
    let mut count = 0;
    let have = extras.get(&name).unwrap_or(&0);
    let real_amount= amount-have;
    let rounded = round::ceil(real_amount as f64 /form.amount as f64, 0) as i128;
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
        let (x, y, z) =build(map.clone(), extras, part.0, part.1 * rounded, used);
        used = z;
        extras = y;
        count += x;
    }

    return (count, extras, used);
}
