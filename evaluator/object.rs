use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use crate::environment::Env;
use parser::ast::BlockStatement;

pub type EvalError = String;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Rc<Object>),
    Function(Vec<String>, BlockStatement, Env),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Null => write!(f, "null"),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::ReturnValue(expr) => write!(f, "{}", expr),
            Object::Function(params, body,  _env) => {
                write!(f, "fn({}) {{ {} }}", params.join(", "), body)
            }
        }
    }
}
