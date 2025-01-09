use crate::nlp;
use csv::Writer;
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct FasttextPair {
    pub label: String,
    pub text: String,
}

pub fn parse_csv(input_file: &Path) -> Result<Vec<FasttextPair>, std::io::Error> {
    let file = File::open(input_file)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut data: Vec<FasttextPair> = Vec::new();

    for result in rdr.deserialize() {
        let mut r: FasttextPair = result.expect("bruh");
        r.text = nlp::tokenize(&r.text);
        data.push(r);
    }

    Ok(data)
}

pub fn write_fasttext_data_to_file(
    data: Vec<FasttextPair>,
    output_file: &Path,
) -> Result<(), std::io::Error> {
    let formatted_output: Vec<String> = fasttext_vec_to_str(data);

    write_to_file(output_file, &formatted_output)?;

    Ok(())
}

fn fasttext_vec_to_str(vec: Vec<FasttextPair>) -> Vec<String> {
    vec.into_iter()
        .map(|pair| format!("{} {}", pair.label, pair.text))
        .collect::<Vec<String>>()
}

pub fn write_to_file(filename: &Path, lines: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

fn deterministic_shuffle_vector<T>(vec: &mut [T]) {
    let seed: u64 = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    vec.shuffle(&mut rng)
}

pub fn split_dataset_csv(
    input_file: &Path,
    training_path: &Path,
    validation_path: &Path,
) -> Result<(), std::io::Error> {
    let file = File::open(input_file)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut data: Vec<FasttextPair> = Vec::new();

    for result in rdr.deserialize() {
        data.push(result.expect("bruh"));
    }

    deterministic_shuffle_vector(&mut data);

    let split_index = (data.len() * 3) / 4;

    let training_data = &data[..split_index];
    let validation_data = &data[split_index..];

    let mut training_wtr = Writer::from_path(training_path).unwrap();
    let mut validation_wtr = Writer::from_path(validation_path).unwrap();

    for pair in training_data {
        training_wtr.serialize(pair)?;
    }

    for pair in validation_data {
        validation_wtr.serialize(pair)?;
    }

    training_wtr.flush()?;
    validation_wtr.flush()?;

    Ok(())
}
