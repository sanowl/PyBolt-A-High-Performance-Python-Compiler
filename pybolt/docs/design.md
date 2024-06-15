# PyBolt Design Document

## Introduction

This document describes the design and architecture of PyBolt.

## Modules

### Lexer

The lexer is responsible for tokenizing the input source code.

### Parser

The parser converts tokens into an abstract syntax tree (AST).

### Semantic Analyzer

The semantic analyzer performs type checking and other semantic validations.

### IR Generator

The IR generator converts the AST into an intermediate representation (IR).

### Optimizer

The optimizer performs various code optimizations on the IR.

### Code Generator

The code generator produces the final machine code from the optimized IR.

### Runtime

The runtime executes the generated machine code.
