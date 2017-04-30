use errors::*;
use nom::*;
use utils::Countable;

use std::vec::Vec;
use std::path::Path;
use std::fs::File;
use std::io::Read;


pub struct Assocs<C: Countable> {
    pub seq_len: usize,
    pub vec: Vec<(Vec<C>, f64)>
}

impl<C> Assocs<C>
    where C: Countable
{
    pub fn read(path: &Path, data: &C::Data) -> Result<Self> {
        let mut buf = Vec::new();
        let mut file = File::open(path)
            .chain_err(|| "could not open file")?;
        file.read_to_end(&mut buf)
            .chain_err(|| "error reading file")?;

        match parse_assocs(buf.as_slice(), data) {
            IResult::Done(_, assocs) => Ok(assocs),
            IResult::Incomplete(i) => bail!("Incomplete: {:?}", i),
            IResult::Error(e) => Err(e).chain_err(|| "parse error"),
        }
    }
}


pub fn parse_assocs<'a, C>(input: &'a [u8], data: &C::Data)
                           -> IResult<&'a [u8], Assocs<C>>
    where C: Countable
{
    do_parse!(input,
        num_assocs: le_u64 >>
        seq_len: le_u64 >>
        assocs: count!(
            tuple!(count!(apply!(countable, data), seq_len as usize), le_f64),
            num_assocs as usize) >>
        (Assocs {
            seq_len: seq_len as usize,
            vec: assocs
        })
    )
}

fn countable<'a, C>(input: &'a [u8], data: &C::Data) -> IResult<&'a [u8], C>
    where C: Countable
{
    let u8_to_countable = |num| C::from_num_checked(data, num as usize);
    map_res!(input, le_u8, u8_to_countable)
}
