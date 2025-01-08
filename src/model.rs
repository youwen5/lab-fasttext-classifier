use std::path::Path;

use fasttext::{Args, FastText, ModelName};

pub fn train(out: &Path, dataset: &Path) -> Result<(), String> {
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

pub fn query(filename: &Path, query: &str) -> Vec<fasttext::Prediction> {
    let mut text = FastText::new();

    let _ = text.load_model(filename.to_str().unwrap());
    text.predict(query, 3, 0.3).unwrap()
}
