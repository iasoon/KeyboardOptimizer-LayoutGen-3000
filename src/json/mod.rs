mod domain;
mod utils;
mod reader;


use data::Domain;
use Result;

use self::utils::JsonBuffer;
use self::reader::Reader;
use self::domain::DomainData;


pub fn read_config(path: &str) -> Result<Domain> {
    JsonBuffer::from_file(path).map(|data: DomainData| {
        data.mk_name_reader().read(data)
    })
}
