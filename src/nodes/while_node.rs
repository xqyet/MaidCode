use crate::{lexing::position::Position, nodes::ast_node::AstNode};

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition_node: Box<AstNode>,
    pub body_node: Box<AstNode>,
    pub pos_start: Option<Position>,
    pub pos_end: Option<Position>,
}

impl WhileNode {
    pub fn new(condition_node: Box<AstNode>, body_node: Box<AstNode>) -> Self {
        Self {
            condition_node: condition_node.clone(),
            body_node: body_node.clone(),
            pos_start: condition_node.position_start(),
            pos_end: body_node.position_end(),
        }
    }
}
