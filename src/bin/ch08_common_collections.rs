use core::fmt;
use std::collections::HashMap;
use std::fmt::Result;
use std::io;
use std::io::Result as IOResult;

fn f1() -> fmt::Result {
    fmt::Result::Ok(())
}
fn f2() -> io::Result<()> {
    io::Result::Ok(())
}

fn f3() -> Result {
    Result::Ok(())
}
fn f4() -> IOResult<()> {
    IOResult::Ok(())
}
fn main() {
    let mut v: Vec<i32> = Vec::new();

    let v1 = vec![1, 2, 3];

    v.push(9);
    v.push(10);

    println!("{:?},{:?}", v, v1);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    let does_not_exist: Option<&i32> = v3.get(100);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    match does_not_exist {
        Some(data) => println!("The 100 element is {data}"),
        None => println!("There is no 100 element."),
    }

    //Option result
    let does_not_exist = v3.get(100);
    // panicked at 'index out of bounds'
    // let does_not_exist = &v3[100];

    let first = v[0];
    v.push(6);
    println!("The first element is: {first}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    println!("Let’s move on to the next collection type: String!");

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('b');

    println!("s is '{s}',s1 is '{s1}',s2 is '{s2}'");

    let s3 = s1 + s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {s3}, s2 is {s2}, s1 no longer valid after the addition");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {s}");
    let s1 = String::from("hello");
    let hello = "Здравствуйте";
    let answer = &hello[0..6];
    println!("{answer}");
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert("Yellow".to_string(), 50);

    let score = scores.get("Blue").copied(); //Option<i32>
    let score = score.unwrap_or(0);
    println!("score is {score}");

    let score = scores.get("Blue"); //Option<&i32>
    match score {
        Some(data) => {
            println!("score is {data}");
        }
        None => (),
    }
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("HashMap Overwrite key by call insert: {:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!(
        "Adding a Key and Value Only If a Key Isn’t Present by call entry method: {:?}",
        scores
    );

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map);
}
