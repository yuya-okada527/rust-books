fn main() {
    // 文字列
    // let s1: String = String::from("Hello, World!");
    // let s2: &str = &s1;
    // let mut s3: String = s2.to_string();
    // println!("s1: {s1}");
    // s1 = "Changed".to_string();
    // println!("s1: {s1}");
    // println!("s2: {s2}");
    // println!("s3: {s3}");

    // タプル
    // let mut t = (1, "2");
    // let first = t.0;
    // let second = t.1;
    // t.0 = 2;
    // t.1 = "3";
    // println!("first: {first}");   // -> 1
    // println!("second: {second}"); // -> 2

    // 配列
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    // a[2] = b[2];
    println!("{:?}", &a[1..3]);
}
