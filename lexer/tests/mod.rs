#![allow(unused)]

use lexer::*;

use pest::Parser;

//#[test]
fn parser_py() {
    let file = include_str!("nophp.php");
    let files = vec![file.to_string()];
    let lexer = Lexer::new(&files);
    lexer.parse().unwrap();
}

#[test]
fn parse_pest() {
    let file = include_str!("nophp.php");
    let parsed_file = NoPhpParser::parse(Rule::php, file).unwrap();
}
