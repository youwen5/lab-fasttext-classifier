extern crate serde;

use crate::data::FasttextPair;
use std::path::Path;

use fasttext::{Args, FastText, LossName, ModelName};
use rayon::prelude::*;

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

pub fn tokenize_and_query(model_path: &Path, query: &str) -> Vec<fasttext::Prediction> {
    let mut ft_model = FastText::new();
    let query_tokenized = crate::nlp::tokenize(query);

    let _ = ft_model.load_model(model_path.to_str().unwrap());
    ft_model.predict(&query_tokenized, 2, 0.2).unwrap()
}

pub fn benchmark(model_path: &Path, validation_data: Vec<FasttextPair>) -> f64 {
    let valid_count = validation_data
        .par_iter()
        .map(|pair| {
            let predictions = tokenize_and_query(model_path, &pair.text);

            if !predictions.is_empty() && predictions[0].label == pair.label {
                1
            } else {
                0
            }
        })
        .sum::<usize>();

    valid_count as f64 / validation_data.len() as f64
}
