use chrono::prelude::*;

pub fn today() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d").to_string()
}
