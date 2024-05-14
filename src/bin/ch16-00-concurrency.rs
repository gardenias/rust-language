use std::{sync::mpsc::channel, thread};

fn main() {

    let (tx, rx) = channel();
    let sender = thread::spawn(move || {
        tx.send("Hello, thread".to_owned())
            .expect("Unable to send on channel");
    });
    let receiver = thread::spawn(move || {
        let value = rx.recv().expect("Unable to receive from channel");
        println!("{value}");
    });

    let _sender_result = sender.join();
    let _receiver_result = receiver.join();

}
