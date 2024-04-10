use std::time::Duration;
use std::{thread, vec};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Yellow,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_yellow = 0;

        (&self.shirts).into_iter().for_each(|color| match color {
            ShirtColor::Red => num_red += 1,
            ShirtColor::Blue => num_blue += 1,
            ShirtColor::Yellow => num_yellow += 1,
        });

        ShirtColor::Red
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Yellow,
            ShirtColor::Blue,
        ],
    };
    let user_pref1 = Some(ShirtColor::Yellow);
    let ga1 = store.giveaway(user_pref1);

    println!("The user with reference {:?} get {:?}", user_pref1, ga1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // captures an immutable reference

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // captures a mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // Using move to force the closure for the thread to take ownership of list
    let list = vec![1, 2, 3];
    println!("Before Defining closure {:?}", list);
    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();
    // compile err: borrow of moved value `list`
    // println!("After call closure {:?}", list);

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = 0;

    list.sort_by_key(|r| {
        sort_operations += 1;
        r.width
    });
    println!("{:#?}", list);

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
