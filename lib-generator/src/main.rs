use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub const SOUND_LIB_PATH: &str = "/public/sounds/";
pub const SOUND_LIB_JSON_NAME: &str = "/lib.json";

fn main() {
    let mut lib_hash: HashMap<Category, Vec<Sample>> = HashMap::new();

    if let Ok(abs_path_to_sounds_dir) = fs::canonicalize(PathBuf::from("../public/sounds/")) {
        match fs::read_dir(&abs_path_to_sounds_dir) {
            Ok(dir) => {
                for file_path in dir.flatten() {
                    if let Ok(file_type) = file_path.file_type() {
                        if file_type.is_dir() {
                            let dir_name_os_str = file_path.file_name();
                            let dir_name = dir_name_os_str.to_str().unwrap();
                            let current_category = get_category_by_str(dir_name);
                            lib_hash.insert(current_category, vec![]);

                            match fs::read_dir(file_path.path()) {
                                Ok(sample_dir_path) => {
                                    for sample_dir_entry in sample_dir_path.flatten() {
                                        let sample_path = sample_dir_entry.path();
                                        let filename =
                                            sample_path.file_stem().unwrap().to_str().unwrap();

                                        let sample = Sample {
                                            id: format!("{dir_name}_{filename}"),
                                            category: current_category,
                                            duration: mp3_duration::from_path(
                                                sample_dir_entry.path(),
                                            )
                                            .unwrap()
                                            .as_secs_f32(),
                                            filename: filename.to_string(),
                                            filepath: format!(
                                                "{SOUND_LIB_PATH}{dir_name}/{filename}.mp3"
                                            ),
                                        };

                                        lib_hash.get_mut(&current_category).unwrap().push(sample);
                                    }
                                }
                                Err(err) => println!("{:?}", err),
                            }
                        }
                    }
                }

                let mut lib_file_path = abs_path_to_sounds_dir.into_os_string();
                lib_file_path.push(SOUND_LIB_JSON_NAME);

                match fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&lib_file_path)
                {
                    Ok(file) => {
                        let mut writer = BufWriter::new(file);
                        serde_json::to_writer(&mut writer, &lib_hash).unwrap();
                        writer.flush().unwrap();
                    }
                    Err(err) => println!("{:?}", err),
                }
            }
            Err(err) => println!("Error reading dir {:?}", err),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Category {
    Boom,
    Doors,
    Construction,
    Eerie,
    People,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub id: String,
    pub filename: String,
    pub filepath: String,
    pub category: Category,
    pub duration: f32,
}

fn get_category_by_str(s: &str) -> Category {
    match s {
        "boom" => Category::Boom,
        "doors" => Category::Doors,
        "construction" => Category::Construction,
        "eerie" => Category::Eerie,
        "people" => Category::People,
        _ => Category::Boom,
    }
}
