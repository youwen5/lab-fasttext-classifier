use std::env;
use std::path::Path;

use fasttext::{Args, FastText, ModelName};

fn train_model(out: &Path, dataset: &Path) -> Result<(), String> {
    let mut args = Args::new();
    args.set_input(dataset.to_str().unwrap()).unwrap();
    args.set_model(ModelName::SUP);
    args.set_lr(1.0);
    args.set_epoch(25);
    //args.set_loss(LossName::SOFTMAX);
    let mut ft_model = FastText::new();
    ft_model.train(&args).unwrap();

    ft_model.save_model(out.to_str().unwrap())
}

fn test_model(filename: &Path, query: &str) -> Vec<fasttext::Prediction> {
    let mut text = FastText::new();

    let _ = text.load_model(filename.to_str().unwrap());
    text.predict(query, 3, 0.3).unwrap()
}

fn cli() {
    let pattern = env::args().nth(1).expect("no command given!");
    match pattern.as_str() {
        "test" => {
            let model_path = env::args().nth(2).expect("no path to model given");
            let query = env::args().nth(3).expect("no query to run");

            println!("{:?}", test_model(Path::new(&model_path), &query));
        }
        "train" => {
            let out_path = env::args().nth(2).expect("no path to model given");
            let dataset_path = env::args().nth(3).expect("no path to dataset given");
            train_model(Path::new(&out_path), Path::new(&dataset_path)).unwrap();
        }
        _ => panic!("subcommand does not exist"),
    }
}

fn main() {
    cli()
}
