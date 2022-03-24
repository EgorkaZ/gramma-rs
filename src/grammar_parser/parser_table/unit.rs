use std::ops::Deref;

use super::RuleId;

pub enum GrUnit
{
    NTerm { id: UnitId, rules: Vec<RuleId>, res_type: String, is_sym: bool },
    Tok { id: UnitId, is_eps: bool },
}

impl GrUnit
{
    pub fn id(&self) -> UnitId
    {
        match self {
            Self::NTerm { id, .. } => *id,
            Self::Tok{ id, .. } => *id,
        }
    }

    pub fn is_eps(&self) -> bool
    {
        match self {
            GrUnit::NTerm { .. } => false,
            GrUnit::Tok{ is_eps, .. } => *is_eps,
        }
    }

    pub fn is_tok(&self) -> bool
    {
        match self {
            GrUnit::Tok{ .. } => true,
            _ => false,
        }
    }

    pub fn is_nterm(&self) -> bool
    {
        match self {
            GrUnit::NTerm{ .. } => true,
            _ => false,
        }
    }

    pub fn is_sym(&self) -> bool
    {
        match self {
            GrUnit::NTerm{ is_sym, .. } => *is_sym,
            GrUnit::Tok{ .. } => false,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnitId(pub usize);

impl Deref for UnitId
{
    type Target = usize;

    fn deref(&self) -> &Self::Target
    { &self.0 }
}
