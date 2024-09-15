use std::env;
use systemforge_core::analyze_file_system;

fn main() {
    //Get the arguments from the cli
    let args :Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: systemforge-cli <command> [args]");
        return
    }

    let command = &args[1];

    match command.as_str() {
        "analyze-fs" => {
            if args.len()< 3{
                println!("Usage: systemforge-cli analyze-fs <path>");
            } else {
                let path = &args[2];
                analyze_file_system(path);
            }
        },
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: analyze-fs");
        }
        
    }
}
