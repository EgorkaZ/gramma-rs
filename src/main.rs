use mt3::{LexerParser, AutomataBuilder, Lexer, DFALexer};

fn main()
{
    let parser = LexerParser::new();
    let mut nfa = AutomataBuilder::new();
    let input = r#"
        lexer

        tok Lexer = "lexer";
        tok LexerEnd = "lexer_end";

        lexer_end
    "#;

    let res = parser.parse(&mut nfa, Lexer::new(input));

    // println!("hold it: '{}'", &input[241..]);

    let dfa = match res {
        Ok(res) => nfa.build(res),
        Err(err) => panic!("Failure((9(\n{:?}", err),
    };

    let input = "lexer lexer_end";

    DFALexer::new(&dfa, &input).for_each(|mb_lexeme| match mb_lexeme {
        Ok(lexeme) => println!("token '{}'", lexeme),
        Err(err) => println!("error {}", err)
    });
}
