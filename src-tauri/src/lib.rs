// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn get_sound_lib() -> Vec<Sample> {
//     let lib = vec![Sample {
//         id: "".to_string(),
//         filepath: "".to_string(),
//         category: Category::Boom,
//         filename: 1,
//     }];
//     lib
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // .invoke_handler(tauri::generate_handler![get_sound_lib])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
