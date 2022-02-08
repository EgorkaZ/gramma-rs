mod grammar;
pub mod tokenizer;
mod lexer;
mod registry;

pub use grammar::LexerParser;
pub use lexer::{AutomataBuilder, DFA, Transition, DFALexer, Conflict};
pub use tokenizer::Lexer;
pub use registry::{Registry, UnitId};

use lexer::*;
use tokenizer::Error;
use registry::RegError;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum GrammarError<'input>
{
    Tokenize(Error<'input>),
    Registry(RegError),
}
