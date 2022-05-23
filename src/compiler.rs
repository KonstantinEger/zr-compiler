use crate::tokenizer::{TokenStream, Token};

pub struct Compiler<'a> {
    ts: TokenStream<'a>,
    curr_token: Token<'a>,
}

impl<'a> Compiler<'a> {
    pub fn init(source: &'a str) -> Self {
        let mut ts = TokenStream::from(source);
        let ct = ts.next().unwrap().unwrap();
        Self {
            ts,
            curr_token: ct,
        }
    }

    fn next_token(&mut self) {
        if let Some(t) = self.ts.next() {
            self.curr_token = t.unwrap();
        }
    }

    pub fn compile(&mut self) -> String {
        self.program()
    }

    fn program(&mut self) -> String {
        format!(r###"
<html>
<head>
</head>
<body>
<canvas id="canv"></canvas>
<script>
const canvas = document.querySelector("#canv");
const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, canvas.width, canvas.height);
ctx.fillStyle = "red";
ctx.fillRect(10, 20, 30, 40);

const turtle = {{
    move(x) {{}},
    turnRight(x) {{}},
    setStrokeWidth(x) {{}},
    setColor(c) {{}},
}};
{}
</script>
</body>
</html>
        "###, self.anweisungen())
    }

    fn anweisungen(&mut self) -> String {
        match self.curr_token {
            Token::End | Token::BClose => String::new(),
            _ => {
                let mut result = String::new();
                result.push_str(&self.anweisung());
                result.push_str(&self.anweisungen());
                result
            }
        }
    }

    fn anweisung(&mut self) -> String {
        match self.curr_token {
            Token::Vw => {
                self.next_token();
                match self.curr_token {
                    Token::Number(n) => {
                        self.next_token();
                        format!("turtle.move({});\n", n)
                    },
                    o => panic!("expected number after VW, got {}", o.str_value())
                }
            },
            Token::Re => {
                self.next_token();
                match self.curr_token {
                    Token::Number(n) => {
                        self.next_token();
                        format!("turtle.turnRight({});\n", n)
                    },
                    o => panic!("expected number after RE, got {}", o.str_value())
                }
            },
            Token::Stift => {
                self.next_token();
                match self.curr_token {
                    Token::Number(n) => {
                        self.next_token();
                        format!("turtle.setStrokeWidth({});\n", n)
                    },
                    o => panic!("expected number after STIFT, got {}", o.str_value())
                }
            },
            Token::Farbe => {
                self.next_token();
                match self.curr_token {
                    Token::Number(n) => {
                        self.next_token();
                        format!("turtle.setColor({})\n", n)
                    },
                    o => panic!("expected number after FARBE, got {}", o.str_value())
                }
            },
            Token::Wh => {
                self.next_token();
                match self.curr_token {
                    Token::Number(n) => {
                        self.next_token();
                        match self.curr_token {
                            Token::BOpen => {
                                self.next_token();
                                let aw_string = self.anweisungen();
                                match self.curr_token {
                                    Token::BClose => {
                                        self.next_token();
                                        let n: usize = n.parse().unwrap();
                                        let mut result = String::with_capacity(aw_string.len() * n);
                                        for _ in 0..n {
                                            result.push_str(&aw_string);
                                        }
                                        result
                                    },
                                    o => panic!("expected ] after 'WH {{number}} [ <instructions>', got {}", o.str_value())
                                }
                            },
                            o => panic!("expected [ after 'WH {{number}}', got {}", o.str_value())
                        }
                    },
                    o => panic!("expected number after WH, got {}", o.str_value())
                }
            },
            Token::End => String::new(),
            o => panic!("expected VW, RE, STIFT, FARBE, WH, instead got {}", o.str_value())
        }
    }
}

