use std::collections::HashMap;
use crate::parser::{ Program, Node, Expr };

#[allow(dead_code)]
pub struct IR {
    namespace: HashMap<String, NamespaceElement>,
    constants: Vec<Constant>
}

#[allow(dead_code)]
enum Constant {
    StringLiteral(String)
}

#[allow(dead_code)]
enum Type {
    Void,
    String
}

struct FunctionArgument (String, Type);

#[allow(dead_code)]
enum NamespaceElement {
    Variable,
    Function(Function)
}

#[allow(dead_code)]
enum Function {
    UserDefined { arguments: Vec<FunctionArgument>, body: Vec<IRExpr>, impure: bool },
    // Function::BuiltIn doesn't have a `body` field as this will be filled in
    // by the compiler itself
    BuiltIn { arguments: Vec<FunctionArgument>, impure: bool },
    // Function::ToBeDefined is used for functions that are called before they
    // are defined, allowing for definitions later in code than the reference
    // to them.
    // If any functions are still Function::ToBeDefined at the end of IR generation,
    // a reference error will be returned
    ToBeDefined
}

// We have different types here than the AST (see crate::parser::Expr vs IRExpr)
// because these need to carry more detailed information

// For example, a function call containing a string literal would actually be a
// reference into the constant 'pool' (IR.constants) rather than how it is represented
// in crate::parser
enum IRExpr {
    FunctionCall(String)
}

impl IR {
    fn handle_expr(&mut self, expr: &Expr) -> IRExpr {
	match expr {
	    Expr::FunctionCall { name, arguments: _ } => {
		// Check if function exists
		if let Some(defined) = self.namespace.get(name) {
		    match defined {
			NamespaceElement::Variable => todo!("Can't call a variable"),
			// If it's a function, great!
			NamespaceElement::Function(_) => {}
		    }
		} else {
		    self.namespace.insert(name.clone(), NamespaceElement::Function(Function::ToBeDefined));
		}
		
		return IRExpr::FunctionCall(name.clone());
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
				Function::ToBeDefined => {},
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
            "print".to_string(), 
            NamespaceElement::Function(Function::BuiltIn { arguments: vec![FunctionArgument("input".to_string(), Type::String)], impure: true })
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
			Function::ToBeDefined => todo!("Function {name} was called, but is not defined anywhere"),
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
