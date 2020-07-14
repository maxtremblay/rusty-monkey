mod lexer;
mod parser;
mod repl;

use repl::ReadEvalPrintLoop;

fn main() {
    ReadEvalPrintLoop::start().run();
}
