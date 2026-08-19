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
        .replace("1 1", "10");

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
        }
    }
    output.push_str("}\n");
    output
}





fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("src/input.mario")?;

    let tokens = tokenize(&input);
    let generated_code = generate(&tokens);

    fs::write("src/output.c", generated_code)?;

    println!("Successfully saved generated code to output.c");
    let output = Command::new("gcc")
        .args([
            "-O3", 
            "-S",  
            "-march=native", 
            "src/output.c", 
            "-o", 
            "src/output.s"
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
            "src/output.c", 
            "-o", 
            "src/output.exe"
        ])
        .output()
        .expect("Failed to execute gcc. Is it installed and in your PATH?");

    if output_exe.status.success() {
        println!("Successfully built output.exe");
        let _run_status = Command::new("./src/output.exe")
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
 * Execution  : 50.05 ms (Down from 109.25 ms baseline)
 * Speedup    : ~2.18x Faster
 * Throughput : ~159.8 Million Operations / sec
 *
 * Days              : 0
 * Hours             : 0
 * Minutes           : 0
 * Seconds           : 0
 * Milliseconds      : 50
 * Ticks             : 500572
 * TotalDays         : 5.79365740740741E-07
 * TotalHours        : 1.39047777777778E-05
 * TotalMinutes      : 0.000834286666666667
 * TotalSeconds      : 0.0500572
 * TotalMilliseconds : 50.0572
 * ============================================================================
 */