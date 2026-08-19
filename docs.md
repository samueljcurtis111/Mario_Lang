# MarioLang Compiler & Transpiler Documentation

This documentation covers the architecture, syntax, compilation pipeline, and performance of the optimized **MarioLang Transpiler**. Built in Rust, this ahead-of-time (AOT) compiler takes MarioLang source code, performs peephole optimizations on the instruction stream, and emits highly optimized C code. The C code is then automatically compiled into a native binary using GCC.

---

## 1. Overview & Architecture

MarioLang is a Brainfuck-derivative language where the only valid keyword is `mario`. The compiler works by counting how many times `mario` appears in a single block of text to determine the instruction.

The transpiler uses a three-stage pipeline:
1. **Frontend / Tokenizer:** Parses the `.mario` file, counts words, and maps them to a string of intermediate IDs.
2. **Optimizer:** Runs a peephole optimization pass over the intermediate string, collapsing redundant instructions (like 16 consecutive additions) into single mathematical operations, and recognizing complex loops to replace them with direct memory manipulations.
3. **Backend / Code Gen:** Maps the optimized tokens to C code, linking it with a static 30,000-byte memory tape (`preface.c`), and invokes GCC (`-O3 -march=native`) to handle the final hardware-level loop unrolling and vectorization.

---

## 2. Language Specification

You have a tape of 30,000 memory cells (bytes), all starting at `0`, and a pointer resting on the first cell. 

Instructions are defined by **counting consecutive occurrences of the word `mario`** separated by a single space. 
To move to the next instruction, you must use **two spaces** or a **newline**.

### Base Instruction Set

| `mario` Count | Token | Brainfuck | Generated C Code | Description |
|---|---|---|---|---|
| **1** | `Add` | `+` | `++*ptr;` | Increment the current memory cell |
| **2** | `Sub` | `-` | `--*ptr;` | Decrement the current memory cell |
| **3** | `Right` | `>` | `++ptr;` | Move the pointer one cell to the right |
| **4** | `Left` | `<` | `--ptr;` | Move the pointer one cell to the left |
| **5** | `Read` | `,` | `*ptr=getchar();` | Read a character from stdin into the cell |
| **6** | `Write` | `.` | `putchar(*ptr);` | Print the current cell as an ASCII character |
| **7** | `BeginLoop` | `[` | `while (*ptr) {` | Jump past the matching EndLoop if cell is 0 |
| **8** | `EndLoop` | `]` | `}` | Jump back to matching BeginLoop |
| **9** | `Clear` | `[-]` | `*ptr = 0;` | Zero out the current memory cell |

---

## 3. How to Code in MarioLang

Writing MarioLang requires careful attention to spacing. 
* A **single space** separates the `mario`s that make up *one* instruction.
* A **double space** (or a newline) separates *different* instructions.

### Example 1: Basic Math
Let's say you want to increment a cell, move right, and decrement the next cell.
1. Increment (`+`) = 1 mario
2. Move Right (`>`) = 3 marios
3. Decrement (`-`) = 2 marios

Here is what that looks like in MarioLang:

    mario  mario mario mario  mario mario

*Notice the double spaces between the groups.* 


Alternatively, using newlines makes it much easier to read:

    mario
    mario mario mario
    mario mario


### Example 2: Printing "A" (ASCII 65)
To print "A", we need to get a memory cell to the value of 65, then print it. Instead of typing `mario` 65 times, we can use a loop.

    mario
    mario 
    mario 
    mario 
    mario 
    mario 
    mario 
    mario 
    mario 
    mario
    mario mario mario mario mario mario mario
    mario mario mario
    mario 
    mario 
    mario 
    mario 
    mario 
    mario
    mario mario mario mario
    mario mario
    mario mario mario mario mario mario mario mario
    mario mario mario
    mario mario mario mario mario mario

**What this does:**
1. Adds 10 to Cell 0 (`mario` x10) - *Note: Our peephole optimizer will compress this!*
2. Begins a loop (`7`)
3. Moves right to Cell 1 (`mario mario mario`)
4. Adds 6 to Cell 1 (`mario` x6)
5. Moves left to Cell 0 (`mario mario mario mario`)
6. Subtracts 1 from Cell 0 (`mario mario`)
7. Ends loop (`8`). By the end, Cell 1 is `10 * 6 = 60`.
8. Moves right to Cell 1 (`mario mario mario`)
9. Prints the character (`mario` x6) -> Wait, we need 65. So add 5 more before printing! 

---

## 4. The Peephole Optimizer

To achieve maximum throughput, the transpiler intercepts the raw tokens before they hit the C generator and rewrites inefficient sequences.

### Run-Length Encoding (Add2-16 / Sub2-16)
Normally, incrementing a cell by 5 looks like `1 1 1 1 1` (internally). The compiler catches sequences of repeated Adds and Subs up to 16 blocks long and collapses them.
* Raw: `++*ptr; ++*ptr; ++*ptr;`
* Optimized: `*ptr += 3;`


## 5. Usage & Pipeline

The compiler now accepts standard command-line arguments and automatically names the output files based on your input file.

### Building & Running

    # To compile the code
    mario "name_of_file".mario

    # Run it 
    "name_of_file".exe
    


### Output Artifacts
If you pass `script.mario`, the compiler will drop three artifacts in the same directory:
1. `script.c` - The raw, optimized C code generated by the transpiler.
2. `script.s` - The native assembly instructions generated by GCC.
3. `script.exe` - The final executable. The transpiler will automatically run this for you if the build succeeds.


---

## 6. Benchmarks

Thanks to the peephole optimization pass reducing instruction bloat before hitting GCC's `-O3` pipeline, we see massive performance gains over the unoptimized baseline.

* **Workload:** 8,000,000 iterations + Clear & Multi-Add optimizations
* **Pipeline:** `.mario` -> Rust Peephole Transpiler -> GCC `-O3 -march=native`
* **Hardware Profile:** Baseline x86_64

| Metric | Unoptimized (Baseline) | Peephole Optimized | Improvement |
|---|---|---|---|
| **Total Execution Time** | 109.25 ms | **45.509 ms** | **~58% Faster** |
| **Throughput** | ~73.2M Ops/sec | **~159.8M Ops/sec** | **2.18x** |

Raw Benchmark Output:

    Days              : 0
    Hours             : 0
    Minutes           : 0
    Seconds           : 0
    Milliseconds      : 45
    Ticks             : 455090
    TotalSeconds      : 0.045509
    TotalMilliseconds : 45.509