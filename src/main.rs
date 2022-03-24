use std::{fs::File, io::Write};

use gramma_rs::{parser::ParserBase, codegen::{parser_into_code, generate_actions}};
use gramma_rs::degenerated::ExprParser;

fn main()
{

    let lexemes = r#"
    grammar
        sym Sym
        { S => {}
        }

        S
        { L Eq R => {}
        , R => {}
        }

        L
        { Star R => {}
        , Id => {}
        }

        R
        { L => {}
        }
    grammar_end
    lexer
        tok Id = { alpha, "_" } { alnum, "_" }*;
        alpha = { [a-z], [A-Z] };
        alnum = { alpha, [0-9] };

        tok Star = "*";
        tok Eq = "=";
    lexer_end
    "#;

    let lexemes = r#"
        grammar

        sym Program : (i32, i64)
        { Lines => {}
        }

        Lines : ()
        { Stmt => {}
        , Lines Stmt => {}
        }

        Stmt : HashMap<Lol, Box<Kek>>
        { Decl => {}
        , IfStmt => {}
        , Expr Semicolon => {}
        }

        Decl : ()
        { Type Id MbAssign Semicolon => {}
        }

        MbAssign : ()
        { Eps => {}
        , Eq Expr => {}
        }

        Expr : ()
        { Id => {}
        , Int => {}
        , Number => {}
        , BoolExpr => {}
        , LPar Expr RPar => {}
        , Id LPar ArgComma Expr RPar => {}
        }

        ArgComma : ()
        { e:Expr => { vec![e] }
        , args:ArgComma Comma last:Expr => { args.push(last); args }
        }

        BoolExpr : ()
        { Bool => {}
        , Expr CmpOp Expr => {}
        }

        IfStmt : ()
        { If BoolExpr Colon => {}
        }

        grammar_end

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
        tok Colon = ":";
        tok Comma = ",";

        tok If = "if";
        tok CmpOp = { "==", "!=", "<=", "=>", "<", ">" };

        tok LPar = "(";
        tok RPar = ")";
        tok EOL = "\n";

        lexer_end
    "#;

    let lexemes = r#"
    grammar

        sym Expr : i32
        { e:SubExpr => { e }
        }

        SubExpr : i32
        { lhs:SubExpr Plus  rhs:Term => { lhs + rhs }
        , lhs:SubExpr Minus rhs:Term => { lhs - rhs }
        , t:Term => { t }
        , Minus t:Term => { -t }
        }

        Term : i32
        { lhs:Term Star rhs:Fact => { lhs * rhs }
        , lhs:Term Div  rhs:Fact => { lhs / rhs }
        , f:Fact => { f }
        }

        Fact : i32
        { LPar e:SubExpr RPar => { e }
        , n:Num => { n.parse().unwrap() }
        }

    grammar_end
    lexer

        tok Num = [1-9] [0-9]*;

        tok Plus = "+";
        tok Star = "*";
        tok Minus = "-";
        tok Div = "/";


        tok LPar = "(";
        tok RPar = ")";

    lexer_end
    "#;

    let base = ParserBase::new(lexemes);

    let input = "- 1 - (2 + 4)";

    let parser = ExprParser::new();
    let res = parser.parse(input)
        .unwrap_or_else(|err| panic!("No parse: {err}"));
    println!("Oh, my god, I've parsed {res}");


    // let mut file = File::create("src/degenerated.rs").expect("Couldn't create");
    // parser_into_code(&base).into_iter()
    //     .chain(generate_actions(&base))
    //     .try_for_each(|line| writeln!(file, "{line}"))
    //     .expect("Couldn't write line");
}
