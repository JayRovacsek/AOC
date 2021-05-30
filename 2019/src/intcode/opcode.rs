#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
    RelativeAdjust,
}