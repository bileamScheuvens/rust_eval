use std::env;

use rust_eval::{
    evaluator::evaluator::Evaluator, lexer::lexer::Lexer, parser::parser::ShuntiyardParser,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let exp = &args[2];
        let lexer = Lexer::new(exp.into());
        let mut parser = ShuntiyardParser::new(lexer);
        let result = parser.parse();
        match result {
            Ok(ast) => {
                let evalaluator = Evaluator {};
                match evalaluator.eval(&ast) {
                    Some(result) => print!("Result of evaluation: {}", result.to_string()),
                    None => print!("Cannot be evaluated"),
                }
            }
            Err(_) => panic!("Error while parsing"),
        }
    } else {
        println!("No expression found")
    }
}
