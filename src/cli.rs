use crate::blog::generate_blogs;
use crate::server::preview_server;

fn print_help() {
    println!("Usage: mdb [COMMAND]");
    println!("Commands:");
    println!("    h, help            Prints this message.");
    println!("    v, version         Prints mdb version.");
    println!("    p, preview         Spins up a server on which you can live preview how your markdown will look.");
    println!("    g, generate        Generates html articles from your markdown files");
}

enum Command {
    Invalid,
    Help,
    Version,
    Preview,
    Generate,
}

pub fn cli() -> std::io::Result<()> {
    const VERSION: &str = "0.1.0";

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Invalid number of arguments!");
        println!("Use --help");
        return Ok(());
    }

    let command = match args[1].as_str() {
        "h" | "help" => Command::Help,
        "v" | "version" => Command::Version,
        "p" | "preview" => Command::Preview,
        "g" | "generate" => Command::Generate,
        _ => Command::Invalid,
    };

    match command {
        Command::Invalid => println!("Invalid command!\nUse --help"),
        Command::Version => println!("Version: {}", VERSION),
        Command::Help => print_help(),
        Command::Preview => preview_server()?,
        Command::Generate => generate_blogs(),
    }

    Ok(())
}
