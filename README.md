# rpp-parser

A parser for Reaper project files implemented using [nom](https://github.com/rust-bakery/nom).

## Usage

```rust
use std::fs;

use rpp_parser::{
    parser::{parse_element, Element},
    serialize::serialize_to_string,
};

fn main() {
    let text: String = fs::read_to_string("D:/dubstep.rpp").unwrap();

    let element: Element = parse_element(&text).unwrap();
    println!(
        "{:?} {:?} {:?}",
        element.tag,
        element.attr,
        element.children.len()
    );

    let rpp: String = serialize_to_string(&element);
    println!("{:?}", &rpp[..200]);
}
```

Output:

```plain
"REAPER_PROJECT" ["0.1", "7.18/win64", "1720945588"] 236
"<REAPER_PROJECT 0.1 7.18/win64 1720945588\n  <NOTES 0 2\n  >\n  RIPPLE 0\n  GROUPOVERRIDE 0 0 0\n  AUTOXFADE 385\n  ENVATTACH 0\n  POOLEDENVATTACH 0\n  MIXERUIFLAGS 11 48\n  ENVFADESZ10 40\n  PEAKGAIN 1\n  FEEDB"
```
