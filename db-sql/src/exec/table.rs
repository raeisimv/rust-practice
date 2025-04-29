use crate::parser::ColumnDefinition;
use std::{
    collections::btree_map::Iter,
    collections::{BTreeMap, HashMap},
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
    pub fn iter(&self) -> Iter<'_, usize, StoredRow> {
        self.rows.iter()
    }
}
