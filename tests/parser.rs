use natlint::parser::visitor::Visitable;
use natlint::parser::Parser;
use solang_parser::parse;

#[allow(dead_code)]
fn parse_source(src: &str) -> Parser {
    let (mut source, comments) = parse(src, 0).expect("failed to parse source");
    let mut doc = Parser::new(comments);
    source.visit(&mut doc).expect("failed to visit source");
    doc
}
