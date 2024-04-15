mod compiler;
mod lexer;
mod modules;
mod prelude;

#[cfg(test)]
mod test;

use std::collections::HashMap;

use crate::prelude::*;
use compiler::Compiler;
use lexer::lex_one;

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let ast = lex_one(include_str!("../../nophp.php")).unwrap();

    let ast = ast
        .as_array()
        .expect("Malformed AST Returned (AST does not start with an array)");

    let mut buffer = String::new();
    let mut scope_vars = HashMap::new();
    let mut compiler = Compiler::new(&mut buffer, &mut scope_vars);

    compiler.execute(ast);
    compiler.run();

    println!("==BUFSTR==");
    print!("{}", compiler.get_buffer());
    println!("==BUFEND==");

    Ok(())
}
