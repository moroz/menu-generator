use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;

use handlebars::to_json;
use serde::{Deserialize, Serialize};
use serde_json::value::{Map, Value as Json};

#[derive(Debug, Deserialize, Serialize)]
struct MenuItem {
    #[serde(alias = "Nazwa potrawy (PL)")]
    name: String,
    #[serde(alias = "Kategoria")]
    category: String,
    #[serde(alias = "Cena (PLN)")]
    price: Option<f64>,
}

fn build_reader(filename: String) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn build_template_registry<'a>() -> handlebars::Handlebars<'a> {
    let mut reg = handlebars::Handlebars::new();
    reg.register_template_file("layout", "./templates/layout.tex.hbs")
        .unwrap();
    reg
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let reader = build_reader(filename);
    let mut csv = csv::Reader::from_reader(reader);
    let reg = build_template_registry();

    let items: Vec<MenuItem> = csv.deserialize().filter_map(|r| r.ok()).collect();

    let mut assigns = Map::new();
    assigns.insert("items".to_string(), to_json(&items));

    let result = reg.render("layout", &assigns).unwrap();
    println!("{result}");

    Ok(())
}
