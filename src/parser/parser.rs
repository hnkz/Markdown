use super::Token;
use super::TokenType;
use super::ParseState;
use super::object::MDObject;
use super::object::{
    Head1, Head2, Head3, Head4, Head5,
    CodeBlock, Paragraph,
};
use super::object::string::{
    Str, StrType,
};
use super::object::container:: {
    Container, List, Order, Quote, Inner,
};

#[derive(Debug)]
pub struct Parser {
    idx: usize,
    tokenlen: usize,
    tokens: Vec<Token>,
    state: ParseState,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        // for token in &tokens {
        //     println!("{:?}", token);
        // }
        Parser {
            idx: 0,
            tokenlen: tokens.len(),
            tokens: tokens,
            state: ParseState::None,
        }
    }

    pub fn parse(&mut self) -> Vec<Box<MDObject>> {
        let mut objs: Vec<Box<MDObject>> = Vec::new();
        // let mut lcontainer: Container<List>;
        // let mut ocontainer: Container<Order>;
        // let mut qcontainer: Container<Quote>;
        let mut strs: Str = Str::new();

        while let Some(token) = self.next() {
            match self.state {
                ParseState::None => {
                    match token.get_state() {
                        TokenType::Head1 => {
                            self.state = ParseState::Head1;
                        }
                        TokenType::Head2 => {
                            self.state = ParseState::Head2;
                        }
                        TokenType::Head3 => {
                            self.state = ParseState::Head3;
                        }
                        TokenType::Head4 => {
                            self.state = ParseState::Head4;
                        }
                        TokenType::Head5 => {
                            self.state = ParseState::Head5;
                        }
                        TokenType::List => {
                            let lcontainer = match self.get_recursive_container_for_list(0) {
                                Some(lcontainer) => lcontainer,
                                None => continue   
                            };
                            objs.push(Box::new(lcontainer));
                        }
                        TokenType::Order => {
                            
                        }
                        TokenType::CodeInline => {
                            let val = if let Some(token) = self.next() {
                                if TokenType::Str == token.get_state() {
                                    token.get_val()
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            };

                            if let Some(token) = self.next() {
                                if token.get_state() == TokenType::CodeInline {
                                    strs.push(StrType::CodeInline(val));
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            };

                            self.state = ParseState::Paragraph;
                        }
                        TokenType::CodeBlock => {
                            let val = if let Some(token) = self.next() {
                                if TokenType::Str == token.get_state() {
                                    token.get_val()
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            };

                            if let Some(token) = self.next() {
                                if token.get_state() == TokenType::CodeBlock {
                                    let obj = CodeBlock::new(val);
                                    objs.push(Box::new(obj));
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        }
                        TokenType::TableStart => {
                            
                        }
                        TokenType::URLLinkStart => {

                        }
                        TokenType::ImageLinkStart => {

                        }
                        TokenType::Quote => {

                        }
                        TokenType::Line => {

                        }
                        TokenType::Str => {
                            strs.push(StrType::Normal(token.get_val()));
                            self.state = ParseState::Paragraph;
                        }
                        _ => {

                        }
                    }
                }
                ParseState::Head1 => {
                    match token.get_state() {
                        TokenType::Str => {
                            let val = token.get_val();
                            let obj = Head1::new(val);
                            objs.push(Box::new(obj));
                        }
                        TokenType::NewLine => {
                            self.state = ParseState::None;
                        }
                        _ => {}
                    }
                }
                ParseState::Head2 => {
                    match token.get_state() {
                        TokenType::Str => {
                            let val = token.get_val();
                            let obj = Head2::new(val);
                            objs.push(Box::new(obj));
                        }
                        TokenType::NewLine => {
                            self.state = ParseState::None;
                        }
                        _ => {}
                    }
                }
                ParseState::Head3 => {
                    match token.get_state() {
                        TokenType::Str => {
                            let val = token.get_val();
                            let obj = Head3::new(val);
                            objs.push(Box::new(obj));
                        }
                        TokenType::NewLine => {
                            self.state = ParseState::None;
                        }
                        _ => {}
                    }
                }
                ParseState::Head4 => {
                    match token.get_state() {
                        TokenType::Str => {
                            let val = token.get_val();
                            let obj = Head4::new(val);
                            objs.push(Box::new(obj));
                        }
                        TokenType::NewLine => {
                            self.state = ParseState::None;
                        }
                        _ => {}
                    }
                }
                ParseState::Head5 => {
                    match token.get_state() {
                        TokenType::Str => {
                            let val = token.get_val();
                            let obj = Head5::new(val);
                            objs.push(Box::new(obj));
                        }
                        TokenType::NewLine => {
                            self.state = ParseState::None;
                        }
                        _ => {}
                    }
                }
                ParseState::Paragraph => {
                    self.before();
                    let line = if let Some(line) = self.get_line_str() {
                        line   
                    } else {
                        self.state = ParseState::None;
                        self.before();
                        objs.push(Box::new(Paragraph::new(strs)));
                        strs = Str::new();
                        continue;
                    };
                    strs.add_vec(line);
                }
                _ => {}
            }
        }
        objs
    }

    fn get_recursive_container_for_list(&mut self, level: u32) -> Option<Container<List>> {
        let mut recursive_flag = false;
        let mut container = Container::<List>::new();
        let mut newlevel = level;
        // uuuuuuuuuuuuuuuum
        self.before();
        while let Some(token) = self.next() {
            match token.get_state() {
                TokenType::List => {
                    if newlevel < level {
                        self.before();
                        break;
                    }
                    if recursive_flag {
                        // read inner
                        let inner_container = match self.get_recursive_container_for_list(level + 1) {
                            Some(inner) => inner,
                            None => { return None; }
                        };
                        container.push(Inner::Container(Box::new(inner_container)));
                        recursive_flag = false;
                    } else {
                        let line = match self.get_line_str() {
                            Some(line) => line,
                            None => { return None; }
                        };
                        container.push(Inner::Val(List::new(line)));
                    }
                    newlevel = 0;
                }
                TokenType::Space => {
                    newlevel = 1;
                    while let Some(token) = self.next() {
                        if token.get_state() == TokenType::Space {
                            newlevel += 1;
                        } else {
                            break;
                        }
                    }

                    newlevel /= 2;

                    self.before();
                    if level < newlevel {
                        recursive_flag = true;
                    } else if newlevel < level {
                        for _ in 0..(newlevel*2) {
                            self.before();
                        }
                        break;
                    }
                }
                TokenType::Tab => {
                    let mut newlevel = 1;
                    while let Some(token) = self.next() {
                        if token.get_state() == TokenType::Space {
                            newlevel += 1;
                        } else {
                            for _ in 0..newlevel*2 {
                                self.before();
                            }
                            break;
                        }
                    }

                    if level < newlevel {
                        recursive_flag = true;
                    } else if newlevel < level {
                        for _ in 0..(newlevel*2) {
                            self.before();
                        }
                        break;
                    }

                    self.before();
                }
                _ => {
                    self.before();
                    break;
                }
            }
        }

        Some(container)
    }

    fn get_line_str(&mut self) -> Option<Str> {
        let mut strs = Str::new();
        while let Some(token) = self.next() {
            if token.get_state() == TokenType::NewLine {
                break;
            }

            let val = match token.get_state() {
                TokenType::Bold => {
                    let token = if let Some(token) = self.next() {
                        token
                    } else { break; };
                    
                    match self.next() {
                        Some(token) => {
                            if token.get_state() != TokenType::Bold {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }

                    StrType::Bold(token.get_val())
                }
                TokenType::Italic => {
                    let token = if let Some(token) = self.next() {
                        token
                    } else { break; };
                    
                    match self.next() {
                        Some(token) => {
                            if token.get_state() != TokenType::Italic {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                    StrType::Italic(token.get_val())
                }
                TokenType::CodeInline => {
                    let token = if let Some(token) = self.next() {
                        token
                    } else { break; };
                    
                    match self.next() {
                        Some(token) => {
                            if token.get_state() != TokenType::CodeInline {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                    StrType::CodeInline(token.get_val())
                }
                TokenType::Strikethrough => {
                    let token = if let Some(token) = self.next() {
                        token
                    } else { break; };
                    
                    match self.next() {
                        Some(token) => {
                            if token.get_state() != TokenType::Strikethrough {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                    StrType::Strikethrough(token.get_val())
                }
                TokenType::Str => {
                    StrType::Normal(token.get_val())
                }
                TokenType::URLLinkStart => {
                    let val = match self.get_url() {
                        Some(val) => val,
                        None => { return None; }   
                    };
                    StrType::Link(val)
                }
                _ => {
                    return None;
                }
            };
            strs.push(val);
        }
        Some(strs)
    }

    fn get_url(&mut self) -> Option<(String, String)> {
        // lazy implementation
        let alt = if let Some(alt) = self.next() {
            alt.get_val()
        } else { return None };
        
        self.next(); // linkend
        self.next(); // urlstart

        let url = if let Some(url) = self.next() {
            url.get_val()
        } else { return None };

        self.next(); // urlend

        Some((alt, url))
    }

    fn next(&mut self) -> Option<Token> {
        if self.idx < self.tokenlen {
            let token = self.tokens[self.idx].clone();
            self.idx += 1;
            Some(token)
        } else {
            None
        }
    }

    fn before(&mut self) -> Option<Token> {
        if self.idx < self.tokenlen && 0 < self.idx  {
            let token = self.tokens[self.idx].clone();
            self.idx -= 1;
            Some(token)
        } else {
            None
        }
    }
}