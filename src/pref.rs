extern crate preferences;
use preferences::{AppInfo, PreferencesMap, Preferences};

const APP_INFO: AppInfo = AppInfo{name: "rocket league map loader", author: "Jeanpeche"};
const PREFS_KEY: &str = "conf";

pub struct Pref {
    custom_path: String,
    game_path: String
}

pub fn load_pref() -> Pref {
    let mut load_pref = PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY);
    if load_pref.is_err() {
        println!("Unable to load existing preferences, initialize preferences");
        save_pref(&Pref::default());
        load_pref = PreferencesMap::<String>::load(&APP_INFO, PREFS_KEY);
    }
    let pref_map = load_pref.unwrap();
    Pref {
        custom_path:  pref_map.get("custom_path").map(|s| &s[..]).unwrap_or("").to_string(),
        game_path: pref_map.get("game_path").map(|s| &s[..]).unwrap_or("").to_string()
    }
}

pub fn save_pref(prefs: &Pref) -> () {
    let mut pref_map: PreferencesMap<String> = PreferencesMap::new();
    pref_map.insert("custom_path".into(), prefs.custom_path.clone());
    pref_map.insert("game_path".into(), prefs.game_path.clone());
    pref_map.save(&APP_INFO, PREFS_KEY).expect("Could not store preferences");
}

impl Default for Pref {
    fn default() -> Self {
        Self {
            custom_path: String::from(""),
            game_path: String::from("")
        }
    }
}

impl Pref {
    pub fn with_paths(custom: String, game: String) -> Pref {
        Self {
            custom_path: custom,
            game_path: game
        }
    }
    pub fn custom_path(&self) -> &str {
        return &self.custom_path;
    }
    pub fn game_path(&self) -> &str {
        return &self.game_path;
    }
}

