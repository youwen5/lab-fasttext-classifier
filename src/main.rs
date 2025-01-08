mod model;

use std::env::args;
use std::path::Path;

fn cli() {
    let pattern = args().nth(1).expect("no command given!");
    match pattern.as_str() {
        "test" => {
            let model_path = args().nth(2).expect("no path to model given");
            let query = args().nth(3).expect("no query to run");

            println!("{:?}", model::query(Path::new(&model_path), &query));
        }
        "train" => {
            let out_path = args().nth(2).expect("no path to model given");
            let dataset_path = args().nth(3).expect("no path to dataset given");
            model::train(Path::new(&out_path), Path::new(&dataset_path)).unwrap();
        }
        _ => panic!("subcommand does not exist"),
    }
}

fn main() {
    cli()
}
