use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if (args[1] == "init") {
        fs::create_dir(".git").unwrap();
        fs::create_dir((".git/objects")).unwrap();
        fs::create_dir((".git/refs")).unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();

        println!("Git repository initialized successfully!");
    } else {
        println!("Unknown command provided: {}", args[1]);
    }

    println!("Hello, world!");
}
