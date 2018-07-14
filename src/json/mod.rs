mod domain;
mod utils;
mod reader;

pub mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Json(::serde_json::Error);
        }
    }
}


use data::Domain;

use self::utils::JsonBuffer;
use self::reader::Reader;
use self::domain::DomainData;
use self::errors::*;

pub fn read_config(path: &str) -> Result<Domain> {
    JsonBuffer::from_file(path).map(|data: DomainData| {
        data.mk_name_reader().read(data)
    })
}
