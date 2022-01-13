#![feature(stdin_forwarders)]

use colored_json::ToColoredJson;
use serde_json::{Map, Value};
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let no_color = args.contains(&"--no-color".to_string());
    let from_file = args.contains(&"--file".to_string());

    if from_file {
        let file_name = args.iter().skip_while(|arg| *arg != "--file").next();
        match file_name {
            None => {
                eprintln!("File name not provided! File is mandatory with --file option");
                std::process::exit(132);
            }
            Some(file_name) => match std::fs::File::open(file_name) {
                Err(_) => {
                    eprintln!("File not found: {}", file_name);
                    std::process::exit(132);
                }
                Ok(file) => {
                    for line in BufReader::new(file).lines() {
                        pretty_log(line.unwrap().replace("\\n\\t", "\n\t"), no_color);
                    }
                }
            },
        }
    } else {
        std::io::stdin()
            .lines()
            .filter_map(|may_be_line| may_be_line.ok().map(|it| it.replace("\\n\\t", "\n\t")))
            .for_each(|line| pretty_log(line, no_color));
    }

    fn pretty_log(line: String, no_color: bool) {
        let log = match &line.rsplit_once(" {") {
            None => line,
            Some((_, log)) => "{".to_owned() + *log,
        };
        match serde_json::from_str::<Map<String, Value>>(&log) {
            Ok(map) => match serde_json::to_string_pretty(&map) {
                Err(_) => println!("{}", log),
                Ok(pretty_log) => {
                    if no_color {
                        println!("{}", pretty_log)
                    } else {
                        match &pretty_log.to_colored_json_auto() {
                            Err(_) => println!("{}", &pretty_log),
                            Ok(colored_json) => println!("{}", colored_json),
                        }
                    }
                }
            },
            Err(_) => println!("{}", log),
        }
    }
}
