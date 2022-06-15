use serde_json::Value;
use std::fs;

#[derive(Debug)]
pub struct Map {
    pub name: String,
    pub author: Option<String>,
    pub path: String
}

impl Default for Map {
    fn default() -> Self {
        Self {
            name: String::from(""),
            author: None,
            path: String::from("")
        }
    }
}

pub fn get_maps(path: &str) -> Vec<Map> {
    let paths = fs::read_dir(path).unwrap();
    let mut maps = vec![];

    for entry in paths {
        let dir_path = entry.unwrap().path();
        let dir_name = dir_path.to_str().unwrap();
        let md = fs::metadata(dir_name).unwrap();
        if md.is_dir() {
            maps.push(get_map(dir_name).unwrap());
        }
    }
    maps
}

fn get_map(path: &str) -> Option<Map> {
    let mut opt_author: Option<String> = None;
    let mut opt_path: Option<String> = None;

    //Get map name from folder name
    let split = path.split("\\");
    let opt_name: Option<String> = split.last().map(|s| s.to_string());

    for entry in fs::read_dir(path).ok()? {
        let file_path = entry.unwrap().path();
        let file_name = file_path.to_str().unwrap();
        let md = fs::metadata(file_name).unwrap();
        if md.is_file() {
            match file_name {
                f if f.ends_with(".udk") => opt_path = Some(file_name.to_string()),
                f if f.ends_with(".vdf") => (),//Do nothing for now
                f if f.ends_with(".json") => {
                    opt_author = parse_json(f);
                    ()
                },//Get map name and author from json
                _ => ()
            }
        }
    }
    let opt_map = opt_path.map(|p| Map { name: opt_name.unwrap_or("".to_string()), author: opt_author, path: p});
    opt_map
}

fn parse_json(path: &str) -> Option<String> {
    let file = fs::read_to_string(path);
    let data = file.ok()?;
    let v: Value = serde_json::from_str(&data).ok()?;
    // Access parts of the data by indexing with square brackets.
    let author = v["author"].as_str().map(|s| String::from(s));
    author
}