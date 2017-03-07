use errors::*;

use std::path::Path;
use std::fs::File;

use serde::{Serialize, Deserialize};
use serde_json;


pub fn read<T>(path: &Path) -> Result<T>
    where T: Deserialize
{
    let file = File::open(path)
        .chain_err(|| "could not open file")?;
    let contents = serde_json::from_reader(file)
        .chain_err(|| "deserializaiton failed")?;
    return Ok(contents);
}

pub fn write<T>(path: &Path, data: &T) -> Result<()>
    where T: Serialize
{
    let mut file = File::create(path)
        .chain_err(|| "could not open file")?;
    serde_json::to_writer(&mut file, data)
        .chain_err(|| "serialization failed")?;
    return Ok(());
}
