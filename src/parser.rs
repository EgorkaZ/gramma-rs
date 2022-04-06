use std::{collections::HashMap, fmt::{Display, Write}, marker::PhantomData};

use crate::{
    RegistryBuilder, DFA, AutomataBuilder, GrammarParser,
    tokenizer::RegLexer, Conflict, Lexer, RuleId,
    KernelId, UnitId, DFALexer
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action
{
    Shift(KernelId),
    Reduce(RuleId),
    Accept(RuleId),
}
pub struct ParserBase
{
    reg: RegistryBuilder,
    dfa: DFA,
    actions: Vec<HashMap<UnitId, Action>>,
}

impl std::fmt::Debug for ParserBase
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str("ParserBase(no better formatting)")
    }
}

impl ParserBase
{
    pub fn from_grammar(grammar: &str) -> Self
    {
        let mut reg = RegistryBuilder::new();
        let dfa = {
            let mut nfa = AutomataBuilder::new();
            let parser = GrammarParser::new();
            match parser.parse(&mut nfa, &mut reg, RegLexer::from(Lexer::new(grammar))) {
                Ok(res) => nfa.build(res),
                Err(lalrpop_util::ParseError::User{ error }) => panic!("{}", error),
                Err(err) => panic!("{:?}", err),
            }
            .unwrap_or_else(|Conflict(f, s)| panic!("Conflict: {:?} vs. {:?}", reg.unit(f), reg.unit(s)))
        };
        reg.init_lalr_items()
            .unwrap_or_else(|err| panic!("Couldn't init LALR items: {err}"));

        Self::from_parts(dfa, reg)
    }

    pub fn from_parts(dfa: DFA, reg: RegistryBuilder) -> Self
    {
        let actions = init_actions(&reg);
        ParserBase{ reg, dfa, actions }
    }

    pub fn create<'base, 'input, Data, ParseRes>(&'base self, to_parse: &'input str) -> Parser<'base, 'input, Data, ParseRes>
    {
        Parser {
            base: self,
            state_stack: vec![self.initial_state()],
            tokens: DFALexer::new(&self.dfa, to_parse),
            data: vec![],
            _phantom: PhantomData,
        }
    }

    fn initial_state(&self) -> ASTNode
    { ASTNode{ base: self, state: self.reg.initial_kern(), data: Node::HangingNTerm{ state: self.reg.initial_kern() } } }

    fn action(&self, kernel_id: KernelId, token: UnitId) -> Option<Action>
    {
        self.actions[*kernel_id].get(&token)
            .copied()
    }

    pub fn dfa(&self) -> &DFA
    { &self.dfa }

    pub fn registry(&self) -> &RegistryBuilder
    { &self.reg }

    pub fn actions(&self) -> &Vec<HashMap<UnitId, Action>>
    { &self.actions }

    pub fn set_sym<T, Res>(mut self) -> Self
        where T: ParsedData<Res>
    {
        let sym = <T as ParsedData<Res>>::sym_id();
        let eoi_id = self.reg.eoi_tok().id();
        self.reg.kernels()
            .for_each(|kern| kern.iter()
                .filter(|item| self.reg.lookaheads(kern.id(), **item).contains(&eoi_id))
                .filter(|item| item.is_final())
                .map(|item| item.rule_id())
                .map(|rule| self.reg.get_rule(rule).unwrap())
                .filter(|rule| rule.from_id() == sym)
                .for_each(|rule| {
                    println!("{:?} will accept by rule {}", kern.id(), DisplayRule(rule.id(), &self.reg));
                    self.actions[*kern.id()].insert(eoi_id, Action::Accept(rule.id()));
                })
            );

        // println!("{sym_kern_id:?} will accept by rule {}", DisplayRule(sym_rule.id(), self.registry()));
        // self.actions[*sym_kern_id].insert(eoi_id, Action::Accept(sym_rule.id()));
        self
    }
}

fn init_actions(reg: &RegistryBuilder) -> Vec<HashMap<UnitId, Action>>
{
    reg.kernels()
        .map(|kern| {
            let mut kern_actions: HashMap<_, _> = reg.units()
                .filter(|unit| unit.is_tok())
                .filter_map(|unit| reg.goto(kern.id(), unit.id())
                    .map(|to_kern| (unit.id(), to_kern))
                )
                .map(|(token, to_kern)| (token, Action::Shift(to_kern.id())))
                .collect();

            kern.iter()
                .filter(|item| item.is_final())
                .flat_map(|item| reg.lookaheads(kern.id(), *item).iter()
                    .map(move |lkhd| (item.rule_id(), lkhd))
                )
                .for_each(|(rule, lkhd)| {
                    let kern_id = kern.id();
                    match kern_actions.insert(*lkhd, Action::Reduce(rule)) {
                        Some(Action::Reduce(prev)) => {
                            reg.print_lalr_items();
                            panic!(
                                "Reduce-reduce conflict on {kern_id:?} [{}]:\n{}\nvs.\n{}",
                                reg.name_by_unit(*lkhd),
                                DisplayRule(rule, reg), DisplayRule(prev, reg)
                            )
                        },
                        Some(Action::Shift(to_kern)) => {
                            reg.print_lalr_items();
                            panic!(
                                "Shift-reduce conflict on {kern_id:?} [{}]: Shift({to_kern:?}) vs. Reduce({})",
                                reg.name_by_unit(*lkhd),
                                DisplayRule(rule, reg)
                            )
                        },
                        Some(Action::Accept(_)) => panic!("HOW"),
                        None => (),
                    }
                });

            kern_actions
        })
        .collect()
}

#[derive(Debug)]
enum Node
{
    Token { name: String, matched: String },
    ParsedNTerm { rule: RuleId, children: Vec<Node> },
    HangingNTerm { state: KernelId },
}

#[derive(Debug)]
pub struct ASTNode<'base>
{
    base: &'base ParserBase,
    state: KernelId,
    data: Node,
}

pub struct Parser<'base, 'input, Data, ParseRes>
{
    base: &'base ParserBase,
    state_stack: Vec<ASTNode<'base>>,
    tokens: DFALexer<'base, 'input>,
    data: Vec<Data>,
    _phantom: PhantomData<ParseRes>,
}

enum StepResult<'base, 'input, Data, ParseRes>
{
    Done(ParseRes),
    Continue(Parser<'base, 'input, Data, ParseRes>),
    Error(ParseError<'base>),
}
use StepResult::*;

impl<'base, 'input, Data, ParseRes> Parser<'base, 'input, Data, ParseRes>
    where Data: ActionCallback + ParsedData<ParseRes>
{
    pub fn parse(self) -> Result<ParseRes, ParseError<'base>>
    {
        let mut parser = self;
        loop {
            match parser.make_step() {
                Continue(new_state) => {
                    parser = new_state;
                    continue
                },
                Done(res) => break Ok(res),
                Error(err) => break Err(err),
            }
        }
    }

    fn make_step(mut self) -> StepResult<'base, 'input, Data, ParseRes>
    {
        let reg = &self.base.reg;
        let (matched, token_id) = match self.tokens.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => return self.parse_error(ErrKind::TokenizeErr(String::from(err))),
            None => {
                when! {
                    self.data.len() == 1 => return Done(<Data as ParsedData<ParseRes>>::extract_data(self.data_pop())),
                    _ => return self.parse_error(ErrKind::UnexpectedEOI),
                }
            }
        };
        let token_name = reg.name_by_unit(token_id);

        loop {
            let top = self.state_top();
            let state = top.state;

            // print!("State: {state:?}, Token: {token_name}, ");

            match self.base.action(state, token_id) {
                Some(Action::Shift(new_state)) => {
                    let tok_name = reg.name_by_unit(token_id);
                    let data = Node::Token { name: String::from(tok_name), matched: String::from(matched) };
                    self.state_push(new_state, data);
                    self.data_push(<Data as ActionCallback>::wrap_token(matched.into()));
                    // println!("Shift, to: {new_state:?}");
                    break Continue(self)
                },
                Some(Action::Reduce(rule_id)) => {
                    let (action_res, new_node) = self.reduce_by_rule(rule_id);
                    let rule = reg.get_rule(rule_id).unwrap();

                    let new_top = self.state_top();
                    let top_id = new_top.state;
                    let new_state = match reg.goto(top_id, rule.from_id()) {
                        Some(new_state) => new_state.id(),
                        None => {
                            break when! {
                                <Data as ParsedData<ParseRes>>::sym_id() == rule.from_id() => {
                                    panic!("How did I get here?")
                                    // self.push(state, new_node, action_res);
                                    // Done(<Data as ParsedData<ParseRes>>::extract_data(self.data_pop()))
                                },
                                _ => self.parse_error(ErrKind::ReduceFail(top_id, rule.from_id()))
                            }
                        },
                    };
                    self.push(new_state, new_node, action_res);
                    // println!("Reduce, rule: {}, to: {new_state:?}", DisplayRule(rule_id, reg));
                },
                Some(Action::Accept(rule_id)) => {
                    // println!("Accept({})", DisplayRule(rule_id, reg));
                    let (data, _node) = self.reduce_by_rule(rule_id);

                    // at this moment we should have empty data stack and only Hanging value in state stack
                    if self.state_stack.len() > 1 {
                        break self.parse_error(ErrKind::UnexpectedAccept(rule_id))
                    }
                    break Done(<Data as ParsedData<ParseRes>>::extract_data(data))
                }
                None => break self.parse_error(ErrKind::UnexpectedToken(token_id)),
            }
        }
    }

    fn reduce_by_rule(&mut self, rule_id: RuleId) -> (Data, Node)
    {
        let rule = self.base.registry().get_rule(rule_id).unwrap();

        if rule.is_eps_rule() {
            // push eps to data stack, as it will be taken now
            self.data_push(Data::wrap_token(String::new()));
        }
        let children = self.states_to_reduce(rule.to().len());
        let action_args = self.data_to_reduce(rule.to().len());

        let data = <Data as ActionCallback>::run_action(action_args, rule_id);
        let node = Node::ParsedNTerm { rule: rule_id, children };
        (data, node)
    }

    fn state_top(&mut self) -> &ASTNode<'base>
    { &self.state_stack.last().unwrap() }

    fn data_pop(&mut self) -> Data
    { self.data.pop().unwrap() }

    fn push(&mut self, kern_id: KernelId, node: Node, data: Data)
    {
        self.state_push(kern_id, node);
        self.data_push(data);
    }

    fn state_push(&mut self, new_state: KernelId, data: Node)
    { self.state_stack.push(ASTNode{ base: self.base, state: new_state, data }) }

    fn data_push(&mut self, new_data: Data)
    { self.data.push(new_data) }

    fn states_to_reduce(&mut self, count: usize) -> Vec<Node>
    {
        self.state_stack
            .drain((self.state_stack.len() - count)..)
            .map(|ast_node| ast_node.data)
            .collect()
    }

    fn data_to_reduce(&mut self, count: usize) -> Vec<Data>
    {
        self.data
            .drain((self.data.len() - count)..)
            .collect()
    }

    fn parse_error(self, kind: ErrKind) -> StepResult<'base, 'input, Data, ParseRes>
    { Error(ParseError{ base: self.base, states: self.state_stack, kind }) }
}

pub trait ActionCallback: Sized
{
    fn run_action(args: Vec<Self>, rule: RuleId) -> Self;

    fn wrap_token(token_str: String) -> Self;
}

pub trait ParsedData<ParseResult>
{
    fn extract_data(self) -> ParseResult;

    fn sym_id() -> UnitId;
}

#[derive(Debug)]
pub struct ParseError<'base>
{
    base: &'base ParserBase,
    states: Vec<ASTNode<'base>>,
    kind: ErrKind,
}

#[derive(Debug)]
enum ErrKind
{
    UnexpectedEOI,
    UnexpectedToken(UnitId),
    ReduceFail(KernelId, UnitId),
    UnexpectedAccept(RuleId),
    TokenizeErr(String),
}

impl<'base> Display for ParseError<'base>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        use ErrKind::*;
        match &self.kind {
            UnexpectedEOI => {
                f.write_str("Unexpected EOI")
            }
            UnexpectedToken(token_id) => {
                let token_str = self.base.reg.name_by_unit(*token_id);
                f.write_fmt(format_args!("UnexpectedToken: {token_str}"))
            },
            ReduceFail(kern_id, unit_id) => {
                let unit_str = self.base.reg.name_by_unit(*unit_id);
                f.write_fmt(format_args!("ReduceFail. Was on state {}, then parsed {unit_str}", **kern_id))
            }
            TokenizeErr(unparsed) => {
                f.write_fmt(format_args!("TokenizeErr. Unparsed part: {unparsed}"))
            }
            UnexpectedAccept(rule_id) => {
                f.write_fmt(format_args!("UnexpectedAccept. Rule was {{{}}}", DisplayRule(*rule_id, self.base.registry())))
            }
        }?;
        f.write_str("\nstack:")?;
        self.states.iter()
            .rev()
            .try_for_each(|node| f.write_char('\n')
                .and_then(|()| Display::fmt(node, f))
            )
    }
}

impl<'base> std::error::Error for ParseError<'base> {}

impl<'base> Display for ASTNode<'base>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut indent = String::new();
        f.write_fmt(format_args!("state: {}\n", *self.state))?;
        print_node(f, &self.data, self.base, &mut indent)
    }
}

fn print_node(f: &mut std::fmt::Formatter<'_>, node: &Node, base: &ParserBase, indent: &mut String) -> std::fmt::Result
{
    f.write_str(&indent)?;
    match node {
        Node::Token { name, matched } => f.write_fmt(format_args!("tok {name}{{{matched}}}")),
        Node::ParsedNTerm { rule, children } => {
            f.write_fmt(format_args!("NTerm{{{}}}", DisplayRule(*rule, &base.reg)))?;
            indent.push('-');
            children.iter()
                .try_for_each(|child| f.write_char('\n')
                    .and_then(|()| print_node(f, child, base, indent))
                )?;
            indent.pop();
            Ok(())
        },
        Node::HangingNTerm { state } => f.write_fmt(format_args!("Hanging{{ {} }}", **state)),
    }
}

#[derive(Clone, Copy)]
struct DisplayRule<'base>(RuleId, &'base RegistryBuilder);

impl<'base> Display for DisplayRule<'base>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let DisplayRule(rule_id, reg) = *self;

        let rule = reg.get_rule(rule_id).unwrap();
        let from = reg.name_by_unit(rule.from_id());
        f.write_str(from)
            .and_then(|()| f.write_str(" ->"))?;
        rule.to().iter()
            .map(|to_unit| reg.name_by_unit(*to_unit))
            .try_for_each(|to_str| f.write_fmt(format_args!(" {to_str}")))
    }
}
