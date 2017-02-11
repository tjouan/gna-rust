extern crate glob;

use self::glob::glob;

pub fn find(path: &str) -> Vec<String> {
    let pattern = format!("{}/**/.git", path);

    glob(&pattern).unwrap().filter_map(Result::ok).map(|entry| {
        entry.display().to_string()
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::find;

    #[test]
    fn returns_git_repositories_paths() {
        assert_eq!(vec![".git"], find("."))
    }
}
