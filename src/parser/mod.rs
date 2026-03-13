pub mod ast;

use pest::iterators::Pair;
use pest_derive::Parser;

use ast::*;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"] // relative to src
pub struct ForgeParser;

use pest::Parser;

pub fn parse_program(src: &str) -> Result<Program, pest::error::Error<Rule>> {
    let mut pairs = ForgeParser::parse(Rule::program, src)?;
    let program = pairs.next().unwrap();

    let mut functions = Vec::new();
    let mut statements = Vec::new();

    for pair in program.into_inner() {
        match pair.as_rule() {
            Rule::function_def => functions.push(parse_function(pair)),
            Rule::instruction | Rule::pointer_assign | Rule::function_call => {
                statements.push(parse_statement(pair))
            }
            _ => {}
        }
    }

    Ok(Program {
        functions,
        statements,
    })
}

fn parse_function(pair: Pair<Rule>) -> Function {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut params = Vec::new();
    let mut body = Vec::new();

    for p in inner {
        match p.as_rule() {
            Rule::param_list => {
                for param in p.into_inner() {
                    params.push(param.as_str().to_string());
                }
            }
            Rule::block => {
                for stmt in p.into_inner() {
                    body.push(parse_statement(stmt));
                }
            }
            _ => {}
        }
    }

    Function { name, params, body }
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::instruction => Statement::Instruction(parse_instruction(pair)),

        Rule::pointer_assign => {
            let mut inner = pair.into_inner();

            let left = parse_reg(inner.next().unwrap());
            let right = parse_reg(inner.next().unwrap());

            Statement::PointerAssign { left, right }
        }

        Rule::function_call => Statement::FunctionCall(parse_function_call(pair)),

        _ => unreachable!(),
    }
}

fn parse_instruction(pair: Pair<Rule>) -> Instruction {
    let mut inner = pair.into_inner();

    let opcode = inner.next().unwrap().as_str().to_string();

    let mut operands = Vec::new();

    if let Some(list) = inner.next() {
        for op in list.into_inner() {
            operands.push(parse_operand(op));
        }
    }

    Instruction { opcode, operands }
}

fn parse_function_call(pair: Pair<Rule>) -> FunctionCall {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();

    let mut args = Vec::new();

    if let Some(list) = inner.next() {
        for arg in list.into_inner() {
            args.push(parse_operand(arg));
        }
    }

    FunctionCall { name, args }
}

fn parse_operand(pair: Pair<Rule>) -> Operand {
    match pair.as_rule() {
        Rule::number => Operand::Number(pair.as_str().parse().unwrap()),

        Rule::global_reg | Rule::local_reg => Operand::Reg(parse_reg(pair)),

        Rule::reg => Operand::Reg(parse_reg(pair.into_inner().next().unwrap())),

        Rule::pointer_expr => {
            let inner = pair.into_inner().next().unwrap();
            Operand::Pointer(Box::new(parse_operand(inner)))
        }

        _ => unreachable!(),
    }
}

fn parse_reg(pair: Pair<Rule>) -> Reg {
    match pair.as_rule() {
        Rule::global_reg => Reg::Global(pair.as_str()[1..].to_string()),

        Rule::local_reg => Reg::Local(pair.as_str()[1..].to_string()),

        _ => unreachable!(),
    }
}
