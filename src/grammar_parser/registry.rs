use std::{
    ops::{Deref, DerefMut},
    fmt::{Debug, Display}, ptr::{self, NonNull},
    borrow::{Borrow, BorrowMut},
    collections::{HashMap, HashSet},
    hash::Hash, error::Error, mem::MaybeUninit,
};

use super::parser_table::{GrUnit, Rule, UnitId, RuleId, build_first, ItemSet, ItemId, create_items, KernelId, ItemDFA, init_lookaheads};

pub trait IdToIdentified
{
    type Identified;

    fn search<'reg>(self, registry: &'reg RegistryBuilder) -> Option<&'reg Self::Identified>;
}

pub trait IdToIdentifiedMut: IdToIdentified
{
    fn search_mut<'reg>(self, registry: &'reg mut RegistryBuilder) -> Option<&'reg mut Self::Identified>;
}

#[derive(Clone, Copy)]
pub struct RegEntry<Id, RegRef>
    where Id: IdToIdentified,
      RegRef: Borrow<RegistryBuilder>,
{
    info_id: Id,
    owner: RegRef,
}

impl<Id, RegRef> RegEntry<Id, RegRef>
    where RegRef: Borrow<RegistryBuilder>,
              Id: IdToIdentified + Copy,
{
    pub fn id(&self) -> Id
    { self.info_id }
}

impl<Id, RegRef> PartialEq for RegEntry<Id, RegRef>
    where Id: IdToIdentified + PartialEq,
      RegRef: Borrow<RegistryBuilder>,
{
    fn eq(&self, other: &Self) -> bool
    {
        assert!(ptr::eq(self.owner.borrow(), other.owner.borrow()));
        self.info_id == other.info_id
    }
}

impl<Id, RegRef> Eq for RegEntry<Id, RegRef>
    where Id: IdToIdentified + Eq,
      RegRef: Borrow<RegistryBuilder>,
{}

impl<Id, RegRef> Hash for RegEntry<Id, RegRef>
    where Id: IdToIdentified + Hash,
      RegRef: Borrow<RegistryBuilder>,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    { self.info_id.hash(state); }
}

pub type UnitEntry<'a> = RegEntry<UnitId, &'a RegistryBuilder>;
pub type UnitEntryMut<'a> = RegEntry<UnitId, &'a mut RegistryBuilder>;
pub type RuleEntry<'a> = RegEntry<RuleId, &'a RegistryBuilder>;
pub type RuleEntryMut<'a> = RegEntry<RuleId, &'a mut RegistryBuilder>;
pub type KernEntry<'a> = RegEntry<KernelId, &'a RegistryBuilder>;

pub struct RegistryBuilder
{
    named_units: HashMap<String, UnitId>,
    units: Vec<Option<GrUnit>>,
    rules: Vec<Rule>,
    kerns: Vec<ItemSet<ItemId>>,
    goto: Vec<HashMap<UnitId, KernelId>>,
    lookaheads: HashMap<(KernelId, ItemId), HashSet<UnitId>>,
    initial_kern: Option<KernelId>,
}

impl RegistryBuilder
{
    pub fn new() -> Self
    { Self::default() }

    pub fn unit_by_name(&mut self, name: &str) -> UnitId
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

    pub fn name_by_unit(&self, id: UnitId) -> &str
    {
        self.named_units
            .iter()
            .find(|(_, curr_id)| **curr_id == id)
            .map(|(name, _)| &name[..])
            .unwrap_or("UNNAMED_UNIT")
    }

    pub fn names_to_units(&self) -> impl Iterator<Item = (&String, &UnitId)>
    { self.named_units.iter() }

// setters
    pub fn set_tok(&mut self, name: &str) -> Result<UnitId, RegError>
    {
        let id = self.unit_by_name(name);
        // println!("setting 'tok' to '{name}' ({id:?})");

        match &mut self.units[*id] {
            Some(_) => Err(RegError::redefined_id(id, self)),
            unit_entry@None => {
                *unit_entry = Some(GrUnit::Tok{ id, is_eps: false });
                Ok(id)
            }
        }
    }

    pub fn set_nterm(&mut self, name: &str, res_type: String, rules: Vec<(Vec<(UnitId, Option<&str>)>, &str)>, is_sym: bool) -> Result<UnitId, RegError>
    {
        let id = self.unit_by_name(name);
        let rules_cnt = self.rules.len();
        match &mut self.units[*id] {
            Some(_) => Err(RegError::redefined_id(id, self)),
            unit_entry@None => {
                let rules = rules.into_iter()
                    .map(|(rule, action)| rule.into_iter()
                        .fold((vec![], vec![], action), |(mut ids, mut names, _), (part_id, name)| {
                            ids.push(part_id);
                            names.push(name.map(String::from));
                            (ids, names, action)
                        }))
                    .map(|(ids, names, action)| Rule::new(id, ids, names, action.into()));
                let rule_ids = (0..rules.len())
                        .map(|id| id + rules_cnt)
                        .map(RuleId)
                        .collect();
                *unit_entry = Some(GrUnit::NTerm{ id, rules: rule_ids, res_type, is_sym });
                self.rules.extend(rules);
                Ok(id)
            }
        }
    }

    pub fn assert_units_defined(&self) -> Result<(), RegError>
    {
        let undefined = self.units
            .iter()
            .position(|unit| unit.is_none())
            .map(UnitId)
            .map(|id| RegError::undefined_id(id, self));

        match undefined {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }

// unit getters
    pub fn get_unit(&self, id: UnitId) -> Option<UnitEntry<'_>>
    { self.as_entry(id) }

    /// NB! will panic if unit isn't initialized
    pub fn unit(&self, id: UnitId) -> UnitEntry<'_>
    {
        self.get_unit(id)
            .expect(&format!("Unit {} was not initialized", *id))
    }

    pub fn eps_tok(&self) -> UnitEntry<'_>
    { self.unit(UnitId(0)) }

    pub fn eoi_tok(&self) -> UnitEntry<'_>
    { self.unit(UnitId(1)) }

    fn get_unit_mut(&mut self, id: UnitId) -> Option<UnitEntryMut<'_>>
    { self.as_entry_mut(id) }

    pub fn units(&self) -> impl Iterator<Item = UnitEntry<'_>>
    {
        self.units.iter()
            .map(|unit| UnitEntry{ info_id: unit.as_ref().unwrap().id(), owner: self })
    }

    pub fn pseudo_unit(&self) -> UnitId
    {
        let unit_id = self.named_units.get(":PseudoToken:")
            .expect("Pseudo token is not initialized");
        *unit_id
    }

// rule getters
    pub fn get_rule(&self, id: RuleId) -> Option<RuleEntry<'_>>
    { self.as_entry(id) }

    pub fn get_rule_mut(&mut self, id: RuleId) -> Option<RuleEntryMut<'_>>
    { self.as_entry_mut(id) }

    fn as_entry_mut<Id>(&mut self, id: Id) -> Option<RegEntry<Id, &mut Self>>
        where Id: IdToIdentifiedMut + Copy,
    {
        match id.search_mut(self) {
            Some(_) => Some(RegEntry{ info_id: id, owner: self }),
            None => None,
        }
    }

    pub fn rules(&self) -> impl Iterator<Item = RuleEntry<'_>> + Clone
    {
        self.rules.iter()
            .enumerate()
            .map(|(id, _rule)| RuleEntry{ info_id: RuleId(id), owner: self })
    }

// items
    pub fn next_item(&self, item: ItemId, next_unit: UnitId) -> Option<ItemId>
    {
        let rule = self.get_rule(item.rule_id()).unwrap();
        let to = rule.to();

        when! {
            item.pos() >= to.len() => None,
            to[item.pos()] == next_unit => item.next_item(),
            _ => None
        }
    }

    pub fn item_unit(&self, item: ItemId) -> UnitEntry<'_>
    {
        let rule = self.get_rule(item.rule_id()).unwrap();
        let to_id = rule.to()[item.pos()];
        self.unit(to_id)
    }

    pub fn rule_to_item(&self, rule_id: RuleId) -> ItemId
    {
        let rule = self.get_rule(rule_id).unwrap();
        let to = rule.to();

        let item_len = if rule.is_eps_rule() {
            0
        } else {
            to.len()
        };
        ItemId::begin(rule_id, item_len)
    }

    pub fn init_kernels(&mut self, items: ItemDFA)
    {
        let mapping: HashMap<_, _> = items.keys()
            .enumerate()
            .map(|(idx, set)| (set.as_ref(), KernelId(idx)))
            .collect();

        let mut kern_dfa: HashMap<KernelId, HashMap<UnitId, KernelId>> = items.iter()
            .map(|(set, gotos)| {
                let set = mapping.get(set.as_ref()).unwrap();
                let gotos = gotos.iter()
                    .map(|(unit, to)| {
                        let to = unsafe { to.as_ref() };
                        (*unit, *mapping.get(&to).unwrap())
                    })
                    .collect();
                (*set, gotos)
            })
            .collect();
        self.goto = (0..kern_dfa.len())
            .map(KernelId)
            .map(|from| kern_dfa.remove(&from).unwrap())
            .collect();

        let mut kerns = Vec::with_capacity(mapping.len());
        for _ in 0..mapping.len() {
            kerns.push(MaybeUninit::<ItemSet<ItemId>>::uninit());
        }
        for (set, _) in items.iter() {
            let idx = mapping.get(set.as_ref()).unwrap();
            kerns[**idx].write(set.deref().clone());
        }
        self.kerns = kerns.into_iter()
            .map(|val| unsafe { val.assume_init() })
            .collect();
        self.initial_kern = self.kerns.iter()
            .enumerate()
            .flat_map(|(id, items)| items.iter()
                .map(move |item| (item, KernelId(id)))
            )
            .find(|(item, _)| item.pos() == 0)
            .map(|(_, kern_id)| kern_id);
    }

    pub fn init_lalr_items(&mut self) -> Result<(), RegError>
    {
        let items_dfa = create_items(self);
        self.init_kernels(items_dfa);

        let _pseudo_tok = self.set_tok(":PseudoToken:");
        let nterm_first = build_first(self)?;
        // for (unit_id, entry) in nterm_first.iter() {
        //     let unit_name = self.name_by_unit(*unit_id);
        //     print!("{unit_name}:");

        //     for tok in entry.firsts() {
        //         let tok_name = self.name_by_unit(tok.id());
        //         print!(" {tok_name},")
        //     }
        //     println!("")
        // }

        self.lookaheads = init_lookaheads(self, &nterm_first);
        Ok(())
    }

    pub fn print_lalr_items(&self)
    {
        println!("Items (kernels)");
        for kernel in self.kernels() {
            println!("----------------------{:?}", kernel.id());
            for item in kernel.iter() {
                let rule_id = item.rule_id();
                let rule = self.get_rule(rule_id).unwrap();

                let from = rule.from();
                let from_str = self.name_by_unit(from.id());
                print!("{from_str} -> ");
                for (idx, curr_to_id) in rule.to().iter().enumerate() {
                    if item.pos() == idx {
                        print!("(*) ");
                    }
                    let curr_to_str = self.name_by_unit(*curr_to_id);
                    print!("{curr_to_str} ");
                }
                if item.pos() == rule.to().len() {
                    print!("(*) ");
                }

                if let Some(curr_lkhds) = self.lookaheads.get(&(kernel.id(), *item)) {
                    print!("[");
                    let mut iter = curr_lkhds.iter();
                    if let Some(fst) = iter.next() {
                        let fst_str = self.name_by_unit(*fst);
                        print!("{fst_str}");
                    }
                    iter.map(|curr| self.name_by_unit(*curr)).for_each(|curr| print!(", {curr}"));
                    print!("]");
                }
                println!("")
            }

            let gotos = &self.goto[*kernel.id()];
            for (tok_id, to_kern) in gotos {
                let tok_str = self.name_by_unit(*tok_id);
                println!("  ---({tok_str})---> {to_kern:?}");
            }
        }
    }

    pub fn kernel(&self, kern_id: KernelId) -> KernEntry
    { self.as_entry(kern_id)
        .expect("Kernel {kern_id} was not initialized") }

    pub fn kernels(&self) -> impl Iterator<Item = KernEntry>
    {
        self.kerns.iter()
            .enumerate()
            .map(|(id, _)| KernEntry{ info_id: KernelId(id), owner: self })
    }

    pub fn goto(&self, kern_id: KernelId, unit: UnitId) -> Option<KernEntry>
    {
        let kern_goto = &self.goto[*kern_id];
        kern_goto.get(&unit)
            .map(|res_id| self.kernel(*res_id))
    }

    pub fn lookaheads(&self, kern_id: KernelId, item_id: ItemId) -> &HashSet<UnitId>
    { &self.lookaheads.get(&(kern_id, item_id)).unwrap() }

    pub fn initial_kern(&self) -> KernelId
    { self.initial_kern.unwrap() }

// help for previous
    fn as_entry<Id>(&self, id: Id) -> Option<RegEntry<Id, &Self>>
        where Id: IdToIdentified + Copy,
    {
        id.search(self)
            .map(|_| RegEntry{ info_id: id, owner: self })
    }
}

impl RegistryBuilder
{
    pub fn from_vecs(
        named_units: Vec<(String, UnitId)>,
        tokens: Vec<(UnitId, bool)>,
        nterms: Vec<(UnitId, Vec<RuleId>, String, bool)>,
        rules: Vec<(UnitId, Vec<UnitId>, Vec<Option<String>>, String)>
    ) -> Self
    {
        let named_units = named_units.into_iter().collect();
        let mut units: Vec<Option<GrUnit>> = (0..(tokens.len() + nterms.len()))
            .map(|_| None)
            .collect();
        for (token_id, is_eps) in tokens {
            units[*token_id] = Some(GrUnit::Tok{ id: token_id, is_eps });
        }
        for (nterm_id, rules, res_type, is_sym) in nterms {
            units[*nterm_id] = Some(GrUnit::NTerm{ id: nterm_id, rules, res_type, is_sym })
        }
        assert!(units.iter().all(|unit| unit.is_some()));

        let rules = rules.into_iter()
            .map(|(from, to, names, action)| Rule::new(from, to, names, action))
            .collect();
        let mut res = RegistryBuilder{
            named_units,
            units,
            rules,
            ..Default::default()
        };
        res.init_lalr_items()
            .map(|()| res)
            .unwrap_or_else(|err| panic!("Couldn't construct LALR items: {err}") )
    }
}

impl Default for RegistryBuilder
{
    fn default() -> Self
    {
        let mut reg = RegistryBuilder{
            named_units: HashMap::default(),
            units: vec![],
            rules: vec![],
            kerns: vec![],
            goto: vec![],
            lookaheads: HashMap::default(),
            initial_kern: None,
        };
        let eps_id = reg.set_tok("Eps").unwrap();
        let mut eps = reg.get_unit_mut(eps_id).unwrap();
        *eps = GrUnit::Tok{ id: eps_id, is_eps: true };

        let _eoi_id = reg.set_tok("EOI").unwrap();

        reg
    }
}

impl IdToIdentified for RuleId
{
    type Identified = Rule;

    fn search<'reg>(self, registry: &'reg RegistryBuilder) -> Option<&'reg Self::Identified>
    { registry.rules.get(*self) }
}

impl IdToIdentifiedMut for RuleId
{
    fn search_mut<'reg>(self, registry: &'reg mut RegistryBuilder) -> Option<&'reg mut Self::Identified>
    { registry.rules.get_mut(*self) }
}

impl IdToIdentified for UnitId
{
    type Identified = GrUnit;

    fn search<'reg>(self, registry: &'reg RegistryBuilder) -> Option<&'reg Self::Identified>
    { registry.units[*self].as_ref() }
}

impl IdToIdentifiedMut for UnitId
{
    fn search_mut<'reg>(self, registry: &'reg mut RegistryBuilder) -> Option<&'reg mut Self::Identified>
    { registry.units[*self].as_mut() }
}

impl IdToIdentified for KernelId
{
    type Identified = ItemSet<ItemId>;

    fn search<'reg>(self, registry: &'reg RegistryBuilder) -> Option<&'reg Self::Identified>
    { registry.kerns.get(*self) }
}

impl<RegRef> RegEntry<RuleId, RegRef>
    where RegRef: Borrow<RegistryBuilder>,
{
    pub fn from(&self) -> UnitEntry<'_>
    { self.owner.borrow().unit(self.from_id()) }

    pub fn owner(&self) -> &RegistryBuilder
    { self.owner.borrow() }

    pub fn is_eps_rule(&self) -> bool
    {
        let reg = self.owner();
        match self.to() {
            [to] => Some(reg.unit(*to)), // len == 1
            _ => None
        }
        .filter(|to| to.is_eps()) // and it is Eps
        .is_some()
    }
}

impl<Id, RegRef> Deref for RegEntry<Id, RegRef>
    where Id: IdToIdentified + Copy + Debug,
      RegRef: Borrow<RegistryBuilder>,
{
    type Target = Id::Identified;

    fn deref(&self) -> &Self::Target
    {
        self.info_id.clone().search(self.owner.borrow())
            .unwrap_or_else(|| panic!("Deref on RegEntry with invalid id: {:?}", self.info_id))
    }
}

impl<Id, RegRef> DerefMut for RegEntry<Id, RegRef>
    where Id: IdToIdentifiedMut + Copy + Debug,
      RegRef: BorrowMut<RegistryBuilder>,
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        self.info_id.clone().search_mut(self.owner.borrow_mut())
            .unwrap_or_else(|| panic!("DerefMut on RegEntry with invalid id: {:?}", self.info_id))
    }
}

impl<RegRef> Debug for RegEntry<UnitId, RegRef>
    where RegRef: Borrow<RegistryBuilder>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    { f.write_str(self.owner.borrow().name_by_unit(self.info_id)) }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum RegErrorKind
{
    RedefinedId(UnitId),
    UndefinedId(UnitId),
}

pub struct RegError
{
    kind: RegErrorKind,
    reg: NonNull<RegistryBuilder>,
}

impl RegError
{
    pub fn redefined_id(id: UnitId, reg: &RegistryBuilder) -> Self
    { RegError{ kind: RegErrorKind::RedefinedId(id), reg: NonNull::from(reg) } }

    pub fn undefined_id(id: UnitId, reg: &RegistryBuilder) -> Self
    { RegError{ kind: RegErrorKind::UndefinedId(id), reg: NonNull::from(reg) } }
}

impl Debug for RegError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        Debug::fmt(&self.kind, f)
    }
}

impl PartialEq for RegError
{
    fn eq(&self, other: &Self) -> bool
    { self.kind == other.kind }
}

impl Eq for RegError {}

impl Display for RegError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        use RegErrorKind::*;

        let reg = unsafe { self.reg.as_ref() };
        match self.kind {
            UndefinedId(id) => f.write_fmt(format_args!("Undefined id: {id:?} '{}'", reg.name_by_unit(id))),
            RedefinedId(id) => f.write_fmt(format_args!("Redefined id: {id:?} '{}'", reg.name_by_unit(id))),
        }
    }
}

impl Error for RegError {}
