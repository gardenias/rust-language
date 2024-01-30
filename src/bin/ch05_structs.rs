#[derive(Debug)]
struct User {
    name: String,
    age: u16,
    active: bool,
    sign_in_count: u16,
}

impl User {}

fn build_user_v1(name: String, age: u16) -> User {
    User {
        name,
        age,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn area2(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
fn main() {
    let u1 = User {
        name: String::from("User1"),
        active: false,
        age: 18,
        sign_in_count: 1,
    };
    println!("{:#?}", u1);
    println!("{:?}", u1);

    let mut u2 = User {
        name: String::from("User2"),
        age: u1.age + 10,
        ..u1
    };
    println!("{:?}", u2);

    u2.name = String::from("User2, name changed");

    println!("{:?}", u2);

    let u3 = build_user_v1(
        String::from("User3,Build from a factory method `build_user_v1"),
        40,
    );
    println!("{:?}", u3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color Instance: {:?}; Point Instance: {:?}", black, origin);

    let rect = Rectangle {
        width: 2,
        height: 4,
    };
    println!(
        "{},{},{}",
        rect.area(),
        area(&rect),
        Rectangle::area2(&rect)
    );

    // dbg!(&rect);

    let rect1 = Rectangle {
        width: 3,
        height: 3,
    };
    let rect2 = Rectangle {
        width: 3,
        height: 5,
    };

    println!("{:?} can hold {:?}: {}", rect1, rect, rect1.can_hold(&rect));
    println!("{:?} can hold {:?}: {}", rect2, rect, rect2.can_hold(&rect));

    let square = Rectangle::square(9);
    println!("{:?}:{}", square, square.area())
}
