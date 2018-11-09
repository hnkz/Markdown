use super::Token;
use super::TokenType;

#[derive(Debug)]
pub struct Tokenizer {
    skip_space: bool,
    state: TokenType,
    idx: usize,
    buf: Vec<char>,
    buflen: usize,
}

impl Tokenizer {
    pub fn new(buf: &mut str) -> Tokenizer {
        let buf: Vec<char> = buf.chars().collect();
        let buflen = buf.len();
        Tokenizer {
            skip_space: false,
            state: TokenType::None,
            idx: 0usize,
            buf: buf,
            buflen: buflen,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut t_str = String::new();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.now() {

            // especially case
            if c == '\\' {
                let c = match self.next() {
                    Some(c) => c,
                    None => {
                        continue;
                    }   
                };
            } else if self.skip_space && (c == ' ' || c == '\t') {
                let _c = self.next();
                continue;
            }

            t_str.push(c);

            match self.state {
                TokenType::None => {
                    match c {
                        '#' => {
                            self.setState(TokenType::Head1);
                        }
                        '-' => {
                            self.setState(TokenType::List);
                        }
                        '0' ... '9' => {
                            self.setState(TokenType::Order);
                        }
                        '`' => {
                            self.setState(TokenType::CodeInline);
                        }
                        '|' => {
                            let token = Token::new(t_str.to_owned(), TokenType::TableStart);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::TableStart);
                        }
                        '[' => {
                            let token = Token::new(t_str.to_owned(), TokenType::URLLinkStart);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        ']' => {
                            let token = Token::new(t_str.to_owned(), TokenType::LinkEnd);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        '(' => {
                            let token = Token::new(t_str.to_owned(), TokenType::URLStart);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        ')' => {
                            let token = Token::new(t_str.to_owned(), TokenType::URLEnd);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        '!' => {
                            self.setState(TokenType::ImageLinkStart);
                        }
                        '>' => {
                            self.setState(TokenType::Quote);
                        }
                        '*' => {
                            self.setState(TokenType::Italic);
                        }
                        '~' => {
                            self.setState(TokenType::Strikethrough);
                        }
                        '\t' => {
                            self.setState(TokenType::Tab);
                        }
                        ' ' => {
                            self.setState(TokenType::Space);
                        }
                        '\n' => {
                            let token = Token::new(t_str.to_owned(), TokenType::NewLine);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Head1 => {
                    match c {
                        '#' => {
                            self.setState(TokenType::Head2);
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head1);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Head2 => {
                    match c {
                        '#' => {
                            self.setState(TokenType::Head3);
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head2);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Head3 => {
                    match c {
                        '#' => {
                            self.setState(TokenType::Head4);
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head3);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Head4 => {
                    match c {
                        '#' => {
                            self.setState(TokenType::Head5);
                        }
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head4);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Head5 => {
                    match c {
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Head5);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::List => {
                    match c {
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::List);
                            tokens.push(token);
                            t_str.clear();

                            self.setState(TokenType::None);
                        }
                        '-' => {
                            let c = if let Some(c) = self.next() {
                                c
                            } else {
                                continue;
                            };
                            if c == '-' {
                                let token = Token::new("---".to_owned(), TokenType::Line);
                                tokens.push(token);
                                t_str.clear();
                            }
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Order => {
                    match c {
                        '.' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Order);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                            // read space
                            self.next();
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::CodeInline => {
                    match c {
                        '`' => {
                            self.setState(TokenType::CodeBlock);
                        }
                        _ => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::CodeInline);
                            tokens.push(token);
                            t_str.clear();
                            self.before();
                            self.setState(TokenType::None);
                        }
                    }
                }
                TokenType::CodeBlock => {
                    match c {
                        '`' => {
                            let token = Token::new(t_str.to_owned(), TokenType::CodeBlock);
                            tokens.push(token);
                            t_str.clear();
                            self.next(); // skip newline code
                            while let Some(c) = self.next() {
                                if c == '`' {
                                    let c2 = if let Some(c2) = self.next() {
                                        c2
                                    } else {
                                        break;
                                    };
                                    let c3 = if let Some(c3) = self.next() {
                                        c3
                                    } else {
                                        break;
                                    };
                                    if c2 == '`' && c3 == '`' {
                                        let token = Token::new(t_str.to_owned(), TokenType::Str);
                                        tokens.push(token);
                                        let token = Token::new("```".to_owned(), TokenType::CodeBlock);
                                        tokens.push(token);
                                        self.setState(TokenType::None);
                                        t_str.clear();
                                        break;
                                    }
                                } else {
                                    t_str.push(c);
                                }
                            }
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::TableStart => {
                    // read head
                    let mut rownum = 0;
                    while let Some(c) = self.next() {
                        if c == '|' {
                            rownum += 1;
                            let token = Token::new(t_str.to_owned(), TokenType::TableColumn);
                            tokens.push(token);
                            t_str.clear();
                        } else if c == '\n' {
                            break;
                        } else if c == ' ' {

                        } else {
                            t_str.push(c);
                        }
                    }
                    // read middle head
                    while let Some(c) = self.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                    //read contents
                    self.next(); // read first stick
                    while let Some(c) = self.next() {
                        if c == '|' {
                            let token = Token::new(t_str.to_owned(), TokenType::TableContents);
                            tokens.push(token);
                            t_str.clear();
                        } else if c == '\n' {
                            let c = if let Some(c) = self.next() {
                                c
                            } else {
                                break;
                            };
                            if c != '|' {
                                let token = Token::new("|".to_owned(), TokenType::TableEnd);
                                tokens.push(token);
                                break;
                            }
                        } else {
                            t_str.push(c);
                        }
                    }
                    self.setState(TokenType::None);
                    
                }
                TokenType::ImageLinkStart => {
                    match c {
                        '[' => {
                            let token = Token::new(t_str.to_owned(), TokenType::ImageLinkStart);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Quote => {
                    match c {
                        ' ' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Quote);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Italic => {
                    match c {
                        '*' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Bold);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::Italic);
                            tokens.push(token);
                            t_str.clear();
                            self.before();
                            self.setState(TokenType::None);
                        }
                    }
                }
                TokenType::Strikethrough => {
                    match c {
                        '~' => {
                            let token = Token::new(t_str.to_owned(), TokenType::Strikethrough);
                            tokens.push(token);
                            t_str.clear();
                            self.setState(TokenType::None);
                        }
                        _ => {
                            self.setState(TokenType::Str);
                        }
                    }
                }
                TokenType::Str => {
                    match c {
                        '`' | '[' | ']' | '!' | '(' | ')' | '*' | '~' | '\n' | '|' => {
                            t_str.pop();
                            let token = Token::new(t_str.to_owned(), TokenType::Str);
                            tokens.push(token);
                            t_str.clear();
                            self.before();
                            self.setState(TokenType::None);
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
                            self.before();
                            self.setState(TokenType::None);
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
                            self.before();
                            self.setState(TokenType::None);
                        }
                    }
                }
                _  => {}
            }

            self.next();
        }

        tokens
    }

    fn setState(&mut self, state: TokenType) {
        self.state = state;
        match self.state {
            TokenType::Head1 => {

            }
            TokenType::Head2 => {

            }
            TokenType::Head3 => {

            }
            TokenType::Head4 => {

            }
            TokenType::Head5 => {

            }
            TokenType::List => {

            }
            TokenType::Order => {

            }
            TokenType::CodeInline => {

            }
            TokenType::CodeBlock => {

            }
            TokenType::TableStart => {

            }
            TokenType::TableColumn => {

            }
            TokenType::TableContents => {

            }
            TokenType::TableEnd => {

            }
            TokenType::URLLinkStart => {

            }
            TokenType::LinkEnd => {

            }
            TokenType::URLStart => {

            }
            TokenType::URLEnd => {

            }
            TokenType::ImageLinkStart => {

            }
            TokenType::Quote => {

            }
            TokenType::Bold => {

            }
            TokenType::Italic => {

            }
            TokenType::Strikethrough => {

            }
            TokenType::Line => {

            }
            TokenType::Str => {

            }
            TokenType::NewLine => {

            }
            TokenType::Space => {

            }
            TokenType::Tab => {

            }
            TokenType::None => {

            }
        }
    }

    fn next(&mut self) -> Option<char> {
        self.idx += 1;
        if self.idx < self.buflen {
            Some(self.buf[self.idx])
        } else {
            None
        }
    }

    fn now(&mut self) -> Option<char> {
        if self.idx < self.buflen {
            Some(self.buf[self.idx])
        } else {
            None
        }
    }

    fn before(&mut self) -> Option<char> {
        self.idx -= 1;
        if self.idx < self.buflen {
            Some(self.buf[self.idx])
        } else {
            None
        }
    }
}