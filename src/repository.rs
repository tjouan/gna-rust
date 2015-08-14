use std::process::Command;

fn git_exec(path: &str, args: &[&str]) {
    let mut all_args = vec!["-C", path];
    for arg in args.iter() {
        all_args.push(arg);
    }
    let execution = Command::new("git")
        .args(&all_args)
        .output();

    match execution {
        Ok(o)   => {
            if !o.status.success() {
                println!("ERROR: cannot check repository");
            }
            print!("{}", String::from_utf8_lossy(&o.stdout));
        },
        Err(e)  => panic!("cannot execute process: {}", e)
    };
}

pub fn check(path: &str) {
    println!("*** {} ***", path);

    git_exec(path, &["symbolic-ref", "--short", "HEAD"]);
    git_exec(path, &["status", "--porcelain"]);
}
