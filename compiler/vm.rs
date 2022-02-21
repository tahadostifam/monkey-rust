use std::borrow::Borrow;
use std::rc::Rc;

use byteorder::{BigEndian, ByteOrder};

use object::Object;
use object::Object::Boolean;

use crate::compiler::Bytecode;
use crate::op_code::{cast_u8_to_opcode, Instructions, Opcode};

const STACK_SIZE: usize = 2048;

pub struct VM {
    constants: Vec<Rc<Object>>,
    instructions: Instructions,

    stack: Vec<Rc<Object>>,
    sp: usize, // stack pointer. Always point to the next value. Top of the stack is stack[sp -1]
}

impl VM {
    pub fn new(bytecode: Bytecode) -> VM {
        return VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: vec![Rc::new(Object::Null); STACK_SIZE],
            sp: 0,
        };
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        let ins = self.instructions.data.clone();
        while ip < ins.len() {
            let op: u8 = *ins.get(ip).unwrap();
            let opcode = cast_u8_to_opcode(op);
            match opcode {
                Opcode::OpConst => {
                    let const_index = BigEndian::read_u16(&ins[ip + 1..ip + 3]) as usize;
                    ip += 2;
                    self.push(Rc::clone(&self.constants[const_index]))
                }
                Opcode::OpAdd | Opcode::OpSub| Opcode::OpMul| Opcode::OpDiv => {
                    self.executeBinaryOperation(opcode);
                }
                Opcode::OpPop => {
                    self.pop();
                }
                Opcode::OpTrue => {
                    self.push(Rc::new(Boolean(true)));
                }
                Opcode::OpFalse => {
                    self.push(Rc::new(Boolean(false)));
                }
            }
            ip += 1;
        }
    }

    fn executeBinaryOperation(&mut self, opcode: Opcode) {
        let right = self.pop();
        let left = self.pop();
        match (left.borrow(), right.borrow()) {
            (Object::Integer(l), Object::Integer(r)) => {
                let result = match opcode {
                    Opcode::OpAdd => l + r,
                    Opcode::OpSub => l - r,
                    Opcode::OpMul => l * r,
                    Opcode::OpDiv => l / r,
                    _ => panic!("Unknown opcode for int"),
                };
                self.push(Rc::from(Object::Integer(result)));
            }
            _ => {
                panic!("unsupported add for those types")
            }
        }
    }


    pub fn last_popped_stack_elm(&self) -> Option<Rc<Object>> {
        self.stack.get(self.sp).cloned()
    }

    fn pop(&mut self) -> Rc<Object> {
        let o = Rc::clone(&self.stack[self.sp - 1]);
        self.sp -= 1;
        return o;
    }

    fn push(&mut self, o: Rc<Object>) {
        if self.sp >= STACK_SIZE {
            panic!("Stack overflow");
        };
        self.stack[self.sp] = o;
        self.sp += 1;
    }
}