#![no_std]

mod expression;
mod parser;
mod token;

pub use expression::{Approx, Expression};
pub use parser::approx as parse_approximation;
pub use parser::math_expr as parse_math_expression;
pub use parser::eval as parse_evaluation;

#[derive(Debug)]
pub enum Error {
    NotEnoughMemory,
    InvalidSyntax,
}
