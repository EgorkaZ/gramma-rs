// auto-generated: "lalrpop 0.19.7"
// sha3: be294b9227acece23d4c5e8aaa843f767e3f3290b1ddfd61f71b4c18df5540
use super::{
    tokenizer::{Token::*, *},
    AutomataBuilder, SubNFA, StrEdge, RangeEdge, EpsEdge,
    StatePtr, State
};
use kiam::when;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Lexer {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use super::super::{
    tokenizer::{Token::*, *},
    AutomataBuilder, SubNFA, StrEdge, RangeEdge, EpsEdge,
    StatePtr, State
};
    use kiam::when;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(Token<'input>),
        Variant1(&'input str),
        Variant2((char, char)),
        Variant3(String),
        Variant4(core::option::Option<Token<'input>>),
        Variant5(SubNFA),
        Variant6(alloc::vec::Vec<SubNFA>),
        Variant7((bool, SubNFA)),
        Variant8(alloc::vec::Vec<(bool, SubNFA)>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14, 0, 0, 0, 15, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 14, 0, 0, 0, 15, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 7, 0, 0, 24, 25, 26,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 7, 0, 0, 24, 25, 26,
        // State 5
        0, 0, -29, 0, -29, 0, 0, 23, 0, 0, 0, 0, 0, 7, -29, 0, 24, 25, 26,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 7, 0, 0, 24, 25, 26,
        // State 7
        0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 7, 0, 0, 24, 25, 26,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 7, 0, 0, 24, 25, 26,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, 0, 0, -16, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, 0, 0, 0, -17, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        -19, -19, -19, 0, -19, 0, -19, -19, 0, 0, 0, 0, 0, -19, -19, 0, -19, -19, -19,
        // State 19
        28, 29, -23, 0, -23, 0, 30, -23, 0, 0, 0, 0, 0, -23, -23, 0, -23, -23, -23,
        // State 20
        0, 0, -27, 0, -27, 0, 0, -27, 0, 0, 0, 0, 0, -27, -27, 0, -27, -27, -27,
        // State 21
        0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -22, -22, -22, 0, -22, 0, -22, -22, 0, 0, 0, 0, 0, -22, -22, 0, -22, -22, -22,
        // State 23
        -18, -18, -18, 0, -18, 0, -18, -18, 0, 0, 0, 0, 0, -18, -18, 0, -18, -18, -18,
        // State 24
        -21, -21, -21, 0, -21, 0, -21, -21, 0, 0, 0, 0, 0, -21, -21, 0, -21, -21, -21,
        // State 25
        -20, -20, -20, 0, -20, 0, -20, -20, 0, 0, 0, 0, 0, -20, -20, 0, -20, -20, -20,
        // State 26
        0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, -26, 0, -26, 0, 0, -26, 0, 0, 0, 0, 0, -26, -26, 0, -26, -26, -26,
        // State 28
        0, 0, -25, 0, -25, 0, 0, -25, 0, 0, 0, 0, 0, -25, -25, 0, -25, -25, -25,
        // State 29
        0, 0, -24, 0, -24, 0, 0, -24, 0, 0, 0, 0, 0, -24, -24, 0, -24, -24, -24,
        // State 30
        0, 0, -28, 0, -28, 0, 0, -28, 0, 0, 0, 0, 0, -28, -28, 0, -28, -28, -28,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, 0, 0, 0, -13, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, 0, 0, 0, -12, 0, 0,
        // State 33
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0,
        // State 34
        -10, -10, -10, 0, -10, 0, -10, -10, 0, 0, 0, 0, 0, -10, -10, 0, -10, -10, -10,
        // State 35
        -11, -11, -11, 0, -11, 0, -11, -11, 0, 0, 0, 0, 0, -11, -11, 0, -11, -11, -11,
        // State 36
        0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0,
        // State 37
        0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 19 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -32,
        // State 11
        0,
        // State 12
        -30,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -31,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            3 => 33,
            5 => 18,
            6 => match state {
                2 => 15,
                _ => 11,
            },
            8 => 2,
            9 => 19,
            10 => match state {
                5 => 30,
                _ => 20,
            },
            11 => 5,
            12 => match state {
                3 => 21,
                4 => 26,
                8 => 36,
                9 => 37,
                _ => 7,
            },
            13 => 10,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###"":""###,
            r###"";""###,
            r###""=""###,
            r###""?""###,
            r###""eps""###,
            r###""grammar""###,
            r###""grammar_end""###,
            r###""lexer""###,
            r###""lexer_end""###,
            r###""tok""###,
            r###""{""###,
            r###""}""###,
            r###"Action"###,
            r###"Id"###,
            r###"Rng"###,
            r###"Str"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, '__1>
    where 'input: '__1
    {
        nfa: &'__1 mut AutomataBuilder<'input>,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input, '__1> __state_machine::ParserDefinition for __StateMachine<'input, '__1>
    where 'input: '__1
    {
        type Location = usize;
        type Error = Error<'input>;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = SubNFA;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 19 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.nfa,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Star if true => Some(0),
            Plus if true => Some(1),
            Comma if true => Some(2),
            Colon if true => Some(3),
            Semicolon if true => Some(4),
            Eq if true => Some(5),
            QuestionMark if true => Some(6),
            Eps if true => Some(7),
            GrammarBegin if true => Some(8),
            GrammarEnd if true => Some(9),
            LexerBegin if true => Some(10),
            LexerEnd if true => Some(11),
            Tok if true => Some(12),
            LBrace if true => Some(13),
            RBrace if true => Some(14),
            FatArrow(_) if true => Some(15),
            Id(_) if true => Some(16),
            Range(_, _) if true => Some(17),
            Str(_) if true => Some(18),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 => __Symbol::Variant0(__token),
            15 | 16 => match __token {
                FatArrow(__tok0) | Id(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            17 => match __token {
                Range(__tok0, __tok1) if true => __Symbol::Variant2((__tok0, __tok1)),
                _ => unreachable!(),
            },
            18 => match __token {
                Str(__tok0) if true => __Symbol::Variant3(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct LexerParser {
        _priv: (),
    }

    impl LexerParser {
        pub fn new() -> LexerParser {
            LexerParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            nfa: &mut AutomataBuilder<'input>,
            __tokens0: __TOKENS,
        ) -> Result<SubNFA, __lalrpop_util::ParseError<usize, Token<'input>, Error<'input>>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    nfa,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<SubNFA,__lalrpop_util::ParseError<usize, Token<'input>, Error<'input>>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(nfa, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                // __Lexer = Lexer => ActionFn(0);
                let __sym0 = __pop_Variant5(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(nfa, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (bool, SubNFA), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (char, char), usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SubNFA, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token<'input>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<(bool, SubNFA)>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<SubNFA>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Token<'input>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "tok"? = "tok" => ActionFn(20);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // "tok"? =  => ActionFn(21);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action21::<>(nfa, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("," <LexSeq>) = ",", LexSeq => ActionFn(16);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action16::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("," <LexSeq>)* =  => ActionFn(14);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action14::<>(nfa, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (0, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("," <LexSeq>)* = ("," <LexSeq>)+ => ActionFn(15);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("," <LexSeq>)+ = ",", LexSeq => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("," <LexSeq>)+ = ("," <LexSeq>)+, ",", LexSeq => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(nfa, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("tok"?) = "tok" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ("tok"?) =  => ActionFn(29);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action29::<>(nfa, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (0, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexAlt = "{", LexSeq, "}" => ActionFn(32);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(nfa, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexAlt = "{", LexSeq, ("," <LexSeq>)+, "}" => ActionFn(33);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action33::<>(nfa, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (4, 5)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexDecl = "tok", Id, "=", LexSeq, ";" => ActionFn(34);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action34::<>(nfa, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (5, 6)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexDecl = Id, "=", LexSeq, ";" => ActionFn(35);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action35::<>(nfa, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (4, 6)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexDecl* =  => ActionFn(22);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action22::<>(nfa, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (0, 7)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexDecl* = LexDecl+ => ActionFn(23);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexDecl+ = LexDecl => ActionFn(24);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexDecl+ = LexDecl+, LexDecl => ActionFn(25);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrim = Id => ActionFn(9);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrim = LexAlt => ActionFn(10);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrim = Str => ActionFn(11);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrim = Rng => ActionFn(12);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrim = "eps" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrimQual = LexPrim => ActionFn(5);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrimQual = LexPrim, "?" => ActionFn(6);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action6::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrimQual = LexPrim, "+" => ActionFn(7);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action7::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrimQual = LexPrim, "*" => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrimQual+ = LexPrimQual => ActionFn(17);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexPrimQual+ = LexPrimQual+, LexPrimQual => ActionFn(18);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action18::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (2, 11)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // LexSeq = LexPrimQual+ => ActionFn(3);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(nfa, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Lexer = "lexer", "lexer_end" => ActionFn(36);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36::<>(nfa, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        nfa: &mut AutomataBuilder<'input>,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Lexer = "lexer", LexDecl+, "lexer_end" => ActionFn(37);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(nfa, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (3, 13)
    }
}
pub use self::__parse__Lexer::LexerParser;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, SubNFA, usize),
) -> SubNFA
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, decls, _): (usize, alloc::vec::Vec<(bool, SubNFA)>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> SubNFA
{
    {
        let start = nfa.add_state(State::casual());
        let end = nfa.add_state(State::terminal());

        decls.iter()
            .filter_map(|(is_tok, sub)| when! {
                *is_tok => Some(sub),
                _ => None,
            })
            .for_each(|sub| {
                nfa.add_sym(&start, EpsEdge, sub.input());
                nfa.add_sym(sub.output(), EpsEdge, &end);
            });
        SubNFA(start, end)
    }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, tok, _): (usize, core::option::Option<Token<'input>>, usize),
    (_, id, _): (usize, &'input str, usize),
    (_, _, _): (usize, Token<'input>, usize),
    (_, curr, _): (usize, SubNFA, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> (bool, SubNFA)
{
    {
        nfa.assign_to_id(id, SubNFA::clone(&curr));
        (tok.is_some(), curr)
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, seq, _): (usize, alloc::vec::Vec<SubNFA>, usize),
) -> SubNFA
{
    {
        let input = StatePtr::clone(seq[0].input());
        let mut prev_to = seq[0].output();
        for SubNFA(from, to) in seq[1..].iter() {
            prev_to.extend(from);
            prev_to = to;
        }
        SubNFA(input, StatePtr::clone(prev_to))
    }
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, fst, _): (usize, SubNFA, usize),
    (_, rest, _): (usize, alloc::vec::Vec<SubNFA>, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> SubNFA
{
    {
        let curr = nfa.create_sub();
        nfa.add_sym(curr.input(), EpsEdge, fst.input());
        nfa.add_sym(fst.output(), EpsEdge, curr.output());

        for sub in rest.iter() {
            nfa.add_sym(curr.input(), EpsEdge, sub.input());
            nfa.add_sym(sub.output(), EpsEdge, curr.output());
        }
        curr
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, SubNFA, usize),
) -> SubNFA
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, curr, _): (usize, SubNFA, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> SubNFA
{
    {
        nfa.add_sym(curr.input(), EpsEdge, curr.output());
        curr
    }
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, sub, _): (usize, SubNFA, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> SubNFA
{
    {
        let curr = nfa.create_sub();
        nfa.add_sym(curr.input(), EpsEdge, sub.input());
        nfa.add_sym(sub.output(), EpsEdge, curr.output());
        nfa.add_sym(sub.output(), EpsEdge, sub.input());
        curr
    }
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, sub, _): (usize, SubNFA, usize),
    (_, _, _): (usize, Token<'input>, usize),
) -> SubNFA
{
    {
        let curr = nfa.create_sub();
        // parse "+"
        nfa.add_sym(curr.input(), EpsEdge, sub.input());
        nfa.add_sym(sub.output(), EpsEdge, curr.output());
        nfa.add_sym(sub.output(), EpsEdge, sub.input());
        // and also eps
        nfa.add_sym(curr.input(), EpsEdge, curr.output());
        curr
    }
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, &'input str, usize),
) -> SubNFA
{
    nfa.resolve_id(__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, SubNFA, usize),
) -> SubNFA
{
    __0
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, String, usize),
) -> SubNFA
{
    {
        let curr = nfa.create_sub();
        nfa.add_sym(curr.input(), StrEdge(__0), curr.output());
        curr
    }
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, rng, _): (usize, (char, char), usize),
) -> SubNFA
{
    {
        let curr = nfa.create_sub();
        nfa.add_sym(curr.input(), RangeEdge::new(rng.0, rng.1), curr.output());
        curr
    }
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> SubNFA
{
    {
        let curr = nfa.create_sub();
        nfa.add_sym(curr.input(), EpsEdge, curr.output());
        curr
    }
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<SubNFA>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, v, _): (usize, alloc::vec::Vec<SubNFA>, usize),
) -> alloc::vec::Vec<SubNFA>
{
    v
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, _, _): (usize, Token<'input>, usize),
    (_, __0, _): (usize, SubNFA, usize),
) -> SubNFA
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, SubNFA, usize),
) -> alloc::vec::Vec<SubNFA>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, v, _): (usize, alloc::vec::Vec<SubNFA>, usize),
    (_, e, _): (usize, SubNFA, usize),
) -> alloc::vec::Vec<SubNFA>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, core::option::Option<Token<'input>>, usize),
) -> core::option::Option<Token<'input>>
{
    __0
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, Token<'input>, usize),
) -> core::option::Option<Token<'input>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Token<'input>>
{
    None
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<(bool, SubNFA)>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, v, _): (usize, alloc::vec::Vec<(bool, SubNFA)>, usize),
) -> alloc::vec::Vec<(bool, SubNFA)>
{
    v
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, (bool, SubNFA), usize),
) -> alloc::vec::Vec<(bool, SubNFA)>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, v, _): (usize, alloc::vec::Vec<(bool, SubNFA)>, usize),
    (_, e, _): (usize, (bool, SubNFA), usize),
) -> alloc::vec::Vec<(bool, SubNFA)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, __0, _): (usize, SubNFA, usize),
) -> alloc::vec::Vec<SubNFA>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    (_, v, _): (usize, alloc::vec::Vec<SubNFA>, usize),
    (_, e, _): (usize, SubNFA, usize),
) -> alloc::vec::Vec<SubNFA>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
) -> core::option::Option<Token<'input>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action20(
        nfa,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        nfa,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Token<'input>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action21(
        nfa,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        nfa,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, SubNFA, usize),
) -> alloc::vec::Vec<SubNFA>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action16(
        nfa,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        nfa,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, alloc::vec::Vec<SubNFA>, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, SubNFA, usize),
) -> alloc::vec::Vec<SubNFA>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action16(
        nfa,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        nfa,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, SubNFA, usize),
    __2: (usize, Token<'input>, usize),
) -> SubNFA
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action14(
        nfa,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        nfa,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, SubNFA, usize),
    __2: (usize, alloc::vec::Vec<SubNFA>, usize),
    __3: (usize, Token<'input>, usize),
) -> SubNFA
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action15(
        nfa,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        nfa,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Token<'input>, usize),
    __3: (usize, SubNFA, usize),
    __4: (usize, Token<'input>, usize),
) -> (bool, SubNFA)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action28(
        nfa,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        nfa,
        __temp0,
        __1,
        __2,
        __3,
        __4,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, &'input str, usize),
    __1: (usize, Token<'input>, usize),
    __2: (usize, SubNFA, usize),
    __3: (usize, Token<'input>, usize),
) -> (bool, SubNFA)
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        nfa,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        nfa,
        __temp0,
        __0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, Token<'input>, usize),
) -> SubNFA
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action22(
        nfa,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        nfa,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    nfa: &mut AutomataBuilder<'input>,
    __0: (usize, Token<'input>, usize),
    __1: (usize, alloc::vec::Vec<(bool, SubNFA)>, usize),
    __2: (usize, Token<'input>, usize),
) -> SubNFA
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        nfa,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        nfa,
        __0,
        __temp0,
        __2,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, Error<'input>>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, Error<'input>>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), Error<'input>>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, Error<'input>>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
