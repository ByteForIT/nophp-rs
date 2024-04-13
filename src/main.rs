mod compiler;
mod lexer;
mod modules;
mod prelude;

#[cfg(test)]
mod test;

use crate::prelude::*;
use compiler::Compiler;
use lexer::lex;

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let ast = lex(include_str!("../nophp.php")).unwrap();

    let mut compiler = Compiler::new();

    let ast = ast
        .as_array()
        .expect("Malformed AST Returned (AST does not start with an array)");

    compiler.execute(ast);
    compiler.run();

    Ok(())
}
