use std::{collections::{HashMap, HashSet}, ptr::NonNull, ops::Deref};

use crate::{RegistryBuilder, UnitId};

use super::{ItemSet, GrUnit, ItemId};

pub type ItemDFA = HashMap<Box<ItemSet<ItemId>>, HashMap<UnitId, NonNull<ItemSet<ItemId>>>>;

pub fn create_items<'reg>(reg: &'reg RegistryBuilder) -> ItemDFA
{
    let mut mapping = ItemDFA::new();

    let initial = reg.rules()
        .filter_map(|rule| {
            match *rule.from() {
                GrUnit::NTerm{ is_sym: true, .. } => Some(reg.rule_to_item(rule.id())),
                _ => None,
            }
        })
        .collect();

    mapping.insert(Box::new(initial), Default::default());

    let mut had_changes = true;
    let mut sets: Vec<_> = mapping.keys()
        .map(Box::as_ref)
        .map(NonNull::from)
        .collect();
    while had_changes {
        had_changes = false;

        let mut next_sets = vec![];
        for item_set in sets {
            let item_set = unsafe { item_set.as_ref() };

            let closed_set = closure(reg, item_set.deref().clone());
            for unit in reg.units() {
                let gotos = mapping.get(item_set).unwrap();
                if gotos.contains_key(&unit.id()) {
                    continue;
                }
                let curr_goto = goto(reg, &closed_set, unit.id());
                if !curr_goto.is_empty() {
                    let curr_goto = Box::new(curr_goto);
                    let curr_ref = NonNull::from(curr_goto.as_ref());

                    let inserted = mapping.try_insert(curr_goto, Default::default());
                    let curr_ref = match inserted {
                        Ok(_) => {
                            next_sets.push(curr_ref);
                            curr_ref
                        },
                        Err(err) => {
                            NonNull::from(err.entry.key().as_ref())
                        },
                    };
                    had_changes = true;

                    let gotos = mapping.get_mut(item_set).unwrap();
                    had_changes |= gotos.insert(unit.id(), curr_ref).is_none();
                    assert!(had_changes);
                }
            }
        }
        sets = next_sets;
    }

    mapping
}

fn closure(reg: &RegistryBuilder, mut items: ItemSet<ItemId>) -> ItemSet<ItemId>
{
    let mut changed = true;
    while changed {
        let unfinished_items: HashSet<ItemId> = items.iter()
            .filter(|item| !item.is_final())
            .copied()
            .filter_map(|item_id| {
                let curr_unit = reg.item_unit(item_id);
                match curr_unit.deref() {
                    GrUnit::NTerm{ rules, .. } => {
                        let items: Vec<_> = rules.iter()
                            .copied()
                            .map(|rule_id| reg.get_rule(rule_id))
                            .map(|rule| {
                                let rule = rule.as_ref().unwrap();
                                ItemId::begin(rule.id(), rule.to().len())
                            })
                            .collect();
                        Some(items)
                    },
                    _ => None,
                }
            })
            .flat_map(Vec::into_iter)
            .collect();

        let old_len = items.len();
        items.extend(unfinished_items);
        let new_len = items.len();

        changed = old_len != new_len;
    }
    items
}

fn goto(reg: &RegistryBuilder, curr_set: &ItemSet<ItemId>, unit: UnitId) -> ItemSet<ItemId>
{
    curr_set.iter()
        .copied()
        .filter_map(|item| reg.next_item(item, unit))
        .collect()
}

// fn print(reg: &RegistryBuilder, mapping: &ItemDFA)
// {
//     let set_idx: HashMap<_, _> = mapping.iter()
//         .map(|(key, _)| key)
//         .map(|set| -> *const ItemSet<_> { set.as_ref() })
//         .enumerate()
//         .map(|(idx, ptr)| (ptr as usize, idx))
//         .collect();

//     for (set_ptr, idx) in set_idx.iter() {
//         println!("{idx}:");

//         let set_ptr = (*set_ptr) as *const ItemSet<_>;
//         let set = unsafe { set_ptr.as_ref() }.unwrap();
//         print_item_set(reg, set);
//         let gotos = mapping.get(set).unwrap();
//         for (unit_id, to_set) in gotos.iter() {
//             let unit_str = reg.name_by_unit(*unit_id);
//             let to_set_ptr: *const ItemSet<_> = unsafe { to_set.as_ref() };
//             let to_set_idx = set_idx.get(&(to_set_ptr as usize)).unwrap();
//             println!("`-{unit_str}-> {to_set_idx}");
//         }
//         println!("");
//     }
// }

// fn print_item_set(reg: &RegistryBuilder, set: &ItemSet<ItemId>)
// {
//     let print_item = |item: &ItemId| {
//         let rule = reg.get_rule(item.rule_id()).unwrap();
//         let from_str = reg.name_by_unit(rule.from_id());
//         print!("{from_str} -> ");
//         for (idx, unit_id) in rule.to().iter().copied().enumerate() {
//             if idx == item.pos() {
//                 print!("(*) ");
//             }
//             let unit_str = reg.name_by_unit(unit_id);
//             print!("{unit_str} ");
//         }
//         if item.pos() == rule.to().len() {
//             print!("(*) ");
//         }
//         println!("");
//     };
//     set.iter().for_each(print_item);

//     println!("-------------------------");
//     let closed = closure(reg, set.clone());
//     closed.iter()
//         .filter(|item| !set.contains(**item))
//         .for_each(print_item);
//     println!("=========================");
// }
