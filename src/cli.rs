use std::env::args;
use std::path::Path;

use crate::{data, model};

fn print_help() {
    println!(
        r#"
error: no subcommand specified or subcommand invalid

usage help:

lab-intent-classifier <subcommand>

subcommands:
    train <dataset_path> <model_output_path>
    tokenize <csv_path> <dataset_output_path>
    query <dataset_path> <query>

example usage:
    lab-intent-classifier train ./data/tokenized.txt ./model.bin
    lab-intent-classifier tokenize ./data/dataset.csv ./data/tokenized_new.txt
    lab-intent-classifier query ./model.bin "where is the thermometer?"
"#
    );
}

pub fn run() {
    if let Some(subcommand) = args().nth(1) {
        match subcommand.as_str() {
            "test" => {
                let model_path = args().nth(2).expect("no path to model given");
                let query = args().nth(3).expect("no query to run");

                println!(
                    "{:?}",
                    model::tokenize_and_query(Path::new(&model_path), &query)
                );
            }
            "train" => {
                let dataset_path = args().nth(2).expect("no path to dataset given");
                let out_path = args().nth(3).expect("no path to model given");
                model::train(Path::new(&out_path), Path::new(&dataset_path)).unwrap();
            }
            "tokenize" => {
                let input_path_raw = args().nth(2).expect("no input path");
                let output_path_raw = args().nth(3).expect("no output path");

                let input_path = Path::new(&input_path_raw);
                let output_path = Path::new(&output_path_raw);

                data::process_csv(input_path, output_path).unwrap();
            }
            _ => print_help(),
        }
    } else {
        print_help();
    };
}
