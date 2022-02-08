mod symbol;
mod alphabet;
mod thompson;

use std::{
    rc::{Rc, Weak}, hash::Hash, ptr,
    ops::{Deref},
    collections::{HashSet, HashMap},
    cell::{RefCell}
};
use crate::UnitId;
use self::{alphabet::Alphabet};

pub use self::symbol::{Symbol, RangeEdge, StrEdge, EpsEdge, UnresolvedName, Transition};
pub use thompson::{thompson, DFA, DFABasedLexer as DFALexer, Conflict};

#[derive(Debug)]
pub struct State
{
    tok_id: Option<UnitId>, // Some(id) means that it's terminal state for token with 'id'
    edges: RefCell<Vec<Edge>>,
}

impl State
{
    pub fn casual() -> Self
    { State::default() }

    pub fn terminal(id: UnitId) -> Self
    { State{ tok_id: Some(id), ..State::default() } }

    pub fn with_flag(tok_id: Option<UnitId>) -> Self
    { State{ tok_id, ..State::default() } }

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
    { State{ tok_id: None, edges: RefCell::new(vec![]) } }
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

#[derive(Debug)]
enum NamedNFA
{
    Defined(SubNFA),
    UndefinedQueue(Vec<SubNFA>),
}

#[derive(Debug, Default)]
pub struct AutomataBuilder
{
    named_nfas: HashMap<String, NamedNFA>,
    pub nfa: Automata,
}

impl AutomataBuilder
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

    pub fn resolve_id(&mut self, id: &str) -> SubNFA
    {
        let res = self.create_sub();
        self.resolve_id_for_sub(id, res)
    }

    fn resolve_id_for_sub(&mut self, id: &str, decl_ref: SubNFA) -> SubNFA
    {
        use NamedNFA::*;

        let named_nfa = self.named_nfas.entry(id.into())
            .or_insert_with(|| UndefinedQueue(vec![]));

        match named_nfa {
            Defined(definition) => {
                let definition = SubNFA::clone(definition);
                let def_clone = self.deep_clone(&definition);
                self.add_sym(decl_ref.input(), EpsEdge, def_clone.input());
                self.add_sym(def_clone.output(), EpsEdge, decl_ref.output());
            },
            UndefinedQueue(queue) => {
                queue.push(SubNFA::clone(&decl_ref));

                if decl_ref.input().edges.borrow().is_empty() {
                    self.add_sym(decl_ref.input(), UnresolvedName(id.into()), decl_ref.output());
                }
                assert_eq!(decl_ref.input().edges.borrow().len(), 1);
            }
        }
        decl_ref
    }

    pub fn define_name(&mut self, id: &str, definition: SubNFA) -> SubNFA
    {
        use NamedNFA::*;

        let mut queue = match self.named_nfas.insert(id.into(), Defined(SubNFA::clone(&definition))) {
            Some(Defined(_)) => panic!("Conflicting definitions of {}", id),
            Some(UndefinedQueue(queue)) => queue,
            None => vec![],
        };

        let res = self.create_sub();
        queue.push(SubNFA::clone(&res));

        for decl_ref in queue.into_iter() {
            // it may have a single UndefinedName edge, which now will be deleted
            assert!(decl_ref.input().edges.borrow().len() <= 1,
                "name: {}, len: {}", id, decl_ref.input().edges.borrow().len());
            decl_ref.input().edges.borrow_mut().clear();

            let def_clone = self.deep_clone(&definition);

            self.add_sym(decl_ref.input(), EpsEdge, def_clone.input());
            self.add_sym(def_clone.output(), EpsEdge, decl_ref.output());
        }
        res
    }

    pub fn build(self, start: StatePtr) -> Result<DFA, Conflict>
    {
        self.named_nfas
            .iter()
            .for_each(|(name, mb_def)| match mb_def {
                NamedNFA::UndefinedQueue(_) => panic!("undefined symbol: {}", name),
                _ => ()
            });
        thompson(self.nfa, start)
    }

    pub fn states_cnt(&self) -> usize
    { self.nfa.states.len() }

    fn deep_clone(&mut self, orig: &SubNFA) -> SubNFA
    {
        let mut mapping = HashMap::new();
        let SubNFA(orig_in, orig_out) = orig;
        let (res_in, _) = self.map_to_new(&mut mapping, orig_in);
        let (res_out, _) = self.map_to_new(&mut mapping, orig_out);

        let mut to_copy = vec![StatePtr::clone(&orig_in)];

        while let Some(src_from) = to_copy.pop() {
            let (dst_from, _) = self.map_to_new(&mut mapping, &src_from);
            let edges = src_from.edges.borrow();
            for Edge{ through, to: src_to, .. } in edges.iter() {
                let (dst_to, marked) = self.map_to_new(&mut mapping, src_to);
                if !marked {
                    to_copy.push(StatePtr::clone(src_to));
                }

                match through.deref() {
                    Symbol::Unresolved(UnresolvedName(name)) => {
                        self.resolve_id_for_sub(&name, SubNFA(StatePtr::clone(&dst_from), StatePtr::clone(&dst_to)));
                    }
                    _ => self.nfa.add_edge(Edge{
                            from: Rc::downgrade(&dst_from),
                            through: Rc::clone(through),
                            to: dst_to
                        }),
                }
            }
        }
        SubNFA(res_in, res_out)
    }

    fn map_to_new(&mut self, mapping: &mut HashMap<StatePtr, StatePtr>, from: &StatePtr) -> (StatePtr, bool)
    {
        match mapping.get(from) {
            Some(to) => (StatePtr::clone(to), true),
            None => {
                let res = self.add_casual();
                mapping.insert(StatePtr::clone(from), StatePtr::clone(&res));
                (res, false)
            }
        }
    }
}


#[cfg(test)]
mod tests
{
    use crate::{LexerParser, Lexer, Registry};

    use super::*;

    fn dfa_by_grammar(input: &str) -> DFA
    {
        let lexer_parser = LexerParser::new();
        let mut nfa = AutomataBuilder::new();
        let mut reg = Registry::new();

        let dfa = lexer_parser.parse(&mut nfa, &mut reg, Lexer::new(&input))
            .map(|start| nfa.build(start))
            .expect("Couldn't parse grammar");
        dfa.unwrap_or_else(|Conflict(f, s)| panic!("Conflicting states: {:?}, {:?}", f, s))
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

        DFALexer::new(&dfa, &input)
            .map(|mb_tok| mb_tok.map(|(s, _tok_id)| s))
            .zip(expected.into_iter().map(Ok))
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

        DFALexer::new(&dfa, &input)
            .map(|mb_tok| mb_tok.map(|(s, _tok_id)| s))
            .zip(expected)
            .for_each(|(my_tok, expected)| assert_eq!(my_tok, expected));
    }
}
