use std::io::{self, BufReader};
use std::fs::File;

use serde_json::Value;

fn main() -> io::Result<()> {
    let file = File::open("input.json")?;
    let reader = BufReader::new(file);

    let first_value = serde_json::from_reader(reader)?;

    let mut result = recursive_sum(&first_value, None);
    println!("P1> Total is: {}", result);

    result = recursive_sum(&first_value, Some("red"));
    println!("P2> Total is: {}", result);

    Ok(())
}

fn recursive_sum(value: &Value, filter: Option<&str>) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(str) => str.parse().unwrap_or_default(),
        Value::Array(vec) => {
            vec.iter().map(|v| recursive_sum(v, filter)).sum()
        },
        Value::Object(map) => {
            if let Some(color) = filter {
                if map.iter().any(|(_,value)| value == color) {
                    return 0;
                }
            }

            map.iter().map(|(key, value)| recursive_sum(value, filter)).sum()
        },
        _ => todo!("Value type found not supported")
    }
}
