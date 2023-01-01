use clap::{arg, AppSettings, Arg};

pub struct Arguments {
    pub database: String,
    pub targets: Vec<String>,
}

impl Arguments {
    pub fn new() -> Self {
        let app = clap::app_from_crate!()
            .arg(arg!(--database <database> "database file"))
            .arg(Arg::new("target").multiple_values(true))
            .setting(AppSettings::DeriveDisplayOrder)
            .get_matches();

        let database = match app.value_of("database") {
            Some(f) => f.to_string(),
            None => panic!("please specify a database file."),
        };

        let targets = app
            .values_of("target")
            .map(|args| args.map(|s| s.to_string()).collect::<Vec<String>>())
            .expect("please specify a target files");

        Arguments { database, targets }
    }
}
