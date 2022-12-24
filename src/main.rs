use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use memmap2::Mmap;
use regex::Regex;

fn main() {
    let mut db_path = home::home_dir().unwrap();
    db_path.push("tmp/ClamavDatabase/main.ndb");
    let database = File::open(db_path.as_path()).unwrap();
    let reader = BufReader::new(database);

    let mut signeture: HashMap<String, String> = HashMap::new();
    let re = Regex::new(r"[*?{|]").unwrap();

    for line in reader.lines() {
        let properties: Vec<String> = line.unwrap().split(':').map(|x| x.to_string()).collect();
        if !re.is_match(&properties[3]) {
            signeture.insert(properties[0].clone(), properties[3].clone());
        }
    }

    for sig in signeture.iter() {
        println!("{}, {}", sig.0, sig.1);
    }

    let target_path = Path::new("./eicar_example");
    let target_file = File::open(target_path).unwrap();
    let target = unsafe { Mmap::map(&target_file).unwrap() };

    // let target_pattern: String = target.iter().map(|x| format!("{:x}", x)).collect();
    let mut target_pattern = String::new();
    for b in target.iter() {
        target_pattern.push_str(&format!("{:x}", b));
    }
    println!("pattern: {}", target_pattern);
}
