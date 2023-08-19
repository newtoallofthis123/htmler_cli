use clap::Parser;
use lexer::tokens;

use crate::lexer::tokens::Token;

#[derive(Parser, Debug)]
#[command(name="MdPDF", author="Ishan Joshi", version, about="A Simple CLI to convert MD", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=true)]
    file: String,
}

mod files;
mod lexer;
mod convert;

fn get_inputs()-> Args{
    let args = Args::parse();
    return args;
}

fn main() {
    let args = get_inputs();
    let filename = args.file;
    files::check_file_exits(&filename);
    let contents = files::read_file(&filename).unwrap();
    let mut lexer = lexer::Lexer::new(&contents);
    let mut tokens: Vec<tokens::Token> = Vec::new();
    while let Some(token) = lexer.tokenize() {
        if token == Token::EOF {
            break;
        }
        if token == Token::Skipped {
            continue;
        }
        tokens.push(token);
    }
    println!("{:?}", tokens);
    convert::make_html(tokens);
}
