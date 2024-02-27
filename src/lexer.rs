use core::panic;
use std::usize;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    //keywords
    Const,
    Function,
    If,
    Return,
    Struct,
    Enum,
    Var,
    While,
    Print, //write to default io_out
    Break,

    // booleans
    True,
    False,

    //Operators
    AndBool,
    AndInt,
    Divide,
    Equal,
    Assign,
    GreaterThan,
    LeftShift,
    LessThan,
    Minus,
    Modulo,
    Mult,
    Not,
    OrBool,
    OrInt,
    Plus,
    RightShift,
    XorBool,
    XorInt,

    //Misc
    Arrow,
    Comma,
    Comment,
    Colon,
    Identifier,
    LeftBrace,
    LeftBracket,
    LeftParen,
    Number,
    StringLiteral,
    SemiColon,
    RightBrace,
    RightBracket,
    RightParen,

    //End of File
    EOF,

    //unknown
    Unknown,
}

pub struct Lexer {
    program: Vec<u8>,
    pos: usize,
    end: usize,
}

impl Lexer {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            end: program.len() - 1,
            program,
            pos: 0,
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        let char_bytes = &self.program;

        if self.pos >= self.end {
            return Token::EOF;
        }

        while char_bytes[self.pos].is_ascii_whitespace() {
            if self.pos == self.end {
                return Token::EOF;
            }
            self.pos += 1
        }

        let token = match char_bytes[self.pos] {
            b'+' => Token::Plus,
            b'-' => {
                if self.pos == self.end {
                    Token::Minus
                } else if char_bytes[self.pos + 1] == b'>' {
                    self.pos += 1;
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            b'*' => Token::Mult,
            b'/' => {
                if self.pos == self.end {
                    Token::Divide
                } else if char_bytes[self.pos + 1] == b'/' {
                    while self.pos < self.end {
                        self.pos += 1;
                        if char_bytes[self.pos] == b'\n' {
                            break;
                        }
                    }
                    Token::Comment
                } else {
                    Token::Divide
                }
            }
            b'&' => {
                if self.pos == self.end {
                    Token::AndInt
                } else if char_bytes[self.pos + 1] == b'&' {
                    self.pos += 1;
                    Token::AndBool
                } else {
                    Token::AndInt
                }
            }
            b'|' => {
                if self.pos == self.end {
                    Token::OrInt
                } else if char_bytes[self.pos + 1] == b'|' {
                    self.pos += 1;
                    Token::OrBool
                } else {
                    Token::OrInt
                }
            }
            b'^' => {
                if self.pos == self.end {
                    Token::XorInt
                } else if char_bytes[self.pos + 1] == b'^' {
                    self.pos += 1;
                    Token::XorBool
                } else {
                    Token::XorInt
                }
            }
            b'=' => {
                if self.pos == self.end {
                    Token::Assign
                } else if char_bytes[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b',' => Token::Comma,
            b';' => Token::SemiColon,
            b':' => Token::Colon,
            b'[' => Token::LeftBracket,
            b']' => Token::RightBracket,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b'!' => Token::Not,
            b'>' => {
                if self.pos == self.end {
                    Token::GreaterThan
                } else if char_bytes[self.pos + 1] == b'>' {
                    self.pos += 1;
                    Token::RightShift
                } else {
                    Token::GreaterThan
                }
            }
            b'<' => {
                if self.pos == self.end {
                    Token::LessThan
                } else if char_bytes[self.pos + 1] == b'<' {
                    self.pos += 1;
                    Token::LeftShift
                } else {
                    Token::LessThan
                }
            }
            b'"' => {
                while self.pos < self.end {
                    self.pos += 1;
                    if char_bytes[self.pos] == b'"' {
                        break;
                    }
                }
                Token::StringLiteral
            }
            b'%' => Token::Modulo,

            //for multi character tokens
            other => {
                if other.is_ascii_alphabetic() {
                    let mut current = vec![other];

                    //eat all characters and digits (identifiers can contain digidts)
                    while self.pos < self.end {
                        if char_bytes[self.pos + 1].is_ascii_alphanumeric() {
                            current.push(char_bytes[self.pos + 1]);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }

                    let current_string = String::from_utf8(current).expect("invalid utf8");

                    //match for keywords
                    match current_string.as_str() {
                        "fn" => Token::Function,
                        "const" => Token::Const,
                        "var" => Token::Var,
                        "while" => Token::While,
                        "if" => Token::If,
                        "struct" => Token::Struct,
                        "enum" => Token::Enum,
                        "return" => Token::Return,
                        "true" => Token::True,
                        "false" => Token::False,
                        "print" => Token::Print,
                        "break" => Token::Break,

                        //if its not a keyword, it is an identifier
                        _other => Token::Identifier,
                    }
                } else if other.is_ascii_digit() {
                    //eat the didgits
                    while self.pos < self.end {
                        if char_bytes[self.pos + 1].is_ascii_digit() {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    Token::Number
                } else {
                    Token::Unknown
                }
            }
        };
        self.pos += 1;
        token
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Token;

    use super::Lexer;

    #[test]
    fn test_lexer_simple_program() {
        let program = "fn main() { return 0 }";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Function);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number);
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_new_lines() {
        let program = "fn main() {
return 0 
}";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Function);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number);
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_many_spaces() {
        let program = "fn     main    (  )      {
return     0 
}  ";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Function);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number);
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_operator() {
        let program = "fn main() {
            const a = 10;
            const b = 20;
            var c = a + b;
            var d = true;
            d = false;
            c = c - a;
            c = c | a;
            c = c % b;
            c = c ^ a;
            c = c * a;
            c = c / a;
            c = c & c;
            d = d && true;
            d = d || false;
            d = d ^^ false;
            return d == true;
        }";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Function);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        //const a = 10
        assert_eq!(lexer.get_next_token(), Token::Const);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Number);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //const b = 20;
        assert_eq!(lexer.get_next_token(), Token::Const);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Number);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //var c = a + b;
        assert_eq!(lexer.get_next_token(), Token::Var);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Plus);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //var d = true;
        assert_eq!(lexer.get_next_token(), Token::Var);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::True);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //d = false;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::False);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a - b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Minus);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a | b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::OrInt);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a % b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Modulo);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a ^ b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::XorInt);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a * b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Mult);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a / b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Divide);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //c = a & b;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::AndInt);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //d = d && true;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::AndBool);
        assert_eq!(lexer.get_next_token(), Token::True);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //d = d || false;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::OrBool);
        assert_eq!(lexer.get_next_token(), Token::False);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //d = d ^^ false;
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Assign);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::XorBool);
        assert_eq!(lexer.get_next_token(), Token::False);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        //return d == true;
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::Equal);
        assert_eq!(lexer.get_next_token(), Token::True);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);

        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        //EOF
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_comments() {
        let program = " //this is the main function
                        fn main() {
                            return 0;
                        }";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Comment);
        assert_eq!(lexer.get_next_token(), Token::Function);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }
    #[test]
    fn test_string_literals() {
        let program = " fn main() {
                            return \"this is a string Literal\";
                        }";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Function);
        assert_eq!(lexer.get_next_token(), Token::Identifier);
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::StringLiteral);
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }
}
