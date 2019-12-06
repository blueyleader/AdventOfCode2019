use std::env;
use std::fs;
use std::collections::HashMap;
fn main() {
    let filename = "../inputs/d6.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();//::<Vec<&str>>();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let data: Vec<String> = line.split(")").map(|x| x.to_string()).collect();
        if(map.contains_key(&data[0])){
            let mut tmp = map.get(&data[0]).unwrap().clone();
            tmp.push(data[1].clone());
            map.insert(data[0].clone(), tmp);
        }
        else{
            map.insert(data[0].clone(), vec![data[1].clone()]);
        }
    }

    let you_trail = buildTrail(map.clone(), "COM".to_string(), "YOU".to_string()).unwrap();
    let san_trail = buildTrail(map.clone(), "COM".to_string(), "SAN".to_string()).unwrap();

    let matching = you_trail.iter().rev().zip(san_trail.iter().rev()).filter(|&(a, b)| a==b).count();
    println!("{:?}",you_trail.len() -matching + san_trail.len() - matching -2);
    
}

fn buildTrail(map: HashMap<String, Vec<String>>, name: String, target: String) -> Option<Vec<String>> {
    if(name == target){
        return Some(vec![name]);
    }
    let loc = map.get(&name);
    if(loc.is_some()){
        for l in loc.unwrap() {
            let ret = buildTrail(map.clone(), l.to_string(), target.clone());
            if(ret.is_some()){
                let mut val = ret.unwrap();
                val.push(name);
                return Some(val);
            }
        }
    }
    return None;
}