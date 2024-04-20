mod compiler;
mod modules;

#[allow(unused)]
mod prelude;

#[cfg(test)]
mod test;

use std::collections::HashMap;

use crate::prelude::*;
use compiler::Compiler;
use lexer::Lexer;

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    env_logger::init();

    let files = vec![include_str!("../../nophp.php").to_string()];
    let lexer = Lexer::new(&files);
    let ast = lexer.parse().expect("Error parsing the file");

    let ast = ast[0]
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
