use std::convert::TryFrom;
use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum OpCode {
    Constant,
    Loop,
    Pop,
    GetGlobal,
    GetLocal,
    JumpIfFalse,
    Jump,
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    Greater,
    Less,
    Not,
    GetProperty,
    GetIndex,
    True,
    False,
    Call0,
    Call1,
    Call2,
    Call3,
    CallN,
    Render0,
    Render1,
    Render2,
    Render3,
    RenderN,
    Slice,
    Map,
    Unused,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant => write!(f, "OP_CONSTANT"),
            OpCode::Loop => write!(f, "OP_LOOP"),
            OpCode::Pop => write!(f, "OP_POP"),
            OpCode::GetGlobal => write!(f, "OP_GETGLOBAL"),
            OpCode::GetLocal => write!(f, "OP_GETLOCAL"),
            OpCode::JumpIfFalse => write!(f, "OP_JUMPIFFALSE"),
            OpCode::Jump => write!(f, "OP_JUMP"),
            OpCode::Add => write!(f, "OP_ADD"),
            OpCode::Substract => write!(f, "OP_SUBSTRACT"),
            OpCode::Multiply => write!(f, "OP_MULTIPLY"),
            OpCode::Divide => write!(f, "OP_DIVIDE"),
            OpCode::Modulo => write!(f, "OP_MODULO"),
            OpCode::Equal => write!(f, "OP_EQUAL"),
            OpCode::Greater => write!(f, "OP_GREATER"),
            OpCode::Less => write!(f, "OP_LESS"),
            OpCode::Not => write!(f, "OP_NOT"),
            OpCode::GetProperty => write!(f, "OP_GETPROPERTY"),
            OpCode::GetIndex => write!(f, "OP_GETINDEX"),
            OpCode::True => write!(f, "OP_TRUE"),
            OpCode::False => write!(f, "OP_FALSE"),
            OpCode::Call0 => write!(f, "OP_CALL0"),
            OpCode::Call1 => write!(f, "OP_CALL1"),
            OpCode::Call2 => write!(f, "OP_CALL2"),
            OpCode::Call3 => write!(f, "OP_CALL3"),
            OpCode::CallN => write!(f, "OP_CALLN"),
            OpCode::Render0 => write!(f, "OP_RENDER0"),
            OpCode::Render1 => write!(f, "OP_RENDER1"),
            OpCode::Render2 => write!(f, "OP_RENDER2"),
            OpCode::Render3 => write!(f, "OP_RENDER3"),
            OpCode::RenderN => write!(f, "OP_RENDERN"),
            OpCode::Slice => write!(f, "OP_SLICE"),
            OpCode::Map => write!(f, "OP_MAP"),
            OpCode::Unused => write!(f, "OP_UNUSED"),
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = InvalidOpCodeErr;

    fn try_from(i: u8) -> Result<OpCode, Self::Error> {
        if i <= (OpCode::Unused as u8) {
            unsafe { Ok(std::mem::transmute::<_, OpCode>(i)) }
        } else {
            Err(InvalidOpCodeErr(i))
        }
    }
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> u8 {
        op as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidOpCodeErr(pub u8);

impl fmt::Display for InvalidOpCodeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "invalid opcode: {}", self.0)
    }
}

impl std::error::Error for InvalidOpCodeErr {}
