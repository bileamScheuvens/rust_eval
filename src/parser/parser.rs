use anyhow::Result;

use crate::lexer::lexer::{Lexer, Token};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Number(u8),
    Add(Box<ASTNode>, Box<ASTNode>),
    Multiply(Box<ASTNode>, Box<ASTNode>),
}

#[allow(dead_code)]
impl ASTNode {
    fn evaluate(&self) -> u8 {
        match self {
            ASTNode::Number(value) => *value,
            ASTNode::Add(left, right) => left.evaluate() + right.evaluate(),
            ASTNode::Multiply(left, right) => left.evaluate() * right.evaluate(),
        }
    }
}
/*
#[allow(dead_code)]
#[derive(Debug)]
pub struct ASTNode {
    value: Token,
    l_node: Box<ASTNode>,
    r_node: Box<ASTNode>,
}
impl ASTNode {
    pub fn new(value: Token, l_node: Box<ASTNode>, r_node: Box<ASTNode>) -> ASTNode {
        return ASTNode {
            value: value,
            l_node: l_node,
            r_node: r_node,
        };
    }
}*/
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
                Token::Add(o1) | Token::Mult(o1) => {
                    while self.operator_stack.len() > 0 && self.operator_stack.last() != None {
                        match self.operator_stack.last() {
                            Some(Token::Add(o2)) | Some(Token::Mult(o2)) => {
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

    use crate::parser::parser::ShuntiyardParser;

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
        assert_eq!(2, expression_0.evaluate());

        let expression_1 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(0)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        assert_eq!(0, expression_1.evaluate());

        let expression_2 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        assert_eq!(0, expression_2.evaluate());

        let expression_2 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        assert_eq!(0, expression_2.evaluate());

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
        ];
        let mut parser = ShuntiyardParser::new();

        for (input, result) in inputs {
            //println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(output_queue) => {
                    //println!("Ast {:?}", output_queue);
                    println!(
                        "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                        input,
                        output_queue.evaluate(),
                        result
                    );
                    assert_eq!(output_queue.evaluate(), result);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }
    #[test]
    fn parse_expression_single_test() {
        let inputs = vec![("( 1 + 0  ) * 0", 0)];
        let mut parser = ShuntiyardParser::new();

        for (input, result) in inputs {
            println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(output_queue) => {
                    //println!("Ast {:?}", output_queue);
                    println!(
                        "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                        input,
                        output_queue.evaluate(),
                        result
                    );
                    assert_eq!(output_queue.evaluate(), result);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }
}
