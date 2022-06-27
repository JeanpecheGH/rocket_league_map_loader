extern crate preferences;
use preferences::{AppInfo, Preferences, PreferencesMap};

const APP_INFO: AppInfo = AppInfo {
    name: "rocket league map loader",
    author: "Jeanpeche",
};
const PREFS_KEY: &str = "conf";

pub struct Pref {
    pub custom_path: String,
    pub game_path: String,
    pub last_loaded_map: String,
    pub dark_mode: bool,
}

pub fn load_pref() -> Result<Pref, preferences::PreferencesError> {
    let pref_map = match PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY) {
        Ok(pref_map) => pref_map,
        Err(e) => {
            println!(
                "Unable to load existing preferences, initialize preferences : {}",
                e
            );
            save_pref(&Pref::default())?;
            PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY)?
        }
    };
    Ok(Pref {
        custom_path: pref_map
            .get("custom_path")
            .map_or("", |s| &s[..])
            .to_string(),
        game_path: pref_map
            .get("game_path")
            .map_or("", |s| &s[..])
            .to_string(),
        last_loaded_map: pref_map
            .get("last_loaded_map")
            .map_or("", |s| &s[..])
            .to_string(),
        dark_mode: pref_map
            .get("dark_mode")
            .map_or(false, |s| match s.as_str() {
                "true" => true,
                "false" => false,
                _ => false,
            }),
    })
}

pub fn save_pref(prefs: &Pref) -> Result<(), preferences::PreferencesError> {
    let mut pref_map: PreferencesMap<String> = PreferencesMap::new();
    pref_map.insert("custom_path".into(), prefs.custom_path.clone());
    pref_map.insert("game_path".into(), prefs.game_path.clone());
    pref_map.insert("last_loaded_map".into(), prefs.last_loaded_map.clone());
    pref_map.insert("dark_mode".into(), prefs.dark_mode.to_string());
    pref_map.save(&APP_INFO, PREFS_KEY)?;
    Ok(())
}

impl Default for Pref {
    fn default() -> Self {
        Self {
            custom_path: String::from(""),
            game_path: String::from(""),
            last_loaded_map: String::from(""),
            dark_mode: false,
        }
    }
}
