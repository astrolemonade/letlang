use crate::prelude::*;
pub use super::Generator;

use letlang_ast::{
  *,
  expression::*,
};


impl<'compiler> Generator<'compiler> {
  pub fn gen_expression(&self, node: &Node<Expression>) -> CompilationResult<String> {
    let attrs = node.attrs.as_ref().unwrap();

    match node.data.as_ref() {
      Expression::Symbol(sym) => {
        self.gen_symbol(&node.location, sym, attrs.scope_id)
      },
      Expression::Literal(lit) => {
        self.gen_literal(lit)
      },
      Expression::FunctionCall(data) => {
        self.gen_function_call(&node.location, data)
      },
      Expression::EffectCall(data) => {
        self.gen_effect_call(&node.location, data)
      },
      Expression::UnaryOperation(data) => {
        self.gen_unary_op(&node.location, data)
      },
      Expression::BinaryOperation(data) => {
        self.gen_binary_op(&node.location, data)
      },
      Expression::PatternMatch(data) => {
        self.gen_pattern_match(&node.location, data)
      },
      Expression::TailRecFinal(data) => {
        self.gen_tailrec_final(data)
      },
      Expression::TailRecRecurse(data) => {
        self.gen_tailrec_recurse(&node.location, data)
      },
      Expression::FlowMatch(data) => {
        self.gen_flow_match(&node.location, data)
      },
      Expression::FlowConditional(data) => {
        self.gen_flow_cond(&node.location, data)
      },
      Expression::FlowDoBlock(data) => {
        self.gen_flow_do(&node.location, data)
      }
      _ => {
        todo!();
      }
    }
  }
}

mod symbol;
mod literal;
mod function;
mod effect;
mod unop;
mod binop;
mod pattern;
mod tailrec;
mod flow_match;
mod flow_cond;
mod flow_do;
