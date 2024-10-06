#!/bin/env rust

//   Copyright 2024 Matthew Ralston
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::io::{self, BufRead};

use clap::{arg, builder::NonEmptyStringValueParser, command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: Option<String>,

    /// Number of times to greet
    #[arg(long, default_value_t = false)]
    header: bool,
}

fn main() {
    let args = Args::parse();
    let lines: Option<Vec<String>>;
//    let stdin = io::stdin();
//    let _lines: Vec<String> = stdin.lock().lines()
//        .map(|line| line.expect("Could not read line"))
//        .collect();

    match args.input {
        Some(input) => {
            let lines: Option<Vec<String>> = read_from_tsv_file(input);
            match lines {
                Some(lines) => println!("Lines were {}", lines.join("\n")),
                _ => println!("Did not read a damn thing..."),
            }

        },
        _ => println!("Nothing provided as --input"),
    }
    match args.header {
        true => println!("A --header argument was provided. Flag indicating presence of a header line.."),
        false => println!("No --header line anticipated"),
    }
}


fn read_from_tsv_file(filename: String) -> Option<Vec<String>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};


    let file = File::open(filename).ok()?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines()
        .map(|line| line.ok())
        .collect::<Option<Vec<String>>>()?;

    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }


}
