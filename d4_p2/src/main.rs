use regex::Regex;

fn main() {
    // let input = "347312-805915";
    let min = 347312;
    let max = 805915;

    let dup = Regex::new("(^|[^1])1{2}([^1]|$)|(^|[^2])2{2}([^2]|$)|(^|[^3])3{2}([^3]|$)|(^|[^4])4{2}([^4]|$)|(^|[^5])5{2}([^5]|$)|(^|[^6])6{2}([^6]|$)|(^|[^7])7{2}([^7]|$)|(^|[^8])8{2}([^8]|$)|(^|[^9])9{2}([^9]|$)").unwrap();
    let asc = Regex::new("^1*2*3*4*5*6*7*8*9*$").unwrap();

    let passwords: Vec<i32> = (min..max)
    .filter(|x| dup.is_match(&x.to_string()) && asc.is_match(&x.to_string()))
    .map(|x| x)
    .collect();
    println!("{}",passwords.len());
}