use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Guess the number.");
    println!("Pls input your guess: ");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut  guess).expect("Fail to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!(" -> too small"),
            Ordering::Greater => {
                println!(" ->  to big");
            },

            Ordering::Equal => {
                println!(" -> You win");
                break;
            },
        };
    }

}
