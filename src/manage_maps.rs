const GAME_MAP_FOLDER: &str = "\\TAGame\\CookedPCConsole";
const ORIGINAL_FILE: &str = "Labs_Underpass_P.upk";
const BACKUP_FILE: &str = "BACKUP_FILE.upk";

use std::fs;
use std::io;
use std::path::Path;
use zip;

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

pub fn original_file_exists(game_folder: &str) -> bool {
    let file_path = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, ORIGINAL_FILE);
    Path::new(&file_path).is_file()
}

fn backup_file_exists(game_folder: &str) -> bool {
    let file_path = format!("{}{}\\{}", game_folder, GAME_MAP_FOLDER, BACKUP_FILE);
    Path::new(&file_path).is_file()
}

pub fn unzip(zip_path: &Path, custom_folder: &str) -> Result<(), io::Error> {
    //Example from zip-rs : https://github.com/zip-rs/zip/blob/master/examples/extract.rs
    let file = fs::File::open(zip_path)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let dirs = path.parent().and_then(|d| d.as_os_str().to_str());
                let sub_dir = match dirs {
                    Some(s) if s.len() == 0 => {
                        let no_ext_path = zip_path.with_extension("");
                        let short_path = no_ext_path.file_name();
                        short_path.and_then(|f| f.to_str()).unwrap_or("").to_owned()
                    }
                    Some(s) => String::from(s),
                    None => String::from(""),
                };
                Path::new(custom_folder)
                    .join(sub_dir)
                    .join(path.file_name().unwrap())
            }
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
