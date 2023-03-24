use std::collections::HashMap;

use crate::parser::{ Node, Program, Expr, Statement };

#[allow(dead_code)]
#[derive(Debug)]
pub struct IR {
    pub scope: Vec<HashMap<String, ScopeElement>>
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ScopeElement {
    Function(Vec<Instruction>),
    // PlaceholderFunction is when a function is called,
    // but it is not defined yet.
    // 
    // We need this so that we can define functions later in
    // code than where they are called.
    PlaceholderFunction,
    BuiltInFunction,
    Variable
}

#[derive(Debug, Clone)]
pub enum Instruction {
    PushString(String),
    Call(String),
    Write
}

impl IR {
    // TODO: Maybe rewrite this without `clone` at some point?
    fn get_from_scope(&mut self, name: &str) -> Option<(usize, ScopeElement)> {
        for (i, s) in self.scope.iter().enumerate() {
            if let Some(element) = s.get(name) { 
                return Some((i, element.clone()));
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
                    Some((_, ScopeElement::Function(_))) => {},
                    Some((_, ScopeElement::PlaceholderFunction)) => {},
                    Some((_, ScopeElement::BuiltInFunction)) => {},
                    Some((_, ScopeElement::Variable)) => todo!("Tried to call variable"),
                    None => {
                        self.add_to_scope(&name, ScopeElement::PlaceholderFunction);
                    }
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
                        (_, ScopeElement::Function(_)) => todo!("Function {name} already defined"),
                        (_, ScopeElement::PlaceholderFunction) => {},
                        (_, ScopeElement::BuiltInFunction) => {
                            todo!("Function {name} defined as built-in - We need to decide if we want to allow overwriting of built-in functions");
                        },
                        (_, ScopeElement::Variable) => todo!("Function {name} already defined as a variable") 
                    }
                }

                let body = self.handle_function_body(contents);

                let function = ScopeElement::Function(body); 

                // Functions can only be defined in the top level
                // meaning that we should only have one scope active
                // but we check just in case
                
                if self.scope.len() > 1 {
                    panic!("Functions can only be defined at the top level");
                }

                self.add_to_scope(&name, function); 
            }
        }
    }
}

impl From<Program> for IR {
    fn from(program: Program) -> IR {
        let mut scope = Vec::<HashMap<String, ScopeElement>>::new();
        let mut top_scope = HashMap::<String, ScopeElement>::new();

        
        // Define built-ins

        top_scope.insert("print".to_string(), ScopeElement::BuiltInFunction);

        scope.push(top_scope);
        let mut ir = IR {
            scope
        };

        for node in program {
            ir.handle_node(node);
        }

        // Check for any placeholder functions left
        let map = ir.scope.pop().expect("Should have a scope");
        let mut placeholder_functions = Vec::<String>::new();
        for (name, element) in &map {
            match element {
                ScopeElement::PlaceholderFunction => placeholder_functions.push(name.to_string()),
                _ => {}
            } 
        }

        if placeholder_functions.len() > 0 {
            for func in placeholder_functions {
                eprintln!("ERROR: Function {func} was called, but not defined.");
            }
            std::process::exit(1);
        }
        ir.scope.push(map);

        ir
    }
}
