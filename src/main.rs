use std::fs;

mod parser;
mod serialize;

fn main() {
    let path = r"D:\Audio Projects (Reaper)\temp3.rpp";
    let rpp = fs::read_to_string(&path).unwrap();
    println!("Loaded to string, parsing...");
    let rpp = parser::parse_element(&rpp).unwrap();

    println!("Success!");
}
