
mod common;

#[test]
fn test_slice() {

    common::setup();

    let x32 = 0_u32.to_be_bytes();
    println!("{}", x32.len());

}
