use std::fs::File;
use std::io;
use std::io::BufReader;
use std::{collections::HashMap, env};

use handlebars::{handlebars_helper, to_json};
use serde::{Deserialize, Serialize};
use serde_json::value::Map;

#[derive(Debug, Deserialize, Serialize)]
struct MenuItem {
    name_pl: String,
    name_en: Option<String>,
    description_pl: Option<String>,
    description_en: Option<String>,
    category: String,
    price: Option<f64>,
}

#[derive(Debug, Serialize)]
struct MenuCategory {
    name: String,
    items: Vec<MenuItem>,
}

fn build_reader(filename: String) -> BufReader<File> {
    let file = File::open(filename).unwrap();
    BufReader::new(file)
}

fn escape_strings<'a>(data: &'a str) -> String {
    data.replace("&", "\\&").to_string()
}

fn do_format_price(price: f64) -> String {
    if price == price.round() {
        format!("{} PLN", price as i64)
    } else {
        let str = price.to_string().replace(".", ",");
        format!("{} PLN", str)
    }
}

fn build_template_registry<'a>() -> handlebars::Handlebars<'a> {
    handlebars_helper!(trim: |data: String| data.trim().replace("&amp;", "\\&"));
    handlebars_helper!(format_price: |data: f64| do_format_price(data));

    let mut reg = handlebars::Handlebars::new();
    reg.register_template_file("layout", "./templates/layout.tex.hbs")
        .unwrap();

    reg.register_helper("trim", Box::new(trim));
    reg.register_helper("format_price", Box::new(format_price));
    reg.register_escape_fn(escape_strings);
    reg
}

fn group_dishes(dishes: Vec<MenuItem>) -> HashMap<String, Vec<MenuItem>> {
    let mut result = HashMap::new();
    for item in dishes {
        let key = item.category.clone();
        let vector = result.entry(key).or_insert(vec![]);
        vector.push(item)
    }
    result
}

fn grouped_to_categories(map: HashMap<String, Vec<MenuItem>>) -> Vec<MenuCategory> {
    map.into_iter()
        .map(move |(key, items)| MenuCategory { name: key, items })
        .collect()
}

fn main() -> io::Result<()> {
    let filename = env::args().nth(1).unwrap();
    let reader = build_reader(filename);
    let mut csv = csv::Reader::from_reader(reader);
    let reg = build_template_registry();

    let items: Vec<MenuItem> = csv.deserialize().filter_map(|r| r.ok()).collect();

    let grouped = grouped_to_categories(group_dishes(items));

    let mut assigns = Map::new();
    assigns.insert("groups".to_string(), to_json(&grouped));

    let result = reg.render("layout", &assigns).unwrap();
    println!("{result}");

    Ok(())
}
