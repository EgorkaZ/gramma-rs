use std::{ops::Deref, iter};

use crate::{parser::ParserBase, RuleId, UnitId, GrUnit};

pub fn generate_actions(base: &ParserBase) -> Vec<String>
{
    let mut lines: Vec<_> = base.registry()
        .units()
        .filter(|unit| unit.is_nterm())
        .map(|unit| generate_nterm_actions(base, unit.id()))
        .intersperse(vec!["".into(), "".into()])
        .flat_map(IntoIterator::into_iter)
        .collect();

    lines.extend(["", "// outer world", ""].map(Into::into));
    lines.extend(data_enum(base));
    lines.push(action_callback(base));

    let extract_data = base.registry().units()
        .filter(|unit| unit.is_sym())
        .map(|unit| extract_data(base, unit.id()));
    lines.extend(extract_data);
    lines
}

fn data_enum(base: &ParserBase) -> Vec<String>
{
    let mut lines = vec!["pub enum _Data".into(), "{".into()];
    let variants = base.registry()
        .units()
        .filter_map(|unit| match unit.deref() {
            GrUnit::NTerm { res_type, .. } => Some((unit.id(), res_type.clone())),
            GrUnit::Tok{ .. } => None,
        })
        .map(|(nterm_id, res_type)| (base.registry().name_by_unit(nterm_id), res_type))
        .map(|(nterm_name, res_type)| format!("    _{nterm_name}({res_type}),"));
    lines.extend(variants);
    lines.push("    _Token(String),".into());
    lines.push("}".into());
    lines
}

fn action_callback(base: &ParserBase) -> String
{
    let rule_matchings: String = base.registry().rules()
        .map(|rule| {
            let from_name = base.registry().name_by_unit(rule.from_id());
            let rule_idx = match rule.from().deref() {
                GrUnit::NTerm{ rules, .. } => rules.iter().position(|checked| *checked == rule.id()),
                GrUnit::Tok{ .. } => unreachable!(),
            }.unwrap();
            (rule, format!("_{from_name}_{rule_idx}"), from_name)
        })
        .map(|(rule, func_name, from_name)| {
            let mut line = format!("            {} => match (", *rule.id());
            let checked_args = (0..rule.to().len())
                .map(|_| format!("args.next().unwrap()"))
                .intersperse(",".into());
            line.extend(checked_args);
            line.extend(") {\n".chars());

            line.extend(iter::repeat(' ').take(16));
            line.push('(');
            let matched_args = rule.to().iter()
                .map(|to_id| base.registry().unit(*to_id))
                .enumerate()
                .map(|(idx, to_unit)| when! {
                    to_unit.is_tok() => format!("Self::_Token(_{idx})"),
                    _ => format!("Self::_{}(_{idx})", base.registry().name_by_unit(to_unit.id())),
                })
                .intersperse(",".into());
            line.extend(matched_args);
            line.push(')');

            line.extend(format!(" => Self::_{from_name}({func_name}(").chars());
            let func_args = (0..rule.to().len())
                .map(|idx| format!("_{idx}"))
                .intersperse(",".into());
            line.extend(func_args);
            line.extend(")),\n".chars());

            line.extend(iter::repeat(' ').take(16));
            line += r#"_ => panic!("Invalid rule/argument set: {rule_id:?}"),"#;
            line.push('\n');
            line.extend(iter::repeat(' ').take(12));
            line.extend("},\n".chars());

            line
        })
        .chain(iter::once("            _ => unreachable!(),".into()))
        .collect();
    format!(r#"
impl ActionCallback for _Data
{{
    fn run_action(args: Vec<Self>, rule_id: RuleId, base: &ParserBase) -> Self
    {{
        let mut args = args.into_iter();
        let rule = base.registry().get_rule(rule_id).unwrap();
        let arg_cnt = rule.to().len();
        match *rule_id {{
{rule_matchings}
        }}
    }}

    fn wrap_token(token_str: String) -> Self
    {{ Self::_Token(token_str) }}
}}
"#)
}

fn extract_data(base: &ParserBase, nterm_id: UnitId) -> String
{
    let nterm = base.registry().unit(nterm_id);

    let res_type = match nterm.deref() {
        GrUnit::NTerm{ res_type, .. } => res_type,
        GrUnit::Tok{ .. } => unreachable!(),
    };
    let nterm_name = base.registry().name_by_unit(nterm_id);

    format!(r#"
impl ParsedData<{res_type}> for _Data
{{
    fn extract_data(self) -> {res_type}
    {{
        match self {{
            Self::_{nterm_name}(res) => res,
            _ => panic!("Couldn't extract {res_type}"),
        }}
    }}

    fn sym_id() -> UnitId
    {{ {nterm_id:?} }}
}}
"#)
}

fn generate_nterm_actions(base: &ParserBase, nterm_id: UnitId) -> Vec<String>
{
    let reg = base.registry();
    let nterm = reg.unit(nterm_id);

    let (rules, res_type) = match nterm.deref() {
        GrUnit::NTerm{ rules, res_type, .. } => (rules, res_type),
        GrUnit::Tok{ .. } => unreachable!(),
    };
    rules.iter()
        .enumerate()
        .map(|(idx, rule)| generate_action(base, *rule, idx, res_type))
        .intersperse(vec![String::from("")])
        .flat_map(IntoIterator::into_iter)
        .collect()
}

fn generate_action(base: &ParserBase, rule_id: RuleId, idx: usize, res_type: &str) -> Vec<String>
{
    let reg = base.registry();
    let rule = reg.get_rule(rule_id).unwrap();

    let nterm_name = reg.name_by_unit(rule.from_id());

    let mut lines = vec![];
    lines.push(format!("fn _{nterm_name}_{idx}("));
    let arg_lines = rule.to().iter().zip(rule.arg_names())
        .map(|(to_unit, arg_name)| (reg.unit(*to_unit), arg_name))
        .map(|(to_unit, arg_name)| {
            let arg_name = arg_name.as_ref()
                .map(|name| &name[..])
                .unwrap_or("_");
            let arg_type = match to_unit.deref() {
                GrUnit::NTerm{ res_type, .. } => &res_type,
                GrUnit::Tok{ .. } => "String",
            };
            format!("    {arg_name}: {arg_type},")
        });
    lines.extend(arg_lines);
    lines.push(format!(") -> {res_type}"));

    lines.push(rule.action().into());
    lines
}
