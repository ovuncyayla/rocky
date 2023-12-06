use std::{path::PathBuf, str::FromStr};

fn main() {

    let cfg = jg::Config::from(PathBuf::from_str("config.yaml").unwrap());

    let asd = jg::generate_json(&cfg);

    println!("{:?}", asd);
}
