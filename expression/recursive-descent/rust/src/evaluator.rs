use crate::ast::Expression;

pub struct Evaluator {
    expr: Box<Expression>,
}

impl Evaluator {
    pub fn new(expr: Box<Expression>) -> Self {
        Evaluator { expr }
    }

    pub fn evaluate(&self) -> i32 {
        self.evaluate()
    }
}
