fn main() {
    let mut s: String = String::from("hello");
    let r1: &String = &s;
    let r2: &mut String = &mut s;
    println!("r1 is {r1}");
    //println!("r2 is {r2}");
}

