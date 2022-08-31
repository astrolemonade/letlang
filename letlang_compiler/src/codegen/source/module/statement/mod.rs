use crate::prelude::*;
pub use super::Generator;

use letlang_ast::*;

impl<'compiler> Generator<'compiler> {
  pub fn gen_statement(&self, node: &Node<Statement>) -> CompilationResult<String> {
    match node.data.as_ref() {
      Statement::Import(data) => {
        Ok(self.gen_statement_import(data))
      },
      Statement::FuncDecl(data) => {
        self.gen_statement_func(&node.location, data)
      }
      _ => todo!()
    }
  }
}

mod import;
mod function;
