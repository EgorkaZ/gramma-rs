use std::{collections::HashMap, ops::Deref, iter};

use crate::{DFA, RegistryBuilder, parser::ParserBase, GrUnit, UnitId};

pub fn parser_into_code(base: &ParserBase) -> Vec<String>
{
    let mut lines = vec![];
    lines.push("use crate::{*, lexer::{Symbol::*, *}};".into());
    lines.push("use crate::parser::{ParserBase, ParseError, ActionCallback, ParsedData};".into());

    lines.push("fn create_parser_base() -> ParserBase".into());
    lines.push("{".into());
    lines.extend(dfa_into_code(base.dfa()));
    lines.extend(registry_into_code(base.registry()));
    lines.push(format!("ParserBase::from_parts(dfa, reg)"));
    lines.push("}".into());

    let instances = base.registry()
        .units()
        .filter(|unit| unit.is_sym())
        .map(|unit| sym_parser_into_code(base, unit.id()));
    lines.extend(instances);
    lines
}

fn dfa_into_code(dfa: &DFA) -> Vec<String>
{
    let symbols: Vec<_> = dfa.symbols()
        .map(|sym| sym.as_ref())
        .collect();

    let sym_to_idx: HashMap<_, _> = symbols.iter()
        .enumerate()
        .map(|(idx, sym)| (*sym, idx))
        .collect();

    let states: Vec<_> = dfa.states()
        .cloned()
        .collect();

    let state_to_idx: HashMap<_, _> = states.iter()
        .enumerate()
        .map(|(idx, sym)| (sym.clone(), idx))
        .collect();

    let states: Vec<_> = states.iter()
        .map(|state| {
            let edges = state.edges();
            let edges = edges.borrow();

            let edges: Vec<_> = edges.iter()
                .map(|edge| {
                    let through = edge.through();
                    let through_idx = sym_to_idx.get(through).unwrap();
                    let to_idx = state_to_idx.get(edge.to()).unwrap();
                    (through_idx, to_idx)
                })
                .collect();
            (state, edges)
        })
        .map(|(state, edges)| (state.mb_tok_id(), edges))
        .collect();

    let mut lines = vec![];
    lines.push(format!("let dfa = {{"));
    lines.push(format!("    let states = vec!["));
    let states_lines = states.iter()
        .map(|(mb_tok, edges)| {
            let line = format!("        ({mb_tok:?}, vec![");
            let line = edges.iter()
                .map(|(sym, to_state)| format!("({sym}, {to_state}),"))
                .fold(line, |mut line, curr| line + &curr);
            line + "]),"
        });
    lines.extend(states_lines);
    lines.push(format!("    ];"));
    lines.push(format!("    let start_state = {};", state_to_idx.get(dfa.start()).unwrap()));
    lines.push(format!("    let symbols = vec!["));
    symbols.iter()
        .for_each(|sym| lines.push(format!("        {sym:?},")));
    lines.push(format!("    ];"));
    lines.push(format!("DFA::from_vecs(states, start_state, symbols)"));
    lines.push(format!("}};"));
    lines
}

fn registry_into_code(reg: &RegistryBuilder) -> Vec<String>
{
    let mut lines = vec![];
    lines.push(format!("let named_units = vec!["));
    let names_lines = reg.names_to_units()
        .map(|(name, unit_id)| {
            format!(r#"    (String::from("{name}"), {unit_id:?}),"#)
        });
    lines.extend(names_lines);
    lines.push(format!("];"));

    lines.push(format!("let tokens = vec!["));
    let tokens_lines = reg.units()
        .filter(|unit| unit.is_tok())
        .map(|token| format!("    ({:?}, {}), ", token.id(), token.is_eps()));
    lines.extend(tokens_lines);
    lines.push(format!("];"));

    lines.push(format!("let nterms = vec!["));
    let nterm_lines = reg.units()
        .filter_map(|unit| match unit.deref() {
            GrUnit::NTerm { id, rules, res_type, is_sym } => {
                let mut line = format!("    ({id:?}, vec![");
                line += &rules.iter()
                    .fold(String::new(), |line, curr| line + &format!("{curr:?},"));
                line += &format!(r#"], String::from("{res_type}"), {is_sym}),"#);
                Some(line)
            },
            GrUnit::Tok{ .. } => None,
        });
    lines.extend(nterm_lines);
    lines.push(format!("];"));

    lines.push(format!("let rules = vec!["));
    let rule_lines = reg.rules()
        .map(|rule| {
            let mut line = format!("    ({:?}, vec![", rule.from_id());
            line += &rule.to().iter()
                .fold(String::new(), |line, to_unit| line + &format!("{to_unit:?},"));
            line += "], vec![";
            line += &rule.arg_names().iter()
                .map(|arg_name| match arg_name {
                    Some(name) => format!(r#"Some(String::from("{name}")),"#),
                    None => format!("None,"),
                })
                .fold(String::new(), |line, curr_name| line + &curr_name);
            line += &format!(r###"], String::from(r##"{}"##)),"###, rule.action());
            line
        });
    lines.extend(rule_lines);
    lines.push(format!("];"));

    lines.push(format!("let reg = RegistryBuilder::from_vecs(named_units, tokens, nterms, rules);"));
    lines
}

fn sym_parser_into_code(base: &ParserBase, sym_id: UnitId) -> String
{
    let sym_name = base.registry().name_by_unit(sym_id);
    let sym = base.registry().unit(sym_id);

    let res_type = match sym.deref() {
        GrUnit::NTerm{ res_type, .. } => res_type,
        GrUnit::Tok{ .. } => unreachable!(),
    };

    format!(r#"
pub struct {sym_name}Parser
{{
    base: ParserBase
}}

impl {sym_name}Parser
{{
    pub fn new() -> Self
    {{ {sym_name}Parser{{ base: create_parser_base() }} }}

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<{res_type}, ParseError<'this>>
    {{ self.base.create::<'this, 'input, _Data, {res_type}>(to_parse).parse() }}
}}
"#)
}
