use std::collections::HashMap;
use crate::parser::{ Program, Node, Expr };

#[derive(Debug)]
#[allow(dead_code)]
pub struct IR {
    pub namespace: HashMap<String, NamespaceElement>,
    pub constants: Vec<Constant>
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Constant {
    StringLiteral(String)
}

#[derive(Debug)]
#[allow(dead_code)]
enum Type {
    Void,
    String
}

#[derive(Debug)]
pub struct FunctionArgument (String, Type);

#[derive(Debug)]
#[allow(dead_code)]
pub enum NamespaceElement {
    Variable,
    Function(Function)
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Function {
    UserDefined { arguments: Vec<FunctionArgument>, body: Vec<IRExpr>, impure: bool },
    // Function::BuiltIn doesn't have a `body` field as this will be filled in
    // by the compiler itself
    BuiltIn { arguments: Vec<FunctionArgument>, impure: bool },
    // Function::ToBeDefined is used for functions that are called before they
    // are defined, allowing for definitions later in code than the reference
    // to them.
    // If any functions are still Function::ToBeDefined at the end of IR generation,
    // a reference error will be returned
    ToBeDefined { argument_count: usize }
}

// We have different types here than the AST (see crate::parser::Expr vs IRExpr)
// because these need to carry more detailed information

// For example, a function call containing a string literal would actually be a
// reference into the constant 'pool' (IR.constants) rather than how it is represented
// in crate::parser
#[derive(Debug)]
pub enum IRExpr {
    FunctionCall(String, Vec<IRExpr>),
    // vvv Index into IR.constants
    StringLiteral(usize)
}

impl IR {
    fn handle_expr(&mut self, expr: &Expr) -> IRExpr {
	match expr {
	    Expr::FunctionCall { name, arguments: ast_arguments } => {
		// Check if function exists
		if let Some(defined) = self.namespace.get(name) {
		    match defined {
			NamespaceElement::Variable => todo!("Can't call a variable"),
			// If it's a function, great!
			NamespaceElement::Function(f) => {
			    match f {
				Function::ToBeDefined { argument_count } => {
				    if *argument_count != ast_arguments.len() {
					todo!("Wrong number of arguments to function {name}");
				    }
				},
				Function::BuiltIn { arguments, impure: _ } | Function::UserDefined { arguments, body: _, impure: _ } => {
				    if arguments.len() != ast_arguments.len() {
					todo!("Wrong number of arguments to function {name}");
				    }
				}
			    }
			}
		    }
		} else {
		    self.namespace.insert(name.clone(), NamespaceElement::Function(Function::ToBeDefined { argument_count: ast_arguments.len() }));
		}

		let mut arguments = Vec::<IRExpr>::new();

		for argument in ast_arguments.iter() {
		    arguments.push(self.handle_expr(argument));
		}
		
		return IRExpr::FunctionCall(name.clone(), arguments);
	    },
	    Expr::StringLiteral(s) => {
		self.constants.push(Constant::StringLiteral(s.to_string()));
		return IRExpr::StringLiteral(self.constants.len() - 1);
	    },
	    #[allow(unreachable_patterns)]
	    _ => todo!("Unreachable!")
	}
    }
    
    fn handle_node(&mut self, node: &Node) {
	match node {
	    Node::FunctionDefinition { name, contents, impure } => {
		if let Some(defined) = self.namespace.get(name) {
		    match defined {
			NamespaceElement::Variable => todo!("Tried to declare function with name of variable that exists"),
			NamespaceElement::Function(function) => {
			    match function {
				// If the function has been referenced before, then we don't care that it is technically
				// in the namespace, as this is expected
				Function::ToBeDefined { argument_count: _ } => {},
				_ => todo!("Function {name} already defined!")
			    };
			}
		    }
		}

		let mut body = Vec::<IRExpr>::new();

		for expr in contents.iter() {
		    body.push(self.handle_expr(expr));
		}
		
		self.namespace.insert(name.to_string(), NamespaceElement::Function(
		    Function::UserDefined {
			arguments: Vec::<FunctionArgument>::new(),
			body,
			impure: impure.clone()
		    }
		));
	    },
	    #[allow(unreachable_patterns)]
	    _ => todo!("Unreachable!")
	}
    }
    
    pub fn new() -> IR {
        let mut namespace = HashMap::<String, NamespaceElement>::new();

        // Initalize builtins here

	namespace.insert(
	    String::from("print"),
	    NamespaceElement::Function(
		Function::BuiltIn { arguments: vec![FunctionArgument(String::from("input"), Type::String)], impure: true }
	    )
	);
	
        IR {
            namespace,
	    constants: Vec::<Constant>::new()
        }
    }

    fn error_check(&mut self) {
	// Check for any undefined functions
	for (name, val) in self.namespace.iter() {
	    match val {
		NamespaceElement::Variable => continue,
		NamespaceElement::Function(function) => {
		    match function {
			Function::ToBeDefined { argument_count: _ } =>
			    todo!("Function {name} was called, but is not defined anywhere"),
			_ => continue
		    }
		}
	    }
	}
    }
}

impl From<Program> for IR {
    fn from(program: Program) -> Self {
	let mut iter = program.iter();
	let mut ir = IR::new();
	
	while let Some(node) = iter.next() {
	    ir.handle_node(node);
	}

	ir.error_check();
	
	ir
    }
}
