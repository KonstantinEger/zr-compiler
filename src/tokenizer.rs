use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Vw,
    Re,
    Wh,
    Stift,
    Farbe,
    BOpen,
    BClose,
    Number(&'a str),
    End
}

impl<'a> Token<'a> {
    pub fn str_value(&'a self) -> &'a str {
        match self {
            Token::Vw => "VW",
            Token::Re => "RE",
            Token::Wh => "WH",
            Token::Stift => "STIFT",
            Token::Farbe => "FARBE",
            Token::BOpen => "[",
            Token::BClose => "]",
            Token::Number(n) => n,
            Token::End => "{EOF}",
        }
    }
}

#[derive(Debug)]
pub enum TError<'a> {
    LeadingZero(&'a str),
    SyntaxError(&'a str),
}

pub type Result<'a> = std::result::Result<Token<'a>, TError<'a>>;

pub struct TokenStream<'a> {
    source: &'a str,
    done: bool,
    leading_zeros_rx: Regex,
    number_rx: Regex,
}

impl<'a> TokenStream<'a> {
    pub fn from(source: &'a str) -> Self {
        Self {
            source,
            done: false,
            leading_zeros_rx: Regex::new(r"0\d+").unwrap(),
            number_rx: Regex::new(r"^[1-9]\d*|^\d").unwrap(),
        }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.done {
            let len = self.source.len();
            if len == 0 {
                self.done = true;
                Some(Ok(Token::End))
            } else if len >= 5 && &self.source[0..5] == "STIFT" {
                self.source = self.source[5..].trim();
                Some(Ok(Token::Stift))
            } else if len >= 5 && &self.source[0..5] == "FARBE" {
                self.source = self.source[5..].trim();
                Some(Ok(Token::Farbe))
            } else if len >= 2 && &self.source[0..2] == "VW" {
                self.source = self.source[2..].trim();
                Some(Ok(Token::Vw))
            } else if len >= 2 && &self.source[0..2] == "RE" {
                self.source = self.source[2..].trim();
                Some(Ok(Token::Re))
            } else if len >= 2 && &self.source[0..2] == "WH" {
                self.source = self.source[2..].trim();
                Some(Ok(Token::Wh))
            } else if len >= 1 && &self.source[0..1] == "[" {
                self.source = self.source[1..].trim();
                Some(Ok(Token::BOpen))
            } else if len >= 1 && &self.source[0..1] == "]" {
                self.source = self.source[1..].trim();
                Some(Ok(Token::BClose))
            } else if self.leading_zeros_rx.is_match(self.source) {
                self.done = true;
                Some(Err(TError::LeadingZero(self.source)))
            } else if let Some(c) = self.number_rx.captures(self.source) {
                let m = c.get(0).unwrap();
                self.source = self.source[m.end()..].trim();
                Some(Ok(Token::Number(m.as_str())))
            } else {
                self.done = true;
                Some(Err(TError::SyntaxError(self.source)))
            }
        } else {
            None
        }
    }
}
