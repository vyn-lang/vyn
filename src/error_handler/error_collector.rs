use crate::{error_handler::errors::VynError, utils::Span};
use colored::*;

impl VynError {
    pub fn span(&self) -> Span {
        match self {
            VynError::UnexpectedToken { span, .. } => *span,
            VynError::ExpectedToken { span, .. } => *span,
            VynError::KeywordTypeError { span, .. } => *span,
            VynError::InvalidTypeName { span, .. } => *span,
            VynError::ExpectedType { span, .. } => *span,
            VynError::RegisterOverflow { span, .. } => *span,
            VynError::NotImplemented { span, .. } => *span,
            VynError::InvalidIndexing { span, .. } => *span,
            VynError::IndexOutOfBounds { span, .. } => *span,

            VynError::TypeMismatch { span, .. } => *span,
            VynError::InvalidUnaryOp { span, .. } => *span,
            VynError::InvalidBinaryOp { span, .. } => *span,
            VynError::DeclarationTypeMismatch { span, .. } => *span,
            VynError::UndefinedVariable { span, .. } => *span,
            VynError::VariableRedeclaration {
                redeclaration_span, ..
            } => *redeclaration_span,
            VynError::TypeAliasRedeclaration { span, .. } => *span,
            VynError::ImmutableMutation { span, .. } => *span,
            VynError::LeftHandAssignment { span, .. } => *span,
            VynError::TypeInfer { span, .. } => *span,
            VynError::ArrayLengthMismatch { span, .. } => *span,

            VynError::UnknownAST { span, .. } => *span,
            VynError::UndefinedIdentifier { span, .. } => *span,

            VynError::ArithmeticError { span, .. } => *span,
            VynError::UnaryOperationError { span, .. } => *span,
            VynError::ComparisonOperationError { span, .. } => *span,
            VynError::DivisionByZero { span } => *span,
        }
    }

    pub fn report(&self, source: &str) {
        let span = self.span();

        // Header: Category::Error -> message
        eprintln!(
            "{}{}{}{}",
            self.category().bright_white().bold(),
            "::".white().dimmed(),
            "Error".red().dimmed().bold(),
            format!(" -> {}", self.message()).bright_red()
        );

        eprintln!();

        // Error caused by section
        eprintln!("{}", "Error caused by:".white().dimmed().bold());

        // Main error location
        self.print_code_snippet(source, span, true);

        // Additional context based on error type
        self.print_additional_context(source);

        eprintln!();

        // Hint section
        if let Some(hint_text) = self.hint() {
            eprintln!("{} {}", "Hint:".bright_yellow(), hint_text.bright_white());
        }
    }

    fn print_code_snippet(&self, source: &str, span: Span, highlight: bool) {
        let lines: Vec<&str> = source.lines().collect();

        if span.line == 0 || span.line > (lines.len() as u32) {
            eprintln!(
                "    {} {} {}",
                format!("Ln {}:{}", span.line, span.start_column).cyan(),
                "|".white(),
                "<source unavailable>".dimmed()
            );
            return;
        }

        let line_content = lines[(span.line - 1) as usize];
        let line_label = format!("Ln {}:{}", span.line, span.start_column);

        // Print the line
        if highlight {
            eprintln!(
                "    {} {} {}",
                line_label.cyan().bold(),
                "|".white(),
                line_content.bold().bright_white(),
            );
        } else {
            eprintln!(
                "    {} {} {}",
                line_label.cyan().bold(),
                "|".white(),
                line_content.dimmed(),
            );
        }

        // Print the pointer
        let line_prefix_len = line_label.len();
        let gutter_padding = " ".repeat(line_prefix_len + 3); // +3 for " | "

        // IMPORTANT: Columns are 1-indexed, so subtract 1 for 0-indexed string positioning
        // Also need to handle the actual character width correctly
        let start_pos = (span.start_column as usize).saturating_sub(1);
        let code_padding = " ".repeat(start_pos);

        let width = (span.end_column.saturating_sub(span.start_column) as usize).max(1);
        let pointer = if width == 1 {
            "^".to_string()
        } else {
            "~".repeat(width)
        };

        if highlight {
            eprintln!(
                "    {}{}{}",
                gutter_padding,
                code_padding,
                pointer.bright_red().bold()
            );
        } else {
            eprintln!(
                "    {}{}{}",
                gutter_padding,
                code_padding,
                pointer.cyan().dimmed()
            );
        }
    }

    fn print_additional_context(&self, source: &str) {
        match self {
            VynError::VariableRedeclaration { original_span, .. } => {
                eprintln!();
                eprintln!("{}", "Originally declared here:".white().dimmed());
                self.print_code_snippet(source, *original_span, false);
            }
            VynError::ImmutableMutation { mutation_span, .. } => {
                eprintln!();
                eprintln!("{}", "identifier mutated here".white().dimmed());
                self.print_code_snippet(source, *mutation_span, false);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<VynError>,
}

impl ErrorCollector {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, error: VynError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn report_all(&self, source: &str) {
        for error in &self.errors {
            error.report(source);
            println!()
        }

        if !self.errors.is_empty() {
            let error_word = if self.errors.len() == 1 {
                "error"
            } else {
                "errors"
            };

            eprintln!(
                "{} Could not compile due to {} {}",
                "*".bright_red().bold(),
                self.errors.len().to_string().bright_red().bold(),
                error_word.bright_red()
            );
        }
    }

    pub fn errors(&self) -> &[VynError] {
        &self.errors
    }

    pub fn clear(&mut self) {
        self.errors.clear()
    }
}
