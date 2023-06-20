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
    pub fn evaluate(&self) -> Option<ResultEval> {
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
