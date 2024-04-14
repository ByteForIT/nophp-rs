use crate::lexer::lex;

#[test]
fn test_lexer() {
    let file = include_str!("../../../nophp.php");
    lex(file).unwrap();
    // cba to write a proper assert to test the tokenisation
}
