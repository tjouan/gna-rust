extern crate glob;

use self::glob::glob;

pub fn find(path: &str) -> Vec<String> {
    let pattern = format!("{}/**/.git", path);

    glob(&pattern).unwrap().filter_map(Result::ok).map(|entry| {
        let entry = entry.display().to_string();
        entry
    }).collect()
}
