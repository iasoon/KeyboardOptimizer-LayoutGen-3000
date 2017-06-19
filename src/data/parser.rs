use std::io::Read;
use std::fs::File;
use std::path::Path;
use byteorder::{ReadBytesExt, NetworkEndian};

use cat::*;
use cat::ops::*;
use data::KbDef;
use data::types::*;

// A quick and dirty config parser will do for now

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::string::FromUtf8Error);
    }
}

fn read_u8<R>(stream: &mut R) -> Result<u8>
    where R: Read
{
    let num = try!(stream.read_u8());
    return Ok(num);
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
    let n_bytes = try!(read_u8(stream));
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

fn read_num<R, D>(stream: &mut R, count: Count<D>) -> Result<Num<D>>
    where D: FiniteDomain,
          R: Read
{
    let num = try!(read_u16(stream)) as usize;
    if num < count.as_usize() {
        Ok(internal::to_num(num))
    } else {
        Err("id out of range".into())
    }

}

fn read_count<R>(stream: &mut R) -> Result<usize>
    where R: Read
{
    let num = try!(read_u16(stream));
    return Ok(num as usize);
}

// Quick and dirty...
// eww.
fn read_kb_def<R>(stream: &mut R) -> Result<KbDef>
    where R: Read
{
    let keys = read_token_set(stream)?;
    let layers = read_token_set(stream)?;
    let tokens = read_token_set(stream)?;

    let mut assignments = Vec::new();
    // Read frees
    let num_frees = try!(read_count(stream));
    let mut frees = Vec::with_capacity(num_frees);
    for free_num in 0..num_frees {
        // read free data
        let token_num = try!(read_num(stream, tokens.count()));
        frees.push(token_num);

        // read mask
        let num_locs = try!(read_count(stream));
        for _ in 0..num_locs {
            let loc_num = try!(read_num(stream, LocNum {
                key_count: keys.count(),
                layer_count: layers.count(),
            }.count()));
            assignments.push(Assignment::Free {
                free_num: internal::to_num(free_num),
                loc_num: loc_num,
            })
        }
    }

    // Read locks
    let num_locks = try!(read_count(stream));
    let mut locks = Vec::with_capacity(num_locks);
    for lock_num in 0..num_locks {
        let mut layer_map = layers.map(|_| None);
        let num_entries = try!(read_u16(stream)) as usize;
        for _ in 0..num_entries {
            let layer_num = try!(read_num(stream, layers.count()));
            let token_num = try!(read_num(stream, tokens.count()));
            *layer_map.get_mut(layer_num) = Some(token_num);
        }
        locks.push(layer_map);

        // read mask
        let num_keys = try!(read_count(stream));
        for _ in 0..num_keys {
            let key_num = try!(read_num(stream, keys.count()));
            assignments.push(Assignment::Lock {
                lock_num: internal::to_num(lock_num),
                key_num: key_num,
            });
        }
    }

    Ok(KbDef {
        keys: keys,
        layers: layers,
        tokens: tokens,

        frees: Table::from_vec(frees),
        locks: Table::from_vec(locks),
    })
}

fn read_config<R>(stream: &mut R) -> Result<KbDef>
    where R: Read
{
    unimplemented!()
}

pub fn parse(path: &Path) -> Result<KbDef>
{
    let mut file = File::open(path)?;
    return read_config(&mut file);
}
