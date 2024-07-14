use std::{fs, ops::Div};

use serialize::serialize_to_string;

mod parser;
mod serialize;

fn main() {
    let path = r"D:\Audio Projects (Reaper)\New\unwelcome school 2\unwelcome school 2_5.RPP";
    let rpp = fs::read_to_string(&path).unwrap();

    println!("Loaded to string, parsing");
    let element = parser::parse_element(&rpp).unwrap();

    println!("Serialializing to result.rpp");
    let text = serialize_to_string(&element);
    fs::write("result.rpp", text).unwrap();

    println!("Benchmark loop");
    let loops = 100;
    let now = std::time::Instant::now();
    for _ in 0..loops {
        serialize_to_string(&element);
    }
    let elapsed = now.elapsed();
    println!(
        "Did {} loops. Total: {:?}, Average: {:?}",
        loops,
        elapsed,
        elapsed.div(loops)
    );
}
