use std::{ops::Deref, collections::HashSet, hash::Hash, borrow::Borrow};

use super::{UnitId};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RuleId(pub usize);

impl Deref for RuleId
{
    type Target = usize;

    fn deref(&self) -> &Self::Target
    { &self.0 }
}

pub struct Rule
{
    from: UnitId,
    to: Vec<UnitId>,
    names: Vec<Option<String>>,
    action: String,
}

impl Rule
{
    pub (in crate::grammar_parser)
    fn new(from: UnitId, to: Vec<UnitId>, names: Vec<Option<String>>, action: String) -> Self
    { Rule{ from, to, names, action }}

    pub fn from_id(&self) -> UnitId
    { self.from }

    pub fn to(&self) -> &[UnitId]
    { &self.to }

    pub fn arg_names(&self) -> &[Option<String>]
    { &self.names }

    pub fn action(&self) -> &str
    { &self.action }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId
{
    id: RuleId,
    pos: usize,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KernelId(pub usize);

impl Deref for KernelId
{
    type Target = usize;

    fn deref(&self) -> &Self::Target
    { &self.0 }
}

impl ItemId
{
    pub fn begin(rule_id: RuleId, rule_len: usize) -> Self
    { ItemId{ id: rule_id, pos: 0, len: rule_len } }

    pub fn next_item(&self) -> Option<ItemId>
    {
        when! {
            !self.is_final() => Some(ItemId{ id: self.id, pos: self.pos + 1, len: self.len }),
            _ => None
        }
    }

    pub fn rule_id(&self) -> RuleId
    { self.id }

    pub fn pos(&self) -> usize
    { self.pos }

    pub fn is_final(&self) -> bool
    { self.pos >= self.len }
}

#[derive(Clone)]
pub struct ItemSet<Item>
    where Item: Borrow<ItemId>
{
    items: HashSet<Item>,
    hash: usize,
}

impl<Item> ItemSet<Item>
    where Item: Borrow<ItemId> + Hash + Eq
{
    pub fn new() -> Self
    { ItemSet{ items: HashSet::new(), hash: 0 } }

    pub fn insert(&mut self, item: Item)
    {
        let item_id = item.borrow();
        let hash_upd = *item_id.rule_id() * item_id.pos();
        if self.items.insert(item) {
            self.hash += hash_upd;
        }
    }

    pub fn len(&self) -> usize
    { self.items.len() }

    pub fn is_empty(&self) -> bool
    { self.items.is_empty() }

    pub fn contains(&self, item: ItemId) -> bool
    { self.items.contains(&item) }

    pub fn iter(&self) -> impl Iterator<Item = &Item>
    { self.items.iter() }
}

impl<Item> Default for ItemSet<Item>
    where Item: Borrow<ItemId> + Hash + Eq
{
    fn default() -> Self
    { ItemSet::new() }
}

impl<Item> PartialEq for ItemSet<Item>
    where Item: Borrow<ItemId> + Hash + Eq
{
    fn eq(&self, other: &Self) -> bool
    {
        when! {
            self.hash != other.hash => false,
            _ => self.items.len() == other.items.len() &&
                self.items.iter()
                    .all(|item| other.items.contains(item.borrow()))
        }
    }
}

impl<Item> Eq for ItemSet<Item>
    where Item: Borrow<ItemId> + Eq + Hash
{}

impl<Item> Hash for ItemSet<Item>
    where Item: Borrow<ItemId>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H)
    { self.hash.hash(state) }
}

impl<Item> FromIterator<Item> for ItemSet<Item>
    where Item: Borrow<ItemId> + Hash + Eq
{
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self
    {
        iter.into_iter()
            .fold(ItemSet::new(), |mut items, item| {
                items.insert(item);
                items
            })
    }
}

impl<Item> Extend<Item> for ItemSet<Item>
    where Item: Borrow<ItemId> + Hash + Eq
{
    fn extend<T: IntoIterator<Item = Item>>(&mut self, iter: T)
    {
        iter.into_iter()
            .for_each(|item| self.insert(item));
    }
}

impl<Item> From<Item> for ItemSet<Item>
    where Item: Borrow<ItemId> + Hash + Eq
{
    fn from(item: Item) -> Self
    {
        let mut res = ItemSet::new();
        res.insert(item);
        res
    }
}
