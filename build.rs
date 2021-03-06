use std::error::Error;

extern crate lalrpop;

fn main() -> Result<(), Box<dyn Error>>
{
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process()
}
