//! All the AST types are defined here

use std::convert::TryInto;

#[derive(Debug)]
pub enum Expression {
    Add(AddState),
    Subtract(SubtractState),
    Multiply(MultiplyState),
    Divide(DivideState),
    Raise(RaiseState),
    Value(i32),
}

pub trait Evaluatable {
    fn evaluate(&self) -> i32;
}

impl Evaluatable for Expression {
    fn evaluate(&self) -> i32 {
        match *self {
            Expression::Add(ref add) => add.evaluate(),
            Expression::Subtract(ref subtract) => subtract.evaluate(),
            Expression::Multiply(ref multiply) => multiply.evaluate(),
            Expression::Divide(ref divide) => divide.evaluate(),
            Expression::Raise(ref raise) => raise.evaluate(),
            Expression::Value(val) => val,
        }
    }
}

#[derive(Debug)]
pub struct AddState {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl AddState {
    pub fn new(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        AddState { lhs, rhs }
    }
}

impl Evaluatable for AddState {
    fn evaluate(&self) -> i32 {
        self.lhs.evaluate() + self.rhs.evaluate()
    }
}

#[derive(Debug)]
pub struct SubtractState {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl SubtractState {
    pub fn new(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        SubtractState { lhs, rhs }
    }
}

impl Evaluatable for SubtractState {
    fn evaluate(&self) -> i32 {
        self.lhs.evaluate() - self.rhs.evaluate()
    }
}

#[derive(Debug)]
pub struct MultiplyState {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl MultiplyState {
    pub fn new(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        MultiplyState { lhs, rhs }
    }
}

impl Evaluatable for MultiplyState {
    fn evaluate(&self) -> i32 {
        self.lhs.evaluate() * self.rhs.evaluate()
    }
}

#[derive(Debug)]
pub struct DivideState {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl DivideState {
    pub fn new(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        DivideState { lhs, rhs }
    }
}

impl Evaluatable for DivideState {
    fn evaluate(&self) -> i32 {
        self.lhs.evaluate() / self.rhs.evaluate()
    }
}

#[derive(Debug)]
pub struct RaiseState {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl RaiseState {
    pub fn new(lhs: Box<Expression>, rhs: Box<Expression>) -> Self {
        RaiseState { lhs, rhs }
    }
}

impl Evaluatable for RaiseState {
    fn evaluate(&self) -> i32 {
        self.lhs
            .evaluate()
            .pow(self.rhs.evaluate().try_into().unwrap())
    }
}
