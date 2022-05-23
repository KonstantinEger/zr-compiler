mod tokenizer;
mod compiler;

use compiler::Compiler;

fn main() {
    let source = "WH 4 [ VW 30 RE 90 ]";
    let mut zrc = Compiler::init(source);
    let output = zrc.compile();
    println!("{}", output);
}
