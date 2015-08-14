use repository;
use run_control;

pub fn run() {
    for e in run_control::repos().iter() {
        repository::check(e);
    }
}
