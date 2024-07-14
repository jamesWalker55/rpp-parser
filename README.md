# rpp-parser

A parser for Reaper project files implemented using [nom](https://github.com/rust-bakery/nom).

## Usage

```rust
use std::fs;

use rpp_parser::{
    parser::{parse_element, Child, Element},
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

    println!();
    for child in element.children.iter() {
        match child {
            Child::Line(child) => {
                println!("{:?}", &child);
            }
            Child::Element(child) => {
                println!("{:?}", &child.tag);
            }
        }
    }

    let rpp: String = serialize_to_string(&element);
    println!();
    println!("{:?}", &rpp[..200]);
}

```

Output:

```plain
"REAPER_PROJECT" ["0.1", "7.18/win64", "1720945588"] 236

"NOTES"
["RIPPLE", "0"]
["GROUPOVERRIDE", "0", "0", "0"]
["AUTOXFADE", "385"]
["ENVATTACH", "0"]
["POOLEDENVATTACH", "0"]
["MIXERUIFLAGS", "11", "48"]
["ENVFADESZ10", "40"]
["PEAKGAIN", "1"]
["FEEDBACK", "0"]
["PANLAW", "1"]
["PROJOFFS", "0", "0", "0"]
["MAXPROJLEN", "0", "600"]
["GRID", "1151", "8", "1", "8", "1", "0", "0", "0"]
["TIMEMODE", "1", "5", "-1", "30", "0", "0", "-1"]
["VIDEO_CONFIG", "0", "0", "256"]
["PANMODE", "3"]
["CURSOR", "111.15789473684211"]
["ZOOM", "20.81655352044544", "1953", "0"]
["VZOOMEX", "0", "1229"]
["USE_REC_CFG", "0"]
["RECMODE", "1"]
["SMPTESYNC", "0", "30", "100", "40", "1000", "300", "0", "0", "1", "0", "0"]
["LOOP", "1"]
["LOOPGRAN", "0", "4"]
["RECORD_PATH", "Audio", ""]
"RECORD_CFG"
"APPLYFX_CFG"
["RENDER_FILE", ""]
["RENDER_PATTERN", "unwelcome", "school", "2"]
["RENDER_FMT", "0", "2", "44100"]
["RENDER_1X", "0"]
["RENDER_RANGE", "3", "0", "0", "18", "1000"]
["RENDER_RESAMPLE", "10", "10", "1"]
["RENDER_ADDTOPROJ", "0"]
["RENDER_STEMS", "8"]
["RENDER_DITHER", "0"]
["TIMELOCKMODE", "2"]
["TEMPOENVLOCKMODE", "1"]
["ITEMMIX", "1"]
["DEFPITCHMODE", "589824", "0"]
["TAKELANE", "1"]
["SAMPLERATE", "44100", "0", "0"]
"RENDER_CFG"
["LOCK", "1"]
"METRONOME"
["GLOBAL_AUTO", "-1"]
["TEMPO", "190", "4", "4"]
["PLAYRATE", "1", "1", "0.25", "4"]
["SELECTION", "0", "0"]
["SELECTION2", "0", "0"]
["MASTERAUTOMODE", "0"]
["MASTERTRACKHEIGHT", "0", "0"]
["MASTERPEAKCOL", "16576"]
["MASTERMUTESOLO", "24"]
["MASTERTRACKVIEW", "1", "0.6667", "0.5", "0.5", "0", "0", "0", "0", "0", "0", "0", "0", "0", "0"]
["MASTERHWOUT", "0", "0", "0.64382266579432", "0", "0", "0", "0", "-1"]
["MASTER_NCH", "2", "2"]
["MASTER_VOLUME", "0.82268743809333", "0", "-1", "-1", "1"]
["MASTER_PANMODE", "3"]
["MASTER_FX", "1"]
["MASTER_SEL", "0"]
"MASTERFXLIST"
"POOLEDENV"
"POOLEDENV"
"POOLEDENV"
"POOLEDENV"
"MASTERPLAYSPEEDENV"
"TEMPOENVEX"
["MARKER", "6", "11.36842105263158", "render", "5", "24803314", "1", "R", "{8C219FB0-2FF8-4161-ACB1-769D6DCFEF31}", "0"]
["MARKER", "6", "217.26315789473682", "", "5"]
["MARKER", "1", "12.63157894736842", "", "1", "0", "1", "R", "{7DECCE7B-D73A-4CB9-82CF-9501EA627AF3}", "0"]
["MARKER", "1", "32.8421052631579", "", "1"]
["MARKER", "4", "50.52631578947368", "", "1", "0", "1", "R", "{649D1FB5-4538-4785-ADF7-A2CB20F23D3F}", "0"]
["MARKER", "4", "90.94736842105263", "", "1"]
["MARKER", "5", "90.94736842105263", "", "1", "0", "1", "R", "{F289C943-75D8-4DF6-9E7B-4DD85F61B5C6}", "0"]
["MARKER", "5", "111.15789473684211", "", "1"]
["MARKER", "9", "111.15789473684211", "", "1", "0", "1", "R", "{937A9027-070A-4133-8F1E-20F75586C81D}", "0"]
["MARKER", "9", "131.36842105263156", "", "1"]
["MARKER", "8", "131.36842105263156", "", "1", "0", "1", "R", "{24E7A6F4-BFEF-4374-ACDA-0A647B787B46}", "0"]
["MARKER", "8", "151.57894736842104", "", "1"]
["MARKER", "10", "151.57894736842104", "", "1", "0", "1", "R", "{04BEA6EF-459E-42BD-A67A-893774809519}", "0"]
["MARKER", "10", "171.78947368421052", "", "1"]
["MARKER", "7", "212.21052631578948", "midi", "end", "1", "0", "1", "R", "{35874322-C7B8-495E-BE30-4E97CA3E5889}", "0"]
["MARKER", "7", "216", "", "1"]
["MARKER", "12", "260.21052631578948", "", "1", "0", "1", "R", "{8849231E-8ABC-41E4-96E3-D164AE30785A}", "0"]
["MARKER", "12", "280.42105263157896", "", "1"]
["MARKER", "3", "349.89473684210526", "", "1", "0", "1", "R", "{A2109698-7D51-4CCF-B75B-1F357C1A8D1A}", "0"]
["MARKER", "3", "461.05263157894734", "", "1"]
"PROJBAY"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"TRACK"
"EXTENSIONS"

"<REAPER_PROJECT 0.1 7.18/win64 1720945588\n  <NOTES 0 2\n  >\n  RIPPLE 0\n  GROUPOVERRIDE 0 0 0\n  AUTOXFADE
385\n  ENVATTACH 0\n  POOLEDENVATTACH 0\n  MIXERUIFLAGS 11 48\n  ENVFADESZ10 40\n  PEAKGAIN 1\n  FEEDB"
```
