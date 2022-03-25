use std::{collections::{HashMap, HashSet}, ops::{BitOrAssign}, ptr::NonNull};

use crate::{UnitId, grammar_parser::registry::{UnitEntry, RegError}, RegistryBuilder};

use super::GrUnit;

#[derive(Debug, Default, Clone)]
pub struct FirstEntry<'reg>
{
    first: HashSet<UnitEntry<'reg>>,
    has_eps: bool,
}

impl FirstEntry<'_>
{
    pub fn non_eps(&self) -> impl Iterator<Item = UnitEntry<'_>>
    {
        self.firsts()
            .filter(|unit| !unit.is_eps())
    }

    pub fn firsts(&self) -> impl Iterator<Item = UnitEntry<'_>>
    { self.first.iter().copied() }

    /// returns if there were changes in `self.first`
    fn extend(&mut self, other: Self) -> bool
    {
        let old_size = self.first.len();
        *self |= other;
        let new_size = self.first.len();

        old_size != new_size
    }
}

impl BitOrAssign for FirstEntry<'_>
{
    fn bitor_assign(&mut self, rhs: Self)
    {
        rhs.first.into_iter()
            .for_each(|unit| {
                self.first.insert(unit);
            });
        self.has_eps |= rhs.has_eps;
    }
}

pub fn build_first<'reg>(reg: &'reg RegistryBuilder) -> Result<HashMap<UnitId, FirstEntry<'reg>>, RegError>
{
    let mut had_changes = true;
    let rules_it = reg.rules();
    let mut nterm_firsts = HashMap::new();
    loop {
        if !had_changes {
            break;
        }
        had_changes = false;
        for rule in rules_it.clone() {
            let (from_id, to_ids) = (rule.from_id(), rule.to());

            let upd_first = first(reg, &nterm_firsts, to_ids.iter());
            let curr_first = nterm_firsts
                .entry(from_id)
                .or_insert_with(|| {had_changes = true; /*println!("Inserting {}", reg.name_by_unit(from_id));*/ FirstEntry::default()});

            had_changes |= curr_first.extend(upd_first);
        }
    }


    reg.units()
        .try_for_each(|unit| {
            let id = unit.id();
            match *unit {
                GrUnit::NTerm{..} if nterm_firsts.contains_key(&id) => Ok(()),
                GrUnit::NTerm{..} => Err(RegError::undefined_id(id, reg)),
                GrUnit::Tok{..} => Ok(()),
            }
        })
        .map(|()| nterm_firsts)
}

pub fn first<'reg, 'it>(
    reg: &'reg RegistryBuilder,
    nterm_firsts: &HashMap<UnitId, FirstEntry<'reg>>,
    units: impl Iterator<Item = &'it UnitId>,
) -> FirstEntry<'reg>
{ first_rec(reg, NonNull::from(nterm_firsts), units, false) }

fn first_rec<'reg, 'it>(
    reg: &'reg RegistryBuilder,
    nterm_firsts: NonNull<HashMap<UnitId, FirstEntry<'reg>>>,
    mut units: impl Iterator<Item = &'it UnitId>,
    had_eps: bool) -> FirstEntry<'reg>
{
    let nterm_firsts = unsafe { nterm_firsts.as_ref() };
    match units.next() {
        None => { assert!(had_eps); FirstEntry{ first: HashSet::from_iter([reg.eps_tok()]), has_eps: true }},
        Some(&unit_id) => {
            let unit = reg.unit(unit_id);
            when! {
                unit.is_tok() => {
                    FirstEntry{ first: HashSet::from_iter([unit]), has_eps: unit.is_eps() }
                },
                _ => {
                    let mut first_set = HashSet::new();
                    let res = nterm_firsts.get(&unit_id);
                    let has_eps = res.map(|fst_entry| fst_entry.has_eps).unwrap_or(false);
                    let to_add = res.into_iter()
                        .flat_map(|fst_entry| fst_entry.non_eps());
                    first_set.extend(to_add);

                    let mut res = FirstEntry{ first: first_set, has_eps: false };
                    if has_eps {
                        res |= first_rec(reg, NonNull::from(nterm_firsts), units, has_eps);
                    }
                    res
                },
            }
        },
    }
}