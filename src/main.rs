#[macro_use]
extern crate serde_derive;

mod data;
mod model;
mod nlp;

use std::env::args;
use std::path::Path;

fn cli() {
    let pattern = args().nth(1).expect("no command given!");
    match pattern.as_str() {
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
        _ => panic!("subcommand does not exist"),
    }
}

fn main() {
    cli()
}
