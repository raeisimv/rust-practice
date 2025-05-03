use crate::parser::ColumnDefinition;
use std::{
    collections::btree_map::Iter,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

pub type StoredRow = HashMap<String, String>;
pub type ColumnInfo = Vec<ColumnDefinition>;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    rows: BTreeMap<usize, StoredRow>,
    columns: ColumnInfo,
}

impl Table {
    pub fn new(columns: ColumnInfo) -> Table {
        Self {
            rows: BTreeMap::new(),
            columns,
        }
    }
    pub fn insert(&mut self, rows: Vec<String>) -> usize {
        let id = self.rows.last_key_value().map_or(0, |x| x.0 + 1);

        let values = rows
            .iter()
            .zip(self.columns.iter())
            .map(|(v, c)| (c.name.0.clone(), v.clone()))
            .collect();
        self.rows.insert(id, values);

        id
    }
    pub fn delete(&mut self, id: usize) {
        self.rows.remove(&id);
    }
    pub fn get(&self, id: usize) -> Option<&StoredRow> {
        self.rows.get(&id)
    }
    pub fn iter(&self) -> impl Iterator<Item = Row> {
        self.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct Row<'a> {
    id: usize,
    columns: Rc<ColumnInfo>,
    values: &'a HashMap<String, String>,
}
impl<'a> Row<'a> {
    pub fn new(id: usize, columns: Rc<ColumnInfo>, values: &'a HashMap<String, String>) -> Self {
        Self {
            id,
            columns,
            values,
        }
    }
}

pub struct TableIter<'a> {
    map_iter: Iter<'a, usize, StoredRow>,
    columns: Rc<ColumnInfo>,
}

impl<'a> TableIter<'a> {
    pub fn new(map_iter: Iter<'a, usize, StoredRow>, columns: Rc<ColumnInfo>) -> Self {
        Self { map_iter, columns }
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter
            .next()
            .map(|(id, data)| Row::new(id.clone(), self.columns.clone(), data))
    }
}

impl<'a> IntoIterator for &'a Table {
    type Item = Row<'a>;

    type IntoIter = TableIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let col_info = Rc::new(self.columns.clone());
        TableIter::new(self.rows.iter(), col_info)
    }
}
