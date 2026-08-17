use std::fs;
use self::Token::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,       // mario
    Sub,       // mario mario
    Right,     // mario mario mario
    Left,      // mario mario mario mario
    Read,      // mario mario mario mario mario
    Write,     // mario mario mario mario mario mario
    BeginLoop, // mario mario mario mario mairo mario mario
    EndLoop,   // mario mario mario mario mairo mario mario mario
}




fn tokenize(input: &str) -> Vec<Token> {
    fn convert_marios(input: &str) -> String {
        input
            .split(|c| c == '\n' || c == '\r')
            .flat_map(|line| line.split("  "))
            .filter_map(|group| {
                let words: Vec<&str> = group.split_whitespace().collect();
                if words.is_empty() {
                    None
                } else {
                    Some(words.len().to_string())
                }
            })
            .collect::<Vec<_>>()
            .join("  ")
    }

    let converted = convert_marios(input);
    let input = converted.as_str();

    let mut tokens = Vec::<Token>::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '1' => tokens.push(Add),
            '2' => tokens.push(Sub),
            '3' => tokens.push(Right),
            '4' => tokens.push(Left),
            '5' => tokens.push(Read),
            '6' => tokens.push(Write),
            '7' => tokens.push(BeginLoop),
            '8' => tokens.push(EndLoop),
            _ => {}
        }
    }
    tokens
}

fn generate(tokens: &[Token]) -> String {
let mut output = String::from(include_str!("preface.c"));
for &token in tokens {
        match token {
            Add => {
                output.push_str("\t++*ptr;\n");
            }
            Sub => {

                output.push_str("\t--*ptr;\n");
            }
            Right => {
                output.push_str("\t++ptr;\n");
            }
            Left => {
                output.push_str("\t--ptr;\n");
            }
            Read => {
                output.push_str("\t*ptr=getchar();\n");
            }
            Write => {
                output.push_str("\tputchar(*ptr);\n");
            }
            BeginLoop => {
                output.push_str("\twhile (*ptr) {\n");
            }
            EndLoop => {
                output.push_str("\t}\n");
            }
        }
    }
    output.push_str("}\n");
    output
}



fn main() -> std::io::Result<()> {
    let input = "mario"; 

    let tokens = tokenize(input);
    let generated_code = generate(&tokens);

    fs::write("output.c", generated_code)?;

    println!("Successfully saved generated code to output.c");
    Ok(())
}