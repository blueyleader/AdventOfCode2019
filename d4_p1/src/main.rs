use regex::Regex;

fn main() {
    // let input = "347312-805915";
    let min = 347312;
    let max = 805915;

    let dup = Regex::new("(11|22|33|44|55|66|77|88|99)").unwrap();
    let asc = Regex::new("^1*2*3*4*5*6*7*8*9*$").unwrap();

    let passwords: Vec<i32> = (min..max)
    .filter(|x| dup.is_match(&x.to_string()) && asc.is_match(&x.to_string()))
    .map(|x| x)
    .collect();
    println!("{}",passwords.len());
}
