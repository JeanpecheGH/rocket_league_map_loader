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

pub fn unzip(zip_path: &Path, custom_folder: &str) {
    //Example from zip-rs : https://github.com/zip-rs/zip/blob/master/examples/extract.rs
    let file = fs::File::open(zip_path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let dirs = path.parent().unwrap();
                let dir_str = dirs.as_os_str().to_str().unwrap();
                let sub_dir = match dir_str.len() {
                    0 => {
                        let no_ext_path = zip_path.with_extension("");
                        let short_path = no_ext_path.file_name();
                        short_path.map(|f| f.to_str().unwrap()).unwrap().to_owned()
                    },
                    _ => String::from(dir_str)
                };
                Path::new(custom_folder).join(sub_dir).join(path.file_name().unwrap())
            },
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}

