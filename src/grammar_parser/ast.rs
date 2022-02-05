#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unit<'input>
{
    Term(&'input str),
    NonTerm(&'input str),
    String(&'input str),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transit<'input>
{
    pub from: Unit<'input>,
    pub to: Vec<Unit<'input>>,
    pub code: Option<&'input str>,
}

#[derive(Default, Debug)]
pub struct Transitions<'input>(pub Vec<Transit<'input>>);
