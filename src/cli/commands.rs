use crate::cli::args::{CliArgs, Commands};
use crate::cli::phases::{Phase, PhaseTracker};
use crate::cli::theme::Theme;
use crate::compiler::compiler::Compiler;
use crate::compiler::disassembler::disassemble;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::vyn_vm::vm::VynVM;
use colored::*;
use std::fs;
use std::path::PathBuf;

pub const VERSION: &str = "0.1.0";

pub struct CommandHandler {
    args: CliArgs,
}

impl CommandHandler {
    pub fn new(args: CliArgs) -> Self {
        Self { args }
    }

    pub fn execute(&self) -> Result<(), i32> {
        match &self.args.command {
            Commands::Run { file } => self.run_file(file),
            Commands::Check { file } => self.check_file(file),
            Commands::Disasm { file } => self.disasm_file(file),
            Commands::Version => self.show_version(),
        }
    }

    fn run_file(&self, file: &PathBuf) -> Result<(), i32> {
        let source = self.read_file(file)?;
        let file_name = self.get_file_name(file);

        let mut tracker = PhaseTracker::new(
            file_name,
            !self.args.no_progress,
            self.args.verbose,
            self.args.quiet,
            self.args.slow_mode,
        );

        tracker.start();

        // Compile the program
        let bytecode = match self.compile_program(&source, &mut tracker) {
            Ok(bc) => bc,
            Err(code) => return Err(code),
        };

        tracker.finish();

        if !self.args.quiet {
            println!();
        }

        // Execute the program
        let mut vm = VynVM::new(
            bytecode.instructions,
            bytecode.constants,
            bytecode.string_table,
        );

        if let Err(e) = vm.execute() {
            if !self.args.quiet {
                eprintln!();
                e.report(&source);
            }
            return Err(2);
        }

        Ok(())
    }

    fn check_file(&self, file: &PathBuf) -> Result<(), i32> {
        let source = self.read_file(file)?;
        let file_name = self.get_file_name(file);

        let mut tracker = PhaseTracker::new(
            file_name,
            !self.args.no_progress,
            self.args.verbose,
            self.args.quiet,
            self.args.slow_mode,
        );

        tracker.start();

        // Tokenize
        tracker.begin_phase(Phase::Tokenizing);
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize();
        tracker.complete_phase(Phase::Tokenizing);

        // Parse
        tracker.begin_phase(Phase::Parsing);
        let mut parser = Parser::new(tokens);
        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(errors) => {
                tracker.clear_display();
                if !self.args.quiet {
                    errors.report_all(&source);
                }
                return Err(1);
            }
        };
        tracker.complete_phase(Phase::Parsing);

        // Type check
        tracker.begin_phase(Phase::TypeChecking);
        let mut type_checker = crate::type_checker::type_checker::TypeChecker::new();
        if let Err(errors) = type_checker.check_program(&program) {
            tracker.clear_display();
            if !self.args.quiet {
                errors.report_all(&source);
            }
            return Err(1);
        }
        tracker.complete_phase(Phase::TypeChecking);

        tracker.finish();

        if !self.args.quiet {
            println!();
            println!(
                "{} {}",
                Theme::PHASE_COMPLETE.color(Theme::SUCCESS).bold(),
                "No type errors found".bright_white()
            );
        }

        Ok(())
    }

    fn disasm_file(&self, file: &PathBuf) -> Result<(), i32> {
        let source = self.read_file(file)?;
        let file_name = self.get_file_name(file);

        let mut tracker = PhaseTracker::new(
            file_name,
            !self.args.no_progress,
            self.args.verbose,
            self.args.quiet,
            self.args.slow_mode,
        );

        tracker.start();

        // Compile the program
        let bytecode = match self.compile_program(&source, &mut tracker) {
            Ok(bc) => bc,
            Err(code) => return Err(code),
        };

        tracker.finish();

        if !self.args.quiet {
            println!();
        }

        // Disassemble
        disassemble(&bytecode);

        Ok(())
    }

    fn compile_program(
        &self,
        source: &str,
        tracker: &mut PhaseTracker,
    ) -> Result<crate::compiler::compiler::Bytecode, i32> {
        // Tokenize
        tracker.begin_phase(Phase::Tokenizing);
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        tracker.complete_phase(Phase::Tokenizing);

        // Parse
        tracker.begin_phase(Phase::Parsing);
        let mut parser = Parser::new(tokens);
        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(errors) => {
                tracker.clear_display();
                if !self.args.quiet {
                    errors.report_all(source);
                }
                return Err(1);
            }
        };
        tracker.complete_phase(Phase::Parsing);

        // Type check
        tracker.begin_phase(Phase::TypeChecking);
        let mut type_checker = crate::type_checker::type_checker::TypeChecker::new();
        if let Err(errors) = type_checker.check_program(&program) {
            tracker.clear_display();
            if !self.args.quiet {
                errors.report_all(source);
            }
            return Err(1);
        }
        tracker.complete_phase(Phase::TypeChecking);

        // Compile
        tracker.begin_phase(Phase::Compiling);
        let mut compiler = Compiler::new();
        let bytecode = match compiler.compile_program(program) {
            Ok(bc) => bc,
            Err(errors) => {
                tracker.clear_display();
                if !self.args.quiet {
                    errors.report_all(source);
                }
                return Err(1);
            }
        };
        tracker.complete_phase(Phase::Compiling);

        Ok(bytecode)
    }

    fn show_version(&self) -> Result<(), i32> {
        println!("{} {}", "vyn".cyan().bold(), VERSION.bright_white());
        println!("{}", "Vyn Programming Language".white().dimmed());
        Ok(())
    }

    fn read_file(&self, file: &PathBuf) -> Result<String, i32> {
        fs::read_to_string(file).map_err(|e| {
            if !self.args.quiet {
                eprintln!(
                    "{}{}{} {}",
                    "Error".red().bold(),
                    "::".white().dimmed(),
                    "IO".bright_white().bold(),
                    format!("-> Could not read file: {}", e).bright_red()
                );
            }
            1
        })
    }

    fn get_file_name(&self, file: &PathBuf) -> String {
        file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
}
