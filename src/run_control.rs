use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

pub fn repos() -> Vec<String> {
    let path = Path::new("/home/tj/.gna.rc");

    let file = match File::open(&path) {
        Ok(f)   => f,
        Err(e)  => panic!("cannot open `{}': {}",
            path.display(),
            Error::description(&e)
        )
    };

    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        let line = line.unwrap();
        line
    }).collect()
}
