use crate::parser::parser::{ASTNode, ResultEval};
pub struct SimpRule {
    pub name: String,
    pub can_apply: fn(&ASTNode) -> bool,
    pub apply: fn(&ASTNode) -> ASTNode,
}
pub struct Simplifier { 
    rules: Vec<SimpRule>
}


impl Simplifier {
    pub fn new(rules: Option<Vec<SimpRule>>) -> Self {
        let default_rules = vec![
            // multiply by zero
            SimpRule {
                name: "Rule_Mul0".to_string(),
                can_apply: |node| match node {
                    ASTNode::Multiply(left, right) => {
                        if let ASTNode::Number(value) = **left {
                            if value == 0 {
                                return true;
                            }
                        }
                        if let ASTNode::Number(value) = **right {
                            if value == 0 {
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => false,
                },
                apply: |_| ASTNode::Number(0),
            },
            // multiply by one
            SimpRule {
                name: "Rule_Mul1".to_string(),
                can_apply: |node| match node {
                    ASTNode::Multiply(left, right) => {
                        if let ASTNode::Number(value) = **left {
                            if value == 1 {
                                return true;
                            }
                        }
                        if let ASTNode::Number(value) = **right {
                            if value == 1 {
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => false,
                },
                apply: |node| {
                    if let ASTNode::Multiply(left, right) = node {
                        if let ASTNode::Number(value) = **left {
                            if value == 1 {
                                return *right.clone();
                            }
                        }
                        return *left.clone();
                    } else {
                        panic!("Should not happen");
                    };
                },
            },
            // add zero
            SimpRule {
                name: "Rule_Add0".to_string(),
                can_apply: |node| match node {
                    ASTNode::Add(left, right) => {
                        if let ASTNode::Number(value) = **left {
                            if value == 0 {
                                return true;
                            }
                        }
                        if let ASTNode::Number(value) = **right {
                            if value == 0 {
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => false,
                },
                apply: |node| {
                    if let ASTNode::Add(left, right) = node {
                        if let ASTNode::Number(value) = **left {
                            if value == 0 {
                                return *right.clone();
                            }
                        }
                        return *left.clone();
                    } else {
                        panic!("Should not happen");
                    };
                },
            },
        ];

        Simplifier {
            rules: rules.unwrap_or(default_rules),
        }
    }


    pub fn simplify(&self, node: ASTNode){
        let mut queue = vec![node];
        while !queue.is_empty() {
            let currentnode = queue.pop().unwrap();
            let mut modified = false;
            for rule in &self.rules {
                if (rule.can_apply)(&currentnode) {
                    let newnode = (rule.apply)(&currentnode);
                    match newnode {
                        ASTNode::Add(left, right) | ASTNode::Multiply(left, right) | ASTNode::Or(left, right)=> {
                            queue.push(*left);
                            queue.push(*right);
                            modified = true;
                        }
                        _ => {}
                    }
                }
            }
            if !modified {
                match currentnode {
                    ASTNode::Add(left, right) | ASTNode::Multiply(left, right) | ASTNode::Or(left, right)=> {
                        queue.push(*left);
                        queue.push(*right);
                    }
                    _ => {}
                }
            }
        }
    }
}
