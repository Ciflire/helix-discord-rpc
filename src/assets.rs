use std::{error::Error, path::Path};

/// This function return the corresponding asset id which is the name of the file in the discord developper portal
/// We match first on the filename if it is known it is returned, else we match on the extension.
/// If the extension is unknown then it falls back to helix icon
/// Note that you are not able to know in advance the asset id, only @ciflire can, so if you have any remark please open an issue
pub fn get_asset(filename: String) -> String {
    let extension = Path::new(&filename).extension().unwrap().to_str().unwrap();
    let res = match filename.as_str() {
        "Cargo.toml" => "cargo".to_string(),
        "Cargo.lock" => "cargo".to_string(),
        "hyprland.conf" => "hyprland".to_string(),
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

        _ => "helix".to_string(),
    }
}
