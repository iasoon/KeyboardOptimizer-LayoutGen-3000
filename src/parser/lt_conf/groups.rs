use errors::*;
use parser::{Parser, KbParser};

use data::lt_conf::{Lock as LockData, Locks as LocksData};
use model::*;

impl<'a> Parser<Groups> for KbParser<'a> {
    type Repr = LocksData;

    fn parse(&self, repr: &LocksData) -> Result<Groups> {
        let mut reader = LocksReader::new(self);
        try!(reader.read_locks(repr));
        Ok(reader.mk_groups())
    }
}

struct LocksReader<'a> {
    parser: &'a KbParser<'a>,
    token_lock: Vec<Option<LockId>>,
    locks: Vec<Lock>,
}

impl<'a> LocksReader<'a> {
    fn new(parser: &'a KbParser<'a>) -> Self {
        LocksReader {
            token_lock: vec![None; parser.kb_conf.tokens.len()],
            locks: Vec::new(),
            parser: parser,
        }
    }

    fn mk_groups(self) -> Groups {
        let mut builder = GroupsBuilder::new(self);
        builder.fill_groups();
        return builder.groups;
    }

    fn read_locks(&mut self, data: &Vec<LockData>) -> Result<()> {
        for lock_data in data.iter() {
            try!(self.read_lock(lock_data));
        }
        Ok(())
    }

    fn read_lock(&mut self, lock_data: &LockData) -> Result<()> {
        let lock_num = self.locks.len();
        let lock = self.parse_lock(lock_data)?;
        for TokenId(token_num) in lock.members() {
            self.token_lock[token_num] = Some(LockId(lock_num));
        }
        self.locks.push(lock);
        Ok(())
    }

    fn parse_lock(&self, lock_data: &LockData) -> Result<Lock> {
        let mut vec = vec![None; self.parser.kb_conf.layers.len()];
        for (layer_name, token) in lock_data.iter() {
            let LayerId(layer_num) = self.parser.parse(layer_name)?;
            let token_id = self.parser.parse(token)?;
            vec[layer_num] = Some(token_id);
        }
        Ok(Lock::new(vec))
    }
}

struct GroupsBuilder {
    groups: Groups,
    token_lock: Vec<Option<LockId>>,
    lock_group: Vec<Option<GroupId>>,
}

impl GroupsBuilder {
    fn new<'a>(reader: LocksReader<'a>) -> Self {
        GroupsBuilder {
            token_lock: reader.token_lock,
            lock_group: vec![None; reader.locks.len()],
            groups: Groups {
                token_group: Vec::new(),
                groups: Vec::new(),
                locks: reader.locks,
                frees: Vec::new(),
            }
        }
    }

    fn fill_groups(&mut self) {
        for token_num in 0..self.token_lock.len() {
            let group_id = self.get_token_group(token_num);
            self.groups.token_group.push(group_id);
        }
    }

    fn get_token_group(&mut self, token_num: usize) -> GroupId {
        if let Some(LockId(lock_num)) = self.token_lock[token_num] {
            self.get_lock_group(lock_num)
        } else {
            self.get_free_group(token_num)
        }
    }

    fn get_lock_group(&mut self, lock_num: usize) -> GroupId {
        if let Some(group_id) = self.lock_group[lock_num] {
            return group_id;
        } else {
            let group_id = GroupId(self.groups.groups.len());
            self.groups.groups.push(Group::Locked(LockId(lock_num)));
            self.lock_group[lock_num] = Some(group_id);
            return group_id;
        }
    }

    fn get_free_group(&mut self, token_num: usize) -> GroupId {
        let free_id = FreeId(self.groups.frees.len());
        self.groups.frees.push(TokenId(token_num));

        let group_id = GroupId(self.groups.groups.len());
        self.groups.groups.push(Group::Free(free_id));

        return group_id;
    }
}
