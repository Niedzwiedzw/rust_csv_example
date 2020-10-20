use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::convert::TryFrom;
use std::io::BufReader;
use std::{env, fs};

#[derive(Deserialize, Debug)]
pub struct CsvFood {
    pub name: String,
    pub quantity: u32,
    pub category: String,
    pub active: String,
}

#[derive(Debug, Serialize)]
pub struct Food {
    pub name: String,
    pub quantity: u32,
    pub category: String,
    pub active: bool,
}

impl TryFrom<CsvFood> for Food {
    type Error = String;
    fn try_from(csv: CsvFood) -> Result<Self, Self::Error> {
        let CsvFood {
            name,
            quantity,
            category,
            active,
        } = csv;
        Ok(Self {
            name,
            quantity,
            category,
            active: match &active[..] {
                "yeah" | "TRUE" => true,
                "NO" | "FALSE" => false,
                _ => return Err(format!("invalid value found in `active`: `{}`", active)),
            },
        })
    }
}

fn load_csv_raw<T: AsRef<std::path::Path>>(
    path: T,
) -> Result<Vec<CsvFood>, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut csv_reader = csv::Reader::from_reader(buf_reader);
    let mut results = vec![];
    for result in csv_reader.deserialize() {
        results.push(result?);
    }
    Ok(results)
}

fn main() {
    let mut foods = load_csv_raw(env::args().skip(1).next().expect("specify file path"))
        .expect("loading csvs failed")
        .into_iter()
        .map(std::convert::TryInto::try_into)
        .filter_map::<Food, _>(|result| match result {
            Ok(food) => Some(food),
            Err(e) => {
                eprintln!("<ERROR> :: {}", e);
                None
            },
        })
        .filter(|food| food.active)
        .collect::<Vec<Food>>();

    foods.sort_by(|one, other| one.category.cmp(&other.category));
    println!(
        "{}",
        to_string_pretty(&foods).expect("this will always serialize")
    );
}
