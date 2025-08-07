use std::{error::Error, path::Path};

/// This function return the corresponding asset id which is the name of the file in the discord developper portal
/// We match first on the filename if it is known it is returned, else we match on the extension.
/// If the extension is unknown then it falls back to helix icon
/// Note that you are not able to know in advance the asset id, only @ciflire can, so if you have any remark please open an issue
pub fn get_asset(filename: String) -> String {
    let extension = Path::new(&filename).extension().unwrap().to_str().unwrap();
    let res = match filename.as_str() {
        "Cargo.toml" | "Cargo.lock" => "cargo".to_string(),
        "hyprland.conf" | "hyprlock.conf" | "hyprpaper.conf" | "hypridle.conf" => {
            "hyprland".to_string()
        }
        "CMakeLists.txt" => "cmake".to_string(),
        ".gitignore" | ".gitsubmodules" => "git".to_string(),
        _ => String::new(),
    };
    if !res.is_empty() {
        return res;
    }
    match extension {
        "c" | "h" => "c".to_string(),
        "cpy" | "copy" | "cbl" | "cob" | "cobol" => "cobol".to_string(),
        "cpp" | "hpp" => "cpp".to_string(),
        "cs" => "csharp".to_string(),
        "css" | "scss" => "css".to_string(),
        "gleam" => "gleam".to_string(),
        "glsl" => "glsl".to_string(),
        "gd" => "godot".to_string(),
        "go" => "go".to_string(),
        "hlsl" => "hlsl".to_string(),
        "html" => "html".to_string(),
        "java" | "class" => "java".to_string(),
        "js" => "js".to_string(),
        "json" => "json".to_string(),
        "lua" => "lua".to_string(),
        "md" => "markdown".to_string(),
        "nix" => "nix".to_string(),
        "py" => "python".to_string(),
        "rs" => "rust".to_string(),
        "sh" | "nu" | "bat" => "shell".to_string(),
        "scm" | "ss" => "scheme".to_string(),
        "swift" => "swift".to_string(),
        "tex" => "tex".to_string(),
        "toml" => "toml".to_string(),
        "ts" => "ts".to_string(),
        "typ" => "typst".to_string(),
        "xaml" => "xaml".to_string(),
        "xml" => "xml".to_string(),
        "yaml" => "yaml".to_string(),
        "zig" => "zig".to_string(),
        _ => "helix".to_string(),
    }
}
