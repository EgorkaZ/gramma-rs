mod grammar;
pub mod tokenizer;
pub mod lexer;
mod registry;
mod parser_table;

use std::{fmt::Display, error::Error};

pub use grammar::{LexerParser, TotalGrammarParser as GrammarParser};
pub use lexer::{AutomataBuilder, DFA, Transition, DFALexer, Conflict};
pub use tokenizer::Lexer;
pub use registry::RegistryBuilder;
pub use parser_table::{UnitId, KernelId, RuleId, ItemId, LAItemId, GrUnit, ItemSet};

use lexer::*;
use tokenizer::Error as LexError;
use registry::RegError;

#[derive(Debug, Eq, PartialEq)]
pub enum GrammarError<'input>
{
    Tokenize(LexError<'input>),
    Registry(RegError),
}

impl<'input, 'reg> From<LexError<'input>> for GrammarError<'input>
{
    fn from(err: LexError<'input>) -> Self
    { GrammarError::Tokenize(err) }
}

impl<'input, 'reg> From<RegError> for GrammarError<'input>
{
    fn from(err: RegError) -> Self
    { GrammarError::Registry(err) }
}

impl Display for GrammarError<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let to_print: &dyn Error;
        match self {
            GrammarError::Registry(err) => {
                to_print = err;
                f.write_str("Registry error. ")
            },
            GrammarError::Tokenize(err) => {
                to_print = err;
                f.write_str("Tokenizer error. ")
            }
        }
        .and_then(|_| Display::fmt(to_print, f))
    }
}

impl Error for GrammarError<'_> {}
