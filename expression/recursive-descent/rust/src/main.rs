use rd::evaluator::Evaluator;
use rd::lexer::Lexer;
use rd::parser::Parser;
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

fn main() {
    let mut lexer;
    let mut parser;
    let mut evaluator;

    loop {
        print!(">> ");

        let input = get_input();
        lexer = Lexer::new(input);
        let tokens = lexer.scan();

        parser = Parser::new(tokens);
        let expr = parser.parse();
        println!("{:#?}", expr);

        evaluator = Evaluator::new(expr);
        println!("{}", evaluator.evaluate());
    }
}
