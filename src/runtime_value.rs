#[derive(Debug)]
pub enum RuntimeValue {
    IntegerLiteral(i32),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(String),
}
