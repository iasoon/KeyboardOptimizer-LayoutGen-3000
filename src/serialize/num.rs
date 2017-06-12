use std::fmt;

use serde::{Serialize, Serializer};
use serde::de::{self, Deserializer, DeserializeSeed, Visitor};

use cat::{FiniteDomain, Count, Num};
use cat::internal::to_num;

impl<D: FiniteDomain> Serialize for Num<D> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.as_usize() as u64)
    }
}

pub struct NumDeserializer<D: FiniteDomain> {
    count: Count<D>,
}

impl<D> NumDeserializer<D>
    where D: FiniteDomain
{
    pub fn new(count: Count<D>) -> Self {
        NumDeserializer {
            count: count,
        }
    }
}

struct NumVisitor<D: FiniteDomain> {
    count: Count<D>,
}

impl<'de, D> Visitor<'de> for NumVisitor<D>
    where D: FiniteDomain
{
    type Value = Num<D>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a numeric id")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Num<D>, E>
        where E: de::Error
    {
        if (value as usize) < self.count.as_usize() {
            Ok(to_num(value as usize))
        } else {
            Err(E::custom(format!("id out of range: {}", value)))
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Num<D>, E>
        where E: de::Error
    {
        if value >= 0 && (value as usize) < self.count.as_usize(){
            Ok(to_num(value as usize))
        } else {
            Err(E::custom(format!("id out of range: {}", value)))
        }
    }
}

impl<'de, D> DeserializeSeed<'de> for NumDeserializer<D>
    where D: FiniteDomain
{
    type Value = Num<D>;

    fn deserialize<S>(self, deserializer: S) -> Result<Self::Value, S::Error>
        where S: Deserializer<'de>
    {
        deserializer.deserialize_u64(NumVisitor {
            count: self.count,
        })
    }
}
