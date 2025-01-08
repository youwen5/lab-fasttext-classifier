use itertools::Itertools;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::HashSet;
use stopwords::{Language, Spark, Stopwords};
use vtext::tokenize::{Tokenizer, VTextTokenizer};

pub fn tokenize(text: &str) -> String {
    // convert all to lowercase
    let lc_text = text.to_lowercase();

    // tokenise the words
    let tok = VTextTokenizer::default();
    let tokens: Vec<&str> = tok.tokenize(lc_text.as_str()).collect();

    // stem the words
    let en_stemmer = Stemmer::create(Algorithm::English);
    let tokens: Vec<String> = tokens
        .iter()
        .map(|x| en_stemmer.stem(x).into_owned())
        .collect();
    let mut tokens: Vec<&str> = tokens.iter().map(|x| x.as_str()).collect();

    // remove the stopwords
    let stops: HashSet<_> = Spark::stopwords(Language::English)
        .unwrap()
        .iter()
        .collect();
    tokens.retain(|s| !stops.contains(s));

    // join the tokens and return
    tokens.iter().join(" ")
}
