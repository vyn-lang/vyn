use std::collections::HashMap;

use crate::{
    compiler::{
        compiler::{Bytecode, Compiler},
        disassembler::disassemble,
    },
    lexer::Lexer,
    parser::parser::Parser,
    utils::{self, throw_error},
};

type CommandFn = fn(&[String]);

struct Command {
    name: &'static str,
    description: &'static str,
    usage: &'static str,
    args_count: usize,
    function: CommandFn,
}

fn get_commands() -> HashMap<&'static str, Command> {
    let mut commands = HashMap::new();

    commands.insert(
        "help",
        Command {
            name: "help",
            description: "Shows this help menu",
            usage: "help",
            args_count: 0,
            function: command_help,
        },
    );

    commands.insert(
        "run",
        Command {
            name: "run",
            description: "Runs a Hydor source file",
            usage: "run <file>",
            args_count: 1,
            function: command_run,
        },
    );

    commands.insert(
        "disassemble",
        Command {
            name: "disassemble",
            description: "Disassembles bytecode from a Hydor file",
            usage: "disassemble <file>",
            args_count: 1,
            function: command_disassemble,
        },
    );

    commands
}

pub fn run(args: Vec<String>) {
    if args.len() < 2 {
        throw_error(
            "Usage: hydor <command> [args...]\nRun 'hydor help' for more info",
            1,
        );
    }

    let commands = get_commands();
    let command_name = &args[1];

    // fetch commands
    let command = match commands.get(command_name.as_str()) {
        Some(cmd) => cmd,
        None => throw_error(
            &format!(
                "Unknown command '{}'. Run 'hydor help' for available commands",
                command_name
            ),
            1,
        ),
    };

    // calculate args
    let provided_args = args.len() - 2;
    if provided_args != command.args_count {
        throw_error(&format!("Usage: hydor {}", command.usage), 1);
    }

    (command.function)(&args[2..]);
}

fn command_help(_args: &[String]) {
    let commands = get_commands();

    println!("Usage: hydor <command> [args...]\n");
    println!("Commands:");

    let mut cmd_list: Vec<_> = commands.values().collect();
    cmd_list.sort_by_key(|c| c.name);

    for cmd in cmd_list {
        println!("  {:<20} {}", cmd.usage, cmd.description);
    }
}

fn command_run(args: &[String]) {
    let path = &args[0];
    let source = utils::read_file(path.to_string());
    let bytecode = compile_source(&source);

    // disassemble for now
    disassemble(&bytecode);
}

fn command_disassemble(args: &[String]) {
    let path = &args[0];
    let source = utils::read_file(path.to_string());
    let bytecode = compile_source(&source);
    disassemble(&bytecode);
}

fn compile_source(source: &str) -> Bytecode {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(errors) => {
            errors.report_all(source);
            std::process::exit(1);
        }
    };

    let mut compiler = Compiler::new();
    match compiler.compile_program(program) {
        Ok(bytecode) => bytecode,
        Err(errors) => {
            errors.report_all(source);
            std::process::exit(1);
        }
    }
}
