use crate::{*, lexer::{Symbol::*, *}};
use crate::parser::{ParserBase, ParseError, ActionCallback, ParsedData};
fn create_parser_base() -> ParserBase
{
let dfa = {
    let states = vec![
        (Some(UnitId(12)), vec![(5, 8),]),
        (None, vec![(1, 6),(2, 2),(0, 3),(3, 4),(7, 5),(8, 7),(6, 0),]),
        (Some(UnitId(11)), vec![]),
        (Some(UnitId(7)), vec![]),
        (Some(UnitId(4)), vec![]),
        (Some(UnitId(6)), vec![]),
        (Some(UnitId(10)), vec![]),
        (Some(UnitId(9)), vec![]),
        (Some(UnitId(12)), vec![(5, 8),]),
    ];
    let start_state = 1;
    let symbols = vec![
        Str(StrEdge(String::from("*"))),
        Str(StrEdge(String::from("("))),
        Str(StrEdge(String::from(")"))),
        Str(StrEdge(String::from("+"))),
        Eps(EpsEdge),
        Range(RangeEdge('0', '9')),
        Range(RangeEdge('1', '9')),
        Str(StrEdge(String::from("-"))),
        Str(StrEdge(String::from("/"))),
    ];
DFA::from_vecs(states, start_state, symbols)
};
let named_units = vec![
    (String::from("RPar"), UnitId(11)),
    (String::from("Minus"), UnitId(6)),
    (String::from("Num"), UnitId(12)),
    (String::from("Fact"), UnitId(8)),
    (String::from(":PseudoToken:"), UnitId(13)),
    (String::from("Expr"), UnitId(3)),
    (String::from("SubExpr"), UnitId(2)),
    (String::from("Eps"), UnitId(0)),
    (String::from("EOI"), UnitId(1)),
    (String::from("Plus"), UnitId(4)),
    (String::from("Term"), UnitId(5)),
    (String::from("Star"), UnitId(7)),
    (String::from("Div"), UnitId(9)),
    (String::from("LPar"), UnitId(10)),
];
let tokens = vec![
    (UnitId(0), true), 
    (UnitId(1), false), 
    (UnitId(4), false), 
    (UnitId(6), false), 
    (UnitId(7), false), 
    (UnitId(9), false), 
    (UnitId(10), false), 
    (UnitId(11), false), 
    (UnitId(12), false), 
    (UnitId(13), false), 
];
let nterms = vec![
    (UnitId(2), vec![RuleId(1),RuleId(2),RuleId(3),RuleId(4),], String::from("i32"), false),
    (UnitId(3), vec![RuleId(0),], String::from("i32"), true),
    (UnitId(5), vec![RuleId(5),RuleId(6),RuleId(7),], String::from("i32"), false),
    (UnitId(8), vec![RuleId(8),RuleId(9),], String::from("i32"), false),
];
let rules = vec![
    (UnitId(3), vec![UnitId(2),], vec![Some(String::from("e")),], String::from(r##"{ e }"##)),
    (UnitId(2), vec![UnitId(2),UnitId(4),UnitId(5),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs + rhs }"##)),
    (UnitId(2), vec![UnitId(2),UnitId(6),UnitId(5),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs - rhs }"##)),
    (UnitId(2), vec![UnitId(5),], vec![Some(String::from("t")),], String::from(r##"{ t }"##)),
    (UnitId(2), vec![UnitId(6),UnitId(5),], vec![None,Some(String::from("t")),], String::from(r##"{ -t }"##)),
    (UnitId(5), vec![UnitId(5),UnitId(7),UnitId(8),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs * rhs }"##)),
    (UnitId(5), vec![UnitId(5),UnitId(9),UnitId(8),], vec![Some(String::from("lhs")),None,Some(String::from("rhs")),], String::from(r##"{ lhs / rhs }"##)),
    (UnitId(5), vec![UnitId(8),], vec![Some(String::from("f")),], String::from(r##"{ f }"##)),
    (UnitId(8), vec![UnitId(10),UnitId(2),UnitId(11),], vec![None,Some(String::from("e")),None,], String::from(r##"{ e }"##)),
    (UnitId(8), vec![UnitId(12),], vec![Some(String::from("n")),], String::from(r##"{ n.parse().unwrap() }"##)),
];
let reg = RegistryBuilder::from_vecs(named_units, tokens, nterms, rules);
ParserBase::from_parts(dfa, reg)
}

pub struct ExprParser
{
    base: ParserBase
}

impl ExprParser
{
    pub fn new() -> Self
    { ExprParser{ base: create_parser_base() } }

    pub fn parse<'this, 'input>(&'this self, to_parse: &'input str) -> Result<i32, ParseError<'this>>
    { self.base.create::<'this, 'input, _Data, i32>(to_parse).parse() }
}

fn _SubExpr_0(
    lhs: i32,
    _: String,
    rhs: i32,
) -> i32
{ lhs + rhs }

fn _SubExpr_1(
    lhs: i32,
    _: String,
    rhs: i32,
) -> i32
{ lhs - rhs }

fn _SubExpr_2(
    t: i32,
) -> i32
{ t }

fn _SubExpr_3(
    _: String,
    t: i32,
) -> i32
{ -t }


fn _Expr_0(
    e: i32,
) -> i32
{ e }


fn _Term_0(
    lhs: i32,
    _: String,
    rhs: i32,
) -> i32
{ lhs * rhs }

fn _Term_1(
    lhs: i32,
    _: String,
    rhs: i32,
) -> i32
{ lhs / rhs }

fn _Term_2(
    f: i32,
) -> i32
{ f }


fn _Fact_0(
    _: String,
    e: i32,
    _: String,
) -> i32
{ e }

fn _Fact_1(
    n: String,
) -> i32
{ n.parse().unwrap() }

// outer world

pub enum _Data
{
    _SubExpr(i32),
    _Expr(i32),
    _Term(i32),
    _Fact(i32),
    _Token(String),
}

impl ActionCallback for _Data
{
    fn run_action(args: Vec<Self>, rule_id: RuleId, base: &ParserBase) -> Self
    {
        let mut args = args.into_iter();
        let rule = base.registry().get_rule(rule_id).unwrap();
        let arg_cnt = rule.to().len();
        match *rule_id {
            0 => match (args.next().unwrap()) {
                (Self::_SubExpr(_0)) => Self::_Expr(_Expr_0(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            1 => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_SubExpr(_0),Self::_Token(_1),Self::_Term(_2)) => Self::_SubExpr(_SubExpr_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            2 => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_SubExpr(_0),Self::_Token(_1),Self::_Term(_2)) => Self::_SubExpr(_SubExpr_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            3 => match (args.next().unwrap()) {
                (Self::_Term(_0)) => Self::_SubExpr(_SubExpr_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            4 => match (args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_Term(_1)) => Self::_SubExpr(_SubExpr_3(_0,_1)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            5 => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Term(_0),Self::_Token(_1),Self::_Fact(_2)) => Self::_Term(_Term_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            6 => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Term(_0),Self::_Token(_1),Self::_Fact(_2)) => Self::_Term(_Term_1(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            7 => match (args.next().unwrap()) {
                (Self::_Fact(_0)) => Self::_Term(_Term_2(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            8 => match (args.next().unwrap(),args.next().unwrap(),args.next().unwrap()) {
                (Self::_Token(_0),Self::_SubExpr(_1),Self::_Token(_2)) => Self::_Fact(_Fact_0(_0,_1,_2)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            9 => match (args.next().unwrap()) {
                (Self::_Token(_0)) => Self::_Fact(_Fact_1(_0)),
                _ => panic!("Invalid rule/argument set: {rule_id:?}"),
            },
            _ => unreachable!(),
        }
    }

    fn wrap_token(token_str: String) -> Self
    { Self::_Token(token_str) }
}


impl ParsedData<i32> for _Data
{
    fn extract_data(self) -> i32
    {
        match self {
            Self::_Expr(res) => res,
            _ => panic!("Couldn't extract i32"),
        }
    }

    fn sym_id() -> UnitId
    { UnitId(3) }
}

