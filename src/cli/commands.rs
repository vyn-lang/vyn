use crate::cli::args::{CliArgs, Commands};
use crate::cli::phases::{Phase, PhaseTracker};
use crate::compiler::compiler::VynCompiler;
use crate::compiler::disassembler::disassemble;
use crate::ir::builder::VynIRBuilder;
use crate::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::type_checker::static_evaluator::StaticEvaluator;
use crate::type_checker::type_checker::TypeChecker;
use colored::*;
use std::fs;
use std::path::PathBuf;

pub const VERSION: &str = "0.12.0";

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

        // Static evaluation
        tracker.begin_phase(Phase::StaticEvaluation);
        let mut static_eval = StaticEvaluator::new();
        let mut static_errors = crate::error_handler::error_collector::ErrorCollector::new();
        if let Err(_) = static_eval.evaluate_program(&program, &mut static_errors) {
            tracker.clear_display();
            if !self.args.quiet {
                static_errors.report_all(&source);
            }
            return Err(1);
        }
        tracker.complete_phase(Phase::StaticEvaluation);

        // Type check
        tracker.begin_phase(Phase::TypeChecking);
        let mut type_checker = TypeChecker::new(&static_eval);
        if let Err(errors) = type_checker.check_program(&program) {
            tracker.clear_display();
            if !self.args.quiet {
                errors.report_all(&source);
            }
            return Err(1);
        }
        tracker.complete_phase(Phase::TypeChecking);

        // Build IR
        tracker.begin_phase(Phase::IRBuilding);
        let mut ir_builder = VynIRBuilder::new(&static_eval, &type_checker.symbol_type_table);
        let ir = match ir_builder.build_ir(&program) {
            Ok(ir) => ir,
            Err(errors) => {
                tracker.clear_display();
                if !self.args.quiet {
                    errors.report_all(&source);
                }
                return Err(1);
            }
        };
        tracker.complete_phase(Phase::IRBuilding);

        // Compiling
        tracker.begin_phase(Phase::Compiling);
        let mut compiler = VynCompiler::new();
        let bc = match compiler.compile_ir(&ir) {
            Ok(bc) => bc,
            Err(errors) => {
                tracker.clear_display();
                if !self.args.quiet {
                    errors.report_all(&source);
                }
                return Err(1);
            }
        };
        tracker.complete_phase(Phase::Compiling);

        tracker.finish();

        if !self.args.quiet {
            println!("\n{}", "Generated IR:".bright_green().bold());
            for (i, instr) in ir.instructions.iter().enumerate() {
                println!("  {}: {:?}", i, instr);
            }
            println!("\n{}", "Disassembled Bytecode:".bright_green().bold());
            disassemble(&bc);
        }

        Ok(())
    }

    fn check_file(&self, file: &PathBuf) -> Result<(), i32> {
        todo!("File checker not implemented");
    }

    fn disasm_file(&self, file: &PathBuf) -> Result<(), i32> {
        todo!("File disassembler not implemented");
    }
    //     let source = self.read_file(file)?;
    //     let file_name = self.get_file_name(file);
    //
    //     let mut tracker = PhaseTracker::new(
    //         file_name,
    //         !self.args.no_progress,
    //         self.args.verbose,
    //         self.args.quiet,
    //         self.args.slow_mode,
    //     );
    //
    //     tracker.start();
    //
    //     // Compile the program
    //     let bytecode = match self.compile_program(&source, &mut tracker) {
    //         Ok(bc) => bc,
    //         Err(code) => return Err(code),
    //     };
    //
    //     tracker.finish();
    //
    //     if !self.args.quiet {
    //         println!();
    //     }
    //
    //     // Disassemble
    //     disassemble(&bytecode);
    //
    //     Ok(())
    // }

    fn compile_program(&self, source: &str, tracker: &mut PhaseTracker)
    /*-> Result<crate::compiler::compiler::Bytecode, i32>*/
    {
        todo!("Program compiler not implemented");
    }
    //     // Tokenize
    //     tracker.begin_phase(Phase::Tokenizing);
    //     let mut lexer = Lexer::new(source);
    //     let tokens = lexer.tokenize();
    //     tracker.complete_phase(Phase::Tokenizing);
    //
    //     // Parse
    //     tracker.begin_phase(Phase::Parsing);
    //     let mut parser = Parser::new(tokens);
    //     let program = match parser.parse_program() {
    //         Ok(p) => p,
    //         Err(errors) => {
    //             tracker.clear_display();
    //             if !self.args.quiet {
    //                 errors.report_all(source);
    //             }
    //             return Err(1);
    //         }
    //     };
    //     tracker.complete_phase(Phase::Parsing);
    //
    //     // Static evaluation
    //     tracker.begin_phase(Phase::StaticEvaluation);
    //     let mut static_eval = StaticEvaluator::new();
    //     let mut static_errors = crate::error_handler::error_collector::ErrorCollector::new();
    //     if let Err(_) = static_eval.evaluate_program(&program, &mut static_errors) {
    //         tracker.clear_display();
    //         if !self.args.quiet {
    //             static_errors.report_all(source);
    //         }
    //         return Err(1);
    //     }
    //     tracker.complete_phase(Phase::StaticEvaluation);
    //
    //     // Type check
    //     tracker.begin_phase(Phase::TypeChecking);
    //     let mut type_checker = crate::type_checker::type_checker::TypeChecker::new(&static_eval);
    //     if let Err(errors) = type_checker.check_program(&program) {
    //         tracker.clear_display();
    //         if !self.args.quiet {
    //             errors.report_all(source);
    //         }
    //         return Err(1);
    //     }
    //     tracker.complete_phase(Phase::TypeChecking);
    //
    //     // Compile
    //     tracker.begin_phase(Phase::Compiling);
    //     let mut compiler = Compiler::new(&static_eval);
    //     let bytecode = match compiler.compile_program(program) {
    //         Ok(bc) => bc,
    //         Err(errors) => {
    //             tracker.clear_display();
    //             if !self.args.quiet {
    //                 errors.report_all(source);
    //             }
    //             return Err(1);
    //         }
    //     };
    //     tracker.complete_phase(Phase::Compiling);
    //
    //     Ok(bytecode)
    // }

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
