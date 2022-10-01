use std::process;

fn main() {
    if let Err(e) = rrbot::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
