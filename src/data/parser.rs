use std::io::Read;
use byteorder::{ReadBytesExt, NetworkEndian};

use cat::{FiniteDomain, Table};
use data::KbDef;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
    }
}

fn read_u16<R>(stream: &mut R) -> Result<u16>
    where R: Read
{
    let num = try!(stream.read_u16::<NetworkEndian>());
    return Ok(num);
}

fn read_f64<R>(stream: &mut R) -> Result<f64>
    where R: Read
{
    let num = try!(stream.read_f64::<NetworkEndian>());
    return Ok(num);
}

fn read_string<R>(stream: &mut R) -> Result<String>
    where R: Read
{
    let n_bytes = try!(read_u16(stream));
    let mut buf = vec![0x0; n_bytes as usize];
    try!(stream.read_exact(buf.as_mut_slice()));
    let string = try!(String::from_utf8(buf));
    return Ok(string);
}

fn read_vec<R, T, F>(stream: &mut R, parse_elem: F) -> Result<Vec<T>>
    where F: Fn(&mut R) -> Result<T>,
          R: Read
{
    let elem_count = try!(read_u16(stream)) as usize;
    let mut vec = Vec::with_capacity(elem_count);
    for _ in 0..elem_count {
        let elem = try!(parse_elem(stream));
        vec.push(elem);
    }
    return Ok(vec);
}

fn read_token_set<R, D>(stream: &mut R) -> Result<Table<D, String>>
    where D: FiniteDomain<Type = String>,
          R: Read
{
    let elems = try!(read_vec(stream, |stream| read_string(stream)));
    return Ok(Table::from_vec(elems));
}

fn read_config<R>(stream: &mut R) -> Result<KbDef>
    where R: Read
{
    let keys = read_token_set(stream)?;
    let layers = read_token_set(stream)?;
    let tokens = read_token_set(stream)?;
    Ok(KbDef {
        keys: keys,
        layers: layers,
        tokens: tokens,
    })
}
