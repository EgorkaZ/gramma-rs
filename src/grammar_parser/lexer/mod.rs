mod symbol;
mod alphabet;
mod thompson;

use std::{
    rc::{Rc, Weak}, hash::Hash, ptr,
    ops::{Deref},
    collections::{HashSet, HashMap},
    cell::{RefCell}
};

use self::{alphabet::Alphabet};
pub use self::symbol::{Symbol, RangeEdge, StrEdge, EpsEdge, Transition};
pub use thompson::{thompson, DFA, dfaBASEDlexer as DFALexer};

#[derive(Debug)]
pub struct State
{
    is_term: bool,
    edges: RefCell<Vec<Edge>>,
}

impl State
{
    pub fn casual() -> Self
    { State::default() }

    pub fn terminal() -> Self
    { State{ is_term: true, ..State::default() } }

    pub fn with_flag(is_term: bool) -> Self
    { State{ is_term, ..State::default() } }

    pub fn extend(&self, from: &State)
    {
        let from_edges = from.edges.borrow();
        let it = from_edges.iter();
        self.edges.borrow_mut().extend(it.cloned());
    }
}

impl Default for State
{
    fn default() -> Self
    { State{ is_term: false, edges: RefCell::new(vec![]) } }
}


#[derive(Debug, Clone)]
pub struct Edge
{
    from   : Weak<State>,
    through: Rc<Symbol>,
    to     : StatePtr,
}

#[derive(Debug, Clone)]
pub struct StatePtr(Rc<State>);

impl PartialEq for StatePtr
{
    fn eq(&self, other: &Self) -> bool
    { Rc::ptr_eq(&self.0, &other.0) }
}

impl Eq for StatePtr {}

impl Hash for StatePtr
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    { ptr::hash(Rc::as_ptr(&self.0), state); }
}

impl Deref for StatePtr
{
    type Target = Rc<State>;

    fn deref(&self) -> &Self::Target
    { &self.0 }
}

impl From<State> for StatePtr
{
    fn from(state: State) -> Self
    { StatePtr(Rc::new(state)) }
}

#[derive(Debug)]
pub struct Automata
{
    states: HashSet<StatePtr>,
    pub symbols: Alphabet,
}

impl Automata
{
    pub fn new() -> Self
    { Automata::default() }

    pub fn add_state(&mut self, state: &StatePtr)
    {
        match self.states.get(state) {
            Some(_) => (),
            None => {
                self.states.insert(StatePtr::clone(state));
            }
        }
    }

    #[allow(unused_mut)]
    pub fn add_through_sym<S>(&mut self, from: StatePtr, sym: S, to: StatePtr)
        where Symbol: From<S>
    {
        let sym: Rc<Symbol> = self.symbols.add_sym(sym);
        let to_insert = Edge{
            from: Rc::downgrade(&from),
            to: StatePtr::clone(&to),
            through: sym
        };

        from.edges.borrow_mut().push(to_insert);
    }

    pub fn add_edge(&mut self, edge: Edge)
    {
        let from = Weak::upgrade(&edge.from).unwrap();
        from.edges.borrow_mut().push(edge);
    }

    pub fn symbols(&self) -> impl Iterator<Item = &Rc<Symbol>>
    { self.symbols.symbols() }
}

impl Default for Automata
{
    fn default() -> Self
    { Automata{ states: HashSet::default(), symbols: Alphabet::default() } }
}

#[derive(Debug, Clone)]
pub struct SubNFA(pub StatePtr, pub StatePtr);

impl SubNFA
{
    pub fn input(&self) -> &StatePtr
    { &self.0 }

    pub fn output(&self) -> &StatePtr
    { &self.1 }
}

#[derive(Debug, Default)]
pub struct AutomataBuilder<'input>
{
    named_nfas: HashMap<&'input str, SubNFA>,
    pub nfa: Automata,
}

impl<'input> AutomataBuilder<'input>
{
    pub fn new() -> Self
    { AutomataBuilder::default() }

    pub fn add_state(&mut self, state: State) -> StatePtr
    {
        let as_ptr: StatePtr = state.into();
        self.nfa.add_state(&as_ptr);
        as_ptr
    }

    fn add_casual(&mut self) -> StatePtr
    { self.add_state(State::casual()) }

    pub fn create_sub(&mut self) -> SubNFA
    { SubNFA(self.add_casual(), self.add_casual()) }

    pub fn add_sym<S>(&mut self, from: &StatePtr, sym: S, to: &StatePtr)
        where Symbol: From<S>
    {
        self.nfa.add_through_sym(StatePtr::clone(from), sym, StatePtr::clone(to));
    }

    pub fn resolve_id(&mut self, id: &'input str) -> SubNFA
    {
        match self.named_nfas.get(id) {
            Some(found) => SubNFA::clone(found),
            None => {
                let created = self.create_sub();
                self.named_nfas.insert(id, SubNFA::clone(&created));
                created
            }
        }
    }

    pub fn assign_to_id(&mut self, id: &'input str, sub: SubNFA) -> SubNFA
    {
        match self.named_nfas.get(id) {
            Some(SubNFA(f_in, f_out)) => {
                // we want found input to have edges to every state that sub.from has to
                f_in.extend(sub.input());

                // and we want every state that has edge to sub.to to have edge to found output
                let edges_to = sub.output().edges.borrow();
                for Edge{ from: e_from, through: sym, .. } in edges_to.iter() {
                    let from = Weak::upgrade(e_from).unwrap();
                    let new_edge = Edge{
                        from: Weak::clone(e_from),
                        through: Rc::clone(sym),
                        to: StatePtr::clone(f_out)
                    };
                    from.edges.borrow_mut().push(new_edge);
                }
                SubNFA(StatePtr::clone(f_in), StatePtr::clone(f_out))
            },
            None => {
                self.named_nfas.insert(id, SubNFA::clone(&sub));
                sub
            }
        }
    }

    pub fn build(self, self_sub: SubNFA) -> DFA
    { thompson(self.nfa, self_sub) }

    pub fn states_cnt(&self) -> usize
    { self.nfa.states.len() }
}


#[cfg(test)]
mod tests
{
    use crate::{LexerParser, Lexer};

    use super::*;

    fn dfa_by_grammar(input: &str) -> DFA
    {
        let lexer_parser = LexerParser::new();
        let mut nfa = AutomataBuilder::new();

        let dfa = lexer_parser.parse(&mut nfa, Lexer::new(&input))
            .map(|sub_nfa| nfa.build(sub_nfa))
            .expect("Couldn't parse grammar");
        dfa
    }

    #[test]
    fn my_lexer()
    {
        let lexer_grammar = r#"
            lexer

            tok Id = { "_", letter } { "_", letter, digit }*;
            tok Type = { "int", "bool" };
            tok Bool = { "True", "False" };

            letter = { [a-z], [A-Z] };
            digit = [0-9];

            tok Eq = "=";
            tok Semicolon = ";";

            tok Num = { "+", "-" }? [1-9] digit*;

            lexer_end
        "#;
        let dfa = dfa_by_grammar(lexer_grammar);


        let input = "
            int a = 0;
            int kek18 = 84;
            bool false = True;
        ";
        let expected = [
            "int", "a", "=", "0", ";",
            "int", "kek18", "=", "84", ";",
            "bool", "false", "=", "True", ";"];

        DFALexer::new(&dfa, &input).zip(expected.into_iter().map(Ok))
            .for_each(|(my_tok, expected)| assert_eq!(my_tok, expected));
    }

    #[test]
    fn overlapping()
    {
        // lexer must chose
        //  - longer string over shorter one
        //  - string literal over range
        let lexer_grammar = r#"
            lexer

            tok Lexer = "lexer";
            tok LexerEnd = "lexer_end";

            tok String = "boo";
            tok LongString = "b" [o-p]+;

            lexer_end
        "#;
        let dfa = dfa_by_grammar(lexer_grammar);

        let input = "lexer_end lexer booo";
        let expected = [Ok("lexer_end"), Ok("lexer"), Ok("boo"), Err("o")];

        DFALexer::new(&dfa, &input).zip(expected)
            .for_each(|(my_tok, expected)| assert_eq!(my_tok, expected));
    }
}
