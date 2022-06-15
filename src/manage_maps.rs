const GAME_MAP_FOLDER: &str = "\\TAGame\\CookedPCConsole";
const ORIGINAL_FILE: &str = "Labs_Underpass_P.upk";
const BACKUP_FILE: &str = "BACKUP_FILE.upk";


use std::fs;
use std::io;
use std::path::Path;

pub fn load_custom_file(game_folder: &str, custom_map: &str) -> Result<(), io::Error> {
    //If there is no backup file already, this means the current file should be the original one
    if !backup_file_exists(game_folder) {
        save_original_file(game_folder)?;
    }
    let to = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, ORIGINAL_FILE);
    copy_file(custom_map, &to)?;
    Ok(())
}

fn save_original_file(game_folder: &str) -> Result<(), io::Error> {
    let from = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, ORIGINAL_FILE);
    let to = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, BACKUP_FILE);
    copy_file(&from, &to)?;
    Ok(())
}

pub fn restore_original_file(game_folder: &str) -> Result<(), io::Error> {
    let from = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, BACKUP_FILE);
    let to = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, ORIGINAL_FILE);
    copy_file(&from, &to)?;
    fs::remove_file(&from)?;
    Ok(())
}

fn copy_file(from: &str, to: &str) -> Result<(), io::Error> {
    let r = fs::copy(&from, &to);
    match r {
        Ok(_) => return Ok(()),
        Err(e) => {
            eprintln!("Error copying {} to {} : {}", from, to, e);
            let msg = format!("{}: {}", e.to_string(), from);
            let custom_error = io::Error::new(e.kind(), msg);
            return Err(custom_error);
        }
    }
}

fn original_file_exists(game_folder: &str) -> bool {
    let file_path = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, ORIGINAL_FILE);
    Path::new(&file_path).is_file()
}

fn backup_file_exists(game_folder: &str) -> bool {
    let file_path = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, BACKUP_FILE);
    Path::new(&file_path).is_file()
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).is_file()
}