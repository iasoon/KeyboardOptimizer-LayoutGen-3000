use data::*;
use cat::*;
use cat::ops::*;
use std::collections::HashMap;

use super::errors::*;
use super::reader::*;

#[derive(Deserialize)]
pub struct DomainData<'s> {
    keys: Vec<&'s str>,
    values: Vec<&'s str>,
    #[serde(borrow)]
    restrictions: Vec<KeyRestrictionData<'s>>,
    #[serde(borrow)]
    constraints: Vec<ConstraintData<'s>>,
}

#[derive(Deserialize)]
pub struct KeyRestrictionData<'s> {
    key: &'s str,
    #[serde(borrow)]
    restriction: RestrictionData<'s>,
}

#[derive(Deserialize)]
pub struct ConstraintData<'s> {
    origin: &'s str,
    target: &'s str,
    #[serde(borrow)]
    restrictor: HashMap<&'s str, RestrictionData<'s>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RestrictionData<'s> {
    #[serde(borrow)]
    Not(Vec<&'s str>),
    #[serde(borrow)]
    Only(Vec<&'s str>),
}

impl<'s> DomainData<'s> {
    pub fn mk_name_reader(&self) -> NameReader<'s> {
        let keys = Table::from_vec(self.keys.clone());
        let values = Table::from_vec(self.values.clone());
        NameReader::new(keys, values)
    }
}

impl<'s> Reader<Domain> for NameReader<'s> {
    type Repr = DomainData<'s>;

    fn read(&self, repr: DomainData<'s>) -> Result<Domain> {
        let key_restrictions = self.read(repr.restrictions)?;
        let constraint_table = self.read(repr.constraints)?;
        Ok(Domain {
            keys: self.keys().map(|key_name| key_name.to_string()),
            values: self.values().map(|value_name| value_name.to_string()),
            key_restrictions,
            constraint_table,
        })
    }
}

impl<'s> Reader<Table<Key, Restriction>> for NameReader<'s> {
    type Repr = Vec<KeyRestrictionData<'s>>;

    fn read(&self, repr: Self::Repr) -> Result<Table<Key, Restriction>> {
        let mut table = self.keys().map_nums(|_| Restriction::Not(vec![]));
        let restrictions: Vec<KeyRestriction> = self.read_vec(repr)?;
        for r in restrictions.into_iter() {
            table[r.key] = r.restriction;
        }
        return Ok(table);
    }
}

impl<'s> Reader<Table<Key, Table<Key, Restrictor>>> for NameReader<'s> {
    type Repr = Vec<ConstraintData<'s>>;

    fn read(&self, repr: Self::Repr)
        -> Result<Table<Key, Table<Key, Restrictor>>>
    {
        // construct an empty constraint table
        let mut table = self.keys().map_nums(|_| {
            self.keys().map_nums(|_| None)
        });


        let constraints: Vec<Constraint> = self.read_vec(repr)?;
        for c in constraints.into_iter() {
            table[c.origin][c.target] = Some(c.restrictor);
        }
        
        return Ok(table.map_into(|row| {
            row.map_into(|entry| {
                match entry {
                    Some(restrictor) => restrictor,
                    None => self.values().map_nums(|_| {
                        Restriction::Not(vec![])
                    })
                }
            })
        }));
    }
}

impl<'s> Reader<Restriction> for NameReader<'s> {
    type Repr = RestrictionData<'s>;

    fn read(&self, repr: RestrictionData<'s>) -> Result<Restriction> {
        Ok(match repr {
            RestrictionData::Not(values) => {
                Restriction::Not(self.read_vec(values)?)
            },
            RestrictionData::Only(values) => {
                Restriction::Only(self.read_vec(values)?)
            }
        })
    }
}

impl<'s> Reader<Restrictor> for NameReader<'s> {
    type Repr = HashMap<&'s str, RestrictionData<'s>>;

    fn read(&self, repr: Self::Repr) -> Result<Restrictor> {
        let mut tbl = self.values().map_nums(|_| Restriction::Not(Vec::new()));
        for (value_name, restriction_repr) in repr.into_iter() {
            let value_num = self.read(value_name)?;
            let restriction = self.read(restriction_repr)?;
            tbl[value_num] = restriction;
        }
        Ok(tbl)
    }
}

impl<'s> Reader<KeyRestriction> for NameReader<'s> {
    type Repr = KeyRestrictionData<'s>;

    fn read(&self, repr: KeyRestrictionData<'s>) -> Result<KeyRestriction> {
        Ok(KeyRestriction {
            key: self.read(repr.key)?,
            restriction: self.read(repr.restriction)?,
        })
    }
}

impl<'s> Reader<Constraint> for NameReader<'s> {
    type Repr = ConstraintData<'s>;

    fn read(&self, repr: ConstraintData<'s>) -> Result<Constraint> {
        Ok(Constraint {
            origin: self.read(repr.origin)?,
            target: self.read(repr.target)?,
            restrictor: self.read(repr.restrictor)?,
        })
    }
}