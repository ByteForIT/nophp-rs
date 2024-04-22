use lexer::*;
use pest::Parser;

fn main() {
    let file = include_str!("../tests/nophp.php");
    let parsed_file = NoPhpParser::parse(Rule::php_code, file);

    match parsed_file {
        Ok(val) => println!("{val:?}"),
        Err(err) => eprintln!("{err}"),
    }
}
