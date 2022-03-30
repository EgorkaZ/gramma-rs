use std::{ops::Deref, borrow::Borrow, collections::{HashMap, HashSet}, iter};

use crate::{UnitId, RegistryBuilder, grammar_parser::{parser_table::{GrUnit, first}}, KernelId};

use super::{ItemId, ItemSet, FirstEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LAItemId
{
    item: ItemId,
    lookahead: UnitId,
}

impl Deref for LAItemId
{
    type Target = ItemId;

    fn deref(&self) -> &Self::Target
    { &self.item }
}

impl Borrow<ItemId> for LAItemId
{
    fn borrow(&self) -> &ItemId
    { &self.item }
}

pub fn init_lookaheads<'reg>(reg: &'reg RegistryBuilder, nterm_firsts: &HashMap<UnitId, FirstEntry<'reg>>) -> HashMap<(KernelId, ItemId), HashSet<UnitId>>
{
    let mut lookaheads = HashMap::<(KernelId, ItemId), HashSet<UnitId>>::new();
    let mut propagation = HashMap::<(KernelId, ItemId), HashSet<(KernelId, ItemId)>>::new();

    for kern in reg.kernels() {
        for item in kern.iter() {
            lookaheads.insert((kern.id(), *item), Default::default());
        }
    }

    for item in reg.kernel(reg.initial_kern()).iter() {
        let lkhds = lookaheads.get_mut(&(reg.initial_kern(), *item)).unwrap();
        lkhds.insert(reg.eoi_tok().id());
    }

    for kern in reg.kernels() {
        propagate_lookaheads(
            reg, nterm_firsts, kern.id(),
            &mut lookaheads, &mut propagation
        );
    }

    let mut changed = true;
    while changed {
        changed = false;
        let propagate_pairs = propagation.iter()
            .map(|(from, to_set)| to_set.iter().map(|to| (*from, *to)))
            .flatten();
        for (from, to) in propagate_pairs {
            let from_lkhds: Vec<_> = match lookaheads.get(&from) {
                Some(lkhds) => lkhds.into_iter().copied().collect(),
                None => continue,
            };
            let to_lkhds = lookaheads.get_mut(&to).unwrap();

            let old_len = to_lkhds.len();
            to_lkhds.extend(from_lkhds.iter().copied());
            let new_len = to_lkhds.len();

            changed |= old_len != new_len;
        }
    }
    // let kernel_items: HashSet<ItemId> = reg.kernels()
    //     .flat_map(|kern| kern.iter())
    //     .copied()
    //     .collect();
    // lookaheads.into_iter()
    //     .filter(|(item_id, _)| kernel_items.contains(item_id))
    //     .collect()
    lookaheads
}

fn propagate_lookaheads<'reg>(
    reg: &'reg RegistryBuilder,
    nterm_firsts: &HashMap<UnitId, FirstEntry<'reg>>,
    kern_id: KernelId,
    lookaheads: &mut HashMap<(KernelId, ItemId), HashSet<UnitId>>,
    propagation: &mut HashMap<(KernelId, ItemId), HashSet<(KernelId, ItemId)>>)
{
    // print!("closure(");
    // print_item(reg, item_id);
    // println!("):");
    let pseudo_unit = reg.pseudo_unit();
    let kern = reg.kernel(kern_id);

    for item_id in kern.iter() {
        let closed = closure(reg, nterm_firsts, &ItemSet::from(LAItemId{ item: *item_id, lookahead: pseudo_unit }));
        closed.iter()
            .filter(|la_item| !la_item.is_final())
            .map(|la_item| (la_item, reg.item_unit(la_item.item).id()))
            .map(|(la_item, curr_unit)| (la_item, reg.goto(kern_id, curr_unit).unwrap()))
            .for_each(|(la_item, to_kern)| {
                let next_item = la_item.next_item().unwrap();
                if la_item.lookahead == pseudo_unit {
                    propagation.entry((kern_id, *item_id))
                        .or_insert_with(Default::default)
                        .insert((to_kern.id(), next_item));
                } else {
                    lookaheads.entry((to_kern.id(), next_item))
                        .or_insert_with(Default::default)
                        .insert(la_item.lookahead);
                }
            });
        // propagation.get(&(kern_id, *item_id))
        //     .into_iter()
        //     .flat_map(|to_kerns| to_kerns.iter())
        //     .for_each(|to_kern| reg.kernel(*to_kern).iter()
        //         .for_each(|to_item| { print_item(reg, *to_item); println!("") })
        //     )
    }
}

fn closure(
    reg: &RegistryBuilder,
    nterm_firsts: &HashMap<UnitId, FirstEntry>,
    items: &ItemSet<LAItemId>) -> ItemSet<LAItemId>
{
    let mut res = items.clone();
    let mut changed = true;

    while changed {
        let mut unfinished_items = HashSet::new();
        res.iter()
            .filter(|item| !item.is_final())
            .copied()
            .filter_map(|producing_item| match reg.item_unit(*producing_item).deref() { // find rules which begin from current non-term
                GrUnit::NTerm{ rules, .. } => {
                    let closed_items: Vec<_> = rules.iter()
                        .map(|rule_id| reg.rule_to_item(*rule_id))
                        .collect();

                    let first_begin = producing_item.pos() + 1;
                    let rule = reg.get_rule(producing_item.rule_id()).unwrap();
                    let first_iter = rule.to().iter()
                        .skip(first_begin)
                        .chain(iter::once(&producing_item.lookahead));

                    let first = first(reg, nterm_firsts, first_iter);
                    let first_toks: Vec<_> = first.firsts()
                        .filter_map(|unit| match unit.deref() {
                            tok@GrUnit::Tok{ .. } => Some(tok.id()),
                            GrUnit::NTerm { .. } => None,
                        })
                        .collect();
                    Some((closed_items, first_toks))
                },
                GrUnit::Tok{ .. } => None
            })
            .for_each(|(closed_items, first_toks)| {
                let la_items = closed_items.into_iter()
                    .flat_map(|item_id| {
                        first_toks.iter()
                            .map(move |tok| LAItemId{ item: item_id, lookahead: *tok })
                    });
                unfinished_items.extend(la_items);
            });

        let old_len = res.len();
        res.extend(unfinished_items);
        let new_len = res.len();

        changed = old_len != new_len;
    }
    res
}

fn print_item(reg: &RegistryBuilder, item_id: ItemId)
{
    let rule_id = item_id.rule_id();
    let rule = reg.get_rule(rule_id).unwrap();

    let from = rule.from();
    let from_str = reg.name_by_unit(from.id());
    print!("{from_str} -> ");
    for (idx, curr_to_id) in rule.to().iter().enumerate() {
        if item_id.pos() == idx {
            print!("(*) ");
        }
        let curr_to_str = reg.name_by_unit(*curr_to_id);
        print!("{curr_to_str} ");
    }
    if item_id.pos() == rule.to().len() {
        print!("(*) ");
    }
}

#[allow(dead_code)]
fn print_item_lkhd(reg: &RegistryBuilder, lookaheads: &HashMap<ItemId, HashSet<UnitId>>, item_id: ItemId)
{
    print_item(reg, item_id);
    if let Some(curr_lkhds) = lookaheads.get(&item_id) {
        print!("[");
        let mut iter = curr_lkhds.iter();
        if let Some(fst) = iter.next() {
            let fst_str = reg.name_by_unit(*fst);
            print!("{fst_str}");
        }
        iter.map(|curr| reg.name_by_unit(*curr)).for_each(|curr| print!(", {curr}"));
        print!("]");
    }
    println!("")
}
