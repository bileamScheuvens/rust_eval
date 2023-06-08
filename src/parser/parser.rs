use anyhow::Result;

use crate::lexer::lexer::{Lexer, Token};

#[allow(dead_code)]
#[derive(Debug)]
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

pub struct ShuntiyardParser {
    operator_stack: Vec<Token>,
    output_queue: Vec<Token>,
}
impl ShuntiyardParser {
    pub fn new() -> ShuntiyardParser {
        let parser = ShuntiyardParser {
            operator_stack: Vec::new(),
            output_queue: Vec::new(),
        };
        return parser;
    }

    pub fn parse(&mut self, input: String) -> Result<ASTNode> {
        let mut lexer = Lexer::new(input);
        while let Ok(token) = lexer.next_token() {
            match token {
                Token::Zero | Token::One => self.output_queue.push(token),
                Token::Add(_, o1) | Token::Mult(_, o1) => {
                    while self.operator_stack.len() > 0 {
                        match self.operator_stack.last() {
                            Some(Token::Add(_, o2)) | Some(Token::Mult(_, o2)) => {
                                if o1 <= *o2 || o1 < *o2 {
                                    self.output_queue.push(self.operator_stack.pop().unwrap());
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
                        _ => self.output_queue.push(self.operator_stack.pop().unwrap()),
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
            let op = self.operator_stack.pop();
            self.output_queue.push(op.unwrap());
        }
        println!(
            " End Stack {:?} & End output queue {:?}",
            self.operator_stack, self.output_queue
        );
        Ok(ASTNode::Number(0))
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
        let input = "( 1 + 0 ) * 1";
        println!("Expression to parse {:?}", input);
        let mut parser = ShuntiyardParser::new();
        let parse_result = parser.parse(input.into());
        let _ast = match parse_result {
            Ok(ast) => println!("Ast {:?}", ast),
            Err(err) => panic!("Problem while parsing: {:?}", err),
        };
    }
}
