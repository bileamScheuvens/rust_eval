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
    ast: Vec<ASTNode>,
}
impl ShuntiyardParser {
    pub fn new() -> ShuntiyardParser {
        let parser = ShuntiyardParser {
            operator_stack: Vec::new(),
            ast: Vec::new(),
        };
        return parser;
    }

    pub fn add_node(&mut self, operator: &Token) {
        println!("{:?} {:?}", self.ast, self.operator_stack);
        let l_node = self.ast.pop().unwrap();
        println!("{:?} {:?}", self.ast, self.operator_stack);
        let r_node = self.ast.pop().unwrap();
        let node = match operator {
            Token::Add(_, _) => ASTNode::Add(Box::new(l_node), Box::new(r_node)),
            Token::Mult(_, _) => ASTNode::Multiply(Box::new(l_node), Box::new(r_node)),
            _ => unimplemented!("Operator not defined"),
        };
        self.ast.push(node);
    }

    pub fn parse(&mut self, input: String) -> Result<ASTNode> {
        let mut lexer = Lexer::new(input);
        while let Ok(token) = lexer.next_token() {
            match token {
                Token::Zero | Token::One => self
                    .ast
                    .push(ASTNode::Number(token.to_string().parse().unwrap())),
                Token::Add(_, o1) | Token::Mult(_, o1) => {
                    while self.operator_stack.len() > 0 {
                        match self.operator_stack.last().cloned() {
                            t @ (Some(Token::Add(_, o2)) | Some(Token::Mult(_, o2))) => {
                                if o1 <= o2 || o1 < o2 {
                                    let op = t.unwrap();
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
                token, self.operator_stack, self.ast
            )
        }
        while self.operator_stack.len() > 0 {
            // Pop them off and push them to the output_queue
            let op = &self.operator_stack.pop().unwrap();
            self.add_node(op);
        }
        println!(
            " End Stack {:?} & End output queue {:?}",
            self.operator_stack, self.ast
        );
        Ok(self.ast.pop().unwrap())
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::parser::parser::ShuntiyardParser;

    use super::ASTNode;

    /*
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
    */
    #[test]
    fn parse_expression_test() {
        let inputs = vec![
            ("( 1 + 0 ) * 1", 1),
            ("1 + 1 + 1", 3),
            ("0*0*0*0*0*0", 0),
            ("((1+1)*0+1*(1+0))", 1),
        ];
        let mut parser = ShuntiyardParser::new();

        for (input, result) in inputs {
            println!("Expression to parse {:?}", input);
            let parse_result = parser.parse(input.into());
            let _ast = match parse_result {
                Ok(ast) => {
                    println!("Ast {:?}", ast);
                    println!("Evaluation of Ast {:?}", ast.evaluate());
                    assert_eq!(ast.evaluate(), result);
                }
                Err(err) => panic!("Problem while parsing: {:?}", err),
            };
        }
    }
}
