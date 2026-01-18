use std::collections::HashMap;

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    errors::{ErrorCollector, HydorError},
    parser::lookups::Precedence,
    tokens::{Token, TokenInfo, TokenType},
    utils::{Span, Spanned},
};

type PrefixParseFn = fn(&mut Parser) -> Option<Expression>;
type InfixParseFn = fn(&mut Parser, Expression) -> Option<Expression>;
type StatementParseFn = fn(&mut Parser) -> Option<Statement>;

pub struct Parser {
    tokens: Vec<TokenInfo>,
    current: usize,
    delimiter_stack: Vec<TokenType>,

    pub led_parse_fns: HashMap<TokenType, InfixParseFn>,
    pub nud_parse_fns: HashMap<TokenType, PrefixParseFn>,
    pub stmt_parse_fns: HashMap<TokenType, StatementParseFn>,

    pub errors: ErrorCollector,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let mut parser = Self {
            tokens,
            current: 0,
            errors: ErrorCollector::new(),
            delimiter_stack: Vec::new(),

            led_parse_fns: HashMap::new(),
            nud_parse_fns: HashMap::new(),
            stmt_parse_fns: HashMap::new(),
        };

        parser.register_nud(TokenType::Integer, Parser::parse_integer_literal);
        parser.register_nud(TokenType::Float, Parser::parse_float_literal);
        parser.register_nud(TokenType::False, Parser::parse_bool_literal);
        parser.register_nud(TokenType::True, Parser::parse_bool_literal);
        parser.register_nud(TokenType::Identifier, Parser::parse_identifier_literal);
        parser.register_nud(TokenType::String, Parser::parse_string_literal);
        parser.register_nud(TokenType::Nil, Parser::parse_nil_literal);

        parser.register_nud(TokenType::Minus, Parser::parse_unary_expr);
        parser.register_nud(TokenType::Not, Parser::parse_unary_expr);
        parser.register_nud(TokenType::LeftParenthesis, Parser::parse_grouping_expr);

        parser.register_led(TokenType::Plus, Parser::parse_binary_expr);
        parser.register_led(TokenType::Minus, Parser::parse_binary_expr);
        parser.register_led(TokenType::Asterisk, Parser::parse_binary_expr);
        parser.register_led(TokenType::Slash, Parser::parse_binary_expr);
        parser.register_led(TokenType::Caret, Parser::parse_exponent_expr);

        parser.register_led(TokenType::LessThan, Parser::parse_binary_expr);
        parser.register_led(TokenType::LessThanEqual, Parser::parse_binary_expr);
        parser.register_led(TokenType::GreaterThan, Parser::parse_binary_expr);
        parser.register_led(TokenType::GreaterThanEqual, Parser::parse_binary_expr);
        parser.register_led(TokenType::Equal, Parser::parse_binary_expr);
        parser.register_led(TokenType::NotEqual, Parser::parse_binary_expr);

        parser.register_stmt(TokenType::Let, Parser::parse_variable_decl);

        parser
    }

    pub fn parse_program(&mut self) -> Result<Program, ErrorCollector> {
        let mut body: Vec<Statement> = Vec::new();

        while !self.is_eof() {
            self.skip_delimiters(); // Skip leading delimiters

            if self.is_eof() {
                break;
            }

            match self.try_parse_statement() {
                Some(stmt) => body.push(stmt),
                None => self.synchronize(),
            }
        }

        if self.errors.has_errors() {
            Err(std::mem::take(&mut self.errors))
        } else {
            Ok(Program { statements: body })
        }
    }

    fn register_nud(&mut self, token: TokenType, func: PrefixParseFn) {
        self.nud_parse_fns.insert(token, func);
    }

    fn register_led(&mut self, token: TokenType, func: InfixParseFn) {
        self.led_parse_fns.insert(token, func);
    }

    fn register_stmt(&mut self, token: TokenType, func: StatementParseFn) {
        self.stmt_parse_fns.insert(token, func);
    }

    pub(crate) fn advance(&mut self) {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
    }

    fn is_at_delimiter(&self) -> bool {
        matches!(
            self.current_token().token.get_token_type(),
            TokenType::Semicolon | TokenType::Newline
        )
    }

    /// Skip all consecutive delimiters (newlines and semicolons)
    fn skip_delimiters(&mut self) {
        while self.is_at_delimiter() && !self.is_eof() {
            self.advance();
        }
    }

    pub(crate) fn expect(&mut self, token_type: TokenType) -> bool {
        if self.current_token().token.get_token_type() != token_type {
            self.errors.add(HydorError::ExpectedToken {
                expected: token_type,
                got: self.current_token().token.get_token_type(),
                span: self.current_token().span,
            });
            return false;
        }

        self.advance();
        true
    }

    fn expect_delimiter(&mut self) -> bool {
        // If we're inside delimiters (parentheses, etc.), delimiters are optional
        if !self.delimiter_stack.is_empty() {
            self.skip_delimiters();
            return true;
        }

        // Outside delimiters - require a delimiter OR EOF
        let current = self.current_token().token.get_token_type();

        match current {
            TokenType::EndOfFile => true,

            TokenType::Semicolon | TokenType::Newline => {
                // Consume all consecutive delimiters
                self.skip_delimiters();
                true
            }

            _ => {
                self.errors.add(HydorError::ExpectedToken {
                    expected: TokenType::Semicolon,
                    got: current,
                    span: self.current_token().span,
                });
                false
            }
        }
    }

    fn skip_newlines_in_delimiters(&mut self) {
        if !self.delimiter_stack.is_empty() {
            self.skip_delimiters();
        }
    }

    /// Synchronize to the next statement boundary after an error
    /// Just keep advancing until we're past all delimiters (or hit EOF)
    fn synchronize(&mut self) {
        // Skip until we find a delimiter or EOF
        while !self.is_eof() && !self.is_at_delimiter() {
            self.advance();
        }

        // Now skip all the delimiters to get to the next statement
        self.skip_delimiters();
    }

    pub(crate) fn current_token(&self) -> &TokenInfo {
        self.tokens
            .get(self.current)
            .unwrap_or_else(|| self.tokens.last().expect("Token vector is empty!"))
    }

    fn is_eof(&self) -> bool {
        self.current_token().token == Token::EndOfFile || self.current >= self.tokens.len()
    }

    pub fn try_parse_expression(&mut self, precedence: u8) -> Option<Expression> {
        self.skip_newlines_in_delimiters();

        let token_type = self.current_token().token.get_token_type();

        // Get prefix parser function
        let prefix_fn = match self.nud_parse_fns.get(&token_type) {
            Some(f) => *f,
            None => {
                self.errors.add(HydorError::UnexpectedToken {
                    token: token_type,
                    span: self.current_token().span,
                });
                return None;
            }
        };

        let mut left = prefix_fn(self)?;

        // Parse infix expressions
        while !self.is_eof() {
            self.skip_newlines_in_delimiters();

            let token_type = self.current_token().token.get_token_type();
            let next_prec =
                Precedence::get_token_precedence(&token_type).unwrap_or(Precedence::Default);
            let next_prec_value = next_prec.into();

            if precedence >= next_prec_value {
                break;
            }

            let infix_fn = match self.led_parse_fns.get(&token_type) {
                Some(f) => *f,
                None => break,
            };

            left = infix_fn(self, left)?;
        }

        Some(left)
    }

    fn try_parse_statement(&mut self) -> Option<Statement> {
        let stmt_type = self.current_token().token.get_token_type();

        // Try to parse as a statement keyword
        if let Some(stmt_fn) = self.stmt_parse_fns.get(&stmt_type) {
            return stmt_fn(self);
        }

        // Otherwise, treat as expression statement
        let start = self.current_token().clone();

        let expr = match self.try_parse_expression(Precedence::Default.into()) {
            Some(e) => e,
            None => return None,
        };

        // Expression statements require a delimiter
        if !self.expect_delimiter() {
            return None;
        }

        Some(Spanned {
            node: Stmt::Expression { expression: expr },
            span: start.span,
        })
    }
}

// ------------------- EXPRESSIONS -------------------

impl Parser {
    // ------------------- Null Denoted Expressions -------------------
    pub fn parse_integer_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::Integer(n) => n,
            _ => unreachable!(),
        };

        let expr = Expr::IntegerLiteral(value).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_float_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::Float(n) => n,
            _ => unreachable!(),
        };

        let expr = Expr::FloatLiteral(value).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_bool_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let value = match token_info.token {
            Token::True => true,
            Token::False => false,
            _ => unreachable!(),
        };

        let expr = Expr::BooleanLiteral(value).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_identifier_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let ident = match token_info.token.clone() {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        };

        let expr = Expr::Identifier(ident).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_string_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let ident = match token_info.token.clone() {
            Token::String(name) => name,
            _ => unreachable!(),
        };

        let expr = Expr::StringLiteral(ident).spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_nil_literal(&mut self) -> Option<Expression> {
        let token_info = self.current_token();
        let expr = Expr::NilLiteral.spanned(token_info.span);

        self.advance();
        Some(expr)
    }

    pub fn parse_unary_expr(&mut self) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        self.advance(); // Eat operator

        let value = self.try_parse_expression(Precedence::Unary.into())?;
        let val_span = value.span;

        let expr = Expr::Unary {
            operator: operator_info.token,
            right: Box::new(value),
        }
        .spanned(Span {
            line: operator_info.span.line,
            start_column: operator_info.span.start_column,
            end_column: val_span.end_column,
        });

        Some(expr)
    }

    pub fn parse_grouping_expr(&mut self) -> Option<Expression> {
        let left_paren_span = self.current_token().span;
        self.advance(); // Eat '('
        self.delimiter_stack.push(TokenType::LeftParenthesis);

        // Skip newlines after (
        self.skip_newlines_in_delimiters();

        let expr = match self.try_parse_expression(Precedence::Default.into()) {
            Some(e) => e,
            None => {
                self.delimiter_stack.pop();
                return None;
            }
        };

        self.skip_newlines_in_delimiters();

        self.delimiter_stack.pop(); // Remove (

        if !self.expect(TokenType::RightParenthesis) {
            return None;
        }

        let right_paren_span = self
            .tokens
            .get(self.current - 1)
            .map(|t| t.span)
            .unwrap_or(expr.span);

        // Return the expression with updated span to include parentheses
        Some(Spanned {
            node: expr.node,
            span: Span {
                line: left_paren_span.line,
                start_column: left_paren_span.start_column,
                end_column: right_paren_span.end_column,
            },
        })
    }

    // ------------------- Left Denoted Expressions -------------------
    pub fn parse_binary_expr(&mut self, left: Expression) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        let operator_precedence =
            match Precedence::get_token_precedence(&operator_info.token.get_token_type()) {
                Some(p) => p,
                _ => Precedence::Default,
            };

        self.advance(); // Eat operator

        let right = self.try_parse_expression(operator_precedence.into())?;

        let full_span = Span {
            line: left.span.line,
            start_column: left.span.start_column,
            end_column: right.span.end_column,
        };

        let expr = Expr::BinaryOperation {
            left: Box::new(left),
            operator: operator_info.token,
            right: Box::new(right),
        }
        .spanned(full_span);

        Some(expr)
    }

    pub fn parse_exponent_expr(&mut self, left: Expression) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        let operator_precedence: u8 =
            match Precedence::get_token_precedence(&operator_info.token.get_token_type()) {
                Some(p) => p,
                _ => Precedence::Default,
            }
            .into();

        self.advance(); // Eat operator

        // Parse right-associative
        let right = self.try_parse_expression(operator_precedence - 1)?;

        let full_span = Span {
            line: left.span.line,
            start_column: left.span.start_column,
            end_column: right.span.end_column,
        };

        let expr = Expr::BinaryOperation {
            left: Box::new(left),
            operator: operator_info.token,
            right: Box::new(right),
        }
        .spanned(full_span);

        Some(expr)
    }
}

// Statements
impl Parser {
    pub fn parse_variable_decl(&mut self) -> Option<Statement> {
        let let_tok = self.current_token().clone();
        self.advance();

        // No synchronize calls needed anywhere!
        if self.current_token().token.get_token_type() != TokenType::Identifier {
            self.errors.add(HydorError::ExpectedToken {
                expected: TokenType::Identifier,
                got: self.current_token().token.get_token_type(),
                span: self.current_token().span,
            });
            return None;
        }
        let ident = self.parse_identifier_literal()?;

        if !self.expect(TokenType::Colon) {
            return None;
        }

        let an_type = self.try_parse_type()?;
        if !self.expect(TokenType::Assign) {
            return None;
        }

        let value = self.try_parse_expression(Precedence::Default.into())?;
        if !self.expect_delimiter() {
            return None;
        }

        let val_span = value.span.clone();

        Some(
            Stmt::VariableDeclaration {
                identifier: ident,
                value,
                annotated_type: an_type,
                span: Span {
                    line: let_tok.span.line,
                    start_column: let_tok.span.start_column,
                    end_column: val_span.end_column,
                },
            }
            .spanned(let_tok.span),
        )
    }
}
