use super::Token;
use super::TokenType;

#[derive(Debug)]
pub struct Tokenizer {

}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {

        }
    }

    pub fn tokenize(&mut self, buf: &mut str) -> Vec<Token> {
        let mut state = TokenType::None;
        let mut tokens: Vec<Token> = Vec::new();
        let mut t_str = String::new();
        let buf: Vec<char> = buf.chars().collect();
        let mut idx = 0;
        while idx < buf.len() {
            let mut c = buf[idx];

            // escape processing
            if c == '\\' {
                idx+=1;
                if idx < buf.len() {
                    c = buf[idx];
                    state = TokenType::Str;
                } else {
                    continue;
                }
            }
            t_str.push(c);

            match state {
                TokenType::None => {
                    match c {
                        '#' => {
                            state = TokenType::Head1;
                        }
                        '-' => {
                            state = TokenType::List;
                        }
                        '0' ... '9' => {
                            state = TokenType::Order;
                        }
                        '`' => {
                            state = TokenType::CodeInline;
                        }
                        '|' => {
                            let token = Token::new(t_str.to_owned(), TokenType::TableStick);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        '[' => {
                            let token = Token::new(t_str.to_owned(), TokenType::URLLinkStart);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        ']' => {
                            let token = Token::new(t_str.to_owned(), TokenType::LinkEnd);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        '(' => {
                            let token = Token::new(t_str.to_owned(), TokenType::URLStart);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        ')' => {
                            let token = Token::new(t_str.to_owned(), TokenType::URLEnd);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        '!' => {
                            state = TokenType::ImageLinkStart;
                        }
                        '>' => {
                            state = TokenType::Quote;
                        }
                        '*' => {
                            state = TokenType::Italic;
                        }
                        '~' => {
                            state = TokenType::Strikethrough;
                        }
                        '\n' => {
                            let token = Token::new(t_str.to_owned(), TokenType::NewLine);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        '\t' => {
                            state = TokenType::Tab;
                        }
                        ' ' => {
                            state = TokenType::Space;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Head1 => {
                    match c {
                        '#' => {
                            state = TokenType::Head2;
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head1);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Head2 => {
                    match c {
                        '#' => {
                            state = TokenType::Head3;
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head2);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Head3 => {
                    match c {
                        '#' => {
                            state = TokenType::Head4;
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head3);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Head4 => {
                    match c {
                        '#' => {
                            state = TokenType::Head5;
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head4);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Head5 => {
                    match c {
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head5);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::List => {
                    match c {
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::List);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        '-' => {
                            state = TokenType::TableMiddle;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Order => {
                    match c {
                        '.' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Order);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::CodeInline => {
                    match c {
                        '`' => {
                            state = TokenType::CodeBlock;
                        }
                        _ => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::CodeInline);
                            tokens.push(token);
                            t_str.clear();
                            idx -= 1;
                            state = TokenType::None;
                        }
                    }
                }
                TokenType::CodeBlock => {
                    match c {
                        '`' => {
                            let token = Token::new(t_str.to_owned(), TokenType::CodeBlock);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::TableMiddle => {
                    match c {
                        '|' => {
                            let token = Token::new(t_str.to_owned(), TokenType::TableMiddle);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        '-' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Line);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::ImageLinkStart => {
                    match c {
                        '[' => {
                            let token = Token::new(t_str.to_owned(), TokenType::ImageLinkStart);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Quote => {
                    match c {
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Quote);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Italic => {
                    match c {
                        '*' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Bold);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::Italic);
                            tokens.push(token);
                            t_str.clear();
                            idx -= 1;
                            state = TokenType::None;
                        }
                    }
                }
                TokenType::Strikethrough => {
                    match c {
                        '~' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Strikethrough);
                            tokens.push(token);
                            t_str.clear();
                            state = TokenType::None;
                        }
                        _ => {
                            state = TokenType::Str;
                        }
                    }
                }
                TokenType::Str => {
                    match c {
                        '#' | '-' | '`' | '[' | ']' | '!' | '(' | ')' | '>' | '*' | '~' | '\n' | '|' => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::Str);
                            tokens.push(token);
                            t_str.clear();
                            idx -= 1;
                            state = TokenType::None;
                        }
                        _ => {

                        }
                    }
                }
                TokenType::Space => {
                    match c {
                        ' ' => {}
                        _ => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::Space);
                            tokens.push(token);
                            t_str.clear();
                            idx -= 1;
                            state = TokenType::None;
                        }
                    }
                }
                TokenType::Tab => {
                    match c {
                        '\t' => {}
                        _ => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::Space);
                            tokens.push(token);
                            t_str.clear();
                            idx -= 1;
                            state = TokenType::None;
                        }
                    }
                }
                _ | TokenType::TableStick |
                TokenType::URLLinkStart | TokenType::LinkEnd |
                TokenType::URLStart | TokenType::URLEnd |
                TokenType::Line | TokenType::Bold => {}
            }

            idx += 1;
        }

        tokens
    }
}