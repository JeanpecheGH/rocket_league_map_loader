use serde_json::Value;
use std::fs;

#[derive(Debug, Default)]
pub struct Map {
    pub name: String,
    pub author: Option<String>,
    pub path: String,
}

pub fn get_maps(path: &str) -> Option<Vec<Map>> {
    let mut maps = vec![];

    for entry in fs::read_dir(path).ok()? {
        let dir_path = entry.ok()?.path();
        let dir_name = dir_path.to_str()?;
        let md = fs::metadata(dir_name).ok()?;
        if md.is_dir() {
            //https://rust-unofficial.github.io/patterns/idioms/option-iter.html
            //Extend can iterate over an option
            maps.extend(get_map(dir_name));
        }
    }
    Some(maps)
}

fn get_map(path: &str) -> Option<Map> {
    let mut opt_author: Option<String> = None;
    let mut opt_path: Option<String> = None;

    //Get map name from folder name
    let split = path.split('\\');
    let opt_name: Option<String> = split.last().map(|s| s.to_string());

    for entry in fs::read_dir(path).ok()? {
        let file_path = entry.ok()?.path();
        let file_name = file_path.to_str()?;
        let md = fs::metadata(file_name).ok()?;
        if md.is_file() {
            match file_name {
                f if f.ends_with(".udk") || f.ends_with(".upk") => {
                    opt_path = Some(file_name.to_string())
                }
                f if f.ends_with(".vdf") => (), //Do nothing for now
                f if f.ends_with(".json") => opt_author = parse_json(f), //Get map author from json
                _ => (),
            }
        }
    }
    opt_path.map(|p| Map {
        name: opt_name.unwrap_or_default(),
        author: opt_author,
        path: p,
    })
}

fn parse_json(path: &str) -> Option<String> {
    let data = fs::read_to_string(path).ok()?;
    let v: Value = serde_json::from_str(&data).ok()?;
    // Access parts of the data by indexing with square brackets.
    let author = v["author"].as_str().map(String::from);
    author
}
