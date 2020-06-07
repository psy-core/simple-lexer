use simple_lexer::{DFAState, SimpleToken, SimpleTokenReader, Token, TokenReader, TokenType};

fn main() {
    let mut lexer = SimpleLexer::new();

    let mut script = String::from("int age = 45;");
    println!("parse :{}", script);
    let mut token_reader = lexer.tokenize(script);
    token_reader.dump();

    //测试inta的解析
    script = String::from("inta age = 45;");
    println!("\nparse :{}", script);
    token_reader = lexer.tokenize(script);
    token_reader.dump();

    //测试in的解析
    script = String::from("in age = 45;");
    println!("\nparse :{}", script);
    token_reader = lexer.tokenize(script);
    token_reader.dump();

    //测试>=的解析
    script = String::from("age >= 45;");
    println!("\nparse :{}", script);
    token_reader = lexer.tokenize(script);
    token_reader.dump();

    //测试>的解析
    script = String::from("age > 45;");
    println!("\nparse :{}", script);
    token_reader = lexer.tokenize(script);
    token_reader.dump();
}

struct SimpleLexer {
    token_text: String,
    tokens: Vec<Box<dyn Token>>,
    token: SimpleToken,
}

impl SimpleLexer {
    fn new() -> Self {
        SimpleLexer {
            token_text: String::new(),
            tokens: Vec::new(),
            token: SimpleToken::new(TokenType::Identifier, String::new()),
        }
    }

    fn init_token(&mut self, c: char) -> DFAState {
        if self.token_text.len() > 0 {
            self.token.text = self.token_text.clone();
            self.tokens.push(Box::new(self.token.clone()));
            self.token_text = String::new();
            self.token = SimpleToken::new(TokenType::Identifier, String::new())
        }
        let new_state: DFAState;
        if is_alpha(c) {
            //第一个字符是字母
            if c == 'i' {
                new_state = DFAState::IdInt1;
            } else {
                new_state = DFAState::Id; //进入Id状态
            }
            self.token.token_type = TokenType::Identifier;
            self.token_text.push(c);
        } else if is_digit(c) {
            //第一个字符是数字
            new_state = DFAState::IntLiteral;
            self.token.token_type = TokenType::IntLiteral;
            self.token_text.push(c);
        } else if c == '>' {
            //第一个字符是>
            new_state = DFAState::GT;
            self.token.token_type = TokenType::GT;
            self.token_text.push(c);
        } else if c == '+' {
            new_state = DFAState::Plus;
            self.token.token_type = TokenType::Plus;
            self.token_text.push(c);
        } else if c == '-' {
            new_state = DFAState::Minus;
            self.token.token_type = TokenType::Minus;
            self.token_text.push(c);
        } else if c == '*' {
            new_state = DFAState::Star;
            self.token.token_type = TokenType::Star;
            self.token_text.push(c);
        } else if c == '/' {
            new_state = DFAState::Slash;
            self.token.token_type = TokenType::Slash;
            self.token_text.push(c);
        } else if c == ';' {
            new_state = DFAState::SemiColon;
            self.token.token_type = TokenType::SemiColon;
            self.token_text.push(c);
        } else if c == '(' {
            new_state = DFAState::LeftParen;
            self.token.token_type = TokenType::LeftParen;
            self.token_text.push(c);
        } else if c == ')' {
            new_state = DFAState::RightParen;
            self.token.token_type = TokenType::RightParen;
            self.token_text.push(c);
        } else if c == '=' {
            new_state = DFAState::Assignment;
            self.token.token_type = TokenType::Assignment;
            self.token_text.push(c);
        } else {
            new_state = DFAState::Initial; // skip all unknown patterns
        }
        new_state
    }

    fn tokenize(&mut self, code: String) -> SimpleTokenReader {
        self.tokens = Vec::new();
        let mut state = DFAState::Initial;
        for c in code.chars() {
            // println!("{}", c);
            match &state {
                DFAState::Initial => state = self.init_token(c),
                DFAState::Id => {
                    if is_alpha(c) || is_digit(c) {
                        self.token_text.push(c);
                    } else {
                        state = self.init_token(c);
                    }
                }
                DFAState::GT => {
                    if c == '=' {
                        self.token.token_type = TokenType::GE; //转换成GE
                        state = DFAState::GE;
                        self.token_text.push(c);
                    } else {
                        state = self.init_token(c);
                    }
                }
                DFAState::GE
                | DFAState::Assignment
                | DFAState::Plus
                | DFAState::Minus
                | DFAState::Star
                | DFAState::Slash
                | DFAState::SemiColon
                | DFAState::LeftParen
                | DFAState::RightParen => state = self.init_token(c),
                DFAState::IntLiteral => {
                    if is_digit(c) {
                        self.token_text.push(c); //继续保持在数字字面量状态
                    } else {
                        state = self.init_token(c); //退出当前状态，并保存Token
                    }
                }
                DFAState::IdInt1 => {
                    if c == 'n' {
                        state = DFAState::IdInt2;
                        self.token_text.push(c);
                    } else if is_digit(c) || is_alpha(c) {
                        state = DFAState::Id; //切换回Id状态
                        self.token_text.push(c);
                    } else {
                        state = self.init_token(c);
                    }
                }
                DFAState::IdInt2 => {
                    if c == 't' {
                        state = DFAState::IdInt3;
                        self.token_text.push(c);
                    } else if is_digit(c) || is_alpha(c) {
                        state = DFAState::Id; //切换回id状态
                        self.token_text.push(c);
                    } else {
                        state = self.init_token(c);
                    }
                }
                DFAState::IdInt3 => {
                    if is_blank(c) {
                        self.token.token_type = TokenType::Int;
                        state = self.init_token(c);
                    } else if is_digit(c) || is_alpha(c) {
                        state = DFAState::Id; //切换回id状态
                        self.token_text.push(c);
                    } else {
                        state = self.init_token(c);
                    }
                }
                _other => {}
            };
        }
        if self.token_text.len() > 0 {
            self.init_token(' ');
        }
        SimpleTokenReader::new(&self.tokens)
    }
}

fn is_alpha(ch: char) -> bool {
    return ch >= 'a' && ch <= 'z' || ch >= 'A' && ch <= 'Z';
}

fn is_digit(ch: char) -> bool {
    return ch >= '0' && ch <= '9';
}

fn is_blank(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\n';
}
