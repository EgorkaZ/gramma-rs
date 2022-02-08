use mt3::{LexerParser, AutomataBuilder, Lexer, DFALexer, Registry, Conflict};

fn main()
{
    let parser = LexerParser::new();
    let mut nfa = AutomataBuilder::new();
    let mut reg = Registry::new();
    let lexemes = r#"
        lexer

        tok Type = { "int", "bool", "float" };
        tok Bool = { "True", "False" };

        tok Id = IdFst { IdFst, digit }*;

        IdFst = { lCase, uCase, "_" };
        lCase = [a-z];
        uCase = [A-Z];
        digit = [0-9];

        tok Int = "-"? [1-9] digit*;
        tok Number = Int "." digit*;

        tok Eq = "=";
        tok Semicolon = ";";

        lexer_end
    "#;

    let res = parser.parse(&mut nfa, &mut reg, Lexer::new(lexemes));

    // println!("hold it: '{}'", &input[241..]);

    let dfa = match res {
        Ok(res) => nfa.build(res),
        Err(err) => panic!("Failure((9(\n{:?}", err),
    }
    .unwrap_or_else(|Conflict(f, s)| panic!("Conflict: {:?} vs. {:?}", reg.unit(f), reg.unit(s)));

    let input = "
        int lol = 42;
        float kek = 42.24;

        bool fuck_my_life = True;
    ";

    DFALexer::new(&dfa, &input)
        .for_each(|mb_lexeme| match mb_lexeme {
            Ok((as_str, tok_id)) => println!("'{:?}': '{}'", reg.unit(tok_id), as_str),
            Err(err) => println!("error {}", err)
        });
}
