mod cmdline;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use cmdline::Arguments;
use memmap2::Mmap;
use regex::{Regex, RegexSet, RegexSetBuilder};

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

fn matching(re_sig: &RegexSet, target_pattern: String) -> Vec<&String> {
    re_sig
        .matches(&target_pattern)
        .into_iter()
        .map(|index| &re_sig.patterns()[index])
        .collect()
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

    let matched = matching(&re_sig, target_pattern);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_regex(signetures: Vec<&str>) -> RegexSet {
        RegexSetBuilder::new(signetures)
            .unicode(false)
            .size_limit(std::usize::MAX)
            .build()
            .unwrap()
    }

    #[test]
    fn regex_match_test() {
        let signetures: Vec<&str> = vec!["abc", "efg", "abcdef", "test", "pattern"];
        let re_sig = prepare_regex(signetures);
        let target_pattern = "abcdefg".to_string();
        let matched = matching(&re_sig, target_pattern);

        assert_eq!(matched.len(), 3);
        assert_eq!(matched[0], "abc");
        assert_eq!(matched[1], "efg");
        assert_eq!(matched[2], "abcdef");
        assert_ne!(matched[2], "test");
    }

    #[test]
    fn sig2hash_test() {
        let signeture = "Legacy.Trojan.Agent-1:0:*:dd6d70241f674d8fc13e1eb3af731a7b5c43173c1cdd75722fa556c373b65c5275d513147b070077757064080386898ae75c6fb7f717b562ef636f6d6d613f2e0e202f6336c5eed52064f120228e2f6d27c101\n\
            Win.Trojan.Hotkey-1:0:*:c01640006a3cffb684000000ff159cef420089869800000089be940000008bc75f5ec20400565733ff8bf1397c240c741fff762089be8c000000ff1560ef42\n\
            Doc.Trojan.Nori-1:0:*:6d706f6e656e74732e4974656d28556e292e436f64654d6f64756c652e4c696e657328322c203129203c3e20222749726f6e22205468656e\n\
            Doc.Trojan.Layla-1:0:*:6572436f707920536f757263653a3d4b544f2c2044657374696e6174696f6e3a3d4b4f474f2c204e616d653a3d224d6143524f534f4654222c204f626a6563743a3d77644f7267616e697a\n\
            Win.Worm.Gaobot-1:0:*:3467072092830ddc2d8a88a47d904500811b760af9089389402573087f2bb8fc4e49434b200d0abf626f742e7365ffdbfff263757265656c657465207368610d73202f206469736162dbdf7b6b1207636f6d3b2b666c751f64dcd6c5de6e730f0b297404201bfb597bd70817206361630e2b7175697427ff60c9de0725476c6f";
        let signeture_db = BufReader::new(signeture.as_bytes());

        let signetures = get_signeture(signeture_db);
        dbg!(&signetures);
        assert!(signetures.contains_key("Legacy.Trojan.Agent-1"));
        assert_eq!(
            signetures.get("Win.Trojan.Hotkey-1"),
            Some(&"c01640006a3cffb684000000ff159cef420089869800000089be940000008bc75f5ec20400565733ff8bf1397c240c741fff762089be8c000000ff1560ef42".to_string())
        );
        assert_eq!(
            signetures.get("Doc.Trojan.Layla-1"),
            Some(&"6572436f707920536f757263653a3d4b544f2c2044657374696e6174696f6e3a3d4b4f474f2c204e616d653a3d224d6143524f534f4654222c204f626a6563743a3d77644f7267616e697a".to_string())
        );
        assert_ne!(signetures.get("Doc.Trojan.Layla-1"), Some(&"0".to_string()));
        assert_eq!(signetures.get("test"), None);
    }
}
