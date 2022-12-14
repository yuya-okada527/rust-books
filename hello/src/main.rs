fn main() {
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    // let mut s3: String = s2.to_string();
    println!("s1: {s1}");
    // s1 = "Changed".to_string();
    println!("s1: {s1}");
    println!("s2: {s2}");
    // println!("s3: {s3}");
}
