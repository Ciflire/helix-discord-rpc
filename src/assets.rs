use std::{error::Error, path::Path};

pub fn get_asset(filename: String) -> String {
    let extension = Path::new(&filename).extension().unwrap().to_str().unwrap();
    let res = match filename.as_str() {
        "Cargo.toml" => "cargo".to_string(),
        "Cargo.lock" => "cargo".to_string(),
        _ => String::new(),
    };
    if !res.is_empty() {
        return res;
    }
    match extension {
        "agda" => "agda".to_string(),
        "ahk" => "ahk".to_string(),
        "lagda" => "agda".to_string(),
        "md" => "markdown".to_string(),
        "prop" => "android".to_string(),
        "rs" => "rust".to_string(),

        "scm" => "lisp".to_string(),

        _ => "python".to_string(),
    }
}
