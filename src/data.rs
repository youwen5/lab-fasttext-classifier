use crate::nlp;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct FasttextPair {
    label: String,
    text: String,
}

pub fn process_csv(input_file: &Path, output_file: &Path) -> Result<(), std::io::Error> {
    let file = File::open(input_file)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut data: Vec<FasttextPair> = Vec::new();

    for result in rdr.deserialize() {
        let mut r: FasttextPair = result.expect("bruh");
        r.text = nlp::tokenize(&r.text);
        data.push(r);
    }

    let formatted_output: Vec<String> = data
        .into_iter()
        .map(|pair| format!("{} {}", pair.label, pair.text))
        .collect();

    write_to_file(
        output_file.to_str().expect("could not write output file"),
        &formatted_output,
    )?;

    Ok(())
}

pub fn write_to_file(filename: &str, lines: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
