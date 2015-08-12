use std::error::Error;
use std::env::{home_dir};
use std::fs::{File,OpenOptions};
use std::io::{BufReader,BufRead,Write};
use std::path::PathBuf;

fn file_path() -> PathBuf {
    match home_dir() {
        Some(ref p) => PathBuf::from(format!("{}/.gna.rc", p.display())),
        None        => panic!("cannot determine home directory")
    }
}

fn file(open_options: &OpenOptions) -> File {
    let path = file_path();

    match open_options.open(&path) {
        Ok(f)   => f,
        Err(e)  => panic!("cannot open `{:?}': {}",
            path,
            Error::description(&e)
        )
    }
}

pub fn repos() -> Vec<String> {
    let reader = BufReader::new(file(OpenOptions::new().read(true)));

    reader.lines().map(|line| {
        let line = line.unwrap();
        line
    }).collect()
}

pub fn save_repos(repos: Vec<String>) {
    let mut file = file(OpenOptions::new().create(true).write(true));

    for repo in repos.iter() {
        match writeln!(file, "{}", repo) {
            Err(e)  => panic!("cannot write to `{:?}': {}",
                file_path(),
                Error::description(&e)
            ),
            _ => ()
        }
    }
}
