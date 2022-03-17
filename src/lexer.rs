use logos::Logos;
use std::error::Error;

pub fn check_tokens(mut lexer: logos::Lexer<Token>) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = vec![];

    while let Some(token) = lexer.next() {
        if let Token::Error = token {
            return Err(format!("found {} at {:?}", lexer.slice(), lexer.span().start).into());
        }

        tokens.push(token);
    }

    for token in &tokens {
        match token {
            Token::Integer(_) => (),
            Token::String(_) => (),
            Token::Plus => (),
            Token::Minus => (),
            Token::Star => (),
            Token::Slash => (),
            Token::Pop => (),
            Token::Putuint => (),
            Token::Printf => (),
            Token::Syscall0 => (),
            Token::Syscall1 => (),
            Token::Syscall2 => (),
            Token::Syscall3 => (),
            Token::Syscall4 => (),
            Token::Syscall5 => (),
            Token::Syscall6 => (),
            Token::Error => return Err(format!("found unexpected error token {:?}", token).into()),
        }
    }

    Ok(tokens)
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"[+-]?[0-9]+", |lex| lex.slice().parse())]
    Integer(i32),

    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#, |lex| lex.slice()
        .strip_prefix('"').unwrap()
        .strip_suffix('"').unwrap()
        .to_string()
    )]
    String(String),

    #[regex(r"\+")]
    Plus,

    #[regex(r"-")]
    Minus,

    #[regex(r"\*")]
    Star,

    #[regex("/")]
    Slash,

    #[regex("pop")]
    Pop,

    #[regex("putuint")]
    Putuint,

    #[regex("printf")]
    Printf,

    #[regex("syscall0")]
    Syscall0,

    #[regex("syscall1")]
    Syscall1,

    #[regex("syscall2")]
    Syscall2,

    #[regex("syscall3")]
    Syscall3,

    #[regex("syscall4")]
    Syscall4,

    #[regex("syscall5")]
    Syscall5,

    #[regex("syscall6")]
    Syscall6,

    #[error]
    #[regex(r"\s+", logos::skip)]
    #[regex(r"//[^\n]", logos::skip)]
    Error,
}
