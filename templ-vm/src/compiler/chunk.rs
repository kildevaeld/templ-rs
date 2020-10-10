use super::error::CompileError;
use super::opcode::OpCode;
use std::convert::TryFrom;
use std::fmt;
use templ_runtime::Value;

macro_rules! byte_instruction {
    ($name:expr, $chunk:expr, $offset: expr, $fmt:expr) => {{
        let code = $chunk.get_code($offset + 1)?;
        writeln!($fmt, "{:24}{:4}", $name, code as u8)?;
        $offset + 2
    }};
}

macro_rules! constant_instruction {
    ($name:expr, $chunk:expr, $offset: expr, $fmt:expr) => {{
        let code = $chunk.get_code($offset + 1)? as u8;
        let constant = $chunk.get_constant(code as usize).expect("constant");
        writeln!($fmt, "{:24}{:4} '{:?}'", $name, code, constant)?;
        $offset + 2
    }};
}

#[allow(unused_macros)]
macro_rules! constant_instruction_n {
    ($name:expr, $chunk:expr, $offset: expr, $n:expr, $fmt:expr) => {{
        let code = $chunk.get_code($offset + 1)? as u8;
        let constant = $chunk.get_constant(code as usize).expect("constant n");
        writeln!(
            $fmt,
            "{:24}{:4} '{:?}'",
            format!("{}_{}", $name, $n),
            code,
            constant
        )?;
        $offset + 2
    }};
}

macro_rules! initializer {
    ($name: expr, $chunk: expr, $offset: expr, $fmt: expr) => {{
        let mut jump = ($chunk.code[$offset + 1] as i16) << 8;
        jump |= $chunk.code[$offset + 2] as i16;
        writeln!($fmt, "{:24}{:4}", $name, jump)?;
        $offset + 3
    }};
}

macro_rules! jump_instruction {
    ($name:expr, $chunk:expr,$offset: expr, $sign:expr, $fmt:expr) => {{
        let mut jump = ($chunk.code[$offset + 1] as i16) << 8;
        jump |= $chunk.code[$offset + 2] as i16;

        writeln!(
            $fmt,
            "{:24}{:4} -> {}",
            $name,
            $offset,
            $offset + 3 + $sign * (jump as usize)
        )?;
        $offset + 3
    }};
}

macro_rules! jump_instruction_neg {
    ($name:expr, $chunk:expr,$offset: expr, $sign:expr, $fmt:expr) => {{
        let mut jump = ($chunk.code[$offset + 1] as i16) << 8;
        jump |= $chunk.code[$offset + 2] as i16;

        writeln!(
            $fmt,
            "{:24}{:4} -> {}",
            $name,
            $offset,
            ($offset as isize) + 3 + -1 * (jump as isize)
        )?;
        $offset + 3
    }};
}

macro_rules! simple_instruction {
    ($name:expr, $offset: expr, $fmt:expr) => {{
        writeln!($fmt, "{}", $name)?;
        $offset + 1
    }};
}

macro_rules! simple_instruction_n {
    ($name:expr, $offset: expr, $n: expr, $fmt:expr) => {{
        writeln!($fmt, "{}_{}", $name, $n)?;
        $offset + 1
    }};
}

pub type ChunkIndex = usize;

#[derive(Debug, PartialEq, Default)]
pub struct ChunkBuilder {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl ChunkBuilder {
    pub fn new() -> ChunkBuilder {
        ChunkBuilder {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn build(self) -> Chunk {
        Chunk {
            code: self.code,
            constants: self.constants,
        }
    }

    pub fn add_constant(&mut self, value: Value) -> ChunkIndex {
        for kv in self.constants.iter().enumerate() {
            if kv.1 == &value {
                return kv.0;
            }
        }
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn make_constant(&mut self, value: Value) -> ChunkIndex {
        let constant = self.add_constant(value);
        constant
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant, constant as u8)
    }

    pub fn emit<BS: Into<u8>>(&mut self, code: BS) {
        self.code.push(code.into());
    }

    pub fn emit16<BS: Into<u16>>(&mut self, code: BS) {
        let code = code.into();
        self.emit(((code >> 8) & 0xff) as u8);
        self.emit((code & 0xff) as u8);
    }

    pub fn emit_bytes<BS: Into<u8>, BS2: Into<u8>>(&mut self, code: BS, byte: BS2) {
        self.emit(code);
        self.emit(byte);
    }

    pub fn emit_jump<BS: Into<u8>>(&mut self, code: BS) -> usize {
        self.emit(code);
        self.emit(0xff);
        self.emit(0xff);
        self.len() - 2
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = (self.len() - offset - 2) as u16;

        self.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.code[offset + 1] = (jump & 0xff) as u8;
    }

    pub fn patch_break(&mut self, offset: usize) {
        let jump = (self.len() - offset - 1) as u16;

        self.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.code[offset + 1] = (jump & 0xff) as u8;
    }

    pub fn patch_loop(&mut self, offset: usize, start: usize) {
        let jump = offset - start + 2;
        //println!("jump {} {} {}", offset, jump, start);
        self.code[offset] = ((jump >> 8) & 0xff) as u8;
        self.code[offset + 1] = (jump & 0xff) as u8;
    }

    pub fn emit_loop(&mut self, start: usize) {
        self.emit(OpCode::Loop);
        let offset = self.len() - start + 2;
        self.emit(((offset >> 8) & 0xff) as u8);
        self.emit((offset & 0xff) as u8);
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.code.len()
    }

    #[inline(always)]
    pub fn get_code(&self, offset: usize) -> Result<OpCode, CompileError> {
        Ok(OpCode::try_from(self.code[offset])?)
    }

    #[inline(always)]
    pub fn get(&self, offset: usize) -> u8 {
        self.code[offset]
    }
}

pub struct Chunk {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl Chunk {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.code.len()
    }

    #[inline(always)]
    pub fn get_code(&self, offset: usize) -> Result<OpCode, CompileError> {
        Ok(OpCode::try_from(self.code[offset])?)
    }

    #[inline(always)]
    pub fn get_constant(&self, constant: ChunkIndex) -> Option<&Value> {
        self.constants.get(constant)
    }

    #[inline(always)]
    pub fn constant(&self, constant: ChunkIndex) -> &Value {
        &self.constants[constant]
    }

    #[inline(always)]
    pub fn get(&self, offset: usize) -> u8 {
        self.code[offset]
    }

    pub fn dissamble(&self, nested: bool) -> String {
        self.do_dissamble(nested, 0)
    }

    fn do_dissamble(&self, nested: bool, indent: i32) -> String {
        let mut i = 0;
        let mut out = String::new();

        while i < self.code.len() {
            i = self
                .disamble_offset(i, &mut out, indent, nested)
                .expect("disamble offset");
        }
        out
    }

    fn disamble_offset(
        &self,
        offset: usize,
        f: &mut dyn std::fmt::Write,
        oindent: i32,
        nested: bool,
    ) -> Result<usize, CompileError> {
        let indent = (0..oindent).map(|_| " ").collect::<Vec<_>>().join("");
        write!(f, "{}", indent)?;
        write!(f, "{:04}   | ", offset)?;

        let opcode = OpCode::try_from(self.code[offset])?;
        let m = match opcode {
            OpCode::Unused => simple_instruction!(opcode, offset, f),
            OpCode::Constant => constant_instruction!(opcode, self, offset, f),
            // OpCode::Nil => simple_instruction!(opcode, offset, f),
            OpCode::True => simple_instruction!(opcode, offset, f),
            OpCode::False => simple_instruction!(opcode, offset, f),
            OpCode::Pop => simple_instruction!(opcode, offset, f),
            OpCode::GetLocal => byte_instruction!(opcode, self, offset, f),
            OpCode::GetGlobal => constant_instruction!(opcode, self, offset, f),
            // OpCode::DefineGlobal => constant_instruction!(opcode, self, offset, f),
            // OpCode::SetLocal => byte_instruction!(opcode, self, offset, f),
            // OpCode::SetGlobal => constant_instruction!(opcode, self, offset, f),
            OpCode::Equal => simple_instruction!(opcode, offset, f),
            // OpCode::InstanceOf => simple_instruction!("OP_INSTANCEOF", offset, f),
            OpCode::Greater => simple_instruction!(opcode, offset, f),
            OpCode::Less => simple_instruction!(opcode, offset, f),
            OpCode::Add => simple_instruction!(opcode, offset, f),
            OpCode::Substract => simple_instruction!(opcode, offset, f),
            OpCode::Multiply => simple_instruction!(opcode, offset, f),
            OpCode::Divide => simple_instruction!(opcode, offset, f),
            OpCode::Modulo => simple_instruction!(opcode, offset, f),
            OpCode::Not => simple_instruction!(opcode, offset, f),
            // OpCode::Negate => simple_instruction!("OP_NEGATE",  offset, f),
            // OpCode::Return => simple_instruction!(opcode, offset, f),
            OpCode::Jump => jump_instruction!(opcode, self, offset, 1, f),
            OpCode::Loop => jump_instruction_neg!(opcode, self, offset, 1, f),
            OpCode::JumpIfFalse => jump_instruction!(opcode, self, offset, 1, f),
            // OpCode::Array => byte_instruction!("OP_ARRAY",  self, offset, f),
            // OpCode::Map => byte_instruction!("OP_MAP",  self, offset, f),
            OpCode::GetProperty => constant_instruction!(opcode, self, offset, f),
            // OpCode::SetProperty => constant_instruction!(opcode, self, offset, f),
            // OpCode::SetIndex => simple_instruction!(opcode, offset, f),
            OpCode::GetIndex => simple_instruction!(opcode, offset, f),
            // OpCode::Method => constant_instruction!("OP_METHOD",  self, offset, f),
            // OpCode::CloseUpValue => simple_instruction!(opcode, offset, f),
            OpCode::Slice => initializer!(opcode, self, offset, f),
            OpCode::Call0 | OpCode::Call1 | OpCode::Call2 | OpCode::Call3 => simple_instruction_n!(
                "OP_CALL",
                offset,
                ((opcode as u8) - (OpCode::Call0 as u8)),
                f
            ),
            OpCode::CallN => byte_instruction!("OP_CALL", self, offset, f),
            OpCode::Render0 | OpCode::Render1 | OpCode::Render2 | OpCode::Render3 => {
                simple_instruction_n!(
                    "OP_RENDER",
                    offset,
                    ((opcode as u8) - (OpCode::Render0 as u8)),
                    f
                )
            }
            OpCode::RenderN => byte_instruction!("OP_RENDER", self, offset, f),
        };

        Ok(m)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        while i < self.code.len() {
            i = self
                .disamble_offset(i, f, 0, false)
                .expect("disamble offset");
        }
        Ok(())
    }
}
