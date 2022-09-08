use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MenuItem {
    #[serde(alias = "Nazwa potrawy (PL)")]
    name: String,
    #[serde(alias = "Kategoria")]
    category: String,
    #[serde(alias = "Cena (PLN)")]
    price: f64,
}

fn build_reader(filename: String) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let reader = build_reader(filename);
    let mut csv = csv::Reader::from_reader(reader);

    for result in csv.deserialize() {
        let result: MenuItem = result?;
        println!("{:?}", result);
    }

    Ok(())
}
