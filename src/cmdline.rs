use clap::{arg, AppSettings};

pub struct Arguments {
    pub database: String,
    pub target: String,
}

impl Arguments {
    pub fn new() -> Self {
        let app = clap::app_from_crate!()
            .arg(arg!(--database "database file"))
            .arg(arg!(<target> "target file"))
            .setting(AppSettings::DeriveDisplayOrder)
            .get_matches();

        let database = match app.value_of("database") {
            Some(f) => f.to_string(),
            None => panic!("please specify a database file."),
        };

        let target = match app.value_of("target") {
            Some(f) => f.to_string(),
            None => panic!("please specify a target file."),
        };

        Arguments { database, target }
    }
}
