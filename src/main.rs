mod cli;
mod run_control;

fn main() {
    cli::run(std::env::args().collect());
}
