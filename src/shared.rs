use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc, Weekday};
use core::{fmt, str};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const EMPTY_SOUND: &str = "data:audio/mp3;base64,/+MYxAALM2H8CABNSR8lJeb1//ZLXL3GxwA////Gfvrf/xEGRl3ZCQcBk9PXDQQnx2IAMLJ3sQYgITEFNRTMuOTguMgAAAAA/+MYxAAKu2IcCABNSR/B/IvkRlkrv84Pyb////7n8/c/lr/fJKmETd2jQJys1AEVDNFHQFpMdayYgpqKZlxycFxkAAAAAAAA/+MYxAAKe2YcEACTTAGpb/f/3S3S6O/7W/3///39ay7IZqJYLGUjmMC5G1Nc8iFSopqzJM8YMJiCmopmXHJwXGQAAAAAAAAA/+MYxAAKC2YgEACTTAG+3/9/X3/Rfomn+///qv0N6Is4txSqUMFVEcRLCVlx3prPbiPEc32mIKaimZccnBcZAAAAAAAAAAAA/+MYxAAKg2IgEABNSQJ/58/POVS7+9r/////fjP+339sTMyB1mTB2z2dDass4zE3BUU4J2U650xBTUUzLjk4LjIAAAAAAAAA/+MYxAAJU2okEACNTAG/9++v/X/P9f////+uyp8Il6TGMRimB1ZjTsLkOWmwSBskEEJTEFNRTMuOTguMgAAAAAAAAAAAAAAA/+MYxAAJ+2YcCACNTA//1P9Nb9LmrJ3XWv6dP///TyMay3cjqYhoIWpExQM8ynALZwUHc9aYgpqKZlxycFxkAAAAAAAAAAAA/+MYxAAKW2YgCABHSBz/Pp6aeWoP8y//////Pz+2WGZV0OMKgUUBUxjAxGpkMBCiFDAZIFMKyYgpqKZlxycFxkAAAAAAAAAA/+MYxAAKu2YgCABNSh7/Pf/Pwy5f/l/////71Xdv3mtn/L22PUBMYmQIoYcEFlhR5g6wGbJJIjSYgpqKZlxycFxkAAAAAAAA/+MYxAAKC2IcCABNSB/Oal2Z+f/8vX/9f//f/9qqn8bN9FlNL/sfNJuuUUwd5d9CQtENjoumIKaimZccnBcZAAAAAAAAAAAA/+MYxAAJu2YkCACNTh//3//vp6fX////+TTyorVFxZGKoCrAwxwgGAH44lFvsUgRTA65epiCmopmXHJwXGQAAAAAAAAAAAAA/+MYxAAKO2IgCABNSx+W/LVazr/pn/P//////fWfO+bnzo72tqS5rlyoYd7PkgxG6KjjwJMcmIKaimZccnBcZAAAAAAAAAAA/+MYxAAIc2okCACNahrtp///////2/vZVyHGKa4I5kQ4UOSaBMxSl1ERtEjgdPExBTUUzLjk4LjIAAAAAAAAAAAAAAAAAAAA/+MYxAAJY2IkCACNTx+3/6f//r/r////9K/SqDujkdnKGDmVRJxRs6nSQ9ZTOSoDMJjkxBTUUzLjk4LjIAAAAAAAAAAAAAAA/+MYxAAKs2IUAABNMS//xevtSToh2GeV/DmxZiw/r//f+T5uYBDKKh4TMgptcCWR0cgReJASNEkxBTUUzLjk4LjIAAAAAAAA/+MYxAAK62HwCACTTR//5lLmXf3X/WWbzfb//R/TurfoYGJMYoYUVkdWK0pKxWeKRkwKk4TJSzUyYgpqKZlxycFxkAAAAAAA/+MYxAAAAANIAAAAAExBTUUzLjk4LjIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

#[derive(Debug, PartialEq)]
pub enum Operation {
    Inc,
    Dec,
}
pub const SOUND_LIB_PATH: &str = "/public/sounds/";
pub const SOUND_LIB_JSON_PATH: &str = "/public/sounds/lib.json";
pub const GRID_ROWS_MIN: u16 = 1;
pub const GRID_ROWS_MAX: u16 = 20;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Category {
    Boom,
    Doors,
    People,
    Construction,
    Eerie,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Boom => write!(f, "boom"),
            Category::Doors => write!(f, "doors"),
            Category::People => write!(f, "people"),
            Category::Construction => write!(f, "construction"),
            Category::Eerie => write!(f, "eerie"),
        }
    }
}

impl Category {
    pub fn get_emoji(&self) -> char {
        match self {
            Category::Boom => '🏀',
            Category::Doors => '🚪',
            Category::People => '🤦',
            Category::Construction => '🔨',
            Category::Eerie => '👻',
        }
    }

    pub fn iter() -> impl Iterator<Item = Category> {
        [
            Category::Boom,
            Category::Doors,
            Category::People,
            Category::Construction,
            Category::Eerie,
        ]
        .iter()
        .copied()
    }
}

impl FromStr for Category {
    type Err = ();
    fn from_str(v: &str) -> Result<Category, Self::Err> {
        match v {
            "boom" => Ok(Category::Boom),
            "doors" => Ok(Category::Doors),
            "people" => Ok(Category::People),
            "construction" => Ok(Category::Construction),
            "eerie" => Ok(Category::Eerie),
            _ => Err(()),
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

impl Ord for Sample {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.filename.cmp(&other.filename)
    }
}

impl PartialOrd for Sample {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.filename.cmp(&other.filename))
    }
}

impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        self.filename == other.filename
    }
}

impl Eq for Sample {}

pub fn format_filename(filename: &str) -> String {
    format!("{}{}", filename[..1].to_uppercase(), &filename[1..],).replace("_", " ")
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub volume: f32,
    pub gap_duration: u64,
    pub random_playback: bool,
    pub grid_data: Vec<Option<Sample>>,
    pub created: DateTime<Utc>,
}

impl Ord for Preset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.created.cmp(&other.created)
    }
}

impl PartialOrd for Preset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.created.cmp(&other.created))
    }
}

impl PartialEq for Preset {
    fn eq(&self, other: &Self) -> bool {
        self.created == other.created
    }
}

impl Eq for Preset {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedSchedule {
    pub id: String,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub preset: Preset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringSchedule {
    pub id: String,
    pub weekdays: Vec<Weekday>,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub preset: Preset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringScheduleOverlap {
    pub weekdays: Vec<Weekday>,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScheduleType {
    Planned,
    Reccuring,
}

pub fn is_mobile() -> bool {
    web_sys::window().unwrap().navigator().max_touch_points() > 0
}

pub fn grid_row_size() -> u16 {
    if is_mobile() {
        4
    } else {
        6
    }
}
