use lexer::*;

#[test]
fn current_spec_valid() {
    let file = include_str!("nophp.php");
    let files = &[file];
    let lexer = Lexer::new(files);
    lexer.parse().unwrap();
}
