use std::fs;

use itertools::Itertools;
use parser::Element;
use serialize::serialize_to_string;

mod parser;
mod serialize;

fn write_element_size_tree(
    buf: &mut String,
    element: &Element,
    max_percentage: f64,
    indent_level: usize,
) {
    let size = serialize_to_string(element).len();

    // first line
    for _ in 0..indent_level {
        buf.push_str("  ")
    }
    buf.push_str(element.tag);
    buf.push(' ');
    buf.push_str(format!("{max_percentage:.02}%").as_str());
    buf.push(' ');
    buf.push_str(element.attr.iter().join(" ").as_str());
    buf.push('\n');

    // records of things by their size
    let mut records = vec![];
    let mut lines_size: u64 = 0;
    for child in element.children.iter() {
        match child {
            parser::Child::Line(child) => {
                lines_size += child.iter().map(|x| x.len() as u64).sum::<u64>()
            }
            parser::Child::Element(child) => {
                let child_size = serialize_to_string(child).len();
                records.push((child, child_size as u64));
            }
        }
    }
    let lines_element = Element {
        tag: "__TEMP_LINES__",
        attr: vec![],
        children: vec![],
    };
    records.push((&lines_element, lines_size));

    // sort records in ascending order
    records.sort_by_key(|(_, size)| *size);

    // reverse iterate
    for (element, child_size) in records.iter().rev() {
        let child_percentage = max_percentage * *child_size as f64 / size as f64;

        if element.tag == "__TEMP_LINES__" {
            for _ in 0..(indent_level + 1) {
                buf.push_str("  ")
            }
            buf.push_str("[Lines]");
            buf.push(' ');
            buf.push_str(format!("{child_percentage:.02}%").as_str());
            buf.push('\n');
        } else {
            write_element_size_tree(buf, element, child_percentage, indent_level + 1);
        }
    }
}

fn main() {
    let path = r"D:\Audio Projects (Reaper)\New\unwelcome school 2\unwelcome school 2_5.RPP";
    let rpp = fs::read_to_string(path).unwrap();

    println!("Loaded to string, parsing");
    let element = parser::parse_element(&rpp).unwrap();

    println!("Measure size to ./size.txt");
    let mut buf = String::new();
    write_element_size_tree(&mut buf, &element, 100.0, 0);
    fs::write("size.txt", buf).unwrap();
}

#[cfg(test)]
mod tests {
    use std::ops::Div;

    use super::*;

    #[test]
    fn write_to_file() {
        let path = r"D:\Audio Projects (Reaper)\New\unwelcome school 2\unwelcome school 2_5.RPP";
        let rpp = fs::read_to_string(&path).unwrap();

        println!("Loaded to string, parsing");
        let element = parser::parse_element(&rpp).unwrap();

        println!("Serialializing to result.rpp");
        let text = serialize_to_string(&element);
        fs::write("result.rpp", text).unwrap();
    }

    #[test]
    fn benchmark() {
        let path = r"D:\Audio Projects (Reaper)\New\unwelcome school 2\unwelcome school 2_5.RPP";
        let rpp = fs::read_to_string(&path).unwrap();

        println!("Loaded to string, parsing");
        let element = parser::parse_element(&rpp).unwrap();

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
}
