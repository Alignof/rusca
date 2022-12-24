use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let mut filename = home::home_dir().unwrap();
    filename.push("tmp/ClamavDatabase/main.ndb");
    let file = File::open(filename.as_path()).unwrap();
    let reader = BufReader::new(file);
    let mut signeture: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let properties: Vec<String> = line.unwrap().split(':').map(|x| x.to_string()).collect();
        signeture.insert(properties[0].clone(), properties[3].clone());
    }

    for sig in signeture.iter() {
        println!("{}, {}", sig.0, sig.1);
    }
}
