use std::{collections::HashMap, path::Path};

use crate::parser::{
    self,
    ast::{Operand, Statement},
};

#[derive(Debug, Clone)]
pub struct Registry {
    pub registry: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub registry: Registry,
    pub body: Vec<Statement>,
    pub params: Vec<String>,
}

pub struct Runtime {
    pub registry: Registry,
    pub functions: HashMap<String, Function>,
    pub opcodes: HashMap<String, Box<dyn Fn(Vec<Operand>)>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
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
                        .expect(&format!("Invalid opcode: {}", inst.opcode));

                    (op)(inst.operands);
                }
                _ => {}
            }
        }
    }

    pub fn call_function(&mut self, fn_name: &str, params: Vec<Operand>) {
        let f = self
            .functions
            .get_mut(fn_name)
            .expect(&format!("Function does not exist: {}", fn_name))
            .clone();

        self.execute(f.body, f.registry);
    }

    pub fn register_opcode(&mut self, opcode: &str, f: impl Fn(Vec<Operand>) + 'static) {
        self.opcodes.insert(opcode.to_lowercase(), Box::new(f));
    }
}

impl Registry {
    pub fn params(&mut self, params: Vec<String>) {
        for (v, p) in self.registry.values_mut().zip(params) {
            *v = p;
        }
    }
}
