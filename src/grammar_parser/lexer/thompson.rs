use std::{collections::{HashSet, HashMap}, hash::Hash, rc::Rc, ops::{Deref, DerefMut}, ptr::NonNull};

use kiam::when;
use static_assertions::assert_eq_size;

use crate::{grammar_parser::lexer::State};

use super::{StatePtr, Edge, symbol::{Transition, TransitError}, Automata, SubNFA, Symbol, alphabet::Alphabet};

/// Keeps hash_set and hash of the whole set, so it hashes and compares a bit faster
///
/// NB! Considers equality of pointers as equality of objects
#[derive(Debug, Eq)]
struct StateSet
{
    set: HashSet<StatePtr>,
    hash: usize,
    is_term: bool,
}

impl StateSet
{
    fn new() -> Self
    { StateSet{ set: HashSet::new(), hash: 0, is_term: false } }

    fn add(&mut self, nfa_state: &StatePtr)
    {
        if let None = self.set.get(nfa_state) {
            let state_ptr: *const State = Rc::as_ptr(nfa_state);
            let state_num = state_ptr as usize;
            assert_eq_size!(*const State, usize);

            self.hash += state_num;
            self.is_term |= nfa_state.is_term;

            self.set.insert(StatePtr::clone(nfa_state));
        }
    }

    fn is_empty(&self) -> bool
    { self.set.is_empty() }

    fn size(&self) -> usize
    { self.set.len() }
}

impl PartialEq for StateSet
{
    fn eq(&self, other: &Self) -> bool
    {
        if self.hash != other.hash {
            return false;
        }
        if self.set.len() != other.set.len() {
            return false;
        }
        self.set.iter()
            .all(|in_self| other.set.contains(in_self))
    }
}

impl Hash for StateSet
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    { self.hash.hash(state); }
}

impl From<StatePtr> for StateSet
{
    fn from(single: StatePtr) -> Self
    {
        let hash = Rc::as_ptr(&single) as usize;
        let mut set = HashSet::new();
        let is_term = single.is_term;
        set.insert(single);
        StateSet{ set, hash, is_term  }
    }
}

pub struct DFA
{
    automata: Automata,
    start: StatePtr,
}

impl DFA
{
    fn new(start: StatePtr, symbols: Alphabet) -> Self
    {
        let mut automata = Automata::new();
        automata.symbols = symbols;
        automata.add_state(&start);
        DFA{ automata, start }
    }
}

impl Deref for DFA
{
    type Target = Automata;

    fn deref(&self) -> &Self::Target
    { &self.automata }
}

impl DerefMut for DFA
{
    fn deref_mut(&mut self) -> &mut Self::Target
    { &mut self.automata }
}

/// `from` + all the states accessible through any amount of
/// eps-transitions
fn eps_closure(mut set: StateSet, from: &StatePtr) -> StateSet
{
    set.add(from);

    from.edges.borrow()
        .iter()
        .filter_map(|Edge{ through, to, ..}| if through.is_eps() {
            Some(to)
        } else {
            None
        })
        .fold(set, eps_closure)
}

/// eps-closures of states accessible through `sym` from `from` states
fn sym_closure(mut set: StateSet, from: &StateSet, sym: &Symbol) -> StateSet
{
    for state in from.set.iter() {
        set = state.edges.borrow()
            .iter()
            .filter_map(|Edge{ through, to, .. }| when! {
                std::ptr::eq(Rc::as_ptr(&through), sym) => Some(to),
                _ => None,
            })
            .fold(set, eps_closure);
    }
    set
}

/// second argument must be sub NFA equal to the automata itself
/// TODO: manage it by lifetimes, ownership and things
pub fn thompson(nfa: Automata, SubNFA(entry, _exit): SubNFA) -> DFA
{
    let mut dfa_states: HashMap<Box<StateSet>, StatePtr> = HashMap::new();

    let start = eps_closure(StatePtr::clone(&entry).into(), &entry);
    let start = Box::new(start);
    let start_ref = NonNull::from(start.deref());

    let start_dfa = State::with_flag(start.is_term).into();
    dfa_states.insert(start, StatePtr::clone(&start_dfa));

    let mut dfa = DFA::new(StatePtr::clone(&start_dfa), nfa.symbols.clone());

    let mut stack = Vec::new();
    stack.push((start_ref, start_dfa));

    while let Some((curr, curr_from)) = stack.pop() {
        let curr = unsafe { curr.as_ref() };
        dfa.add_state(&curr_from);

        for sym in nfa.symbols() {
            if sym.is_eps() {
                continue
            }
            let new_set = Box::new(sym_closure(StateSet::new(), curr, sym));
            let new_set_ref = NonNull::from(new_set.deref());
            let is_term = new_set.is_term;
            if new_set.set.is_empty() {
                continue
            }

            let new_state = match dfa_states.get(&new_set) {
                Some(to) => StatePtr::clone(to),
                None => {
                    let new_state = when! {
                        is_term => State::terminal(),
                        _ => State::casual(),
                    }.into();
                    dfa_states.insert(new_set, StatePtr::clone(&new_state));
                    stack.push((new_set_ref, StatePtr::clone(&new_state)));
                    dfa.add_state(&new_state);
                    new_state
                }
            };
            dfa.add_edge(Edge{
                from: Rc::downgrade(&curr_from),
                through: Rc::clone(sym),
                to: StatePtr::clone(&new_state)
            });
        }
    }

    sort_edges(dfa)
}

fn sort_edges(dfa: DFA) -> DFA
{
    dfa.states
        .iter()
        .for_each(|state| {
            let mut edges = state.edges.borrow_mut();
            edges.sort_unstable_by_key(|edge| Rc::clone(&edge.through));
        });
    dfa
}

impl Transition for DFA
{
    fn try_pass<It>(&self, last_accepted: &mut It) -> Result<usize, super::symbol::TransitError>
        where It: Iterator<Item = char> + Clone
    {
        let mut it = last_accepted.clone();
        let mut real_steps = 0;
        let mut lookahead_steps = 0;

        let mut curr_state = StatePtr::clone(&self.start);
        loop {
            let inner_curr = StatePtr::clone(&curr_state);
            let edges = inner_curr.edges.borrow();
            let mb_steps = edges.iter()
                .find_map(|Edge{ through, to, .. }| through.try_pass(&mut it)
                    .map(|steps| (steps, to))
                    .ok());

            if let Some((curr_steps, to)) = mb_steps {
                lookahead_steps += curr_steps;
                if to.is_term {
                    last_accepted.advance_by(lookahead_steps).unwrap();
                    real_steps += lookahead_steps;
                    lookahead_steps = 0;
                }
                curr_state = StatePtr::clone(to)
            } else {
                break
            }
        }

        if real_steps > 0 {
            Ok(real_steps)
        } else {
            Err(TransitError::NoLexemeMatched)
        }
    }
}

// impl DFA
// {
//     fn next_lexeme<'input>(&self, input: &'input str) -> Option<&'input str>
//     {
//         let mut last_accepted = input;
//         let mut real_steps = 0;
//         let mut lookahead_steps = 0;

//         let mut curr_state = StatePtr::clone(&self.start);
//         loop {
//             let inner_curr = StatePtr::clone(&curr_state);
//             let edges = inner_curr.edges.borrow();
//             let 
//         }
//     }
// }

pub struct dfaBASEDlexer<'dfa, 'input>
{
    dfa: &'dfa DFA,
    input: &'input str
}

impl<'dfa, 'input> dfaBASEDlexer<'dfa, 'input>
{
    pub fn new(dfa: &'dfa DFA, input: &'input str) -> Self
    { dfaBASEDlexer{ dfa, input } }
}

impl<'input> Iterator for dfaBASEDlexer<'_, 'input>
{
    type Item = Result<&'input str, &'input str>;

    fn next(&mut self) -> Option<Self::Item> {
        let spaces_before = self.input.chars()
            .take_while(|ch| ch.is_whitespace())
            .map(char::len_utf8)
            .sum();
        self.input = &self.input[spaces_before..];

        if self.input.is_empty() {
            return None
        }

        let mut moved = self.input.chars();
        match self.dfa.try_pass(&mut moved) {
            Ok(steps) => {
                let lexeme_len = self.input.chars()
                    .take(steps)
                    .map(char::len_utf8)
                    .sum();

                let res = &self.input[..lexeme_len];
                self.input = &self.input[lexeme_len..];
                Some(Ok(res))
            },
            Err(_) => Some(Err(self.input)),
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::{AutomataBuilder, LexerParser, Lexer, grammar_parser::lexer::StrEdge};

    use super::*;

    #[test]
    fn sus_nfa()
    {
        let parser = LexerParser::new();
        let mut nfa = AutomataBuilder::new();

        let input = r#"
            lexer

            fst = { "a"
                  , "b"
                  };
            tok lang = fst* "a" "b" "b";

            lexer_end
        "#;

        let res = parser.parse(&mut nfa, Lexer::new(input));
        assert_eq!(nfa.states_cnt(), 16);
        assert!(res.is_ok());
        let a_sym = nfa.nfa.symbols.add_sym(StrEdge::new("a".into()));
        let b_sym = nfa.nfa.symbols.add_sym(StrEdge::new("b".into()));

        assert_eq!(nfa.nfa.symbols.symbols().count(), 3);

        let start = res.unwrap().0;

        let mut set_set = HashSet::new();

        let start_closure = eps_closure(StateSet::new(), &start);
        set_set.insert(&start_closure);

        assert_eq!(set_set.len(), 1);
        assert_eq!(start_closure.size(), 6);

        let p_a = sym_closure(StateSet::new(), &start_closure, &a_sym);
        set_set.insert(&p_a);

        assert_eq!(set_set.len(), 2);
        assert_eq!(p_a.size(), 7);

        let p_b = sym_closure(StateSet::new(), &start_closure, &b_sym);
        set_set.insert(&p_b);

        assert_eq!(set_set.len(), 3);
        assert_eq!(p_b.size(), 6);

        let p_ba = sym_closure(StateSet::new(), &p_b, &a_sym);
        set_set.insert(&p_ba);

        assert_eq!(p_ba.size(), 7);
        assert_eq!(set_set.len(), 3);
    }
}
