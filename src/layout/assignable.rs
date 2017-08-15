use cat::*;
use data::*;

pub trait Assignable {
    fn assign(&mut self, kb_def: &KbDef, assignment: Assignment) {
        self.dispatch_assignment(kb_def, assignment);
    }

    fn dispatch_assignment(&mut self, kb_def: &KbDef, assignment: Assignment) {
        match assignment {
            Assignment::Free { free_num, loc_num } => {
                let group_num = kb_def.group_num().apply(free_num);
                let key_num = kb_def.loc_num().apply(loc_num).key_num;
                self.assign_group(group_num, key_num);

                let token_num = kb_def.frees[free_num];
                self.assign_token(token_num, loc_num);
            },
            Assignment::Lock { lock_num, key_num } => {
                let group_num = kb_def.group_num().apply(lock_num);
                self.assign_group(group_num, key_num);

                let lock_entries = &kb_def.locks[lock_num];
                for (layer_num, &value) in lock_entries.enumerate() {
                    if let Some(token_num) = value {
                        self.assign_token(token_num, kb_def.loc_num().apply(Loc {
                            key_num: key_num,
                            layer_num: layer_num,
                        }));
                    }
                }
            }
        }
    }

    fn assign_token(&mut self, _: Num<Token>, _: Num<Loc>) {}
    fn assign_group(&mut self, _: Num<Group>, _: Num<Key>) {}
}
