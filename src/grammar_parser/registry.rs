use std::borrow::{Borrow};
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::{Deref, IndexMut};
use std::fmt::Debug;

use kiam::when;

pub enum GrUnit
{
    NTerm { rules: Vec<RuleId> },
    Tok { is_eps: bool },
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnitId(usize);

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RuleId(usize);

impl Deref for UnitId
{
    type Target = usize;

    fn deref(&self) -> &Self::Target
    { &self.0 }
}

impl Deref for RuleId
{
    type Target = usize;

    fn deref(&self) -> &Self::Target
    { &self.0 }
}

struct Rule
{
    from: UnitId,
    to: Vec<UnitId>,
}

pub struct InterRule
{
    id: RuleId,
    pos: usize,
}

pub struct RegEntry<'reg, T>
{
    info: &'reg T,
    owner: &'reg Registry,
}

impl<T> Deref for RegEntry<'_, T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    { self.info.borrow() }
}

type UnitEntry<'a> = RegEntry<'a, GrUnit>;
type RuleEntry<'a> = RegEntry<'a, Rule>;

pub struct Registry
{
    named_units: HashMap<String, UnitId>,
    units: Vec<Option<GrUnit>>,
    rules: Vec<Rule>,
}

impl Registry
{
    pub fn new() -> Self
    { Self::default() }

    pub fn get_by_name(&mut self, name: &str) -> UnitId
    {
        match self.named_units.get(name) {
            Some(found) => *found,
            None => {
                let res = UnitId(self.units.len());
                self.units.push(None);
                self.named_units.insert(name.into(), res);
                res
            }
        }
    }

    pub fn set_tok(&mut self, name: &str) -> Result<UnitId, RegError>
    {
        let id = self.get_by_name(name);
        let unit = self.get_mut_unit(id);
        if unit.is_some() {
            return Err(RegError::AcquiredId(id))
        }
        *unit = Some(GrUnit::Tok{ is_eps: false });
        Ok(id)
    }

    pub fn get_unit(&self, id: UnitId) -> Option<UnitEntry<'_>>
    {
        self.units[*id]
            .as_ref()
            .map(|unit| self.as_entry(unit))
    }

    /// NB! will panic if unit isn't initialized
    pub fn unit(&self, id: UnitId) -> UnitEntry<'_>
    {
        self.get_unit(id)
            .expect(&format!("Unit {} was not initialized", *id))
    }


    fn get_mut_unit(&mut self, id: UnitId) -> &mut Option<GrUnit>
    {
        self.units.index_mut(*id)
    }

    fn as_entry<'reg, T>(&'reg self, info: &'reg T) -> RegEntry<'reg, T>
    { RegEntry{ info: info, owner: &self } }

}

impl Default for Registry
{
    fn default() -> Self
    {
        let mut reg = Registry{ named_units: HashMap::default(), units: vec![], rules: vec![] };
        reg.set_tok("eps").unwrap();

        match reg.get_mut_unit(UnitId(0)) {
            eps@None => { *eps = Some(GrUnit::Tok{ is_eps: true }) },
            _ => (),
        }
        reg
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum RegError
{
    AcquiredId(UnitId),
}


impl<T> Debug for RegEntry<'_, T>
    where T: IdSearchible
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let tok_name = self.restore_id(&self.owner)
            .map(UnitId)
            .and_then(|id| self.owner.named_units.iter()
                .find_map(|(name, curr_id)| if id == *curr_id {
                    Some(&name[..])
                } else {
                    None
                }))
            .unwrap_or("UNRECOGNIZED_TOKEN");

        f.write_str(tok_name)
    }
}

trait IdSearchible
{
    fn restore_id<'reg>(&'reg self, reg: &'reg Registry) -> Option<usize>;

    fn count_dist<T>(base: usize, offseted: usize, max_len: usize) -> Option<usize>
    {
        if base > offseted {
            return None
        }

        let dist = offseted - base;
        let idx = dist / size_of::<T>();
        when! {
            idx > max_len => None,
            _ => Some(idx),
        }
    }
}

impl IdSearchible for Option<GrUnit>
{
    fn restore_id<'reg>(&'reg self, reg: &'reg Registry) -> Option<usize>
    {
        let data_begin: *const Option<GrUnit> = reg.units.as_ptr();
        let info_ptr: *const Option<GrUnit> = self;

        Self::count_dist::<Option<GrUnit>>(
            data_begin as usize,
            info_ptr as usize,
            reg.units.len())
    }
}

impl IdSearchible for GrUnit
{
    fn restore_id<'reg>(&'reg self, reg: &'reg Registry) -> Option<usize>
    {
        let data_begin: *const Option<GrUnit> = reg.units.as_ptr();
        let info_ptr: *const GrUnit = self;

        Self::count_dist::<Option<GrUnit>>(
            data_begin as usize,
            info_ptr as usize,
            reg.units.len())
    }
}
