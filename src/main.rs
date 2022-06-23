#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod manage_maps;
mod maps;
mod pref;
mod dialog;

use eframe::egui;
use egui::{Id, Vec2};
use egui_extras::{TableBody, TableBuilder, Size};
use rfd::FileDialog;

use crate::maps::Map;
use crate::pref::Pref;
use crate::dialog::Dialog;
use crate::manage_maps::unzip;

const WIDTH: f32 = 700.0;
const HEIGHT: f32 = 500.0;
const VEC2_SIZE: Vec2 = Vec2 { x: WIDTH, y: HEIGHT};
const TITLE_SUCCESS: &str = "✅ Success";
const TITLE_ERROR: &str = "⚠ Error";

fn main() {
    let r = pref::load_pref();
    let prefs = match r {
        Ok(pref) => pref,
        Err(e) => {
            eprintln!("Error loading preferences : {}", e);
            Pref::default()
        }
    };
    let app = MapLoaderApp::with_pref(prefs);

    //Include the icon directly into the binary
    let icon_bytes = include_bytes!("..\\media\\icon-128.png");
    let icon = image::load_from_memory(icon_bytes).expect("Failed to load icon data").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        resizable: true,
        min_window_size: Some(VEC2_SIZE),
        initial_window_size: Some(VEC2_SIZE),
        icon_data: Some(eframe::IconData {
            rgba: icon.into_raw(),
            width: icon_width,
            height: icon_height,
        }),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Rocket League Map Loader",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

struct MapLoaderApp {
    pref: Pref,
    search: String,
    maps: Vec<Map>,
    dialog: Dialog
}

impl Default for MapLoaderApp {
    fn default() -> Self {
        Self {
            pref: Pref::default(),
            search: String::from(""),
            maps: vec![],
            dialog: Dialog::default()
        }
    }
}

impl MapLoaderApp {
    fn render_dialog(&mut self, ctx: &egui::Context) {
        let window = egui::Window::new(&self.dialog.title)
            .id(Id::new(45i32))
            .resizable(false)
            .collapsible(false)
            .auto_sized()
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0 ,0.0))
            .open(&mut self.dialog.show);

        window.show(ctx, |ui| {
            ui.label(&self.dialog.msg);
        });
    }

    fn get_file_dialog() -> FileDialog {
        let path = std::env::current_dir().ok()
            .and_then(|d| d.to_str().map(|s| s.to_string()));
        let dialog = match path {
            Some(p) => FileDialog::new().set_directory(&p),
            None => FileDialog::new()
        };
        dialog
    }

    fn pick_custom_folder(&mut self) {
        if let Some(path) = Self::get_file_dialog().pick_folder() {
            let custom_path = path.display().to_string();
            println!("Custom map folder replace {} with {}", self.pref.custom_path, custom_path);
            self.pref.custom_path = custom_path;
            let r= pref::save_pref(&self.pref);
            match r {
                Ok(()) => println!("Custom folder set to {}", &self.pref.custom_path ),
                Err(e) => {
                    self.dialog.title = String::from(TITLE_ERROR);
                    self.dialog.msg = e.to_string();
                    self.dialog.show = true;
                }
            }
        }
    }

    fn pick_game_folder(&mut self) {
        if let Some(path) = Self::get_file_dialog().pick_folder() {
            let game_path = path.display().to_string();
            println!("Game folder replace {} with {}", self.pref.game_path, game_path);
            self.pref.game_path = game_path;
            let r= pref::save_pref(&self.pref);
            match r {
                Ok(()) => println!("Game folder set to {}", &self.pref.game_path),
                Err(e) => {
                    self.dialog.title = String::from(TITLE_ERROR);
                    self.dialog.msg = e.to_string();
                    self.dialog.show = true;
                }
            }
        }
    }

    fn import_new_map(&mut self) {
        let extensions: Vec<&str> = vec!("zip");
        if let Some(path) = Self::get_file_dialog().add_filter("Zip files", &extensions).pick_file() {
            let r = unzip(path.as_path(), &self.pref.custom_path);
            let zip_path = path.file_name().and_then(|s| s.to_str()).unwrap_or("UNDEFINED");
            match r {
                Ok(()) => {
                    self.dialog.title = String::from(TITLE_SUCCESS);
                    let msg = format!("File \"{}\" successfully imported", zip_path);
                    self.dialog.msg = String::from(msg);
                }
                Err(e) => {
                    self.dialog.title = String::from(TITLE_ERROR);
                    self.dialog.msg = e.to_string();
                }
            }
            self.dialog.show = true;
        }
    }

    fn nested_menus(&mut self, ui: &mut egui::Ui) {
        if ui.button("Set custom maps folder").clicked() {
            self.pick_custom_folder();
            ui.close_menu();
        }
        if ui.button("Set game folder").clicked() {
            self.pick_game_folder();
            ui.close_menu();
        }
        if ui.button("Import new map").clicked() {
            self.import_new_map();
            ui.close_menu();
        }
    }

    fn pick_theme(&mut self, ui: &mut egui::Ui) {
        let style = (*ui.ctx().style()).clone();
        match style.visuals.dark_mode {
            true => {
                if ui.add(egui::Button::new("☀").frame(false))
                    .on_hover_text("Switch to light mode")
                    .clicked()
                {
                    ui.ctx().set_visuals(egui::Visuals::light());
                    self.pref.dark_mode = false;
                    pref::save_pref(&self.pref).ok();
                }
            }
            false => {
                if ui.add(egui::Button::new("🌙").frame(false))
                    .on_hover_text("Switch to dark mode")
                    .clicked()
                {
                    ui.ctx().set_visuals(egui::Visuals::dark());
                    self.pref.dark_mode = true;
                    pref::save_pref(&self.pref).ok();
                }
            }
        }
    }

    fn populate_table(&mut self, body: &mut TableBody) {
        let maps = &self.maps;
        let search = &self.search;
        let last_loaded_map = self.pref.last_loaded_map.clone();
        body.row(5.0, |mut row|{
            row.col(|ui| {
                ui.separator();
            });
            row.col(|ui| {
                ui.separator();
            });
            row.col(|ui| {
                ui.separator();
            });
        });
        for m in maps.iter() {
            let lower_search = search.to_lowercase();
            let author: &str = m.author.as_deref().unwrap_or("Unknown");
            if lower_search.is_empty()
                || m.name.to_lowercase().contains(lower_search.as_str())
                || author.to_lowercase().contains(lower_search.as_str()) {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.horizontal_centered(|ui| {
                            ui.label(&m.name);
                        });
                    });
                    row.col(|ui| {
                        ui.horizontal_centered(|ui| {
                            ui.label(author);
                        });
                    });
                    row.col(|ui| {
                        ui.horizontal_centered(|ui| {
                            if m.name.eq(last_loaded_map.as_str()) {
                                ui.label(egui::RichText::new("LOADED").strong());
                            } else {
                                if ui.button("LOAD").clicked() {
                                    let r = manage_maps::load_custom_file(&self.pref.game_path, &m.path);
                                    match r {
                                        Ok(()) => {
                                            self.dialog.title = String::from(TITLE_SUCCESS);
                                            let msg = format!("\"{}\" successfully loaded", m.name);
                                            self.dialog.msg = String::from(msg);
                                            self.pref.last_loaded_map = m.name.clone();
                                            pref::save_pref(&self.pref).ok();
                                        }
                                        Err(e) => {
                                            self.dialog.title = String::from(TITLE_ERROR);
                                            self.dialog.msg = e.to_string();
                                        }
                                    }
                                    self.dialog.show = true;
                                };
                            }
                        });
                    });
                });
            };
        }
    }

    fn with_pref(pref: Pref) -> Self {
        Self {
            pref: pref,
            search: String::from(""),
            maps: Vec::new(),
            dialog: Dialog::default()
        }
    }
}


impl eframe::App for MapLoaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //Dialog test
        self.render_dialog(ctx);

        //Set theme based on preferences
        let pref_dark_mode = self.pref.dark_mode;
        match pref_dark_mode {
            true => ctx.set_visuals(egui::Visuals::dark()),
            false => ctx.set_visuals(egui::Visuals::light())
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            //When showing a dialog, the whole UI underneath is disabled
            ui.set_enabled(!self.dialog.show);

            if self.pref.custom_path.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.label("");
                        ui.label("The custom maps path is not defined");
                        ui.label("");
                        ui.label("Please select the folder where your custom maps are stored");
                        ui.label("");
                        if ui.button("Select custom maps folder").clicked() {
                            Self::pick_custom_folder(self);
                        }
                    });
            }
            else if self.pref.game_path.is_empty() || !manage_maps::original_file_exists(&self.pref.game_path) {
                ui.vertical_centered(|ui| {
                    ui.label("");
                    ui.label("The game path is not defined or is not set to the game folder");
                    ui.label("");
                    ui.label("Please select the folder where your game in installed");
                    ui.label("");
                    ui.label("(Should be something like \"Epic Games/rocketleague\")");
                    ui.label("");
                    if ui.button("Select game folder").clicked() {
                        Self::pick_game_folder(self);
                    }
                });
            } else {
                //Only load maps now that we have set the custom path
                let folder_maps = maps::get_maps(&self.pref.custom_path).unwrap_or(vec![]);
                self.maps = folder_maps;

                ////////////////////
                //// Main Panel ////
                ////////////////////
                ui.horizontal(|ui| {
                    ui.menu_button("☰", |ui| Self::nested_menus(self, ui));
                    Self::pick_theme(self, ui);
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.add_sized(Vec2::new(100.0, ui.available_height()), egui::TextEdit::singleline(&mut self.search));
                        ui.label("Search: ");
                        if !self.pref.last_loaded_map.eq("") {
                            if ui.button("Restore original map").clicked() {
                                let r = manage_maps::restore_original_file(&self.pref.game_path);
                                match r {
                                    Ok(()) => {
                                        self.dialog.title = String::from(TITLE_SUCCESS);
                                        self.dialog.msg = String::from("Original map sucessfully restored");
                                        self.pref.last_loaded_map = String::from("");
                                        pref::save_pref(&self.pref).ok();
                                    }
                                    Err(e) => {
                                        self.dialog.title = String::from(TITLE_ERROR);
                                        self.dialog.msg = e.to_string();
                                    }
                                }
                                self.dialog.show = true;
                            }
                        }
                    });
                });
            ui.separator();
            TableBuilder::new(ui)
                .striped(true)
                .column(Size::remainder().at_least(100.0))
                .column(Size::relative(0.35).at_least(100.0))
                .column(Size::exact(60.0))
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Title");
                    });
                    header.col(|ui| {
                        ui.heading("Author");
                    });
                    header.col(|ui| {
                        ui.heading("");
                    });
                })
                .body(|mut body| {
                    self.populate_table(&mut body);
                });
            }
        });
    }
}

