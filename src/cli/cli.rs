use crate::cli::{args::CliArgs, commands::CommandHandler};

pub fn run() {
    let args = CliArgs::parse();
    let handler = CommandHandler::new(args);

    if let Err(code) = handler.execute() {
        std::process::exit(code);
    }
}
