use super::compiler::{chunk::Chunk, Constant, OpCode};
use super::template::Template;
use id_arena::{Arena, Id};
use smallvec::SmallVec;
use std::convert::TryFrom;
use std::fmt::Write;
use templ_runtime::{Renderable, Value};

macro_rules! op_cmp {
    ($state: expr, $method: ident, $templates: expr) => {{
        let right = $state.pop_value($templates).expect("right");
        let left = *$state.peek_value(0).expect("left");
        let top = $state.top() - 1;
        $state[top] = Value::Boolean(left.$method(&right));
    }};
}

macro_rules! op_bin {
    ($state: expr,  $mc: expr, $method: ident) => {{
        let right = $state.pop_value().expect("right");
        let left = *$state.peek_value(0).expect("left");
        let top = $state.top() - 1;
        $state[top] = left.$method($mc, &right);
    }};
}

struct VMState {
    ip: usize,
}

impl VMState {
    #[inline(always)]
    pub fn next(&mut self, template: &Template) -> Option<OpCode> {
        if self.ip >= template.chunk.len() {
            None
        } else {
            match OpCode::try_from(self.read_byte(template)) {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        }
    }

    #[inline(always)]
    pub fn read_byte(&mut self, template: &Template) -> u8 {
        let b = template.chunk.get(self.ip);
        self.ip += 1;
        b
    }

    #[inline(always)]
    pub fn read_short(&mut self, template: &Template) -> u16 {
        let mut jump = (template.chunk.get(self.ip) as u16) << 8;
        jump |= template.chunk.get(self.ip + 1) as u16;
        self.ip += 2;
        jump
    }

    #[inline(always)]
    pub fn read_constant<'a>(&mut self, template: &'a Template) -> Option<&'a Value> {
        let b = self.read_byte(template);
        template.chunk.get_constant(b as usize)
    }
}

pub fn run_vm<'a>(template: &'a Template, args: Vec<Value>) -> String {
    let mut arena: Arena<VMValue<'a>> = Arena::default();
    let mut stack = args
        .into_iter()
        .take(template.params.len())
        .map(|m| arena.alloc(VMValue::Value(m)))
        .collect::<SmallVec<[Id<VMValue>; 24]>>();
    let mut state = VMState { ip: 0 };

    let mut output = String::new();

    loop {
        let op = match state.next(template) {
            Some(op) => op,
            None => return output,
        };

        match op {
            OpCode::GetLocal => {
                let idx = state.read_byte(template) as usize;
                let value = stack[idx];
                stack.push(value);
            }
            OpCode::Render0 | OpCode::Render1 | OpCode::Render2 | OpCode::Render3 => {
                let count = (op as u8) - (OpCode::Render0 as u8);
                if count == 0 {
                    let value = stack.pop().expect("pop");
                    let value = &arena[value];
                    value.render(&mut output, &[]).unwrap();
                } else {
                    let idx = stack.len() - 1 - count as usize;
                    panic!("not !! {} {}", count, op);
                }
            }
            OpCode::Constant => {
                let constant = state.read_constant(template).unwrap();
                let idx = arena.alloc(VMValue::ValueRef(constant));
                stack.push(idx);
            }
            OpCode::Add => {
                let right = &arena[stack.pop().unwrap()];
                let left = &arena[*stack.last().unwrap()];

                let v = right.op_add(left);
                *stack.last_mut().unwrap() = arena.alloc(VMValue::Value(v));
            }
            OpCode::Equal => {
                let right = &arena[stack.pop().unwrap()];
                let left = &arena[*stack.last().unwrap()];

                let v = right.op_eq(left);
                *stack.last_mut().unwrap() = arena.alloc(VMValue::Value(Value::Bool(v)));
            }
            OpCode::JumpIfFalse => {
                let offset = state.read_short(&template);
                let v = &arena[*stack.last().unwrap()];
                if !v.as_boolean().expect("boolean") {
                    state.ip += offset as usize;
                }
            }
            OpCode::Jump => {
                let offset = state.read_short(&template);
                state.ip += offset as usize;
            }
            OpCode::Pop => {
                stack.pop();
            }
            OpCode::Greater => {
                let right = &arena[stack.pop().unwrap()];
                let left = &arena[*stack.last().unwrap()];

                let v = right.op_gt(left);
                *stack.last_mut().unwrap() = arena.alloc(VMValue::Value(Value::Bool(v)));
            }
            _ => {
                unimplemented!("opcode {}", op);
            }
        }
    }
}

pub(crate) enum VMValue<'a> {
    Value(Value),
    ValueRef(&'a Value),
}

impl<'a> std::ops::Deref for VMValue<'a> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        match self {
            VMValue::ValueRef(r) => r,
            VMValue::Value(v) => &v,
        }
    }
}
