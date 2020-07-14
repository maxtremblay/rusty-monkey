mod lexer;
mod repl;

use repl::ReadEvalPrintLoop;

fn main() {
    ReadEvalPrintLoop::start().run();
}
