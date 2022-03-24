use std::{fmt::Display, error};

use combine::{
    self,
    parser::{char::{string, char, letter, spaces, alpha_num}, range::recognize},
    Parser,
    choice, EasyParser, many, none_of, many1, not_followed_by, attempt, between,
};
use crate::grammar_parser::GrammarError;

use crate::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Error<'input>
{
    pub unparsed: &'input str,
}

impl Display for Error<'_>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    { f.write_fmt(format_args!("Unparsed part:\n{}", self.unparsed)) }
}

impl error::Error for Error<'_> {}

fn error<T>(unparsed: &str) -> Result<T, Error>
{ Err(Error{ unparsed }) }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'input>
{
    LexerBegin,
    LexerEnd,
    Tok,
    Id(&'input str),
    LBrace,
    RBrace,
    Range(char, char),
    Str(String), // this is because there may be escaped symbols in input string (TOTHINK: is there other way?)
    Eq,
    QuestionMark,
    Star,
    Plus,
    Comma,
    Semicolon,
    Eps,

    GrammarBegin,
    GrammarEnd,
    FatArrow(&'input str),
    Colon,
    Sym,
    LT,
    GT,
    LPar,
    RPar,
}

pub struct Lexer<'input>
{
    remained: &'input str,
    shift: usize,
}

impl<'input> Lexer<'input>
{
    pub fn new(input: &'input str) -> Lexer<'input>
    { Lexer{ remained: input, shift: 0 } }
}

pub type Spanned<Tok, Loc, Err> = Result<(Loc, Tok, Loc), Err>;

pub type MbToken<'input> = Spanned<Token<'input>, usize, Error<'input>>;

impl<'input> Iterator for Lexer<'input>
{
    type Item = MbToken<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remained.is_empty() {
            return None
        }

        let mut parser = spaces().with(choice!(
            Self::p_keyword("lexer").map(|_| Token::LexerBegin),
            Self::p_keyword("lexer_end").map(|_| Token::LexerEnd),
            Self::p_keyword("eps").map(|_| Token::Eps),
            Self::p_keyword("tok").map(|_| Token::Tok),
            Self::p_keyword("sym").map(|_| Token::Sym),
            Self::p_keyword("grammar").map(|_| Token::GrammarBegin),
            Self::p_keyword("grammar_end").map(|_| Token::GrammarEnd),

            Self::p_identifier().map(Token::Id),
            Self::p_string().map(Token::Str),
            Self::p_code().map(Token::FatArrow),
            Self::p_range().map(|(from, to)| Token::Range(from, to)),

            char('{').map(|_| Token::LBrace),
            char('}').map(|_| Token::RBrace),
            char(',').map(|_| Token::Comma),
            char('?').map(|_| Token::QuestionMark),
            char('*').map(|_| Token::Star),
            char('+').map(|_| Token::Plus),
            char('=').map(|_| Token::Eq),
            char(';').map(|_| Token::Semicolon),
            char(':').map(|_| Token::Colon),
            char('<').map(|_| Token::LT),
            char('>').map(|_| Token::GT),
            char('(').map(|_| Token::LPar),
            char(')').map(|_| Token::RPar)
        )).skip(spaces());

        match parser.easy_parse(self.remained) {
            Ok((tok, remained)) => {
                let delta = self.remained.len() - remained.len();
                let new_shift = self.shift + delta;
                let res = (self.shift, tok, new_shift);

                self.shift = new_shift;
                self.remained = remained;
                Some(Ok(res))
            },
            Err(err) => {
                eprintln!("{:?}", err);
                Some(error(self.remained))
            },
        }
    }
}

// util
impl<'input> Lexer<'input>
{
    fn p_identifier() -> impl EasyParser<&'input str, Output = &'input str>
    {
        recognize(
        letter()
            .or(char('_'))
            .and(many::<Count, _, _>(
                alpha_num().or(char('_'))
                ))
            )
    }

    fn p_string() -> impl EasyParser<&'input str, Output = String>
    {
        let escaped = char('\\')
            .with(choice!(
                char('"').map(|_| '"'),
                char('\\').map(|_| '\\'),
                char('n').map(|_| '\n'),
                char('r').map(|_| '\r')
            ))
            .or(none_of(['"', '\\']));

        between(char('"'), char('"'), many1::<String, _, _>(escaped))
    }

    fn p_range() -> impl EasyParser<&'input str, Output = (char, char)>
    {
        let escaped = char('\\')
            .with(choice!(
                char('\\').map(|_| '\\'),
                char('n').map(|_| '\n'),
                char('r').map(|_| '\r')
            ))
            .or(none_of([']', '\\']));

        let range = escaped.clone()
            .and(char('-').with(escaped));

        let full_range = between(char('['), char(']'), range);

        full_range
    }

    fn p_keyword(keyword: &'static str) -> impl EasyParser<&'input str, Output = &'input str>
    {
        attempt(
            string(keyword)
                .skip(not_followed_by(alpha_num().or(char('_'))))
        )
    }

    fn p_code() -> impl EasyParser<&'input str, Output = &'input str>
    {
        attempt(string("=>")
            .skip(spaces())
            .with(recognize(util::code())))
    }
}

#[derive(Debug, Default)]
struct Count(usize);

impl Extend<char> for Count
{
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        let iter = iter.into_iter();
        let cnt: usize = iter.map(|ch| ch.len_utf8()).sum();
        self.0 += cnt;
    }
}

pub struct RegLexer<'input>
{
    lexer: Lexer<'input>,
}

impl<'input> From<Lexer<'input>> for RegLexer<'input>
{
    fn from(lexer: Lexer<'input>) -> Self
    { RegLexer{ lexer } }
}

impl<'input> Iterator for RegLexer<'input>
{
    type Item = Spanned<Token<'input>, usize, GrammarError<'input>>;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.lexer.next()
            .map(|mb_tok| mb_tok.map_err(GrammarError::Tokenize))
    }
}

#[cfg(test)]
mod playground
{
    use super::*;
    use combine::{EasyParser};


    #[test]
    fn identifier()
    {
        let mut parser = Lexer::p_identifier();

        let input = "_кек__лол2 the rest";
        let res = parser.easy_parse(input);
        println!("{:?}", res);
        assert_eq!(res, Ok(("_кек__лол2", " the rest")));

        let input = "lPar = ";
        let res = parser.easy_parse(input);
        println!("{:?}", res);
        assert_eq!(res, Ok(("lPar", " = ")));
    }


    #[test]
    fn string()
    {
        let mut parser = Lexer::p_string();

        let input = r#"" kek, \"lol\" \n" fuuuh"#;
        let res = parser.easy_parse(input);
        println!("{:?}", res);
        assert_eq!(res, Ok((" kek, \"lol\" \n".into(), " fuuuh")));
    }

    #[test]
    fn range()
    {
        let mut parser = Lexer::p_range();
        let res = parser.easy_parse("[a-z];");

        println!("{:?}", res);
        assert_eq!(res, Ok((('a', 'z'), ";")));
    }

    #[test]
    fn minimal()
    {
        let input = "\nlexer hi lexer_end  ";
        let expected = [Token::LexerBegin, Token::Id("hi"), Token::LexerEnd]
            .into_iter().map(Ok);

        Lexer::new(input)
            .map(|mb_tok| mb_tok.map(|(_, tok, _)| tok))
            .zip(expected)
            .for_each(|(tok, expected_tok)| assert_eq!(tok, expected_tok));
    }

    #[test]
    fn piece_of_code()
    {
        let mut parser = Lexer::p_code();

        let input = "=> { if a == 3 { Right(lol) } else { Left(kek) } } something else";
        let res = parser.easy_parse(input);
        println!("{:?}", res);
        assert_eq!(res, Ok(("{ if a == 3 { Right(lol) } else { Left(kek) } }", " something else")));
    }

    #[test]
    fn smol_grammar()
    {
        let input = r"
            grammar

            Expr
            { l:Expr add r:Fact => { l + r }
            , l:Expr sub r:Fact => { l - r }
            }

            Fact
            { n:num => { i32::from(n).unwrap() }
            , lPar inner:Expr rPar => { inner }
            }

            grammar_end
        ";

        use Token::*;
        let expected = [
            GrammarBegin,

            Id("Expr"),
            LBrace, Id("l"), Colon, Id("Expr"), Id("add"), Id("r"), Colon, Id("Fact"), FatArrow("{ l + r }"),
            Comma , Id("l"), Colon, Id("Expr"), Id("sub"), Id("r"), Colon, Id("Fact"), FatArrow("{ l - r }"),
            RBrace,

            Id("Fact"),
            LBrace, Id("n"), Colon, Id("num"), FatArrow("{ i32::from(n).unwrap() }"),
            Comma , Id("lPar"), Id("inner"), Colon, Id("Expr"), Id("rPar"), FatArrow("{ inner }"),
            RBrace,

            GrammarEnd
        ].into_iter().map(Ok);

        Lexer::new(input)
            .map(|mb_tok| mb_tok.map(|(_, tok, _)| tok))
            .zip(expected)
            .for_each(|(tok, expected_tok)| assert_eq!(tok, expected_tok))
    }

    #[test]
    fn all_together()
    {
        let input = r#"
            lexer

            tok lPar = "(";
            tok rPar = ")";
            digit = [0-9];
            slash = "\\";
            quote = "\"";

            tok Num = "-"? [1-9] digit+;

            lCase = [a-z];
            uCase = [A-Z];

            tok Id = { lCase, uCase, "_" } { lCase, uCase, "_", digit }+;

            lexer_end
        "#;

        use Token::*;
        let expected = [
            LexerBegin,

            Tok, Id("lPar"), Eq, Str("(".into()), Semicolon,
            Tok, Id("rPar"), Eq, Str(")".into()), Semicolon,
            Id("digit"), Eq, Range('0', '9'), Semicolon,
            Id("slash"), Eq, Str("\\".into()), Semicolon,
            Id("quote"), Eq, Str("\"".into()), Semicolon,

            Tok, Id("Num"), Eq, Str("-".into()), QuestionMark, Range('1', '9'), Id("digit"), Plus, Semicolon,

            Id("lCase"), Eq, Range('a', 'z'), Semicolon,
            Id("uCase"), Eq, Range('A', 'Z'), Semicolon,

            Tok, Id("Id"), Eq, LBrace, Id("lCase"), Comma, Id("uCase"), Comma, Str("_".into()), RBrace,
                LBrace, Id("lCase"), Comma, Id("uCase"), Comma, Str("_".into()), Comma, Id("digit"), RBrace, Plus, Semicolon,

            LexerEnd
            ].into_iter().map(Ok);

        Lexer::new(input)
            .map(|mb_tok| mb_tok.map(|(_, tok, _)| tok))
            .zip(expected)
            .for_each(|(tok, expected_tok)| assert_eq!(tok, expected_tok))
    }
}
