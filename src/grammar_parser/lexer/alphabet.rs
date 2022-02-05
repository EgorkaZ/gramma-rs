use std::{collections::HashSet, rc::Rc};

use super::symbol::{Symbol};

#[derive(Debug, Clone)]
pub struct Alphabet
{
    symbols: HashSet<Rc<Symbol>>,
}

impl Alphabet
{
    pub fn new() -> Self
    { Alphabet::default() }

    pub fn add_sym<T>(&mut self, sym: T) -> Rc<Symbol>
        where Symbol: From<T>
    { self.get_or_insert(sym) }

    pub fn symbols(&self) -> impl Iterator<Item = &Rc<Symbol>>
    { self.symbols.iter() }

    // private
    fn get_or_insert<T>(&mut self, val: T) -> Rc<Symbol>
        where Symbol: From<T>
    {
        let as_sym: Symbol = val.into();
        match self.symbols.get(&as_sym) {
            Some(found) => Rc::clone(found),
            None => {
                let inserted = Rc::new(as_sym);
                self.symbols.insert(Rc::clone(&inserted));
                inserted
            }
        }
    }
}

impl Default for Alphabet
{
    fn default() -> Self
    { Alphabet{ symbols: HashSet::default() } }
}
