// the standard library: enum.IpAddr & enum.Option

#[derive(Debug)]
struct QuitMessage; // unit struct

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String); // tuple struct
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
enum Message {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}
impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let quit_msg_struct = QuitMessage {};
    let move_msg_struct = MoveMessage { x: 12, y: 21 };
    let writ_msg_struct = WriteMessage(String::from("writ msg"));
    let change_color_msg_struct = ChangeColorMessage(1, 2, 3);

    let quit = Message::Quit(quit_msg_struct);
    let move_enum = Message::Move(move_msg_struct);
    let msg: Message = Message::Write(writ_msg_struct);
    let change_color = Message::ChangeColor(change_color_msg_struct);
    quit.call();
    move_enum.call();
    msg.call();
    change_color.call();

    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // OR  _ => (),
        // OR  _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}

    fn reroll() {}

    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Other type coin")
    }
}
