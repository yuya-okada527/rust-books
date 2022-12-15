// use std::vec;

// use std::num;

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
    // let mut a: [i32; 3] = [0, 1, 2];
    // let b: [i32; 3] = [0; 3];
    // a[1] = b[1];
    // a[2] = b[2];
    // println!("{:?}", &a[1..3]);

    // 構造体
    // struct Person {
    //     name: String,
    //     age: u32,
    // }
    // let p = Person {
    //     name: String::from("John"),
    //     age: 8,
    // };
    // println!("name: {}, age: {}", &p.name, &p.age);

    // enum
    // enum Event {
    //     Quit,
    //     KeyDown(u8),
    //     MouseDown { x: i32, y: i32},
    // }

    // let e1 = Event::Quit;
    // let e2 = Event::MouseDown { x: 10, y: 10 };

    // Option
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // Result
    // let result: Result<i32, String> = Ok(200);
    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err),
    // };
    // if let Ok(code) = result {
    //     println!("code: {}", code);
    // }
    // println!("code: {}", result.unwrap_or(-1));
    // let result: Result<i32, String> = Err("error".to_string());
    // println!("code: {}", result.unwrap_or(-1));
    // fn func(code: i32) -> Result<i32, String> {
    //     println!("code: {}", code);
    //     Ok(100)
    // }
    // let result: Result<i32, String> = Ok(200);
    // let next_result = result.and_then(func);
    // println!("next_result: {}", next_result.unwrap_or(-1));
    // let result: Result<i32, String> = Err("error".to_string());
    // let next_result = result.and_then(func);
    // println!("next_result: {}", next_result.unwrap_or(-1));

    // Vec
    // let v1 = vec![1, 2, 3, 4, 5];
    // let v2 = vec![0; 5];
    // println!("v1: {}", v1[0]);
    // println!("v2: {}", v2[2]);
    // for el in &v1 {
    //     println!("{}", el);
    // }

    // Box
    // fn print(s: Box<[u8]>) {
    //     println!("{:?}", s);
    // }
    // let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    // print(Box::new(byte_array));

    // let & mut
    // let immut_val = 10;
    // let mut mut_val = 20;
    // mut_val += immut_val;
    // println!("{}", mut_val);

    // if
    // let number = 1;
    // if 0 < number {
    //     println!("0 < number");
    // } else if number > 0 {
    //     println!("0 > number");
    // } else {
    //     println!("0 == number");
    // }
    // let number = 1;
    // let result = if 0 <= number {
    //     number
    // } else {
    //     -number
    // };
    // println!("{}", result);

    // loop
    // let mut count = 0;
    // let result = loop {
    //     println!("count: {}", count);
    //     count += 1;
    //     if count == 10 {
    //         break count;
    //     }
    // };
    // println!("{}", result);

    // while
    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1
    }
}
