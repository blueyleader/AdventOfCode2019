use std::env;
use std::fs;
use std::collections::HashMap;
fn main() {
    let filename = "../inputs/d6.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();//::<Vec<&str>>();
    //let test: Vec<(&str,&str)> = lines.map(|x| x.split(")").map(|y| y).collect() as (&str,&str)).collect();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let data: Vec<String> = line.split(")").map(|x| x.to_string()).collect();
        //println!("{:?}",data);
        if(map.contains_key(&data[0])){
            let mut tmp = map.get(&data[0]).unwrap().clone();
            tmp.push(data[1].clone());
            map.insert(data[0].clone(), tmp);
        }
        else{
            map.insert(data[0].clone(), vec![data[1].clone()]);
        }
    }
    //println!("{:?}",map);
    let mut stepMap: HashMap<String, i32> = HashMap::new();
    stepMap = buildMap(map, "COM".to_string(), 0, stepMap);
    //println!("{:?}",stepMap);
    // let sum: i32 = stepMap.values().map(|x| *x as i32).sum();
    let sum: i32 = stepMap.values().sum();
    println!("{}",sum);
}

fn buildMap(map: HashMap<String, Vec<String>>, name: String, steps: i32, mut stepMap: HashMap<String, i32>) -> HashMap<String, i32> { //(HashMap<String, Vec<String>>, HashMap<String, i32>) {
    let name2 =name.clone();
    stepMap.insert(name, steps);

    let locRaw = map.get(&name2);
    if(locRaw.is_some()){
        let loc = locRaw.unwrap();
        for l in loc {
            stepMap = buildMap(map.clone(), l.to_string(), steps+1, stepMap);
        }
    }
    // return (map,stepMap);
    return stepMap;
}