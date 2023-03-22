use std::collections::HashMap;
use crate::parser::{ Program, Expr };

#[allow(dead_code)]
pub struct IR {
    namespace: HashMap<String, Function>
}

#[allow(dead_code)]
enum Type {
    Void,
    String
}

struct FunctionArgument (String, Type);

#[allow(dead_code)]
enum Function {
    UserDefined { arguments: Vec<FunctionArgument>, body: Vec<Expr> },
    BuiltIn { arguments: Vec<FunctionArgument> }
}

impl IR {
    pub fn new() -> IR {
        let mut namespace = HashMap::<String, Function>::new();

        // Initalize builtins here
        namespace.insert(
            "print".to_string(), 
            Function::BuiltIn { arguments: vec![FunctionArgument("input".to_string(), Type::String)] }
        );

        IR {
            namespace
        }
    }
}

impl From<Program> for IR {
    fn from(_program: Program) -> Self {
        IR::new()
    }
}
