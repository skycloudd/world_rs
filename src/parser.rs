use crate::astbuilder::AstNode;
use crate::build_ast;
use crate::lexer::{check_tokens, Token};
use crate::logos::Logos;
use std::error::Error;

pub fn parse(contents: &str) -> Result<Vec<AstNode>, Box<dyn Error>> {
    let lexer = Token::lexer(contents);

    let tokens = match check_tokens(lexer) {
        Ok(t) => t,
        Err(e) => return Err(e.into()),
    };

    let ast = match build_ast(tokens) {
        Ok(a) => a,
        Err(e) => return Err(e.into()),
    };

    Ok(ast)
}
