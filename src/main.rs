use core::fmt;
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IOResult;
use std::io;

fn f1() -> fmt::Result {
    fmt::Result::Ok(())
}
fn f2() -> io::Result<()> {
    io::Result::Ok(())
}

fn f3() -> Result{
    Result::Ok(())
}
fn f4() -> IOResult<()>{
    IOResult::Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map);
}
