use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Category {
    Boom,
    Doors,
    Construction,
    Eerie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub id: String,
    pub filename: String,
    pub filepath: String,
    pub category: Category,
    pub duration: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Preset {
    volume: f32,
    duration: u64,
    random: bool,
    grid_data: Vec<Option<Sample>>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save_session() {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![save_session])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
