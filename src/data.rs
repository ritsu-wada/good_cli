use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    word: Vec<String>,
}

fn setup_jf() -> Vec<Data> {
    let path = "./data.json";
    if !Path::new(path).exists() {
        let mut file = File::create(path).expect("fail to create file");
        file.write_all(b"[]").expect("fali to write init data");
    }

    let file = File::open(path).expect("fali to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|e| {
        eprintln!("fail to paerse JSON: {}. use empty list []", e);
        Vec::new()
    })
}
