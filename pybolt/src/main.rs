mod lexer;
mod parser;
mod semantic;
mod ir;
mod optimizer;
mod codegen;
mod runtime;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use semantic::semantic::SemanticAnalyzer;
use ir::ir::IRGenerator;
use optimizer::optimizer::Optimizer;
use codegen::codegen::CodeGenerator;
use runtime::runtime::Runtime;

fn main() {
    let input = "let x = 42;";

    let mut lexer = Lexer::new(input);
    println!("Lexing...");
    while let Some(token) = lexer.next_token() {
        println!("Token: {:?}", token);
    }

    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer);
    println!("Parsing...");
    parser.parse().expect("Parsing failed");

    let mut semantic_analyzer = SemanticAnalyzer::new();
    println!("Semantic Analysis...");
    semantic_analyzer.analyze(&parser).expect("Semantic analysis failed");

    let mut ir_generator = IRGenerator::new();
    println!("IR Generation...");
    ir_generator.generate(&parser).expect("IR generation failed");

    let mut optimizer = Optimizer::new();
    println!("Optimization...");
    optimizer.optimize(&ir_generator).expect("Optimization failed");

    let mut code_generator = CodeGenerator::new();
    println!("Code Generation...");
    code_generator.generate(&ir_generator).expect("Code generation failed");

    let mut runtime = Runtime::new();
    println!("Running...");
    runtime.run().expect("Runtime execution failed");
}