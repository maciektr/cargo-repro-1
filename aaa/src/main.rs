use serde_json::value::RawValue;

fn main() {
    let _ = serde_json::from_str::<&RawValue>(r#"{"a": "b"}"#);
    println!("Hello, world!");
}
