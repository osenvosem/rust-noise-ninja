use core::{fmt, str};
use js_sys::Promise;
use leptos::{ev, html::audio, wasm_bindgen::JsCast};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

pub const EMPTY_SOUND: &str = "data:audio/mp3;base64,/+MYxAALM2H8CABNSR8lJeb1//ZLXL3GxwA////Gfvrf/xEGRl3ZCQcBk9PXDQQnx2IAMLJ3sQYgITEFNRTMuOTguMgAAAAA/+MYxAAKu2IcCABNSR/B/IvkRlkrv84Pyb////7n8/c/lr/fJKmETd2jQJys1AEVDNFHQFpMdayYgpqKZlxycFxkAAAAAAAA/+MYxAAKe2YcEACTTAGpb/f/3S3S6O/7W/3///39ay7IZqJYLGUjmMC5G1Nc8iFSopqzJM8YMJiCmopmXHJwXGQAAAAAAAAA/+MYxAAKC2YgEACTTAG+3/9/X3/Rfomn+///qv0N6Is4txSqUMFVEcRLCVlx3prPbiPEc32mIKaimZccnBcZAAAAAAAAAAAA/+MYxAAKg2IgEABNSQJ/58/POVS7+9r/////fjP+339sTMyB1mTB2z2dDass4zE3BUU4J2U650xBTUUzLjk4LjIAAAAAAAAA/+MYxAAJU2okEACNTAG/9++v/X/P9f////+uyp8Il6TGMRimB1ZjTsLkOWmwSBskEEJTEFNRTMuOTguMgAAAAAAAAAAAAAAA/+MYxAAJ+2YcCACNTA//1P9Nb9LmrJ3XWv6dP///TyMay3cjqYhoIWpExQM8ynALZwUHc9aYgpqKZlxycFxkAAAAAAAAAAAA/+MYxAAKW2YgCABHSBz/Pp6aeWoP8y//////Pz+2WGZV0OMKgUUBUxjAxGpkMBCiFDAZIFMKyYgpqKZlxycFxkAAAAAAAAAA/+MYxAAKu2YgCABNSh7/Pf/Pwy5f/l/////71Xdv3mtn/L22PUBMYmQIoYcEFlhR5g6wGbJJIjSYgpqKZlxycFxkAAAAAAAA/+MYxAAKC2IcCABNSB/Oal2Z+f/8vX/9f//f/9qqn8bN9FlNL/sfNJuuUUwd5d9CQtENjoumIKaimZccnBcZAAAAAAAAAAAA/+MYxAAJu2YkCACNTh//3//vp6fX////+TTyorVFxZGKoCrAwxwgGAH44lFvsUgRTA65epiCmopmXHJwXGQAAAAAAAAAAAAA/+MYxAAKO2IgCABNSx+W/LVazr/pn/P//////fWfO+bnzo72tqS5rlyoYd7PkgxG6KjjwJMcmIKaimZccnBcZAAAAAAAAAAA/+MYxAAIc2okCACNahrtp///////2/vZVyHGKa4I5kQ4UOSaBMxSl1ERtEjgdPExBTUUzLjk4LjIAAAAAAAAAAAAAAAAAAAA/+MYxAAJY2IkCACNTx+3/6f//r/r////9K/SqDujkdnKGDmVRJxRs6nSQ9ZTOSoDMJjkxBTUUzLjk4LjIAAAAAAAAAAAAAAA/+MYxAAKs2IUAABNMS//xevtSToh2GeV/DmxZiw/r//f+T5uYBDKKh4TMgptcCWR0cgReJASNEkxBTUUzLjk4LjIAAAAAAAA/+MYxAAK62HwCACTTR//5lLmXf3X/WWbzfb//R/TurfoYGJMYoYUVkdWK0pKxWeKRkwKk4TJSzUyYgpqKZlxycFxkAAAAAAA/+MYxAAAAANIAAAAAExBTUUzLjk4LjIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
pub const DEFAULT_GRID_SIZE: u16 = 6;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Inc,
    Dec,
}
pub const GRID_COLUMN_STEP: u16 = 6;
pub const SOUND_LIB_PATH: &str = "/public/sounds/";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Category {
    Boom,
    Doors,
    Construction,
    Eerie,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Boom => write!(f, "boom"),
            Category::Doors => write!(f, "doors"),
            Category::Construction => write!(f, "construction"),
            Category::Eerie => write!(f, "eerie"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    pub id: String,
    pub filename: String,
    pub filepath: String,
    pub category: Category,
    pub duration: f32,
}

pub fn get_category_emoji(cat: Category) -> char {
    match cat {
        Category::Boom => '🏀',
        Category::Doors => '🚪',
        Category::Construction => '🔨',
        Category::Eerie => '👻',
    }
}

pub async fn generate_lib() -> HashMap<Category, Vec<Sample>> {
    let boom: Vec<(Category, &str)> = [
        "drop_1", "drop_2", "drop_3", "drop_4", "hit_1", "hit_2", "hit_3", "hit_4", "hit_5",
        "hit_6", "hit_7",
    ]
    .map(|name| (Category::Boom, name))
    .to_vec();

    let doors: Vec<(Category, &str)> = [
        "open_1", "open_2", "slam_1", "slam_2", "slam_3", "creak_1", "creak_2", "creak_3",
        "creak_4",
    ]
    .map(|name| (Category::Doors, name))
    .to_vec();

    let construction: Vec<(Category, &str)> = [
        "drill_1", "drill_2", "drill_3", "drill_4", "drill_5", "drill_6", "drill_7", "drill_8",
        "drill_9", "hammer_1", "hammer_2", "hammer_3", "hammer_4", "hammer_5", "saw_1", "saw_2",
        "saw_3",
    ]
    .map(|name| (Category::Construction, name))
    .to_vec();

    let cat_name_tuples: Vec<(Category, &str)> = boom
        .into_iter()
        .chain(doors.into_iter())
        .chain(construction.into_iter())
        .collect();

    let promises = js_sys::Array::new();
    for (cat, name) in &cat_name_tuples {
        promises.push(&Promise::new(&mut |resolve, _| {
            let sample_path = format!("{SOUND_LIB_PATH}{cat}/{name}.mp3");
            let audio_elem = audio();
            audio_elem
                .on(ev::loadedmetadata, move |e| {
                    let target = e
                        .target()
                        .unwrap()
                        .unchecked_into::<web_sys::HtmlAudioElement>();

                    resolve
                        .call1(&JsValue::NULL, &JsValue::from(target.duration()))
                        .expect("Error promise resolving");
                })
                .set_src(sample_path.as_str());
        }));
    }

    let durations_js_arr = wasm_bindgen_futures::JsFuture::from(Promise::all(&promises)).await;

    let durations_vec: Vec<f32> =
        serde_wasm_bindgen::from_value::<Vec<f32>>(durations_js_arr.unwrap()).unwrap();

    let mut lib_hash: HashMap<Category, Vec<Sample>> = HashMap::new();
    for (idx, (category, filename)) in cat_name_tuples.into_iter().enumerate() {
        let sample = Sample {
            id: format!("{category}_{filename}"),
            filename: filename.to_string(),
            filepath: format!("{SOUND_LIB_PATH}{category}/{filename}.mp3"),
            category,
            duration: durations_vec[idx],
        };

        lib_hash.entry(category).or_default().push(sample);
    }

    lib_hash
}

pub fn format_filename(filename: &str) -> String {
    format!("{}{}", (filename[..1]).to_uppercase(), &filename[1..],).replace("_", " ")
}

#[derive(Serialize, Deserialize)]
pub struct Preset {
    volume: f32,
    duration: u64,
    random: bool,
    grid_data: Vec<Option<Sample>>,
}

pub const GRID_ROWS_MIN: u16 = 1;
pub const GRID_ROWS_MAX: u16 = 20;
