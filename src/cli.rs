use rayon::ThreadPoolBuilder;
use std::env::args;
use std::path::Path;

use crate::{
    data::{self, write_fasttext_data_to_file},
    model::{self, benchmark},
};

fn print_help() {
    println!(
        r#"
error: no subcommand specified or subcommand invalid

usage help:

lab-intent-classifier <subcommand>

subcommands:
    train <dataset_path> <model_output_path>
        -> trains a new model from a dataset outputted by `tokenize`

    process <csv_path> <dataset_output_path>
        -> processes a csv of (label, text) into a format ready for use in training

    predict <model_path> <query>
        -> uses a trained model to label a query

    split-csv <input_csv_path> <training_csv_output_path> <validation_csv_output_path>
        -> given an input CSV like the one you would pass to `tokenize`, deterministically
        randomize the order and then place the 3/4 into a new CSV to be used for training
        at <training_csv_output_path>, then place the remaining 1/4 into a CSV to be used for
        model validation at <validation_csv_output_path>

    benchmark <model_path> <validation_csv_path> <optional:max_threads>
        -> benchmarks a given model by running a validation dataset through it, generated from
        `split-csv`. uses multiple threads for speed, if it is using too many resources,
        set max_threads to 4 or below.

example usage:
    lab-intent-classifier train ./data/tokenized.txt ./model.bin
    lab-intent-classifier tokenize ./data/dataset.csv ./data/tokenized_new.txt
    lab-intent-classifier predict ./model.bin "where is the thermometer?"
    lab-intent-classifier split-csv ./data/dataset.csv ./data/dataset.train.csv ./data/dataset.valid.csv
    lab-intent-classifier benchmark ./model.bin ./data/dataset.valid.csv
"#
    );
}

pub fn run() {
    if let Some(subcommand) = args().nth(1) {
        match subcommand.as_str() {
            "predict" => {
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
            "process" => {
                let input_path_raw = args().nth(2).expect("no input path");
                let output_path_raw = args().nth(3).expect("no output path");

                let input_path = Path::new(&input_path_raw);
                let output_path = Path::new(&output_path_raw);

                let output_data = data::parse_csv(input_path).unwrap();

                write_fasttext_data_to_file(output_data, output_path).unwrap();
            }
            "split-csv" => {
                let input_csv_path = args().nth(2).expect("no input csv specified");
                let training_dataset_path = args()
                    .nth(3)
                    .expect("no training dataset output path specified");
                let validation_dataset_path = args()
                    .nth(4)
                    .expect("no validation dataset output path specified");

                data::split_dataset_csv(
                    Path::new(&input_csv_path),
                    Path::new(&training_dataset_path),
                    Path::new(&validation_dataset_path),
                )
                .expect("failed to write output CSVs");

                println!("wrote output CSVs successfully");
            }
            "benchmark" => {
                let model_path = args().nth(2).expect("no model path provided");
                let validation_dataset_path =
                    args().nth(3).expect("no validation dataset path specified");
                let max_threads = str::parse::<usize>(&args().nth(4).unwrap_or("10".to_string()))
                    .expect("max_threads must be an int");

                let pool = ThreadPoolBuilder::new()
                    .num_threads(max_threads)
                    .build()
                    .unwrap();

                pool.install(|| {
                    let accuracy = benchmark(
                        Path::new(&model_path),
                        data::parse_csv(Path::new(&validation_dataset_path)).unwrap(),
                    );

                    println!("Accuracy: {}", accuracy);
                });
            }
            _ => print_help(),
        }
    } else {
        print_help();
    };
}
