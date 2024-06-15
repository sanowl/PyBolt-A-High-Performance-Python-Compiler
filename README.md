# PyBolt

## Overview

**PyBolt** is a high-performance Python compiler inspired by Rust. This project aims to provide an efficient way to compile Python code into optimized machine code, leveraging the best practices in lexical analysis, parsing, semantic analysis, intermediate representation (IR) generation, optimization, and code generation.

## Features

- **Lexical Analysis**: Tokenizes the input source code into a series of tokens.
- **Parsing**: Converts the tokens into an abstract syntax tree (AST).
- **Semantic Analysis**: Performs type checking and other semantic validations on the AST.
- **IR Generation**: Converts the AST into an intermediate representation.
- **Optimization**: Performs various optimizations on the IR to improve performance.
- **Code Generation**: Generates efficient machine code from the optimized IR.
- **Runtime Execution**: Executes the generated machine code.

## Project Structure

```plaintext
pybolt
├── Cargo.toml
├── src
│   ├── main.rs
│   ├── lib.rs
│   ├── lexer
│   │   ├── mod.rs
│   │   └── lexer.rs
│   ├── parser
│   │   ├── mod.rs
│   │   └── parser.rs
│   ├── semantic
│   │   ├── mod.rs
│   │   └── semantic.rs
│   ├── ir
│   │   ├── mod.rs
│   │   └── ir.rs
│   ├── optimizer
│   │   ├── mod.rs
│   │   └── optimizer.rs
│   ├── codegen
│   │   ├── mod.rs
│   │   └── codegen.rs
│   ├── runtime
│   │   ├── mod.rs
│   │   └── runtime.rs
├── tests
│   ├── lexer_tests.rs
│   ├── parser_tests.rs
│   ├── semantic_tests.rs
│   ├── optimizer_tests.rs
│   ├── codegen_tests.rs
│   ├── runtime_tests.rs
├── examples
│   ├── example1.py
│   └── example2.py
├── docs
│   ├── README.md
│   └── design.md
├── .gitignore
└── LICENSE
