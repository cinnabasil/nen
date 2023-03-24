use std::collections::HashMap;

use crate::parser::{ Node, Program, Expr, Statement };

#[allow(dead_code)]
#[derive(Debug)]
pub struct IR {
    scope: Vec<HashMap<String, ScopeElement>>
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum ScopeElement {
   Function(Vec<Instruction>),
   Variable
}

#[derive(Debug, Clone)]
enum Instruction {
    PushString(String),
    Call(String)
}

impl IR {
    // TODO: Maybe rewrite this without `clone` at some point?
    fn get_from_scope(&mut self, name: &str) -> Option<ScopeElement> {
        for s in &self.scope {
            if let Some(element) = s.get(name) { 
                return Some(element.clone());
            }
        }
        None
    }

    fn add_to_scope(&mut self, name: &str, element: ScopeElement) {
        let mut scope = self.scope.pop().expect("Should always have at least one scope");
        scope.insert(name.to_string(), element);
        self.scope.push(scope);
    }

    fn handle_expression(&mut self, expression: Expr) -> Vec<Instruction> {
        let mut instructions = Vec::<Instruction>::new();
        
        match expression {
            Expr::FunctionCall { name, arguments } => {
                for argument in arguments {
                    instructions.extend(self.handle_expression(argument));
                } 

                match self.get_from_scope(&name) {
                    Some(ScopeElement::Function(_)) => {},
                    Some(ScopeElement::Variable) => todo!("Tried to call variable"),
                    //None => todo!("Call to undefined function {name}")
                    None => {}
                }
                
                instructions.push(Instruction::Call(name));
            },
            Expr::StringLiteral(s) => {
                instructions.push(Instruction::PushString(s));
            }
        }

        instructions
    }

    fn handle_statement(&mut self, statement: Statement) -> Vec<Instruction> {
        let mut instructions = Vec::<Instruction>::new();

        match statement {
            Statement::Expr(e) => {
                instructions.extend(self.handle_expression(e));
            }
        }

        instructions
    }

    fn handle_function_body(&mut self, contents: Vec<Statement>) -> Vec<Instruction> {
        let mut instructions = Vec::<Instruction>::new();

        for statement in contents {
            instructions.extend(self.handle_statement(statement));
        }

        instructions
    }

    fn handle_node(&mut self, node: Node) {
        match node {
            Node::FunctionDefinition { name, contents, impure: _ } => {
                if let Some(element) = self.get_from_scope(&name) {
                    match element {
                        ScopeElement::Function(_) => todo!("Function {name} already defined"),
                        ScopeElement::Variable => todo!("Function {name} already defined as a variable") 
                    }
                }

                let body = self.handle_function_body(contents);

                let function = ScopeElement::Function(body); 

                self.add_to_scope(&name, function); 
            }
        }
    }
}

impl From<Program> for IR {
    fn from(program: Program) -> IR {
        let mut scope = Vec::<HashMap<String, ScopeElement>>::new();
        scope.push(HashMap::<String, ScopeElement>::new());
        let mut ir = IR {
            scope
        };

        for node in program {
            ir.handle_node(node);
        }

        ir
    }
}
