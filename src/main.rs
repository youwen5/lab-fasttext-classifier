use std::path::Path;

use fasttext::{Args, FastText, LossName, ModelName};

fn train_cooking_model() -> Result<(), String> {
    let mut args = Args::new();
    args.set_input("../data/cooking.train").unwrap();
    args.set_model(ModelName::SUP);
    args.set_lr(1.0);
    args.set_epoch(25);
    //args.set_loss(LossName::SOFTMAX);
    let mut ft_model = FastText::new();
    ft_model.train(&args).unwrap();

    ft_model.save_model("../model-out/out.bin")
}

fn test_cooking_model(filename: &Path) -> Vec<fasttext::Prediction> {
    let mut text = FastText::new();

    let _ = text.load_model(filename.to_str().unwrap());
    text.predict("Safe temperatures to bake cookies at?", 3, 0.2)
        .unwrap()
}

fn main() {
    //train_cooking_model().unwrap();
    let result = test_cooking_model(Path::new("../model-out/out.bin"));
    println!("{:?}", result);
}
