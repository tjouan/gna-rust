mod cli;
mod repository;
mod repo_finder;
mod run_control;

fn main() {
    cli::run(std::env::args().collect());
}
