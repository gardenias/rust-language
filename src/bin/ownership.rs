fn main() {
    println!("4 Chapter!!! Ownership");

    let origin: &str = "hello world";
    let s = String::from(origin);
    let s1 = s;

    takes_ownership(&s1);
    println!("{s1}");

    let sr1 = &s1;
    let sr2 = &s1;
    println!("{sr1}, {sr2}");

    let mut sm2 = String::from("Hello");
    let r1 = &mut sm2;
    // let r2 = &mut sm2;
    r1.push_str(" World");
    println!("{sm2}");

    let x = 5;
    let y = x;
    let z = x;

    makes_copy(x);
    println!("{x}, {y}, {z}");

    let s = String::from(origin);
    println!("fist_word(origin) = {}", fist_word(origin));
    println!("fist_word(&origin[3..]) = {}", fist_word(&origin[3..]));
    println!("fist_word(&s) = {}", fist_word(&s));
    println!("fist_word(&s[..7]) = {}", fist_word(&s[..4]));
}

fn takes_ownership(some_string: &String) {
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}
fn fist_word(some_string: &str) -> &str {
    for (len, &item) in some_string.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &some_string[..len];
        }
    }
    &some_string[..]
}
