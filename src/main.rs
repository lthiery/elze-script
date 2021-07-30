mod types;
use types::*;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "protein-json-to-csv", about = "A utility for translating a protein json read to a csv")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Index of data
    #[structopt(default_value = "0")]
    data_index: usize,

    /// Index of signature
    #[structopt(default_value = "0")]
    signatures_index: usize,
}


fn main() {
    use std::fs;
    use csv::Writer;

    let opt = Opt::from_args();

    let contents = fs::read_to_string(opt.input)
        .expect("Something went wrong reading the file");

    let data: Vec<Fasta> = serde_json::from_str(&contents).unwrap();

    println!("len data {}", data.len());
    let data = &data[opt.data_index];
    println!("len data.signatures {}", data.signatures.len());
    let signatures = &data.signatures[opt.signatures_index];
    println!("len data.signatures.mins {}", signatures.mins.len());
    println!("len data,signatures.abundances {}", signatures.abundances.len());

    let mut wtr = if let Some(output) = opt.output {
        Some(Writer::from_path(output).unwrap())
    } else {
        None
    };

    for (i, hash) in signatures.mins.iter().enumerate() {
        let abundance = signatures.abundances[i];
        let row = Row {
            hash: *hash,
            abundance
        };
        if let Some(wtr) = &mut wtr {
            wtr.serialize(&row).unwrap();
        } else {
            println!("{:?}", row);
        }
    }

    if let Some(wtr) = &mut wtr {
        wtr.flush().unwrap();
    };

}
