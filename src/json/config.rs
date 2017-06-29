use serde::Deserialize;

use data::*;
use cat::*;
use cat::ops::*;

use json::errors::*;
use json::config_reader::ConfigReader;
use json::elements::{Elements, ElementsData};
use json::groups::{Groups, GroupsData};

#[derive(Deserialize)]
pub struct ConfigData<'a> {
    elements: ElementsData,
    #[serde(borrow)]
    groups: GroupsData<'a>,
}



impl<'a> ConfigData<'a> {
    pub fn read(self) -> Result<KbDef> {
        let elements = self.elements.read()?;
        let groups;
        {
            // use a new syntactic block here to contain this borrow
            let reader = ConfigReader::new(&elements);
            groups = self.groups.read(&reader)?
        }

        let token_group = try!(token_group(&elements, &groups));
        let assignment_map = assignment_map(&elements, &groups);
        Ok(KbDef {
            keys: elements.keys,
            layers: elements.layers,
            tokens: elements.tokens,

            frees: groups.frees,
            locks: groups.locks,

            assignments: groups.assignments,

            token_group: token_group,
            assignment_map: assignment_map,
        })
    }
}

fn token_group(elements: &Elements, groups: &Groups)
               -> Result<Table<Token, Group>>
{
    let mut map = elements.tokens.map(|_| None);

    for (free_num, &token_num) in groups.frees.enumerate() {
        *map.get_mut(token_num) = Some(Group::Free(free_num));
    }
    for (lock_num, lock) in groups.locks.enumerate() {
        for (_, &value) in lock.enumerate() {
            if let Some(token_num) = value {
                *map.get_mut(token_num) = Some(
                    Group::Lock(lock_num)
                );
            }
        }
    }
    map.map_res_with_idx(|token_num, &value| {
        if let Some(group) = value {
            Ok(group)
        } else {
            bail!("Token not assigned to a group: {}",
                  elements.tokens.get(token_num))
        }
    })

}

type AssignmentTable<T> = ComposedDict<Assignment, Num<Assignment>, T,
                                       AssignmentNum, Table<Assignment, T>>;
fn assignment_map(elements: &Elements, groups: &Groups)
                   -> AssignmentTable<Option<Num<AllowedAssignment>>>
{
    let num = AssignmentNum {
        free_count: groups.frees.count(),
        lock_count: groups.locks.count(),
        key_count: elements.keys.count(),
        layer_count: elements.layers.count(),
    };
    let mut map = num.map_nums(|_| None).compose(num);
    for (assignment_num, &assignment) in groups.assignments.enumerate() {
        *map.get_mut(assignment) = Some(assignment_num);
    }
    return map;
}
