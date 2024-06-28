use std::collections::HashSet;

use arbitrary::{Arbitrary};

#[derive(Arbitrary, Debug)]
pub struct AddressBook {
    pub friends: HashSet<Friend>,
}

#[derive(Arbitrary, Hash, Eq, PartialEq, Debug)]
pub enum Friend {
    Buddy { name: String },
    Pal { age: usize },
}

fn do_stuff(address_book: AddressBook) {
    println!("{:?}", address_book);
}

fn main() {
    // Get the raw data from the fuzzer or wherever else.
    // let raw_data: &[u8] = &[1, 2, 3, 4];
    //
    // // Wrap that raw data in an `Unstructured`.
    // let mut unstructured = Unstructured::new(raw_data);
    //
    // println!("unstructured len = {:?}", unstructured.arbitrary_len::<u8>());
    //
    // // Generate an arbitrary instance of `MyType` and do stuff with it.
    // if let Ok(value) = <AddressBook as Arbitrary>::arbitrary(&mut unstructured) {
    //     do_stuff(value);
    // }
    // println!("unstructured len = {}", unstructured.arbitrary_len().into());
}
