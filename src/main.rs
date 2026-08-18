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

    let arch = std::env::consts::ARCH;
    let input = fs::read_to_string("src/input.mario")?;

    let tokens = tokenize(&input);
    let generated_code = generate(&tokens);

    fs::write("output.c", generated_code)?;

    println!("Successfully saved generated code to output.c");
    let output = Command::new("gcc")
        .args([
            "-O3", 
            "-S", 
            "-masm=intel", 
            "-march=native", 
            "output.c", 
            "-o", 
            "output.s"
        ])
        .output()
        .expect("Failed to execute gcc. Is it installed and in your PATH?");

    if output.status.success() {
        println!("Created output.s");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("GCC Compilation Failed:\n{}", stderr);
    }

    let architecture = match arch {
        "x86" | "x86_64" => "x86",
        "arm" | "aarch64" => "arm",
        "riscv32" | "riscv64" => "riscv",
        _ => "unknown",
    };

    if architecture == "x86"{ 
        let output_exe = Command::new("gcc")
            .args([
                "-O3", 
                "-masm=intel", 
                "-march=native", 
                "output.c", 
                "-o", 
                "output.exe"
            ])
            .output()
            .expect("Failed to execute gcc. Is it installed and in your PATH?");

        if output_exe.status.success() {
            println!("Successfully built output.exe");
            let _run_status = Command::new(".\\output.exe")
                .status()
                .expect("Failed to run output.exe from src folder");
        } else {
            let stderr = String::from_utf8_lossy(&output_exe.stderr);
            eprintln!("GCC Compilation Failed:\n{}", stderr);
        }
    } else if architecture == "arm" {
        let output_exe = Command::new("gcc")
            .args([
                "-O3", 
                "-march=native", 
                "output.c", 
                "-o", 
                "output.exe"
            ])
            .output()
            .expect("Failed to execute gcc. Is it installed and in your PATH?");

        if output_exe.status.success() {
            println!("Successfully built output.exe");
            let _run_status = Command::new("./output.exe")
                .status()
                .expect("Failed to run output.exe from src folder");
        } else {
            let stderr = String::from_utf8_lossy(&output_exe.stderr);
            eprintln!("GCC Compilation Failed:\n{}", stderr);
        }
    } else if architecture == "riscv" {
        let output_exe = Command::new("riscv64-unknown-elf-gcc")
            .args([
                "-O3", 
                "-march=rv64gc", 
                "output.c", 
                "-o", 
                "output.exe"
            ])
            .output()
            .expect("Failed to execute gcc. Is it installed and in your PATH?");

        if output_exe.status.success() {
            println!("Successfully built output.exe");
            let _run_status = Command::new("./output")
                .status()
                .expect("Failed to run output.exe from src folder");
        } else {
            let stderr = String::from_utf8_lossy(&output_exe.stderr);
            eprintln!("GCC Compilation Failed:\n{}", stderr);
        }
    }




    Ok(())
}

/* 
 * Benchmark: output.exe (8,000,000 iterations generated from input.mario)
 * Compiled: GCC -O3 -march=native
 * Measured via PowerShell Measure-Command:
 * Total Time: 109.25 ms
 *
 Days               : 0
 Hours              : 0
 Minutes            : 0
 Seconds            : 0
 Milliseconds       : 109
 Ticks              : 1092465
 TotalDays          : 1.26442708333333E-06
 TotalHours         : 3.034625E-05
 TotalMinutes       : 0.001820775
 TotalSeconds       : 0.1092465
 TotalMilliseconds  : 109.2465
 */