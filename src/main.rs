mod blog;
mod cli;
mod html;
mod lexer;
mod server;
mod token;

use crate::cli::cli;

fn main() -> std::io::Result<()> {
    cli()
}
