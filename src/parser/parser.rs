use anyhow::Result;

use crate::lexer::lexer::{Lexer, Token};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(u8),
    Bool(bool),
    Add(Box<ASTNode>, Box<ASTNode>),
    Multiply(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
}

#[derive(Debug, PartialEq)]
pub enum ResultEval {
    Int(u8),
    Bool(bool),
    Nothing(),
}

#[allow(dead_code)]
impl ASTNode {
    fn evaluate(&self) -> Option<ResultEval> {
        match self {
            ASTNode::Number(value) => Some(ResultEval::Int(*value)),
            ASTNode::Bool(value) => Some(ResultEval::Bool(*value)),
            ASTNode::Add(left, right) => {
                let l1 = left.evaluate();
                let r1 = right.evaluate();
                if !l1.is_some() && !r1.is_some() {
                    None
                } else {
                    let val_l1 = match l1.unwrap() {
                        ResultEval::Int(value) => value,
                        _ => return None,
                    };
                    let val_l2 = match r1.unwrap() {
                        ResultEval::Int(value) => value,
                        _ => return None,
                    };
                    Some(ResultEval::Int(val_l1 + val_l2))
                }
            }
            ASTNode::Multiply(left, right) => {
                let l1 = left.evaluate();
                let r1 = right.evaluate();
                if !l1.is_some() && !r1.is_some() {
                    None
                } else {
                    let val_l1 = match l1.unwrap() {
                        ResultEval::Int(value) => value,
                        _ => return None,
                    };
                    let val_l2 = match r1.unwrap() {
                        ResultEval::Int(value) => value,
                        _ => return None,
                    };
                    Some(ResultEval::Int(val_l1 * val_l2))
                }
            }
            ASTNode::Or(left, right) => {
                let l1 = left.evaluate();
                if !l1.is_some() {
                    None
                } else {
                    println!("left node: {:?}", l1.as_ref().unwrap());
                    match l1.unwrap() {
                        ResultEval::Int(_) => return None,
                        ResultEval::Bool(true) => return Some(ResultEval::Bool(true)),
                        _ => {
                            let r1 = right.evaluate();
                            if !r1.is_some() {
                                return None;
                            } else {
                                match right.evaluate().unwrap() {
                                    ResultEval::Int(_) => return None,
                                    ResultEval::Bool(true) => return Some(ResultEval::Bool(true)),
                                    _ => return Some(ResultEval::Bool(false)),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct ShuntiyardParser {
    operator_stack: Vec<Token>,
    output_queue: Vec<ASTNode>,
}
impl ShuntiyardParser {
    pub fn new() -> ShuntiyardParser {
        let parser = ShuntiyardParser {
            operator_stack: Vec::new(),
            output_queue: Vec::new(),
        };
        return parser;
    }

    pub fn check_for_zero(&self, l_node: ASTNode, r_node: ASTNode) -> ASTNode {
        // check if left or right side of multiplication is 0 to simply evaluation
        if (l_node == ASTNode::Number(0)) | (r_node == ASTNode::Number(0)) {
            println!("One side of tree is zero");
            return ASTNode::Number(0);
        } else {
            return ASTNode::Multiply(Box::new(l_node), Box::new(r_node));
        }
    }

    pub fn add_node(&mut self, operator: &Token) {
        let l_node = self.output_queue.pop().unwrap();
        let r_node = self.output_queue.pop().unwrap();

        let node = match operator {
            Token::Add(_) => ASTNode::Add(Box::new(l_node), Box::new(r_node)),
            Token::Mult(_) => self.check_for_zero(l_node, r_node),
            Token::Or(_) => ASTNode::Or(Box::new(r_node), Box::new(l_node)),
            _ => unimplemented!("Operator not defined"),
        };
        self.output_queue.push(node);
    }

    pub fn parse(&mut self, input: String) -> Result<ASTNode> {
        let mut lexer = Lexer::new(input);
        while let Ok(token) = lexer.next_token() {
            match token {
                Token::Zero | Token::One => self
                    .output_queue
                    .push(ASTNode::Number(token.to_string().parse().unwrap())),
                Token::True => self.output_queue.push(ASTNode::Bool(true)),
                Token::False => self.output_queue.push(ASTNode::Bool(false)),
                Token::Add(o1) | Token::Mult(o1) | Token::Or(o1) => {
                    while self.operator_stack.len() > 0 && self.operator_stack.last() != None {
                        match self.operator_stack.last() {
                            Some(Token::Add(o2)) | Some(Token::Mult(o2)) | Some(Token::Or(o2)) => {
                                if o1 <= *o2 {
                                    let op = self.operator_stack.pop().unwrap();
                                    self.add_node(&op);
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                    self.operator_stack.push(token)
                }
                Token::LPar => self.operator_stack.push(token),
                Token::RPar => loop {
                    match self.operator_stack.last() {
                        Some(&Token::LPar) => {
                            self.operator_stack.pop().unwrap();
                            break;
                        }
                        _ => {
                            let op = &self.operator_stack.pop().unwrap();
                            self.add_node(op);
                        }
                    }
                },
                Token::Eof => break,
                _ => (),
            }
            println!(
                "Current Token {:?} & Current Stack {:?} & Current output queue {:?}",
                token, self.operator_stack, self.output_queue
            )
        }
        while self.operator_stack.len() > 0 {
            // Pop them off and push them to the output_queue
            let op = &self.operator_stack.pop().unwrap();
            self.add_node(op);
        }
        println!(
            "End Stack {:?} & End output queue {:?}",
            self.operator_stack, self.output_queue
        );
        Ok(self.output_queue.pop().unwrap())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::parser::parser::{ResultEval, ShuntiyardParser};

    use super::ASTNode;

    #[test]
    fn evaluate() -> Result<()> {
        let expression_0 = ASTNode::Add(
            Box::new(ASTNode::Number(1)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(1)),
            )),
        );
        let result = expression_0.evaluate().unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => !unreachable!(),
        };
        assert_eq!(2, val_eval);

        let expression_1 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(0)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        let result = expression_1.evaluate().unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => !unreachable!(),
        };
        assert_eq!(0, val_eval);

        let expression_2 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(0)),
            )),
        );

        let result = expression_2.evaluate().unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => !unreachable!(),
        };
        assert_eq!(0, val_eval);

        let expression_2 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        let result = expression_2.evaluate().unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => !unreachable!(),
        };
        assert_eq!(0, val_eval);

        return Ok(());
    }

    #[test]
    fn parse_expression_test() {
        let inputs = vec![
            ("( 1 + 0 ) * 1", 1),
            ("1 + 1 + 1", 3),
            ("0*0*0*0*0*0", 0),
            ("((1+1)*0+1*(1+0))", 1),
            ("(()()()()(((1))))", 1),
            ("(1*1)*0", 0),
            ("1+0", 1),
        ];
        let mut parser = ShuntiyardParser::new();

        for (input, exp_result) in inputs {
            //println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(output_queue) => {
                    //println!("Ast {:?}", output_queue);
                    let result_eval = output_queue.evaluate().unwrap();
                    let val_eval = match result_eval {
                        ResultEval::Int(value) => value,
                        _ => panic!("There should be no other value then an int"),
                    };

                    println!(
                        "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                        input, val_eval, exp_result
                    );
                    assert_eq!(exp_result, val_eval);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }
    #[test]
    fn parse_expression_single_test() {
        let inputs = vec![("( 1 + 0  ) * 0", 0)];
        let mut parser = ShuntiyardParser::new();

        for (input, exp_result) in inputs {
            println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(output_queue) => {
                    let result_eval = output_queue.evaluate().unwrap();
                    let val_eval = match result_eval {
                        ResultEval::Int(value) => value,
                        _ => panic!("There should be no other value then an int"),
                    };

                    println!(
                        "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                        input, val_eval, exp_result
                    );
                    assert_eq!(exp_result, val_eval);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }

    #[test]
    fn parse_expression_bool_none_test() {
        let inputs = vec![
            (" true + 1", None::<ResultEval>),
            (" false || 1", None::<ResultEval>),
        ];
        let mut parser = ShuntiyardParser::new();

        for (input, exp_result) in inputs {
            println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(output_queue) => {
                    let result_eval = output_queue.evaluate();
                    println!(
                        "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                        input, result_eval, exp_result
                    );
                    assert_eq!(result_eval, exp_result);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }

    #[test]
    fn parse_expression_bool_valid_test() {
        let inputs = vec![
            (" false || true", ResultEval::Bool(true)),
            (" true || 1", ResultEval::Bool(true)),
        ];
        let mut parser = ShuntiyardParser::new();

        for (input, exp_result) in inputs {
            println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(output_queue) => {
                    let result_eval = output_queue.evaluate().unwrap();
                    println!(
                        "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                        input, result_eval, exp_result
                    );
                    assert_eq!(result_eval, exp_result);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }
}
