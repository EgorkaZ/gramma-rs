use std::{hash::Hash, cmp::Ordering, fmt::{Debug, Write}};

use kiam::when;

/// "Metaphorical" symbol: a transition in an automata
#[derive(PartialEq, Eq, Hash)]
pub enum Symbol
{
    Range(RangeEdge),
    Str(StrEdge),
    Eps(EpsEdge),
    Unresolved(UnresolvedName),
}

impl Symbol
{
    fn priority(&self) -> u8
    {
        use Symbol::*;
        match self {
            Str(_) => 0,
            Range(_) => 1,
            Eps(_) => 2,
            Unresolved(_) => 3,
        }
    }
}

impl From<RangeEdge> for Symbol
{
    fn from(rng: RangeEdge) -> Symbol
    { Symbol::Range(rng) }
}

impl From<StrEdge> for Symbol
{
    fn from(str: StrEdge) -> Symbol
    { Symbol::Str(str) }
}

impl From<UnresolvedName> for Symbol
{
    fn from(name: UnresolvedName) -> Symbol
    { Symbol::Unresolved(name) }
}

impl Ord for Symbol
{
    fn cmp(&self, other: &Self) -> Ordering {
        use Symbol::*;
        let prio_cmp = self.priority().cmp(&other.priority());
        if prio_cmp != Ordering::Equal {
            return prio_cmp
        }
        match (self, other) {
            (Eps(_), Eps(_)) => Ordering::Equal,
            (Range(lhs), Range(rhs)) => lhs.cmp(rhs),
            (Str(lhs), Str(rhs)) => {
                when! {
                    // longer string must go earlier
                    lhs.0.len() < rhs.0.len() => Ordering::Greater,
                    lhs.0.len() > rhs.0.len() => Ordering::Less,
                    _ => lhs.0.cmp(&rhs.0)
                }
            },
            _ => panic!("priority comparison worked weird"),
        }
    }
}

impl PartialOrd for Symbol
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// impl From<ChoiceEdge> for Symbol
// {
//     fn from(choice: ChoiceEdge) -> Symbol
//     { Symbol::Choice(choice) }
// }

impl From<EpsEdge> for Symbol
{
    fn from(eps: EpsEdge) -> Symbol
    { Symbol::Eps(eps) }
}

#[derive(Debug)]
pub enum TransitError
{
    OutOfRange((char, char), char),
    Unexpected{ found: char, expected: char },
    NoLexemeMatched,
    EOI,
}

pub trait Transition
{
    type OkRes;

    fn try_pass<It>(&self, it: &mut It) -> Result<Self::OkRes, TransitError>
        where It: Iterator<Item = char> + Clone;

    fn is_eps(&self) -> bool
    { false }
}

impl Transition for Symbol
{
    type OkRes = usize;

    fn try_pass<It>(&self, it: &mut It) -> Result<usize, TransitError>
        where It: Iterator<Item = char> + Clone
    {
        match self {
            Symbol::Range(rng) => rng.try_pass(it),
            Symbol::Str(str) => str.try_pass(it),
            Symbol::Eps(eps) => eps.try_pass(it),
            Symbol::Unresolved(unresolved) => unresolved.try_pass(it),
        }
    }

    fn is_eps(&self) -> bool
    {
        match self {
            Symbol::Range(rng) => rng.is_eps(),
            Symbol::Str(str) => str.is_eps(),
            Symbol::Eps(eps) => eps.is_eps(),
            Symbol::Unresolved(unresolved) => unresolved.is_eps(),
        }
    }
}

impl Debug for Symbol
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            Self::Range(RangeEdge(from, to)) => f.write_char('[')
                .and_then(|_| f.write_char(*from))
                .and_then(|_| f.write_char('-'))
                .and_then(|_| f.write_char(*to))
                .and_then(|_| f.write_char(']')),
            Self::Str(StrEdge(s)) => f.write_char('"')
                .and_then(|_| f.write_str(s))
                .and_then(|_| f.write_char('"')),
            Self::Eps(_) => f.write_str("eps"),
            Self::Unresolved(unresolved) => f.write_fmt(format_args!("Unresolved name: {}", unresolved.0)),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeEdge(char, char);

impl RangeEdge
{
    pub fn new(from: char, to: char) -> Self
    { RangeEdge(from, to) }
}

impl Transition for RangeEdge
{
    type OkRes = usize;

    fn try_pass<It>(&self, it: &mut It) -> Result<usize, TransitError>
        where It: Iterator<Item = char> + Clone
    {
        let RangeEdge(from, to) = *self;
        let mut clone = it.clone();
        match clone.next() {
            Some(ch) if from <= ch && ch <= to => {
                it.next();
                Ok(1)
            },
            Some(ch) => Err(TransitError::OutOfRange((from, to), ch)),
            None => Err(TransitError::EOI),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrEdge(pub String);

impl StrEdge
{
    pub fn new(s: String) -> StrEdge
    { StrEdge(s) }
}

impl Transition for StrEdge
{
    type OkRes = usize;

    fn try_pass<It>(&self, it: &mut It) -> Result<usize, TransitError>
        where It: Iterator<Item = char> + Clone
    {
        let mut clone = it.clone().peekable();
        let mut expected = self.0.chars();
        let mut steps: usize = 0;

        let res = loop {
            match (clone.peek(), expected.next()) {
                (Some(&ch), Some(exp)) if ch == exp => { clone.next(); steps += 1; continue },
                (_, None) => break Ok(steps),
                (None, Some(_)) => break Err(TransitError::EOI),
                (Some(&found), Some(expected)) => break Err(TransitError::Unexpected{ found, expected })
            }
        };

        if let Ok(steps) = res {
            it.advance_by(steps).unwrap();
        }
        res
    }
}

// #[derive(Debug, Eq)]
// pub struct ChoiceEdge(Vec<Rc<Symbol>>);

// impl ChoiceEdge
// {
//     pub fn new(choice: Vec<Rc<Symbol>>) -> Self
//     { ChoiceEdge(choice) }
// }

// impl Transition for ChoiceEdge
// {
//     fn try_pass<It>(&self, it: &mut It) -> Result<usize, TransitError>
//         where It: Iterator<Item = char> + Clone
//     {
//         let mut last_err = TransitError::EmptyChoice;
//         for alt in self.0.iter() {
//             match alt.try_pass(it) {
//                 Ok(steps) => return Ok(steps),
//                 Err(err) => last_err = err,
//             }
//         }
//         Err(last_err)
//     }
// }

// impl PartialEq for ChoiceEdge
// {
//     fn eq(&self, other: &Self) -> bool {
//         let ChoiceEdge(lhs) = self;
//         let ChoiceEdge(rhs) = other;

//         lhs.iter()
//             .zip(rhs)
//             .all(|(l, r)| Rc::ptr_eq(l, r))
//     }
// }

// impl Hash for ChoiceEdge
// {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         let ChoiceEdge(vec) = self;
//         for sym in vec.iter() {
//             ptr::hash(Rc::as_ptr(sym), state)
//         }
//     }
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EpsEdge;

impl Transition for EpsEdge
{
    type OkRes = usize;

    fn try_pass<It>(&self, _it: &mut It) -> Result<usize, TransitError>
        where It: Iterator<Item = char> + Clone
    {
        Ok(0)
    }

    fn is_eps(&self) -> bool
    { true }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnresolvedName(pub String);

impl Transition for UnresolvedName
{
    type OkRes = usize;

    fn try_pass<It>(&self, _it: &mut It) -> Result<Self::OkRes, TransitError>
        where It: Iterator<Item = char> + Clone
    {
        panic!("Unresolved name: {}", self.0);
    }

    // done so, that it will be removed while determinization of NFA
    fn is_eps(&self) -> bool
    { true }
}
