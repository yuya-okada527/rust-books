// use std::vec;
// use std::num;
// use std::io::Write;
// use std::fmt::format;

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
    // let mut count = 0;
    // while count < 10 {
    //     println!("count: {}", count);
    //     count += 1
    // }

    // for
    // for count in 0..10 {
    //     println!("count: {}", count);
    // }

    // let array = [0, 1, 2, 3, 4, 5, 6];
    // for el in &array {
    //     println!("el: {}", el);
    // }

    // label
    // 'main: loop {
    //     println!("main loop start");
    //     'sub: loop {
    //         println!("sub loop start");
    //         break 'main;
    //         println!("sub loop end");
    //     }
    //     println!("main loop end");
    // }

    // match
    // let i: i32 = 1;
    // match i {
    //     1 => println!("1"),
    //     2 => println!("2"),
    //     3 => println!("3"),
    //     _ => println!("misc"),
    // }
    // enum Color {
    //     Red,
    //     Blue,
    //     Green,
    // }
    // let c = Color::Red;
    // match c {
    //     Color::Red => println!("red"),
    //     Color::Blue => println!("blue"),
    //     Color::Green => println!("green"),
    // }
    // let result: Result<i32, String> = Ok(100);
    // let result_number = match result {
    //     Ok(number) => number,
    //     Err(message) => {
    //         println!("Error: {}", message);
    //         -1
    //     }
    // };
    // println!("result_number: {}", result_number);

    // Range
    // for number in 1..=5 {
    //     println!("{}", number);
    // }

    // Iterator
    // struct Iter {
    //     current: usize,
    //     max: usize,
    // }

    // impl Iterator for Iter {
    //     type Item = usize;

    //     fn next(&mut self) -> Option<usize> {
    //         self.current += 1;
    //         if self.current - 1 < self.max {
    //             Some(self.current - 1)
    //         } else {
    //             None
    //         }
    //     }
    // }

    // let it = Iter {
    //     current: 0,
    //     max: 10,
    // };
    // for num in it {
    //     println!("{}", num);
    // }

    // fn
    // fn add(a: i32, b:i32) -> i32 {
    //     a + b
    // }

    // let x = add(1, 2);
    // println!("x = {}", x);

    // fn return
    // fn abs(number: i32) -> i32 {
    //     if number < 0 {
    //         return -number;
    //     }
    //     number
    // }
    // println!("{}", abs(-32));

    // impl
    // struct Person {
    //     name: String,
    //     age: u32,
    // }

    // impl Person {
    //     fn say_name(&self) -> &Self {
    //         println!("I am {}.", self.name);
    //         self
    //     }

    //     fn say_age(&self) -> &Self {
    //         println!("I am {} year(s) old.", self.age);
    //         self
    //     }
    // }

    // let p = Person {
    //     name: String::from("Taro"),
    //     age: 20,
    // };
    // p.say_name().say_age();
    // impl Person {
    //     fn new(name: &str, age: u32) -> Person {
    //         Person {
    //             name: String::from(name),
    //             age: age,
    //         }
    //     }
    // }
    // let p = Person::new("Taro", 20);
    // p.say_name().say_age();

    // macro
    // let s = concat!("A", "b2", 3);
    // println!("{}", s);
    // let s = format!("{}-{:?}", s, ("D", 5));
    // println!("{}", s);
    // let s = format!("{}-{}", "abc", "def");
    // println!("{}", s);
    // print!("hello{}", "\n");
    // println!("hello: {}", "world");
    // eprint!("hello{}", "\n");
    // eprintln!("hello {}", "world");

    // let mut w = Vec::new();
    // write!(&mut w, "{}", "ABC");
    // writeln!(&mut w, "is 123");
    // dbg!(w);

    // panic
    // panic!("it will panic");

    // println!("defined in file: {}", file!());
    // println!("defined on line: {}", line!());
    // let unix = true;
    // println!("is test: {}", cfg!(unix));
    // println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    // assert
    // assert!(true);
    // assert_eq!(1, 1);
    // assert_ne!(1, 0);
    // debug_assert!(true);
    // debug_assert_eq!(1, 2);
    // debug_assert_ne!(1, 0);

    // inimplemented
    // enum Emotion {
    //     Anger,
    //     Happy,
    // }

    // trait Emotional {
    //     fn get_happy(&mut self) -> String;
    //     fn get_anger(&mut self) -> String;
    //     fn tell_state(&self) -> String;
    // }

    // struct HappyPerson {
    //     name: String,
    //     state: Emotion,
    // }

    // impl Emotional for HappyPerson {
    //     fn get_anger(&mut self) -> String {
    //         unimplemented!()
    //     }
    //     fn get_happy(&mut self) -> String {
    //         format!("{} is always happy.", self.name)
    //     }
    //     fn tell_state(&self) -> String {
    //         todo!()
    //     }
    // }

    // let mut p = HappyPerson {
    //     name: "Takashi".to_string(),
    //     state: Emotion::Happy
    // };
    // println!("{}", p.get_happy());

    // trait
    // trait Tweet {
    //     fn tweet(&self);
    //     fn tweet_twice(&self) {
    //         self.tweet();
    //         self.tweet();
    //     }
    //     fn shout(&self) {
    //         println!("Uoooooooooooohh");
    //     }
    // }
    // struct Dove;
    // struct Duck;

    // impl Tweet for Dove {
    //     fn tweet(&self) {
    //         println!("Coo!");
    //     }
    // }
    // impl Tweet for Duck {
    //     fn tweet(&self) {
    //         println!("Quack!");
    //     }
    // }
    // let dove = Dove {};
    // dove.tweet();
    // dove.tweet_twice();
    // dove.shout();

    // let duck = Duck {};
    // let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    // for bird in bird_vec {
    //     bird.tweet();
    // }

    // generics
    // fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
    //     (t, s)
    // }
    // let t1 = make_tuple(1, 1);
    // dbg!(t1);
    // let t2 = make_tuple("Hello", "world");
    // dbg!(t2);
    // let t3 = make_tuple(vec![1, 2, 3], vec![4, 5, 6]);
    // dbg!(t3);
    // let t4 = make_tuple(3, "years old");
    // dbg!(t4);

    // ownership
    // struct Color {
    //     r: i32,
    //     g: i32,
    //     b: i32,
    // }

    // let a = Color {r: 255, g: 255, b: 255};
    // let b = a;
    // println!("{} {} {}", b.r, b.g, b.b);

    // borrowing
    // fn calc_data(data: &String) {
    //     println!("{}", data);
    // }
    // let important_data = "Hello, World!".to_string();
    // calc_data(&important_data);
    // println!("next: {}", important_data);

    // let x = 5;
    // let y = &x;
    // let z = &x;
    // dbg!(x);
    // dbg!(y);
    // dbg!(z);

    // lifetime
    let y;
    {
        let x = 5;
        y = &x;
        dbg!(x);
        dbg!(y);
    }
    // dbg!(y);
}
