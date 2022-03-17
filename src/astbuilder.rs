use crate::lexer::Token;
use std::error::Error;

pub fn build_ast(tokens: Vec<Token>) -> Result<Vec<AstNode>, Box<dyn Error>> {
    let mut ast = vec![];

    for token in tokens {
        match token {
            Token::Integer(i) => ast.push(AstNode::Integer(i)),
            Token::String(s) => ast.push(AstNode::String(s)),
            Token::Plus => ast.push(AstNode::Operator(Operator::Plus)),
            Token::Minus => ast.push(AstNode::Operator(Operator::Minus)),
            Token::Star => ast.push(AstNode::Operator(Operator::Star)),
            Token::Slash => ast.push(AstNode::Operator(Operator::Slash)),
            Token::Pop => ast.push(AstNode::Keyword(Keyword::Pop)),
            Token::Putuint => ast.push(AstNode::Keyword(Keyword::Putuint)),
            Token::Printf => ast.push(AstNode::Keyword(Keyword::Printf)),
            Token::Syscall0 => ast.push(AstNode::Keyword(Keyword::Syscall0)),
            Token::Syscall1 => ast.push(AstNode::Keyword(Keyword::Syscall1)),
            Token::Syscall2 => ast.push(AstNode::Keyword(Keyword::Syscall2)),
            Token::Syscall3 => ast.push(AstNode::Keyword(Keyword::Syscall3)),
            Token::Syscall4 => ast.push(AstNode::Keyword(Keyword::Syscall4)),
            Token::Syscall5 => ast.push(AstNode::Keyword(Keyword::Syscall5)),
            Token::Syscall6 => ast.push(AstNode::Keyword(Keyword::Syscall6)),

            Token::Error => return Err(format!("found unexpected error token {:?}", token).into()),
        }
    }

    Ok(ast)
}

#[derive(Debug)]
pub enum AstNode {
    Integer(i32),
    String(String),
    Operator(Operator),
    Keyword(Keyword),
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
pub enum Keyword {
    Pop,
    Putuint,
    Printf,
    Syscall0,
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,
}
