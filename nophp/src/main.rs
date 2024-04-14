mod compiler;
mod lexer;
mod modules;
mod prelude;

#[cfg(test)]
mod test;

use crate::prelude::*;
use compiler::Compiler;
use lexer::lex_one;

fn main() -> Result<()> {
    color_eyre::install().unwrap();

    let ast = lex_one(include_str!("../../nophp.php")).unwrap();

    let mut buffer = String::new();
    let mut compiler = Compiler::new(&mut buffer);

    let ast = ast
        .as_array()
        .expect("Malformed AST Returned (AST does not start with an array)");

    compiler.execute(ast);
    compiler.run();

    println!("==BUFSTR==");
    print!("{}", compiler.get_buffer());
    println!("==BUFEND==");

    Ok(())
}
