use std::{collections::HashMap, path::Path};

use crate::parser::{self, ast::Statement};

#[derive(Debug, Clone)]
pub struct Registry {}

pub struct Function {
    pub registry: Registry,
    pub body: Vec<Statement>,
    pub params: Vec<String>,
}

pub struct Runtime {
    pub registry: Registry,
    pub functions: HashMap<String, Function>,
    pub opcodes: HashMap<String, Box<dyn Fn(Vec<String>)>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            functions: HashMap::new(),
            opcodes: HashMap::new(),
        }
    }
}

impl Runtime {
    pub fn execute_script(&mut self, path: &Path) {
        let src = std::fs::read_to_string(path).unwrap();

        let ast = parser::parse_program(&src).unwrap();

        for func in ast.functions {
            self.functions.insert(
                func.name,
                Function {
                    registry: Registry::new(),
                    body: func.body,
                    params: func.params,
                },
            );
        }
    }

    pub fn execute(&mut self, statements: Vec<Statement>, reg: Registry) {
        for statement in statements {
            match statement {
                Statement::Instruction(inst) => {
                    let op = self
                        .opcodes
                        .get(&inst.opcode.to_lowercase())
                        .expect("Invalid opcode");

                    (op)(vec![]);
                }
                _ => {}
            }
        }
    }

    pub fn call_function(&mut self, fn_name: &str, params: Vec<u8>) {
        let f = self
            .functions
            .get(fn_name)
            .expect("Function does not exist");

        self.execute(f.body.clone(), f.registry.clone());
    }
}
