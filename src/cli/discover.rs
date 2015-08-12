use run_control;
use repo_finder;

pub fn run(path: &str) {
    run_control::save_repos(repo_finder::find(&path));
}
