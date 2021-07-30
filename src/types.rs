use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Fasta {
    pub class: String,
    pub email: String,
    pub hash_function: String,
    pub filename: String,
    pub license: String,
    pub signatures: Vec<Signature>
}

#[derive(Debug, Deserialize)]
pub struct Signature {
    pub num: usize,
    pub ksize: usize,
    pub seed: usize,
    pub max_hash: usize,
    pub mins: Vec<usize>,
    pub abundances: Vec<usize>,
}

#[derive(Debug, Serialize)]
pub struct Row {
    pub hash: usize,
    pub abundance: usize,
}