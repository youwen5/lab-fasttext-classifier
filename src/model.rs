extern crate serde;

use std::path::Path;

use fasttext::{Args, FastText, LossName, ModelName};

pub fn train(out: &Path, dataset: &Path) -> Result<(), String> {
    let mut args = Args::new();
    args.set_input(dataset.to_str().unwrap()).unwrap();
    args.set_model(ModelName::SUP);
    args.set_lr(1.0);
    args.set_epoch(25);
    args.set_loss(LossName::SOFTMAX);
    let mut ft_model = FastText::new();
    ft_model.train(&args).unwrap();

    ft_model.save_model(out.to_str().unwrap())
}

pub fn tokenize_and_query(filename: &Path, query: &str) -> Vec<fasttext::Prediction> {
    let mut ft_model = FastText::new();
    let query_tokenized = crate::nlp::tokenize(query);

    let _ = ft_model.load_model(filename.to_str().unwrap());
    ft_model.predict(&query_tokenized, 2, 0.2).unwrap()
}
