use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    fist_name: String,
    age: u8,
    phones: Vec<String>,
}
fn untyped_example() -> Result<Person> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "fist_name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Person = serde_json::from_str(data)?;

    Ok(v)
}

fn main() {
    let result = untyped_example();
    let value = result.unwrap();
    println!("{:?}", value);
}
