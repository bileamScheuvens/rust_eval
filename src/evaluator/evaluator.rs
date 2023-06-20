use crate::parser::parser::{ASTNode, ResultEval};
pub struct Evaluator {}

impl Evaluator {
    pub fn eval(&self, node: &ASTNode) -> Option<ResultEval> {
        match node {
            ASTNode::Number(value) => Some(ResultEval::Int(*value)),
            ASTNode::Bool(value) => Some(ResultEval::Bool(*value)),
            ASTNode::Add(left, right) => {
                let l1 = self.eval(left);
                let r1 = self.eval(right);
                if !l1.is_some() && !r1.is_some() {
                    return None;
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
                let l1 = self.eval(left);
                let r1 = self.eval(right);
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
                let l1 = self.eval(left);
                if !l1.is_some() {
                    None
                } else {
                    match l1.unwrap() {
                        ResultEval::Int(_) => return None,
                        ResultEval::Bool(true) => return Some(ResultEval::Bool(true)),
                        _ => {
                            let r1 = self.eval(right);
                            if !r1.is_some() {
                                return None;
                            } else {
                                match r1.unwrap() {
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

#[cfg(test)]
mod test {
    use crate::{
        evaluator::evaluator::Evaluator,
        parser::parser::{ASTNode, ResultEval},
    };
    use anyhow::Result;

    #[test]
    fn eval() -> Result<()> {
        let evaluator = Evaluator {};

        let ast1 = ASTNode::Add(
            Box::new(ASTNode::Number(1)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(1)),
            )),
        );
        let result = evaluator.eval(&ast1).unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => unreachable!(),
        };
        assert_eq!(2, val_eval);

        let ast2 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(0)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        let result = evaluator.eval(&ast2).unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => unreachable!(),
        };
        assert_eq!(0, val_eval);

        let ast3 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(0)),
            )),
        );

        let result = evaluator.eval(&ast3).unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => unreachable!(),
        };
        assert_eq!(0, val_eval);

        let ast4 = ASTNode::Add(
            Box::new(ASTNode::Number(0)),
            Box::new(ASTNode::Multiply(
                Box::new(ASTNode::Number(1)),
                Box::new(ASTNode::Number(0)),
            )),
        );
        let result = evaluator.eval(&ast4).unwrap();
        let val_eval = match result {
            ResultEval::Int(value) => value,
            _ => unreachable!(),
        };
        assert_eq!(0, val_eval);
        Ok(())
    }

    // #[test]
    // fn parse_expression_test() {
    //     let inputs = vec![
    //         ("( 1 + 0 ) * 1", 1),
    //         ("1 + 1 + 1", 3),
    //         ("0*0*0*0*0*0", 0),
    //         ("((1+1)*0+1*(1+0))", 1),
    //         ("(()()()()(((1))))", 1),
    //         ("(1*1)*0", 0),
    //         ("1+0", 1),
    //     ];
    //     let mut parser = ShuntiyardParser::new();

    //     for (input, exp_result) in inputs {
    //         //println!("Expression to parse {:?}", input);
    //         let parse_result = parser.parse(input.into());
    //         let _ast = match parse_result {
    //             Ok(output_queue) => {
    //                 //println!("Ast {:?}", output_queue);
    //                 let result_eval = output_queue.evaluate().unwrap();
    //                 let val_eval = match result_eval {
    //                     ResultEval::Int(value) => value,
    //                     _ => panic!("There should be no other value then an int"),
    //                 };

    //                 println!(
    //                     "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
    //                     input, val_eval, exp_result
    //                 );
    //                 assert_eq!(exp_result, val_eval);
    //             }
    //             Err(err) => panic!("Problem while parsing: {:?}", err),
    //         };
    //     }
    // }

    #[test]
    fn parse_expression_bool_none_test() {
        let evaluator = Evaluator {};
        let inputs = vec![
            (
                ASTNode::Add(Box::new(ASTNode::Bool(true)), Box::new(ASTNode::Number(1))),
                None::<ResultEval>,
            ),
            (
                ASTNode::Or(Box::new(ASTNode::Bool(false)), Box::new(ASTNode::Number(1))),
                None::<ResultEval>,
            ),
        ];

        for (input, exp_result) in inputs {
            println!("Expression to parse {:?}", input);
            let result_eval = evaluator.eval(&input);
            println!(
                "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                input, result_eval, exp_result
            );
            assert_eq!(result_eval, exp_result);
        }
    }

    #[test]
    fn parse_expression_bool_valid_test() {
        let evaluator = Evaluator {};
        let inputs = vec![
            (
                ASTNode::Or(
                    Box::new(ASTNode::Bool(true)),
                    Box::new(ASTNode::Bool(false)),
                ),
                ResultEval::Bool(true),
            ),
            (
                ASTNode::Or(Box::new(ASTNode::Bool(true)), Box::new(ASTNode::Number(1))),
                ResultEval::Bool(true),
            ),
        ];

        for (input, exp_result) in inputs {
            println!("Expression to parse {:?}", input);
            let result_eval = evaluator.eval(&input).unwrap();
            println!(
                "Expression to parse {:?} Evaluation of Ast {:?} excpected value {:?}",
                input, result_eval, exp_result
            );
            assert_eq!(result_eval, exp_result);
        }
    }
}
