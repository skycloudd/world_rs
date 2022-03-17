use crate::astbuilder::{AstNode, Keyword, Operator};
use crate::parser::parse;
use std::error::Error;

pub fn compile_linux_nasm_x86_64(contents: &String) -> Result<String, Box<dyn Error>> {
    let ast = match parse(contents) {
        Ok(a) => a,
        Err(e) => return Err(e.into()),
    };

    // println!("{:#?}", ast);

    let mut output = String::new();
    output.push_str("default rel\n");
    output.push_str("extern printf\n");

    let mut text_section = String::new();
    let mut data_section = String::new();

    data_section.push_str("section .data\n");

    text_section.push_str("section .text\n");
    text_section.push_str("global main\n");

    text_section.push_str("main:\n");

    for node in ast {
        match node {
            AstNode::Integer(i) => {
                text_section.push_str(format!("push {}\n", i).as_str());
            }
            AstNode::String(s) => {
                let label = format!("str{}", data_section.lines().count());
                data_section
                    .push_str(format!("{} db `{}`\n", label, s.replace("`", r"\`")).as_str());
                data_section.push_str(format!("len{0} equ $-{0}\n", label).as_str());
                text_section.push_str(format!("push len{}\n", label).as_str());
                text_section.push_str(format!("push {}\n", label).as_str());
            }
            AstNode::Operator(o) => match o {
                Operator::Plus => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rbx\n");
                    text_section.push_str("add rax, rbx\n");
                    text_section.push_str("push rax\n");
                }
                Operator::Minus => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rbx\n");
                    text_section.push_str("sub rax, rbx\n");
                    text_section.push_str("push rax\n");
                }
                Operator::Star => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rbx\n");
                    text_section.push_str("mul rbx\n");
                    text_section.push_str("push rax\n");
                }
                Operator::Slash => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rbx\n");
                    text_section.push_str("xor rdx, rdx\n");
                    text_section.push_str("div rbx\n");
                    text_section.push_str("push rax\n");
                }
            },
            AstNode::Keyword(k) => match k {
                Keyword::Pop => {
                    text_section.push_str("pop rax\n");
                }
                Keyword::Putuint => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("mov ecx, 0xa\n");
                    text_section.push_str("push rcx\n");
                    text_section.push_str("mov rsi, rsp\n");
                    text_section.push_str(".toascii_digit:\n");
                    text_section.push_str("xor edx, edx\n");
                    text_section.push_str("div ecx\n");
                    text_section.push_str("add edx, '0'\n");
                    text_section.push_str("dec rsi\n");
                    text_section.push_str("mov [rsi], dl\n");
                    text_section.push_str("test eax, eax\n");
                    text_section.push_str("jnz .toascii_digit\n");
                    text_section.push_str("mov eax, 1\n");
                    text_section.push_str("mov edi, 1\n");
                    text_section.push_str("lea edx, [rsp + 1]\n");
                    text_section.push_str("sub edx, esi\n");
                    text_section.push_str("syscall\n");
                }
                Keyword::Printf => {} // TODO: implement
                Keyword::Syscall0 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
                Keyword::Syscall1 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rdi\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
                Keyword::Syscall2 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rdi\n");
                    text_section.push_str("pop rsi\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
                Keyword::Syscall3 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rdi\n");
                    text_section.push_str("pop rsi\n");
                    text_section.push_str("pop rdx\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
                Keyword::Syscall4 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rdi\n");
                    text_section.push_str("pop rsi\n");
                    text_section.push_str("pop rdx\n");
                    text_section.push_str("pop r10\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
                Keyword::Syscall5 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rdi\n");
                    text_section.push_str("pop rsi\n");
                    text_section.push_str("pop rdx\n");
                    text_section.push_str("pop r10\n");
                    text_section.push_str("pop r8\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
                Keyword::Syscall6 => {
                    text_section.push_str("pop rax\n");
                    text_section.push_str("pop rdi\n");
                    text_section.push_str("pop rsi\n");
                    text_section.push_str("pop rdx\n");
                    text_section.push_str("pop r10\n");
                    text_section.push_str("pop r8\n");
                    text_section.push_str("pop r9\n");
                    text_section.push_str("syscall\n");
                    text_section.push_str("push rax\n");
                }
            },
        }
    }

    text_section.push_str("mov rax, 60\n");
    text_section.push_str("mov rdi, 0\n");
    text_section.push_str("syscall\n");

    output = format!("{}\n{}\n{}", output, data_section, text_section);

    println!("{}", output);

    Ok(output)
}
