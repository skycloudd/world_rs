extern crate clap;
extern crate logos;

mod astbuilder;
mod compiler;
mod lexer;
mod parser;

use crate::astbuilder::build_ast;
use crate::compiler::compile_linux_nasm_x86_64;
use clap::Parser;
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::io::Write;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    input: String,

    #[clap(short, long, default_value_t = String::from("out"))]
    out: String,
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(&args.input) {
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };

    let compiled_asm = match compile_linux_nasm_x86_64(&contents) {
        Ok(c) => c,
        Err(e) => return Err(e.into()),
    };

    match fs::File::create(format!("{}.asm", &args.out)) {
        Ok(mut f) => match f.write_all(compiled_asm.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        },
        Err(e) => return Err(e.into()),
    }

    match Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(format!("{}.asm", &args.out))
        .arg("-o")
        .arg(format!("{}.o", &args.out))
        .status()
    {
        Ok(e) => match e.code() {
            Some(0) => println!("[OK] nasm -f elf64 -o {0}.o {}.asm", &args.out),
            Some(e) => return Err(format!("nasm exited with code {}", e).into()),
            None => return Err(format!("nasm exited with an error").into()),
        },
        Err(e) => return Err(e.into()),
    }

    match Command::new("gcc")
        .arg("-fno-pie")
        .arg("-m64")
        .arg("-no-pie")
        .arg("-o")
        .arg(&args.out)
        .arg(format!("{}.o", &args.out))
        .status()
    {
        Ok(e) => match e.code() {
            Some(0) => println!(
                "[OK] gcc -fno-pie -m64 -no-pie -o {} {}.o",
                &args.out, &args.out
            ),
            Some(e) => return Err(format!("gcc exited with code {}", e).into()),
            None => return Err(format!("gcc exited with an error").into()),
        },
        Err(e) => return Err(e.into()),
    }

    match Command::new("chmod").arg("+x").arg(&args.out).status() {
        Ok(e) => match e.code() {
            Some(0) => println!("[OK] chmod +x {}", &args.out),
            Some(e) => return Err(format!("chmod exited with code {}", e).into()),
            None => return Err(format!("chmod exited with an error").into()),
        },
        Err(e) => return Err(e.into()),
    }

    Ok(())
}
