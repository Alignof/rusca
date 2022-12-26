mod cmdline;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use cmdline::Arguments;
use memmap2::Mmap;
use regex::{Regex, RegexSetBuilder};

fn get_signeture<R>(reader: BufReader<R>) -> HashMap<String, String>
where
    R: std::io::Read,
{
    let mut signeture: HashMap<String, String> = HashMap::new();
    let re = Regex::new(r"[^0-9a-f]").unwrap();
    for line in reader.lines() {
        let properties: Vec<String> = line.unwrap().split(':').map(|x| x.to_string()).collect();
        if !re.is_match(&properties[3]) {
            signeture.insert(properties[0].clone(), properties[3].clone());
        }
    }

    signeture
}

fn get_target_pattern(target_path: &str) -> String {
    let target_file = File::open(target_path).unwrap();
    let target = unsafe { Mmap::map(&target_file).unwrap() };
    let target_pattern: String = target.iter().map(|x| format!("{:x}", x)).collect();
    println!("pattern: {}", target_pattern);

    target_pattern
}

fn main() {
    let args = Arguments::new();
    let database = File::open(args.database).unwrap();
    let reader = BufReader::new(database);
    let signeture = get_signeture(reader);

    let target_pattern = get_target_pattern(&args.target);

    let re_sig = RegexSetBuilder::new(signeture.values())
        .unicode(false)
        .size_limit(std::usize::MAX)
        .build()
        .unwrap();

    let matched: Vec<_> = re_sig
        .matches(&target_pattern)
        .into_iter()
        .map(|index| &re_sig.patterns()[index])
        .collect();

    for m in matched {
        println!(
            "{} found.",
            signeture
                .iter()
                .find_map(|(k, v)| if v == m { Some(k) } else { None })
                .unwrap_or(&String::new())
        );
    }
}
