use std::collections::HashMap;
use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    BinaryOperator {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Token,
    },
    Variable(String),
    CodeBlock(Vec<Expr>),
    Assign(String, Box<Expr>),
    Function(Vec<String>, Box<Expr>),
    FunctionCall(Box<Expr>, Vec<Expr>),
    Switch(Vec<Expr>, Vec<Expr>),
    While(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Function {
        args: Vec<String>,
        body: Expr,
        context: Context
    },
    Vector(Vec<Object>),
    Null
}

#[derive(Clone, Debug)]
pub struct Context {
    pub variables: HashMap<String, Object>,
}

impl Expr {
    pub fn eval(&self, context: &mut Context) -> Result<Object, String> {
        match self {
            Expr::Number(value) => Ok(Object::Number(*value)),
            Expr::BinaryOperator { left, right, op } => eval_binary_operator(left, right, op, context),
            Expr::Variable(name) => match context.variables.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Undefined variable: {}", name)),
            },
            Expr::CodeBlock(code) => {
                let mut last_line_eval = Object::Null;
                for line in code {
                    last_line_eval = line.eval(context)?;
                }
                Ok(last_line_eval)
            },
            Expr::Assign(name, value) => {
                let value = value.eval(context)?;
                context.variables.insert(name.clone(), value.clone());
                Ok(value)
            }
            Expr::Function(args, body) => {
                Ok(Object::Function {args: args.clone(), body: *body.clone(), context: context.clone()})
            }
            Expr::FunctionCall(function, args) => {
                if let Object::Function {args: func_args, body, context: func_context} = function.eval(context)? {
                    let mut args_eval = Vec::new();
                    let cloned_context = context.clone();
                    let mut variables = cloned_context.variables.iter().chain(func_context.variables.iter()).collect::<HashMap<_,_>>();
                    for arg in args {
                        args_eval.push(arg.eval(context)?);
                    }
                    for (index, arg_eval) in args_eval.iter().enumerate() {
                        variables.insert(&func_args[index], &arg_eval);

                    }

                    let new_variables: HashMap<String, Object> = variables.into_iter().map(|(str, obj)| (str.clone(), obj.clone())).collect();

                    let result = body.eval(&mut Context {variables: new_variables})?;

                    Ok(result)
                } else {
                    Err("".to_string())
                }



            }
            Expr::Switch(cases, expressions) => {
                if cases.len() != expressions.len() {
                    return Err("jfdjk".to_string());
                }

                for (index, case) in cases.iter().enumerate() {
                    let case_eval =  case.eval(context)?;
                    match case_eval {
                        Object::Number(value) => {
                            if value != 0f64 {
                                return Ok(expressions[index].eval(context)?)
                            }
                        },
                        Object::Function {..} => return Ok(expressions[index].eval(context)?),

                        _ => {}
                    }
                }

                Ok(Object::Null)
            },
            Expr::While(condition, expr) => {
                while object_to_bool(condition.eval(context)?) {
                    expr.eval(context)?;
                }
                Ok(Object::Null)
            }
        }
    }
}

fn object_to_bool(object: Object) -> bool {
    match object {
        Object::Null => false,
        Object::Number(value) => value != 0f64,
        Object::Function {..} => true,
        Object::Vector(..) => true
    }
}

fn eval_binary_operator(left: &Box<Expr>, right: &Box<Expr>, op: &Token, context: &mut Context) -> Result<Object, String> {
    if let Object::Number(left_eval) = left.eval(context)? {
        if let Object::Number(right_eval) = right.eval(context)? {
            Ok(Object::Number(match op {
                Token::Plus => left_eval + right_eval,
                Token::Minus => left_eval - right_eval,
                Token::Mul => left_eval * right_eval,
                Token::Div => left_eval / right_eval,
                Token::Mod => left_eval % right_eval,
                Token::EqualEqual => if left_eval == right_eval {1f64} else {0f64},
                Token::Less => if left_eval < right_eval {1f64} else {0f64},
                Token::Greater => if left_eval > right_eval {1f64} else {0f64},
                Token::LessEqual => if left_eval <= right_eval {1f64} else {0f64},
                Token::GreaterEqual => if left_eval >= right_eval {1f64} else {0f64},
                _ => 0f64
            }))
        } else {
            Err("Right operand is not a number.".to_string())
        }
    } else {
        Err("Left operand is not a number.".to_string())
    }
}
