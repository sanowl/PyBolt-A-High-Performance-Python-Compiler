
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
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.50.0 or later)

### Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/sanowl/PyBolt-A-High-Performance-Python-Compiler.git
cd PyBolt-A-High-Performance-Python-Compiler
```

### Building the Project

To build the project, run:

```bash
cargo build
```

### Running the Project

To run the project, execute:

```bash
cargo run
```

### Testing

To run the tests, use:

```bash
cargo test
```

## Examples

You can find example Python code in the `examples` directory. These examples can be used to test the functionality of PyBolt.

## Documentation

Detailed documentation can be found in the `docs` directory. This includes an overview of the project and a design document that explains the architecture and design decisions.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## Contact

If you have any questions or feedback, feel free to reach out at [sanowl98@gmail.com](mailto:sanowl98@gmail.com).

---

Thank you for using PyBolt! We hope it makes your Python development more efficient and enjoyable.
```


This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## Contact

If you have any questions or feedback, feel free to reach out at [sanowl98@gmail.com](mailto:sanowl98@gmail.com).

---

Thank you for using PyBolt! We hope it makes your Python development more efficient and enjoyable.
```



