use std::{env, path::Path, error::Error, fs::{File, self}, io::Write};

use crate::parser::ParserBase;

use super::{parser_into_code, generate_actions};

pub fn process_grs_files() -> Result<(), Box<dyn Error>>
{
    let curr_dir = env::current_dir()?;
    traverse_dirs(&curr_dir)
}

fn traverse_dirs(dir: &Path) -> Result<(), Box<dyn Error>>
{
    for sub_path in dir.read_dir()? {
        let sub_path = sub_path?;
        let sub_path = sub_path.path();

        when! {
            sub_path.is_dir() => traverse_dirs(&sub_path)?,
            sub_path.is_file() => process_some_file(&sub_path)?,
            _ => ()
        }
    }
    Ok(())
}

fn process_some_file(path: &Path) -> Result<(), Box<dyn Error>>
{
    match path.extension() {
        Some(ext) if ext == "grs" => (),
        Some(_) | None => return Ok(()),
    }
    println!("cargo:rerun-if-changed={}", path.display());

    let grammar = fs::read_to_string(path)?;
    let real_start = grammar.find("grammar").unwrap();

    let copied = &grammar[0..real_start];
    let parser = ParserBase::from_grammar(&grammar[real_start..]);

    let code_path = path.with_extension("rs");
    fs::remove_file(&code_path)?;

    let mut code_file = File::options()
        .create(true)
        .write(true)
        .append(false)
        .open(code_path)?;

    // eprintln!("Writing copied part");
    // eprintln!("{copied}");
    writeln!(code_file, "{copied}")?;

    // eprintln!("Writing initial code part");

    parser_into_code(&parser).into_iter()
        // .inspect(|line| eprintln!("{line}"))
        .try_for_each(|line| writeln!(code_file, "{line}"))?;

    // eprintln!("Writing actions part");
    generate_actions(&parser).into_iter()
        // .inspect(|line| eprintln!("{line}"))
        .try_for_each(|line| writeln!(code_file, "{line}"))?;
    // eprintln!("Written actions part");
    Ok(())
}
