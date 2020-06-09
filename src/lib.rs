mod lexer;
mod token;

use lexer::SimpleLexer;
use token::TokenReader;

pub fn simple_lexer_demo() {
    let mut l = SimpleLexer::new();

    let mut script = String::from("int age = 45;");
    println!("parse :{}", script);
    let mut token_reader = l.tokenize(script);
    token_reader.dump();

    //测试inta的解析
    script = String::from("inta age = 45;");
    println!("\nparse :{}", script);
    token_reader = l.tokenize(script);
    token_reader.dump();

    //测试in的解析
    script = String::from("in age = 45;");
    println!("\nparse :{}", script);
    token_reader = l.tokenize(script);
    token_reader.dump();

    //测试>=的解析
    script = String::from("age >= 45;");
    println!("\nparse :{}", script);
    token_reader = l.tokenize(script);
    token_reader.dump();

    //测试>的解析
    script = String::from("age > 45;");
    println!("\nparse :{}", script);
    token_reader = l.tokenize(script);
    token_reader.dump();
}
