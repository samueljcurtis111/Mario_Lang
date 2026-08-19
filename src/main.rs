use std::fs;
use self::Token::*;
use std::process::Command;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,       // mario 
    Sub,       // mario mario
    Right,     // mario mario mario
    Left,      // mario mario mario mario
    Read,      // mario mario mario mario mario
    Write,     // mario mario mario mario mario mario
    BeginLoop, // mario mario mario mario mario mario mario
    EndLoop,   // mario mario mario mario mario mario mario mario
    Clear,     // mario mario mario mario mario mario mario mario mario
    Add2,
    Add3,
    Add4,
    Add5,
    Add6,
    Add7,
    Add8,
    Add9,
    Add10,
    Add11,
    Add12,
    Add13,
    Add14,
    Add15,
    Add16,
    Sub2,
    Sub3,
    Sub4,
    Sub5,
    Sub6,
    Sub7,
    Sub8,
    Sub9,
    Sub10,
    Sub11,
    Sub12,
    Sub13,
    Sub14,
    Sub15,
    Sub16,
    MoveRight,
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
            .join(" ")
    }

    let converted = convert_marios(input);
    let result = converted
        .replace("2 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2", "25")
        .replace("2 2 2 2 2 2 2 2 2 2 2 2 2 2 2", "26")
        .replace("2 2 2 2 2 2 2 2 2 2 2 2 2 2", "27")
        .replace("2 2 2 2 2 2 2 2 2 2 2 2 2", "28")
        .replace("2 2 2 2 2 2 2 2 2 2 2 2", "29")
        .replace("2 2 2 2 2 2 2 2 2 2 2", "30")
        .replace("2 2 2 2 2 2 2 2 2 2", "31")
        .replace("2 2 2 2 2 2 2 2 2", "32")
        .replace("2 2 2 2 2 2 2 2", "33")
        .replace("2 2 2 2 2 2 2", "34")
        .replace("2 2 2 2 2 2", "35")
        .replace("2 2 2 2 2", "36")
        .replace("2 2 2 2", "37")
        .replace("2 2 2", "38")
        .replace("2 2", "39")
        .replace("1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1", "24")
        .replace("1 1 1 1 1 1 1 1 1 1 1 1 1 1 1", "23")
        .replace("1 1 1 1 1 1 1 1 1 1 1 1 1 1", "22")
        .replace("1 1 1 1 1 1 1 1 1 1 1 1 1", "21")
        .replace("1 1 1 1 1 1 1 1 1 1 1 1", "20")
        .replace("1 1 1 1 1 1 1 1 1 1 1", "19")
        .replace("1 1 1 1 1 1 1 1 1 1", "18")
        .replace("1 1 1 1 1 1 1 1 1", "17")
        .replace("1 1 1 1 1 1 1 1", "16")
        .replace("1 1 1 1 1 1 1", "15")
        .replace("1 1 1 1 1 1", "14")
        .replace("1 1 1 1 1", "13")
        .replace("1 1 1 1", "12")
        .replace("1 1 1", "11")
        .replace("1 1", "10")
        .replace("7 2 4 1 3 8", "40");

    println!("{result}");
    let _input = converted.as_str();

    let mut tokens = Vec::<Token>::new();
    for word in result.split_whitespace() {
        match word {
            "1" => tokens.push(Add),
            "2" => tokens.push(Sub),
            "3" => tokens.push(Right),
            "4" => tokens.push(Left),
            "5" => tokens.push(Read),
            "6" => tokens.push(Write),
            "7" => tokens.push(BeginLoop),
            "8" => tokens.push(EndLoop),
            "9" => tokens.push(Clear),
            "10" => tokens.push(Token::Add2),
            "11" => tokens.push(Token::Add3),
            "12" => tokens.push(Token::Add4),
            "13" => tokens.push(Token::Add5),
            "14" => tokens.push(Token::Add6),
            "15" => tokens.push(Token::Add7),
            "16" => tokens.push(Token::Add8),
            "17" => tokens.push(Token::Add9),
            "18" => tokens.push(Token::Add10),
            "19" => tokens.push(Token::Add11),
            "20" => tokens.push(Token::Add12),
            "21" => tokens.push(Token::Add13),
            "22" => tokens.push(Token::Add14),
            "23" => tokens.push(Token::Add15),
            "24" => tokens.push(Token::Add16),
            "25" => tokens.push(Token::Sub2),
            "26" => tokens.push(Token::Sub3),
            "27" => tokens.push(Token::Sub4),
            "28" => tokens.push(Token::Sub5),
            "29" => tokens.push(Token::Sub6),
            "30" => tokens.push(Token::Sub7),
            "31" => tokens.push(Token::Sub8),
            "32" => tokens.push(Token::Sub9),
            "33" => tokens.push(Token::Sub10),
            "34" => tokens.push(Token::Sub11),
            "35" => tokens.push(Token::Sub12),
            "36" => tokens.push(Token::Sub13),
            "37" => tokens.push(Token::Sub14),
            "38" => tokens.push(Token::Sub15),
            "39" => tokens.push(Token::Sub16),
            "40" => tokens.push(Token::MoveRight),
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
            Clear => {
                output.push_str("\t*ptr = 0;\n");
            }
            Add2 => {
                output.push_str("\t*ptr += 2;\n");
            }
            Add3 => {
                output.push_str("\t*ptr += 3;\n");
            }
            Add4 => {
                output.push_str("\t*ptr += 4;\n");
            }
            Add5 => {
                output.push_str("\t*ptr += 5;\n");
            }
            Add6 => {
                output.push_str("\t*ptr += 6;\n");
            }
            Add7 => {
                output.push_str("\t*ptr += 7;\n");
            }
            Add8 => {
                output.push_str("\t*ptr += 8;\n");
            }
            Add9 => {
                output.push_str("\t*ptr += 9;\n");
            }
            Add10 => {
                output.push_str("\t*ptr += 10;\n");
            }
            Add11 => {
                output.push_str("\t*ptr += 11;\n");
            }
            Add12 => {
                output.push_str("\t*ptr += 12;\n");
            }
            Add13 => {
                output.push_str("\t*ptr += 13;\n");
            }
            Add14 => {
                output.push_str("\t*ptr += 14;\n");
            }
            Add15 => {
                output.push_str("\t*ptr += 15;\n");
            }
            Add16 => {
                output.push_str("\t*ptr += 16;\n");
            }
            Sub2 => {
                output.push_str("\t*ptr -= 2;\n");
            }
            Sub3 => {
                output.push_str("\t*ptr -= 3;\n");
            }
            Sub4 => {
                output.push_str("\t*ptr -= 4;\n");
            }
            Sub5 => {
                output.push_str("\t*ptr -= 5;\n");
            }
            Sub6 => {
                output.push_str("\t*ptr -= 6;\n");
            }
            Sub7 => {
                output.push_str("\t*ptr -= 7;\n");
            }
            Sub8 => {
                output.push_str("\t*ptr -= 8;\n");
            }
            Sub9 => {
                output.push_str("\t*ptr -= 9;\n");
            }
            Sub10 => {
                output.push_str("\t*ptr -= 10;\n");
            }
            Sub11 => {
                output.push_str("\t*ptr -= 11;\n");
            }
            Sub12 => {
                output.push_str("\t*ptr -= 12;\n");
            }
            Sub13 => {
                output.push_str("\t*ptr -= 13;\n");
            }
            Sub14 => {
                output.push_str("\t*ptr -= 14;\n");
            }
            Sub15 => {
                output.push_str("\t*ptr -= 15;\n");
            }
            Sub16 => {
                output.push_str("\t*ptr -= 16;\n");
            }
            MoveRight => {
                output.push_str("\tptr[1] += *ptr;\n \t*ptr = 0;\n")
            }
        }
    }
    output.push_str("}\n");
    output
}





fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: mario <file.mario>");
        return Ok(());
    }

    let input = fs::read_to_string(&args[1])?;

    let tokens = tokenize(&input);
    let generated_code = generate(&tokens);

    let base = std::path::Path::new(&args[1]).with_extension("");
    let c_file = format!("{}.c", base.display());
    let s_file = format!("{}.s", base.display());
    let exe_file = format!("{}.exe", base.display());

    fs::write(&c_file, generated_code)?;

    println!("Successfully saved generated code to output.c");
    let output = Command::new("gcc")
        .args([
"-O3", 
            "-S",  
            "-march=native", 
            &c_file, 
            "-o", 
            &s_file
        ])
        .output()
        .expect("Failed to execute gcc. Is it installed and in your PATH?");

    if output.status.success() {
        println!("Created output.s");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("GCC Compilation Failed:\n{}", stderr);
    }

    let output_exe = Command::new("gcc")
        .args([
            "-O3",  
            "-march=native", 
            &c_file, 
            "-o", 
            &exe_file
        ])
        .output()
        .expect("Failed to execute gcc. Is it installed and in your PATH?");

    if output_exe.status.success() {
        println!("Successfully built {}", exe_file);
        let _run_status = Command::new(&exe_file)
            .status()
            .expect("Failed to run output.exe from src folder");
    } else {
        let stderr = String::from_utf8_lossy(&output_exe.stderr);
        eprintln!("GCC Compilation Failed:\n{}", stderr);
    }
 

    Ok(())
}

/*
 * ============================================================================
 * BENCHMARK RESULT: Optimized Mario AST Compiler
 * ============================================================================
 * Workload   : 8,000,000 iterations + Clear & Multi-Add optimizations
 * Pipeline   : .mario -> Rust Peephole Transpiler -> GCC -O3 -march=native
 * Execution  : 45.509 ms (Down from 109.25 ms baseline)
 * Throughput : ~159.8 Million Operations / sec
 *
Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 45
Ticks             : 455090
TotalDays         : 5.26724537037037E-07
TotalHours        : 1.26413888888889E-05
TotalMinutes      : 0.000758483333333333
TotalSeconds      : 0.045509
TotalMilliseconds : 45.509
 * ============================================================================
 */