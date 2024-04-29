use std::usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    //keywords
    Declaration,
    Definition,
    If,
    Return,

    //Struct,
    //Enum,
    While,
    Print, //write to default io_out
    Break,
    Mut,

    // booleans
    True,
    False,

    //Operators
    AndBool,
    AndInt,
    AndIntAssign,
    Divide,
    DivideAssign,
    Equal,
    Assign,
    GreaterThan,
    LeftShift,
    LessThan,
    Minus,
    MinusAssign,
    //Modulo,
    Mult,
    MultAssign,
    Not,
    OrBool,
    OrInt,
    OrIntAssign,
    Plus,
    PlusAssign,
    Unequal,
    RightShift,
    XorBool,
    XorInt,
    XorIntAssign,

    //Misc
    Arrow,
    Comma,
    Comment(String),
    Colon,
    //Dot,
    Identifier(String),
    LeftBrace,
    LeftBracket,
    LeftParen,
    Number(i32),
    StringLiteral(String),
    SemiColon,
    RightBrace,
    RightBracket,
    RightParen,

    //primitives
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    Str,
    Char,
    Void,
    Bool,
    //Pointer,

    //End of File
    EOF,

    //unknown
    Unknown,
}

pub struct Lexer {
    program: Vec<u8>,
    pos: usize,
    end: usize,
    line: u32,
}

impl Lexer {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            end: program.len() - 1,
            program,
            pos: 0,
            line: 1,
        }
    }

    pub fn current_line(&self) -> u32 {
        self.line
    }

    pub fn get_next_token(&mut self) -> Token {
        if self.pos > self.end {
            return Token::EOF;
        }

        while self.program[self.pos].is_ascii_whitespace() {
            if self.program[self.pos] == b'\n' {
                self.line += 1;
            }
            if self.pos == self.end {
                return Token::EOF;
            }
            self.pos += 1
        }

        let token = match self.program[self.pos] {
            b'+' => {
                if self.pos == self.end {
                    Token::Plus
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::PlusAssign
                } else {
                    Token::Plus
                }
            }
            b'-' => {
                if self.pos == self.end {
                    Token::Minus
                } else if self.program[self.pos + 1] == b'>' {
                    self.pos += 1;
                    Token::Arrow
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::MinusAssign
                } else {
                    Token::Minus
                }
            }
            b'*' => {
                if self.pos == self.end {
                    Token::Mult
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::MultAssign
                } else {
                    Token::Mult
                }
            }
            b'/' => {
                if self.pos == self.end {
                    Token::Divide
                } else if self.program[self.pos + 1] == b'/' {
                    //eat '/'
                    self.pos += 1;

                    let mut comment = String::new();

                    while self.pos < self.end {
                        self.pos += 1;
                        if self.program[self.pos] == b'\n' {
                            break;
                        }
                        comment.push(self.program[self.pos] as char);
                    }
                    Token::Comment(comment)
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::DivideAssign
                } else {
                    Token::Divide
                }
            }
            b'&' => {
                if self.pos == self.end {
                    Token::AndInt
                } else if self.program[self.pos + 1] == b'&' {
                    self.pos += 1;
                    Token::AndBool
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::AndIntAssign
                } else {
                    Token::AndInt
                }
            }
            b'|' => {
                if self.pos == self.end {
                    Token::OrInt
                } else if self.program[self.pos + 1] == b'|' {
                    self.pos += 1;
                    Token::OrBool
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::OrIntAssign
                } else {
                    Token::OrInt
                }
            }
            b'^' => {
                if self.pos == self.end {
                    Token::XorInt
                } else if self.program[self.pos + 1] == b'^' {
                    self.pos += 1;
                    Token::XorBool
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::XorIntAssign
                } else {
                    Token::XorInt
                }
            }
            b'=' => {
                if self.pos == self.end {
                    Token::Assign
                } else if self.program[self.pos + 1] == b'=' {
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
            b'!' => {
                if self.pos == self.end {
                    Token::Not
                } else if self.program[self.pos + 1] == b'=' {
                    self.pos += 1;
                    Token::Unequal
                } else {
                    Token::Not
                }
            }
            b'>' => {
                if self.pos == self.end {
                    Token::GreaterThan
                } else if self.program[self.pos + 1] == b'>' {
                    self.pos += 1;
                    Token::RightShift
                } else {
                    Token::GreaterThan
                }
            }
            b'<' => {
                if self.pos == self.end {
                    Token::LessThan
                } else if self.program[self.pos + 1] == b'<' {
                    self.pos += 1;
                    Token::LeftShift
                } else {
                    Token::LessThan
                }
            }
            b'"' => {
                let mut literal = String::new();
                while self.pos < self.end {
                    self.pos += 1;
                    if self.program[self.pos] == b'"' {
                        break;
                    }
                    literal.push(self.program[self.pos] as char);
                }
                Token::StringLiteral(literal)
            }
            //b'%' => Token::Modulo,

            //for multi character tokens
            other => {
                if other.is_ascii_alphabetic() {
                    let mut current = vec![other];

                    //eat all characters and digits (identifiers can contain digidts)
                    while self.pos < self.end {
                        if self.program[self.pos + 1].is_ascii_alphanumeric() {
                            current.push(self.program[self.pos + 1]);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }

                    let current_string = String::from_utf8(current).expect("invalid utf8");

                    //match for keywords
                    match current_string.as_str() {
                        "fn" => Token::Definition,
                        "while" => Token::While,
                        "if" => Token::If,
                        //"struct" => Token::Struct,
                        //"enum" => Token::Enum,
                        "return" => Token::Return,
                        "true" => Token::True,
                        "false" => Token::False,
                        "print" => Token::Print,
                        "break" => Token::Break,
                        "u8" => Token::U8,
                        "u16" => Token::U16,
                        "u32" => Token::U32,
                        "i8" => Token::I8,
                        "i16" => Token::I16,
                        "i32" => Token::I32,
                        "str" => Token::Str,
                        "void" => Token::Void,
                        "let" => Token::Declaration,
                        "mut" => Token::Mut,
                        "char" => Token::Char,
                        "bool" => Token::Bool,

                        //if its not a keyword, it is an identifier
                        _other => Token::Identifier(current_string),
                    }
                } else if other.is_ascii_digit() {
                    //eat the didgits
                    let mut num_string = (other as char).to_string();
                    while self.pos < self.end {
                        if self.program[self.pos + 1].is_ascii_digit() {
                            num_string.push(self.program[self.pos + 1] as char);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    Token::Number(num_string.parse::<i32>().expect("invalid number"))
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
    use crate::parser::lexer::Token;

    use super::Lexer;

    #[test]
    fn test_lexer_simple_program() {
        let program = "fn main() { return 0 }";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Definition);
        assert_eq!(
            lexer.get_next_token(),
            Token::Identifier("main".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number(0));
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_new_lines() {
        let program = "fn main() {
return 0 
}";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Definition);
        assert_eq!(
            lexer.get_next_token(),
            Token::Identifier("main".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number(0));
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_many_spaces() {
        let program = "fn     main    (  )      {
return     0 
}  ";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Definition);
        assert_eq!(
            lexer.get_next_token(),
            Token::Identifier("main".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number(0));
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_operator() {
        let program = "
            +
            -
            *
            /
            +=
            -=
            *=
            /=
            &&
            ||
            ^^
            &
            |
            ^
            ==
            !=
        ";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(lexer.get_next_token(), Token::Plus);
        assert_eq!(lexer.get_next_token(), Token::Minus);
        assert_eq!(lexer.get_next_token(), Token::Mult);
        assert_eq!(lexer.get_next_token(), Token::Divide);
        assert_eq!(lexer.get_next_token(), Token::PlusAssign);
        assert_eq!(lexer.get_next_token(), Token::MinusAssign);
        assert_eq!(lexer.get_next_token(), Token::MultAssign);
        assert_eq!(lexer.get_next_token(), Token::DivideAssign);
        assert_eq!(lexer.get_next_token(), Token::AndBool);
        assert_eq!(lexer.get_next_token(), Token::OrBool);
        assert_eq!(lexer.get_next_token(), Token::XorBool);
        assert_eq!(lexer.get_next_token(), Token::AndInt);
        assert_eq!(lexer.get_next_token(), Token::OrInt);
        assert_eq!(lexer.get_next_token(), Token::XorInt);
        assert_eq!(lexer.get_next_token(), Token::Equal);
        assert_eq!(lexer.get_next_token(), Token::Unequal);
    }

    #[test]
    fn test_comments() {
        let program = " //this is the main function
                        fn main() {
                            return 0;
                        }";
        let mut lexer = Lexer::new(program.to_string().into_bytes());
        assert_eq!(
            lexer.get_next_token(),
            Token::Comment("this is the main function".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::Definition);
        assert_eq!(
            lexer.get_next_token(),
            Token::Identifier("main".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(lexer.get_next_token(), Token::Number(0));
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
        assert_eq!(lexer.get_next_token(), Token::Definition);
        assert_eq!(
            lexer.get_next_token(),
            Token::Identifier("main".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::LeftParen);
        assert_eq!(lexer.get_next_token(), Token::RightParen);
        assert_eq!(lexer.get_next_token(), Token::LeftBrace);
        assert_eq!(lexer.get_next_token(), Token::Return);
        assert_eq!(
            lexer.get_next_token(),
            Token::StringLiteral("this is a string Literal".to_string())
        );
        assert_eq!(lexer.get_next_token(), Token::SemiColon);
        assert_eq!(lexer.get_next_token(), Token::RightBrace);
        assert_eq!(lexer.get_next_token(), Token::EOF);
    }
}
