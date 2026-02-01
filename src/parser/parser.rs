use std::collections::HashMap;

use crate::{
    ast::ast::{Expr, Expression, Program, Statement, Stmt},
    error_handler::{error_collector::ErrorCollector, errors::VynError},
    parser::{lookups::Precedence, type_parser::TypeTable},
    tokens::{Token, TokenInfo, TokenType},
    type_checker::type_checker::TypeChecker,
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

    // Where type aliases are stored
    pub type_table: TypeTable,

    pub errors: ErrorCollector,
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        let mut parser = Self {
            tokens,
            current: 0,
            errors: ErrorCollector::new(),
            delimiter_stack: Vec::new(),

            type_table: TypeTable::new(),

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
        parser.register_nud(TokenType::LeftBracket, Parser::parse_array_literal);

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
        parser.register_led(TokenType::Assign, Parser::parse_assignment_expr);
        parser.register_led(TokenType::BoxColon, Parser::parse_index_expr);

        parser.register_stmt(TokenType::Let, Parser::parse_variable_decl);
        parser.register_stmt(TokenType::Static, Parser::parse_static_variable_decl);
        parser.register_stmt(TokenType::Type, Parser::parse_type_alias_decl);
        parser.register_stmt(TokenType::Stdout, Parser::parse_stdout_log_decl);
        parser.register_stmt(TokenType::If, Parser::parse_if_stmt_decl);
        parser.register_stmt(TokenType::Loop, Parser::parse_loop_stmt_decl);
        parser.register_stmt(TokenType::Break, Parser::parse_loop_interrupt_stmt);
        parser.register_stmt(TokenType::Continue, Parser::parse_loop_interrupt_stmt);
        parser.register_stmt(TokenType::For, Parser::parse_for_loop_stmt);

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

    fn current_token_is(&self, token: TokenType) -> bool {
        self.current_token().token.get_token_type() == token
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
            self.errors.add(VynError::ExpectedToken {
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
                self.errors.add(VynError::ExpectedToken {
                    expected: TokenType::Semicolon,
                    got: current,
                    span: self.current_token().span,
                });
                false
            }
        }
    }

    fn ignore(&mut self, token_type: TokenType) {
        if self.current_token_type() == token_type {
            self.advance();
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

    pub(crate) fn current_token_type(&self) -> TokenType {
        self.current_token().token.get_token_type()
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
                self.errors.add(VynError::UnexpectedToken {
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

    pub fn parse_array_literal(&mut self) -> Option<Expression> {
        let lb_token_info = self.current_token().clone();

        self.advance();

        let mut elements: Vec<Box<Expression>> = Vec::new();

        while self.current_token_type() != TokenType::RightBracket {
            let e = self.try_parse_expression(Precedence::Default.into())?;
            elements.push(Box::new(e));

            if self.current_token_type() == TokenType::Comma {
                self.advance();
                self.skip_newlines_in_delimiters();
            }
        }

        let rb_token_info = self.current_token().clone();
        self.advance();

        let full_span = Span {
            line: lb_token_info.span.line,
            start_column: lb_token_info.span.start_column,
            end_column: rb_token_info.span.end_column,
        };

        let expr = Expr::ArrayLiteral { elements }.spanned(full_span);
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

    pub fn parse_assignment_expr(&mut self, left: Expression) -> Option<Expression> {
        let operator_info = self.current_token().clone();
        let operator_precedence: u8 =
            match Precedence::get_token_precedence(&operator_info.token.get_token_type()) {
                Some(p) => p,
                _ => Precedence::Default,
            }
            .into();

        self.advance();

        let right = self.try_parse_expression(operator_precedence - 1)?;

        let full_span = Span {
            line: operator_info.span.line,
            start_column: left.span.start_column,
            end_column: right.span.end_column,
        };

        match left.node {
            Expr::Index { target, property } => {
                let expr = Expr::IndexAssignment {
                    target,
                    property,
                    new_value: Box::new(right),
                }
                .spanned(full_span);
                return Some(expr);
            }
            _ => {
                let expr = Expr::VariableAssignment {
                    identifier: Box::new(left),
                    new_value: Box::new(right),
                }
                .spanned(full_span);
                return Some(expr);
            }
        }
    }

    pub fn parse_index_expr(&mut self, left: Expression) -> Option<Expression> {
        let bc_token_info = self.current_token().clone();
        let bc_precedence: u8 =
            Precedence::get_token_precedence(&bc_token_info.token.get_token_type())?.into();
        self.advance();

        // Parse right associatively
        let right = self.try_parse_expression(bc_precedence)?;
        let right_span = right.span;
        let left_span = left.span;

        let full_span = Span {
            line: bc_token_info.span.line,
            start_column: left_span.start_column,
            end_column: right_span.end_column,
        };

        let expr = Expr::Index {
            target: Box::new(left),
            property: Box::new(right),
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

        let mut mutable = false;
        if self.current_token_is(TokenType::At) {
            self.advance();
            mutable = true;
        }

        if self.current_token().token.get_token_type() != TokenType::Identifier {
            self.errors.add(VynError::ExpectedToken {
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

        let mut full_span = Span {
            line: let_tok.span.line,
            start_column: let_tok.span.start_column,
            end_column: self.current_token().span.end_column,
        };

        if self.current_token_type() != TokenType::Assign {
            if self.current_token_type().is_delimiter() {
                return Some(
                    Stmt::VariableDeclaration {
                        identifier: ident,
                        value: None,
                        annotated_type: an_type,
                        mutable,
                    }
                    .spanned(full_span),
                );
            }

            self.errors.add(VynError::ExpectedToken {
                expected: TokenType::Assign,
                got: self.current_token_type(),
                span: full_span,
            });

            return None;
        }

        let value = self.try_parse_expression(Precedence::Default.into())?;

        if !self.expect_delimiter() {
            return None;
        }

        let val_span = value.span.clone();
        full_span.end_column = val_span.end_column;

        Some(
            Stmt::VariableDeclaration {
                identifier: ident,
                value: Some(value),
                annotated_type: an_type,
                mutable,
            }
            .spanned(full_span),
        )
    }

    pub fn parse_static_variable_decl(&mut self) -> Option<Statement> {
        let static_tok_info = self.current_token().clone();
        self.advance();

        if self.current_token().token.get_token_type() != TokenType::Identifier {
            self.errors.add(VynError::ExpectedToken {
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

        let full_span = Span {
            line: static_tok_info.span.line,
            start_column: static_tok_info.span.start_column,
            end_column: value.span.end_column,
        };

        let stmt = Stmt::StaticVariableDeclaration {
            identifier: ident,
            value,
            annotated_type: an_type,
        }
        .spanned(full_span);

        Some(stmt)
    }

    pub fn parse_type_alias_decl(&mut self) -> Option<Statement> {
        self.advance(); // Eat Type Token

        if self.current_token().token.get_token_type() != TokenType::Identifier {
            self.errors.add(VynError::ExpectedToken {
                expected: TokenType::Identifier,
                got: self.current_token().token.get_token_type(),
                span: self.current_token().span,
            });
            return None;
        }

        let ident = self.parse_identifier_literal()?;

        if !self.expect(TokenType::Assign) {
            return None;
        }

        let type_alias = self.try_parse_type()?;

        if !self.expect_delimiter() {
            return None;
        }

        let ident_span = ident.span;
        self.enroll_type_alias(ident.clone(), type_alias.clone());

        let stmt = Stmt::TypeAliasDeclaration {
            identifier: ident,
            value: type_alias,
        }
        .spanned(ident_span);

        Some(stmt)
    }

    pub fn parse_stdout_log_decl(&mut self) -> Option<Statement> {
        let stdout_tok_info = self.current_token().clone();
        self.advance();

        if !self.expect(TokenType::Hashtag) {
            return None;
        }

        let log_value = self.try_parse_expression(Precedence::Default.into())?;

        if !self.expect_delimiter() {
            return None;
        }

        let full_span = Span {
            line: stdout_tok_info.span.line,
            start_column: stdout_tok_info.span.start_column,
            end_column: log_value.span.end_column,
        };

        let stmt = Stmt::StdoutLog { log_value }.spanned(full_span);

        Some(stmt)
    }

    fn parse_block_stmt(&mut self) -> Option<Statement> {
        let lb_tok_info = self.current_token().clone();

        if !self.expect(TokenType::LeftBrace) {
            return None;
        }

        self.skip_delimiters();

        let mut statements: Vec<Statement> = Vec::new();

        while self.current_token_type() != TokenType::RightBrace {
            self.skip_delimiters();

            if self.current_token_type() == TokenType::RightBrace {
                break;
            }

            statements.push(self.try_parse_statement()?);
        }

        if !self.expect(TokenType::RightBrace) {
            return None;
        }

        Some(Stmt::Block { statements }.spanned(lb_tok_info.span))
    }

    fn parse_scope_stmt(&mut self) -> Option<Statement> {
        let lb_tok_info = self.current_token().clone();

        if !self.expect(TokenType::LeftBrace) {
            return None;
        }

        self.skip_delimiters();

        let mut statements: Vec<Statement> = Vec::new();

        while self.current_token_type() != TokenType::RightBrace {
            self.skip_delimiters();

            if self.current_token_type() == TokenType::RightBrace {
                break;
            }

            statements.push(self.try_parse_statement()?);
        }

        if !self.expect(TokenType::RightBrace) {
            return None;
        }

        Some(Stmt::Scope { statements }.spanned(lb_tok_info.span))
    }

    pub fn parse_if_stmt_decl(&mut self) -> Option<Statement> {
        let if_tok_info = self.current_token().clone();
        self.advance();

        let condition = self.try_parse_expression(Precedence::Default.into())?;
        let consequence = self.parse_scope_stmt()?;
        let mut alternate: Option<Statement> = None;

        if self.current_token_type() == TokenType::Else {
            self.advance(); // Eat else token
            alternate = self.parse_scope_stmt();
        }

        let stmt = Stmt::IfDeclaration {
            condition,
            consequence: Box::new(consequence),
            alternate: Box::new(alternate),
        };

        Some(stmt.spanned(if_tok_info.span))
    }

    pub fn parse_loop_stmt_decl(&mut self) -> Option<Statement> {
        let loop_tok_info = self.current_token().clone();
        self.advance();

        let scope_block = self.parse_scope_stmt()?;

        let stmt = Stmt::Loop {
            body: Box::new(scope_block),
        }
        .spanned(loop_tok_info.span);

        Some(stmt)
    }

    pub fn parse_loop_interrupt_stmt(&mut self) -> Option<Statement> {
        let span = self.current_token().span;

        let stmt = match self.current_token_type() {
            TokenType::Continue => Stmt::Continue,
            TokenType::Break => Stmt::Break,
            unknown => unreachable!("{}", unknown),
        };

        self.advance();

        if !self.expect_delimiter() {
            return None;
        }

        Some(stmt.spanned(span))
    }

    pub fn parse_for_loop_stmt(&mut self) -> Option<Statement> {
        let for_tok_info = self.current_token().clone();
        self.advance();

        match self.current_token_type() {
            TokenType::When => {
                self.advance();

                let condition = self.try_parse_expression(Precedence::Default.into())?;
                let body = self.parse_scope_stmt()?;

                let stmt = Stmt::WhenLoop {
                    body: Box::new(body),
                    condition,
                }
                .spanned(for_tok_info.span);

                return Some(stmt);
            }

            TokenType::Every => {
                self.advance();

                let iterator = self.parse_identifier_literal()?;

                if !self.expect(TokenType::In) {
                    return None;
                }

                let range = self.try_parse_expression(Precedence::Default.into())?;
                let body = self.parse_scope_stmt()?;

                let stmt = Stmt::IndexLoop {
                    body: Box::new(body),
                    iterator,
                    range,
                }
                .spanned(for_tok_info.span);

                return Some(stmt);
            }

            _ => {
                self.advance();

                self.errors.add(VynError::UnexpectedToken {
                    token: self.current_token_type(),
                    span: self.current_token().span,
                });
                return None;
            }
        }
    }
}
