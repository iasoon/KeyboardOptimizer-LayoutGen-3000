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
    constraints: Vec<ConstraintData<'s>>,
}

#[derive(Deserialize)]

pub struct ConstraintData<'s> {
    subject: &'s str,
    object: &'s str,
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
        let constraints = self.read_vec(repr.constraints)?;
        Ok(Domain {
            keys: self.keys().map(|key_name| key_name.to_string()),
            values: self.values().map(|value_name| value_name.to_string()),
            constraints: constraints,
        })
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

impl<'s> Reader<Constraint> for NameReader<'s> {
    type Repr = ConstraintData<'s>;

    fn read(&self, repr: ConstraintData<'s>) -> Result<Constraint> {
        Ok(Constraint {
            subject: self.read(repr.subject)?,
            object: self.read(repr.object)?,
            restrictor: self.read(repr.restrictor)?,
        })
    }
}